#![allow(dead_code)]

use std::{collections::HashMap, net::SocketAddr};

use chrono::{DateTime, Utc};
use proto::{
    common::{instance::InstanceId, node::Metrics, service::ServiceId},
    ctl::deployer::DeploymentId,
};
use tokio::sync::{mpsc, oneshot};
use tracing::instrument;

pub struct Discovery {
    rx: mpsc::Receiver<Msg>,
    // TODO: Add more information on workers
    workers: HashMap<SocketAddr, Metrics>,
    services: HashMap<ServiceId, ServiceInfo>,
    instances: HashMap<InstanceId, InstanceInfo>,
    deployments: HashMap<DeploymentId, DeploymentInfo>,
}

impl Discovery {
    #[must_use]
    pub fn new() -> (Discovery, DiscoveryHandle) {
        let (tx, rx) = mpsc::channel(16);
        let actor = Discovery {
            rx,
            workers: HashMap::default(),
            services: HashMap::default(),
            instances: HashMap::default(),
            deployments: HashMap::default(),
        };
        let handle = DiscoveryHandle(tx);
        (actor, handle)
    }

    pub async fn run(mut self) {
        while let Some(msg) = self.rx.recv().await {
            // Attention to back pressure.
            self.handle_msg(msg).await;
        }
    }

    #[instrument(skip(self))]
    async fn handle_msg(&mut self, msg: Msg) {
        match msg {
            Msg::WorkerAdd(addr, metrics) => {
                self.workers.insert(addr, metrics);
            }
            Msg::WorkerDrop(addr) => {
                self.workers.remove(&addr);
            }
            Msg::WorkerQuery(reply) => {
                let entries = self
                    .workers
                    .iter()
                    .map(|(&addr, metrics)| WorkerDetails {
                        addr,
                        metrics: metrics.clone(),
                    })
                    .collect();
                _ = reply.send(entries);
            }
        }
    }
}

#[derive(Clone)]
pub struct DiscoveryHandle(mpsc::Sender<Msg>);

impl DiscoveryHandle {
    async fn send(&self, msg: Msg) {
        _ = self.0.send(msg).await;
    }

    #[allow(dead_code)] // TODO: Remove
    pub async fn add_worker(&self, addr: SocketAddr, metrics: Metrics) {
        self.send(Msg::WorkerAdd(addr, metrics)).await;
    }

    #[allow(dead_code)] // TODO: Remove
    pub async fn drop_worker(&self, addr: SocketAddr) {
        self.send(Msg::WorkerDrop(addr)).await;
    }

    #[allow(dead_code)] // TODO: Remove
    pub async fn query_worker(&self) -> Vec<WorkerDetails> {
        let (tx, rx) = oneshot::channel();
        self.send(Msg::WorkerQuery(tx)).await;
        rx.await.expect("actor must be alive")
    }
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)] // remove this once more variants are added
enum Msg {
    WorkerAdd(SocketAddr, Metrics),
    WorkerDrop(SocketAddr),
    WorkerQuery(oneshot::Sender<Vec<WorkerDetails>>),
    // TODO: add service and instance operations
}

// services: HashMap<ServiceId, ServiceInfo>,
// instances: HashMap<InstanceId, InstanceInfo>,

#[derive(Default)]
pub struct ServiceInfo {
    pub instances: Vec<InstanceId>,
}

#[derive(Default)]
pub struct InstanceInfo {
    pub state: InstanceState,
    pub deployment_id: DeploymentId,
}

#[derive(Copy, Clone, Default, Debug)]
pub enum InstanceState {
    #[default]
    Idle,
    #[allow(dead_code)]
    Running,
    #[allow(dead_code)]
    Terminated,
    #[allow(dead_code)]
    Crashed,
    #[allow(dead_code)]
    Killed,
}

#[derive(Default)]
pub struct DeploymentInfo {
    pub deployed_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct WorkerDetails {
    pub addr: SocketAddr,
    pub metrics: Metrics,
}
