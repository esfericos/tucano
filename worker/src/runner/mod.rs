use std::collections::{HashMap, HashSet};

use eyre::{Context as _, Ok, Report};
use proto::common::instance::{InstanceId, InstanceSpec};
use tokio::{
    net::TcpListener,
    sync::{mpsc, oneshot},
    task,
};

mod container_rt;

pub struct Runner {
    rx: mpsc::Receiver<Msg>,
    instances: HashMap<InstanceId, u16>,
    ports: HashSet<u16>,
    handle: RunnerHandle,
}

impl Runner {
    #[must_use]
    pub fn new() -> (Runner, RunnerHandle) {
        let (tx, rx) = mpsc::channel(16);
        let handle = RunnerHandle(tx);
        let actor = Runner {
            rx,
            instances: HashMap::default(),
            ports: HashSet::default(),
            handle: handle.clone(),
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
            Msg::InstanceDeploy(spec, reply) => {
                let res = self.instance_deploy(spec).await;
                _ = reply.send(res);
            }
            Msg::InstanceTerminate(_id, _reply) => todo!(),
            Msg::InstanceKill(_id, _report) => todo!(),
        }
    }

    async fn instance_deploy(&mut self, spec: InstanceSpec) -> eyre::Result<()> {
        let port = self.get_port_for_instance(spec.instance_id).await?;
        container_rt::spawn_instance(spec, port, self.handle.clone()).await;
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
    pub async fn send(&self, msg: Msg) {
        _ = self.0.send(msg).await;
    }

    #[allow(dead_code)]
    pub async fn deploy_instance(&self, spec: InstanceSpec) -> Result<(), Report> {
        let (tx, rx) = oneshot::channel();
        self.send(Msg::InstanceDeploy(spec, tx)).await;
        rx.await.unwrap()
    }
}

#[allow(clippy::enum_variant_names)] // remove this once more variants are added
#[allow(dead_code)]
pub enum Msg {
    InstanceDeploy(InstanceSpec, oneshot::Sender<Result<(), Report>>),
    InstanceTerminate(InstanceId, oneshot::Sender<Result<(), Report>>),
    InstanceKill(InstanceId, oneshot::Sender<Result<(), Report>>),
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
