use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use bollard::Docker;
use container_rt::ContainerRuntime;
use eyre::{Context as _, Ok, Report};
use proto::{
    clients::CtlClient,
    common::instance::{self, InstanceId, InstanceSpec},
};
use tokio::{
    net::TcpListener,
    sync::{mpsc, oneshot},
    task,
};
use tracing::{error, trace};

mod container_rt;
use crate::proxy::ProxyHandle;

mod handle;
pub use handle::RunnerHandle;

pub struct Runner {
    rx: mpsc::Receiver<Msg>,
    instances: HashMap<InstanceId, u16>,
    ports: HashSet<u16>,
    handle: RunnerHandle,
    proxy_handle: ProxyHandle,
    container_runtime: Arc<ContainerRuntime>,
    ctl_client: CtlClient,
}

impl Runner {
    #[must_use]
    pub fn new(
        docker: Arc<Docker>,
        ctl_client: CtlClient,
        proxy: ProxyHandle,
    ) -> (Runner, RunnerHandle) {
        let (tx, rx) = mpsc::channel(16);
        let handle = RunnerHandle(tx);
        let actor = Runner {
            rx,
            instances: HashMap::default(),
            ports: HashSet::default(),
            handle: handle.clone(),
            proxy_handle: proxy,
            container_runtime: Arc::new(ContainerRuntime::new(docker)),
            ctl_client,
        };
        (actor, handle)
    }

    pub async fn run(mut self) {
        while let Some(msg) = self.rx.recv().await {
            self.handle_msg(msg).await;
        }
    }

    async fn handle_msg(&mut self, msg: Msg) {
        match msg {
            Msg::DeployInstance(spec, reply) => {
                let res = self.deploy_instance(spec).await;
                _ = reply.send(res);
            }
            Msg::TerminateInstance(id, reply) => {
                let res = self.terminate_instance(id);
                _ = reply.send(res);
            }
            Msg::ReportInstanceStatus(id, status) => {
                self.report_instance_status(id, status);
            }
        }
    }

    async fn deploy_instance(&mut self, spec: InstanceSpec) -> eyre::Result<()> {
        let port = self.get_port_for_instance(spec.instance_id).await?;
        let rt = self.container_runtime.clone();
        let handle = self.handle.clone();
        tokio::spawn(async move {
            rt.run_instance_lifecycle(spec, port, handle).await;
        });
        Ok(())
    }

    fn terminate_instance(&mut self, id: InstanceId) -> eyre::Result<()> {
        let rt = self.container_runtime.clone();
        tokio::spawn(async move {
            rt.terminate_instance(id).await;
        });
        Ok(())
    }

    fn report_instance_status(&mut self, instance_id: InstanceId, status: instance::Status) {
        use instance::Status::*;
        match &status {
            Started => (),
            Terminated => self.remove_instance(instance_id),
            Crashed { error: _ } | Killed { reason: _ } | FailedToStart { error: _ } => {
                self.remove_instance(instance_id);
            }
        }

        let ctl_client = self.ctl_client.clone();
        tokio::spawn(async move {
            trace!(?instance_id, ?status, "reporting status");
            if let Err(error) = ctl_client.report_instance_status(instance_id, status).await {
                error!(?error, "failed to report instance status");
            }
        });
    }

    async fn get_port_for_instance(&mut self, id: InstanceId) -> eyre::Result<u16> {
        let port = loop {
            let port = get_port().await?;
            if !self.ports.contains(&port) {
                break port;
            }
        };
        self.instances.insert(id, port);
        self.ports.insert(port);
        self.proxy_handle.add_instance(id, port);
        Ok(port)
    }

    fn remove_instance(&mut self, id: InstanceId) {
        let freed_port = self.instances.remove(&id).unwrap();
        self.ports.remove(&freed_port);
        self.proxy_handle.remove_instance(id);
    }
}

#[allow(dead_code)]
pub enum Msg {
    DeployInstance(InstanceSpec, oneshot::Sender<Result<(), Report>>),
    TerminateInstance(InstanceId, oneshot::Sender<Result<(), Report>>),
    /// Sends a report to `ctl::http` component regarding current
    /// instance status. Furthermore updating discovery
    ReportInstanceStatus(InstanceId, instance::Status),
}

async fn get_port() -> eyre::Result<u16> {
    let listener = TcpListener::bind(("0.0.0.0", 0))
        .await
        .wrap_err("failed to bind while deciding port")?;
    let port = listener.local_addr().expect("must have local_addr").port();
    drop(listener);
    task::yield_now().await;
    Ok(port)
}
