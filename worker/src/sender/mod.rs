use std::net::SocketAddr;

use proto::common::{
    instance::{self, InstanceId},
    node::Metrics,
};
use reqwest::Client;

/// Sends a http request to `ctl::http` component with body
/// containing `instance::Status` information used in `ctl::discovery`
#[derive(Clone, Debug)]
pub struct Sender {
    pub client: Client,

    /// The `ctl::http` URL should be known by the worker since the start.
    /// So it should be passed via `args` to the main function.
    pub ctl_path: SocketAddr,
}

impl Sender {
    /// Creates a `Sender` instance wrapped by an `Arc`.
    #[must_use]
    pub fn new(path: SocketAddr) -> Self {
        Sender {
            client: Client::new(),
            ctl_path: path,
        }
    }

    pub async fn send_status(&self, id: InstanceId, status: instance::Status) -> eyre::Result<()> {
        let path = format!("{:?}/deploy/status", self.ctl_path);
        let _ = self.client.post(path).json(&(&id, &status)).send().await?;
        Ok(())
    }

    pub async fn send_metrics(&self, metrics: Metrics) -> eyre::Result<()> {
        let path = format!("{:?}/worker/metrics", self.ctl_path);
        let _ = self.client.post(path).json(&metrics).send().await?;
        Ok(())
    }
}
