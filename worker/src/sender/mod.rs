use std::{net::SocketAddr, sync::Arc};

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
    pub http_path: SocketAddr,
}

impl Sender {
    /// Creates a `Sender` instance wrapped by an `Arc`.
    #[must_use]
    pub fn new(path: SocketAddr) -> Arc<Self> {
        Arc::new(Sender {
            client: Client::new(),
            http_path: path,
        })
    }
    /// The `ctl::http` URL should be known by the worker since the start.
    /// So it should be passed via `args` to the main function.  
    pub async fn send_status(&self, id: InstanceId, status: instance::Status) -> eyre::Result<()> {
        let path = format!("{:?}/deploy/status", self.http_path);
        let _ = self.client.post(path).json(&(&id, &status)).send().await?;
        Ok(())
    }

    pub async fn send_metrics(&self, metrics: Metrics) -> eyre::Result<()> {
        let path = format!("{:?}/worker/metrics", self.http_path);
        let _ = self.client.post(path).json(&metrics).send().await?;
        Ok(())
    }
}
