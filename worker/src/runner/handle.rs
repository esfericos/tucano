use eyre::Report;
use proto::common::instance::{self, InstanceId, InstanceSpec};
use tokio::sync::{mpsc, oneshot};

use super::Msg;

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

    #[allow(dead_code)]
    pub async fn deploy_instance(&self, spec: InstanceSpec) -> Result<(), Report> {
        self.send_wait(|tx| Msg::DeployInstance(spec, tx)).await
    }

    #[allow(dead_code)]
    pub async fn terminate_instance(&self, id: InstanceId) -> Result<(), Report> {
        self.send_wait(|tx| Msg::TerminateInstance(id, tx)).await
    }

    pub async fn report_instance_status(&self, id: InstanceId, status: instance::Status) {
        self.send(Msg::ReportInstanceStatus(id, status)).await;
    }
}
