use std::{collections::HashMap, net::SocketAddr};

use proto::{
    common::node::Metrics,
    ctl::deployer::{DeployId, DeployStatus, RevisionId},
};
use tokio::sync::{mpsc, oneshot};
use tracing::{debug, instrument};

pub struct Discovery {
    rx: mpsc::Receiver<Msg>,
    // TODO: Add more information on workers
    workers: HashMap<SocketAddr, Metrics>,
    deploys: HashMap<DeployId, DeployDetails>,
}

impl Discovery {
    #[must_use]
    pub fn new() -> (Discovery, DiscoveryHandle) {
        let (tx, rx) = mpsc::channel(16);
        let actor = Discovery {
            rx,
            workers: HashMap::default(),
            deploys: HashMap::default(),
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
            Msg::DeploySchedule(revision_id, reply) => {
                let deploy_id = DeployId::now_v7();
                assert!(!self.deploys.contains_key(&deploy_id));
                self.deploys.insert(
                    deploy_id,
                    DeployDetails {
                        revision_id,
                        status: WorkerDeployStatus::Scheduled,
                    },
                );
                _ = reply.send(deploy_id);
            }
            Msg::DeployPushStatus(deploy_id, worker_addr, status) => {
                let Some(details) = self.deploys.get_mut(&deploy_id) else {
                    debug!(?deploy_id, "queried for unavailable deploy");
                    return;
                };
                details.status = WorkerDeployStatus::Deployed(worker_addr, status);
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

    #[allow(dead_code)] // TODO: Remove
    pub async fn schedule_deploy(&self, revision_id: RevisionId) -> DeployId {
        let (tx, rx) = oneshot::channel();
        self.send(Msg::DeploySchedule(revision_id, tx)).await;
        rx.await.expect("actor must be alive")
    }

    #[allow(dead_code)] // TODO: Remove
    pub async fn push_deploy_status(
        &self,
        deploy_id: DeployId,
        worker_addr: SocketAddr,
        status: DeployStatus,
    ) {
        self.send(Msg::DeployPushStatus(deploy_id, worker_addr, status))
            .await;
    }
}

#[derive(Debug)]
enum Msg {
    WorkerAdd(SocketAddr, Metrics),
    WorkerDrop(SocketAddr),
    WorkerQuery(oneshot::Sender<Vec<WorkerDetails>>),

    DeploySchedule(RevisionId, oneshot::Sender<DeployId>),
    DeployPushStatus(DeployId, SocketAddr, DeployStatus),
}

#[derive(Debug)]
pub struct DeployDetails {
    pub revision_id: RevisionId,
    pub status: WorkerDeployStatus,
}

#[derive(Debug)]
pub enum WorkerDeployStatus {
    /// Deployment is scheduled (not yet in progress).
    Scheduled,
    /// Service is being deployed or running at a given node.
    Deployed(SocketAddr, DeployStatus),
}

#[derive(Debug)]
pub struct WorkerDetails {
    pub addr: SocketAddr,
    pub metrics: Metrics,
}
