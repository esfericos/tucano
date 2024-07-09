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
use crate::{args::WorkerArgs, proxy::ProxyHandle};

pub struct Runner {
    rx: mpsc::Receiver<Msg>,
    instances: HashMap<InstanceId, u16>,
    ports: HashSet<u16>,
    handle: RunnerHandle,
    proxy_handle: ProxyHandle,
    worker_args: Arc<WorkerArgs>,
    container_runtime: Arc<ContainerRuntime>,
    ctl_client: CtlClient,
}

impl Runner {
    #[must_use]
    pub fn new(
        worker_args: Arc<WorkerArgs>,
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
            worker_args,
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
        let port = self.get_available_instance_port().await?;
        self.add_instance(spec.instance_id, port);

        let rt = self.container_runtime.clone();
        let args = self.worker_args.clone();
        let handle = self.handle.clone();
        tokio::spawn(async move {
            rt.run_instance_lifecycle(args, spec, port, handle).await;
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
            Terminated | Crashed { .. } | Killed { .. } | FailedToStart { .. } => {
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

    async fn get_available_instance_port(&mut self) -> eyre::Result<u16> {
        let port = loop {
            let port = get_available_port().await?;
            if !self.ports.contains(&port) {
                break port;
            }
        };
        Ok(port)
    }

    fn add_instance(&mut self, id: InstanceId, port: u16) {
        self.instances.insert(id, port);
        self.ports.insert(port);
        self.proxy_handle.add_instance(id, port);
    }

    fn remove_instance(&mut self, id: InstanceId) {
        let freed_port = self.instances.remove(&id).unwrap();
        self.ports.remove(&freed_port);
        self.proxy_handle.remove_instance(id);
    }
}

#[derive(Clone)]
pub struct RunnerHandle(pub mpsc::Sender<Msg>);

impl RunnerHandle {
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

    pub async fn deploy_instance(&self, spec: InstanceSpec) -> Result<(), Report> {
        self.send_wait(|tx| Msg::DeployInstance(spec, tx)).await
    }

    pub async fn terminate_instance(&self, id: InstanceId) -> Result<(), Report> {
        self.send_wait(|tx| Msg::TerminateInstance(id, tx)).await
    }

    pub async fn report_instance_status(&self, id: InstanceId, status: instance::Status) {
        self.send(Msg::ReportInstanceStatus(id, status)).await;
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

async fn get_available_port() -> eyre::Result<u16> {
    let listener = TcpListener::bind(("0.0.0.0", 0))
        .await
        .wrap_err("failed to bind while deciding port")?;
    let port = listener.local_addr().expect("must have local_addr").port();
    drop(listener);
    task::yield_now().await;
    Ok(port)
}
