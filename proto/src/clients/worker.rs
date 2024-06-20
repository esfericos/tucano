use std::net::IpAddr;

use crate::{
    clients::BaseClient,
    common::instance::{InstanceId, InstanceSpec},
    well_known::WORKER_HTTP_PORT,
    worker::runner::{
        DeployInstanceReq, DeployInstanceRes, TerminateInstanceReq, TerminateInstanceRes,
    },
};

#[derive(Clone)]
pub struct WorkerClient {
    client: BaseClient,
}

impl WorkerClient {
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let client = BaseClient::new();
        WorkerClient { client }
    }

    #[allow(clippy::unused_self)]
    fn url(&self, worker: IpAddr, path: &str) -> String {
        assert!(path.starts_with('/'));
        format!("http://{worker}:{WORKER_HTTP_PORT}{path}")
    }

    pub async fn deploy_instance(
        &self,
        worker: IpAddr,
        instance_spec: InstanceSpec,
    ) -> eyre::Result<DeployInstanceRes> {
        let body = DeployInstanceReq { instance_spec };
        self.client
            .send(self.url(worker, "/runner/deploy-instance"), &body)
            .await
    }

    pub async fn terminate_instance(
        &self,
        worker: IpAddr,
        instance_id: InstanceId,
    ) -> eyre::Result<TerminateInstanceRes> {
        let body = TerminateInstanceReq { instance_id };
        self.client
            .send(self.url(worker, "/runner/terminate-instance"), &body)
            .await
    }
}
