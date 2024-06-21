use std::{collections::HashMap, future::Future, net::IpAddr, sync::Arc};

use proto::{
    clients::WorkerClient,
    common::{
        instance::{self as proto_instance, InstanceId, InstanceSpec},
        service::{ServiceId, ServiceSpec},
    },
    ctl::deployer::{DeployServiceRes, DeploymentId},
};
use tokio::{
    select,
    sync::{mpsc, oneshot},
    task::JoinSet,
};
use tracing::{debug, error, instrument, warn};
use uuid::Uuid;

use crate::{
    deployer::instance::{TerminalKind, Transition},
    worker_mgr::WorkerMgrHandle,
};

mod alloc;
mod instance;

pub struct Deployer {
    rx: mpsc::Receiver<Msg>,
    h: Arc<DeployerHandles>,
    /// Set of deployer-related background-running tasks.
    tasks: JoinSet<()>,
    /// Pending deployment state machine contexts.
    _deployment_statems: HashMap<DeploymentId, u8 /* todo */>,
    /// Instance state machine contexts.
    instance_statems: HashMap<InstanceId, instance::StateCtx>,
    /// Whether the deployer actor is terminating.
    terminating: bool,
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
            tasks: JoinSet::new(),
            _deployment_statems: HashMap::new(),
            instance_statems: HashMap::new(),
            terminating: false,
        };
        (actor, handle)
    }

    #[allow(clippy::match_same_arms)]
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

    #[instrument(skip_all)]
    async fn handle_msg(&mut self, msg: Msg) {
        match msg {
            Msg::DeployService(spec, reply) => {
                _ = reply.send(self.handle_deploy_service(spec).await);
            }
            Msg::TerminateService(id, reply) => {
                self.handle_terminate_service(&id);
                _ = reply.send(Ok(()));
            }
            Msg::ReportInstanceStatus(id, status) => {
                self.trans_instance_state(id, instance::Transition::Status(status));
            }
            Msg::InstanceTransition(id, t) => {
                self.trans_instance_state(id, t);
            }
        }
    }

    async fn handle_deploy_service(&mut self, spec: ServiceSpec) -> eyre::Result<DeployServiceRes> {
        debug!(?spec, "deploying service");
        let workers = self.h.worker_mgr.query_workers().await;
        let instances = alloc::rand_many(&workers, spec.concurrency);
        let deployment_id = DeploymentId(Uuid::now_v7());

        let instances = instances
            // For each allocated instance, schedule a deploy.
            .inspect(|&(instance_id, worker_addr)| {
                self.add_instance_init_state(instance_id, worker_addr, deployment_id);

                let spec = InstanceSpec::from_service_spec_cloned(&spec, instance_id).into();
                self.trans_instance_state(instance_id, Transition::Deploy { spec });
            })
            .collect();

        Ok(DeployServiceRes {
            deployment_id,
            instances,
        })
    }

    fn handle_terminate_service(&mut self, _id: &ServiceId) {
        _ = self;
    }

    fn _lffg_todo_deploy_service(&mut self) {
        _ = self;
    }

    fn add_instance_init_state(&mut self, id: InstanceId, worker_addr: IpAddr, d_id: DeploymentId) {
        let opt = self
            .instance_statems
            .insert(id, instance::StateCtx::new_init(id, worker_addr, d_id));

        // We have just generated a new ID (in Self::handle_deploy_service), so
        // this case shouldn't be possible.
        assert!(opt.is_none());
    }

    fn trans_instance_state(&mut self, id: InstanceId, t: instance::Transition) {
        let Some(statem) = self.instance_statems.remove(&id) else {
            warn!("tried to transition nonexistent instance machine");
            return;
        };

        let next = instance::next(self, statem, t);
        match next.state().kind() {
            TerminalKind::NonTerminal => {
                self.instance_statems.insert(id, next);
            }
            // If the new state is terminal, we don't need to waste memory by
            // keeping track of it, so we don't add it again.
            TerminalKind::SuccessfulTerminal | TerminalKind::UnsuccessfulTerminal => (),
        }
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

    pub async fn deploy_service(&self, spec: ServiceSpec) -> eyre::Result<DeployServiceRes> {
        self.send_wait(|r| Msg::DeployService(spec, r)).await
    }

    pub async fn terminate_service(&self, id: ServiceId) -> eyre::Result<()> {
        self.send_wait(|r| Msg::TerminateService(id, r)).await
    }

    pub async fn report_instance_status(&self, id: InstanceId, status: proto_instance::Status) {
        self.send(Msg::ReportInstanceStatus(id, status)).await;
    }
}

#[derive(Debug)]
enum Msg {
    DeployService(ServiceSpec, oneshot::Sender<eyre::Result<DeployServiceRes>>),
    TerminateService(ServiceId, oneshot::Sender<eyre::Result<()>>),
    ReportInstanceStatus(InstanceId, proto_instance::Status),
    // Internal messages
    InstanceTransition(InstanceId, Transition),
}

/*
================================================================================
(TODO: DATA RECORDS)

    // data records {
services: HashMap<ServiceId, ServiceInfo>,
instances: HashMap<InstanceId, instance::State>,
deployments: HashMap<DeploymentId, DeploymentInfo>,
                    }

lista de serviços rodando

show (service id)
-> instâncias rodando (e os respectivos deployments)
ShowResponse {
  deployments: Vec<(DeploymentId, Vec<(InstanceId, InstanceState)>)>
}

struct DeploymentStateCtx {
    service_id: ServiceId,
    id: DeploymentId,
    // instance_deployment_results: Vec<_>
    // state: State,
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
    pub alive_instances: InstanceId,
}
================================================================================
*/
