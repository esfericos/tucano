use std::{collections::HashMap, future::Future, os::unix::net::SocketAddr, sync::Arc};

use chrono::{DateTime, Utc};
use proto::{
    clients::WorkerClient,
    common::{
        instance::{InstanceId, InstanceSpec},
        service::{ServiceId, ServiceSpec},
    },
    ctl::deployer::DeploymentId,
};
use tokio::{
    select,
    sync::{mpsc, oneshot},
    task::JoinSet,
};
use tracing::{error, info, instrument};

use crate::{deployer::instance::Transition, worker_mgr::WorkerMgrHandle};

mod alloc;
mod instance;

pub struct Deployer {
    rx: mpsc::Receiver<Msg>,
    tasks: JoinSet<()>,
    h: Arc<DeployerHandles>,
    /// Whether the deployer actor is terminating.
    terminating: bool,
    // data records
    services: HashMap<ServiceId, ServiceInfo>,
    instances: HashMap<InstanceId, instance::State>,
    deployments: HashMap<DeploymentId, DeploymentInfo>,
}

struct DeployerHandles {
    deployer_handle: DeployerHandle,
    worker_mgr: WorkerMgrHandle,
    worker_client: WorkerClient,
}

impl Deployer {
    #[must_use]
    pub fn new(
        worker_mgr: WorkerMgrHandle,
        worker_client: WorkerClient,
    ) -> (Deployer, DeployerHandle) {
        let (tx, rx) = mpsc::channel(16);
        let handle = DeployerHandle(tx);
        let actor = Deployer {
            rx,
            h: Arc::new(DeployerHandles {
                deployer_handle: handle.clone(),
                worker_mgr,
                worker_client,
            }),
            terminating: false,
            tasks: JoinSet::new(),
            services: HashMap::new(),
            instances: HashMap::new(),
            deployments: HashMap::new(),
        };
        (actor, handle)
    }

    pub async fn run(mut self) {
        loop {
            select! {
                Some(msg) = self.rx.recv() => {
                    self.handle_msg(msg).await;
                }
                // cancellation_token.cancelled() => {
                //     self.terminating = true
                // }
                maybe_task_result = self.tasks.join_next() => {
                    match maybe_task_result {
                        Some(Ok(())) => (),
                        Some(Err(error)) => {
                            error!(?error, "deployer child task panicked");
                        }
                        None if self.terminating => break,
                        None => (),
                    }
                }
            }
        }
    }

    async fn handle_msg(&mut self, msg: Msg) {
        info!(?msg, "deployer msg");
        match msg {
            Msg::DeployService(spec, reply) => {
                _ = reply.send(self.deploy_service(spec).await);
            }
            Msg::TerminateService(id, reply) => {
                _ = reply.send(self.terminate_service(id).await);
            }
            Msg::InstanceTransition(_, _) => todo!(),
        }
    }

    async fn deploy_service(&mut self, spec: ServiceSpec) -> eyre::Result<()> {
        let workers = self.h.worker_mgr.query_workers().await;
        let instances = alloc::rand_many(&workers, spec.concurrency);

        for (instance, addr) in instances {}
        Ok(())
    }

    async fn terminate_service(&mut self, id: ServiceId) -> eyre::Result<()> {
        Ok(())
    }
}

// Deployer utility functions (not message behavior)
impl Deployer {
    /// Spawns an instance tracked task.
    ///
    /// If the function returns a transition, `instance_task` automatically
    /// sends it to the deployer.
    fn instance_task<F, Fut>(&mut self, id: InstanceId, f: F)
    where
        F: FnOnce(Arc<DeployerHandles>) -> Fut + Send + 'static,
        Fut: Future<Output = Option<instance::Transition>> + Send,
    {
        let h = self.h.clone();
        self.tasks.spawn(async move {
            let deployer_handle = h.deployer_handle.clone();
            let Some(transition) = f(h).await else {
                return;
            };
            deployer_handle
                .send(Msg::InstanceTransition(id, transition))
                .await;
        });
    }
}

#[derive(Clone)]
pub struct DeployerHandle(mpsc::Sender<Msg>);

impl DeployerHandle {
    /// Sends a message.
    async fn send(&self, msg: Msg) {
        _ = self.0.send(msg).await;
    }

    /// Sends a message and waits for a reply.
    async fn send_wait<F, R>(&self, f: F) -> R
    where
        F: FnOnce(oneshot::Sender<R>) -> Msg,
    {
        let (tx, rx) = oneshot::channel();
        self.send(f(tx)).await;
        rx.await.expect("actor must be alive")
    }
}

#[derive(Debug)]
enum Msg {
    DeployService(ServiceSpec, oneshot::Sender<eyre::Result<()>>),
    TerminateService(ServiceId, oneshot::Sender<eyre::Result<()>>),
    // Internal messages
    InstanceTransition(InstanceId, Transition),
}

#[derive(Default)]
pub struct ServiceInfo {
    pub instances: Vec<InstanceId>,
    pub deployments: Vec<DeploymentInfo>,
}

impl ServiceInfo {
    pub fn last_deployment(&self) -> Option<&DeploymentInfo> {
        self.deployments.last()
    }
}

#[derive(Debug)]
pub struct DeploymentInfo {
    pub id: DeploymentId,
    pub at: DateTime<Utc>,
}
