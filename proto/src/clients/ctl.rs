use std::{net::SocketAddr, sync::Arc};

use chrono::{DateTime, Utc};

use crate::{
    clients::BaseClient,
    common::{
        instance::{self, InstanceId},
        node::Metrics,
        service::{ServiceId, ServiceSpec},
    },
    ctl::{
        deployer::{
            DeployServiceReq, DeployServiceRes, RedeploymentPolicy, ReportDeployInstanceStatusReq,
            ReportDeployInstanceStatusRes, TerminateServiceReq, TerminateServiceRes,
        },
        worker::{PushWorkerMetricsReq, PushWorkerMetricsRes},
    },
};

#[derive(Clone)]
pub struct CtlClient {
    base_url: Arc<str>,
    client: BaseClient,
}

impl CtlClient {
    #[must_use]
    pub fn new(ctl_addr: SocketAddr) -> Self {
        let base_url = format!("http://{ctl_addr}").into_boxed_str().into();
        let client = BaseClient::new();
        CtlClient { base_url, client }
    }

    fn url(&self, path: &str) -> String {
        assert!(path.starts_with('/'));
        format!("{base}{path}", base = self.base_url)
    }

    pub async fn push_metrics(
        &self,
        metrics: Metrics,
        recorded_at: DateTime<Utc>,
    ) -> eyre::Result<PushWorkerMetricsRes> {
        let body = PushWorkerMetricsReq {
            metrics,
            recorded_at,
        };
        self.client
            .send(self.url("/worker/push-metrics"), &body)
            .await
    }

    pub async fn deploy_service(
        &self,
        service_spec: ServiceSpec,
        redeployment_policy: RedeploymentPolicy,
    ) -> eyre::Result<DeployServiceRes> {
        let body = DeployServiceReq {
            service_spec,
            redeployment_policy,
        };
        self.client
            .send(self.url("/deployer/deploy-service"), &body)
            .await
    }

    pub async fn terminate_service(
        &self,
        service_id: ServiceId,
    ) -> eyre::Result<TerminateServiceRes> {
        let body = TerminateServiceReq { service_id };
        self.client
            .send(self.url("/deployer/terminate-service"), &body)
            .await
    }

    pub async fn report_instance_status(
        &self,
        instance_id: InstanceId,
        status: instance::Status,
    ) -> eyre::Result<ReportDeployInstanceStatusRes> {
        let body = ReportDeployInstanceStatusReq {
            instance_id,
            status,
        };
        self.client.send(self.url("/deployer/status"), &body).await
    }
}
