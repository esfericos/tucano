use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use bollard::Docker;
use eyre::{Context as _, Ok, Report};
use proto::common::instance::{self, InstanceId, InstanceSpec};
use tokio::{
    net::TcpListener,
    sync::{mpsc, oneshot},
    task,
};

mod container_rt;
use container_rt::ContainerRuntime;

pub struct Runner {
    rx: mpsc::Receiver<Msg>,
    instances: HashMap<InstanceId, u16>,
    ports: HashSet<u16>,
    handle: RunnerHandle,
    container_runtime: ContainerRuntime,
}

impl Runner {
    #[must_use]
    pub fn new(docker: Arc<Docker>) -> (Runner, RunnerHandle) {
        let (tx, rx) = mpsc::channel(16);
        let handle = RunnerHandle(tx);
        let actor = Runner {
            rx,
            instances: HashMap::default(),
            ports: HashSet::default(),
            handle: handle.clone(),
            container_runtime: ContainerRuntime::new(docker),
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
                let res = self.instance_deploy(spec).await;
                _ = reply.send(res);
            }
            Msg::TerminateInstance(_id, _reply) => todo!(),
            Msg::KillInstance(_id, _report) => todo!(),
            Msg::ReportInstanceStatus(_) => todo!(),
        }
    }

    async fn instance_deploy(&mut self, spec: InstanceSpec) -> eyre::Result<()> {
        let port = self.get_port_for_instance(spec.instance_id).await?;
        self.container_runtime
            .spawn_instance(spec, port, self.handle.clone());
        Ok(())
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
        Ok(port)
    }
}

#[derive(Clone)]
pub struct RunnerHandle(mpsc::Sender<Msg>);

impl RunnerHandle {
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub async fn deploy_instance(&self, spec: InstanceSpec) -> Result<(), Report> {
        self.send_wait(|tx| Msg::DeployInstance(spec, tx)).await
    }

    #[allow(dead_code)]
    pub async fn terminate_instance(&self, id: InstanceId) -> Result<(), Report> {
        self.send_wait(|tx| Msg::TerminateInstance(id, tx)).await
    }

    #[allow(dead_code)]
    pub async fn kill_instance(&self, id: InstanceId) -> Result<(), Report> {
        self.send_wait(|tx| Msg::KillInstance(id, tx)).await
    }

    pub async fn report_instance_status(&self, status: instance::Status) {
        self.send(Msg::ReportInstanceStatus(status)).await;
    }
}

#[allow(dead_code)]
pub enum Msg {
    DeployInstance(InstanceSpec, oneshot::Sender<Result<(), Report>>),
    TerminateInstance(InstanceId, oneshot::Sender<Result<(), Report>>),
    KillInstance(InstanceId, oneshot::Sender<Result<(), Report>>),
    ReportInstanceStatus(instance::Status),
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
