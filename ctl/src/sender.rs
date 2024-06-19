use std::net::{IpAddr, SocketAddr};

use eyre::Context;
use proto::{
    common::instance::{InstanceId, InstanceSpec},
    worker::runner::{
        DeployInstanceReq, DeployInstanceRes, TerminateInstanceReq, TerminateInstanceRes,
    },
};
use reqwest::{Client, IntoUrl};

use proto::well_known::WORKER_HTTP_PORT as WH_PORT;
use serde::{de::DeserializeOwned, Serialize};

#[derive(Clone)]
pub struct WorkerSender {
    client: Client,
}

impl WorkerSender {
    #[must_use]
    pub fn new() -> Self {
        let client = Client::new();
        Self { client }
    }

    pub async fn deploy_instance(
        &self,
        worker: IpAddr,
        spec: InstanceSpec,
    ) -> eyre::Result<DeployInstanceRes> {
        let body = DeployInstanceReq {
            instance_spec: spec,
        };
        self.send(worker, "/instance/deploy", &body).await
    }

    pub async fn terminate_instance(
        &self,
        worker: IpAddr,
        id: InstanceId,
    ) -> eyre::Result<TerminateInstanceRes> {
        let body = TerminateInstanceReq { instance_id: id };
        self.send(worker, "/instance/terminate", &body).await
    }

    /// Sends a request to the given path, on the given worker.
    ///
    /// Paths must start with a `/`.
    async fn send<Req, Res>(
        &self,
        worker: IpAddr,
        path: &'static str,
        body: &Req,
    ) -> eyre::Result<Res>
    where
        Req: Serialize,
        Res: DeserializeOwned,
    {
        assert!(path.starts_with("/"));
        let url = format!("http://{worker}:{WH_PORT}{path}");
        let res = self
            .client
            .post(&url)
            .json(body)
            .send()
            .await
            .wrap_err("failed to send request to worker")?
            .json::<Res>()
            .await
            .wrap_err("failed to parse response from worker")?;
        Ok(res)
    }
}
