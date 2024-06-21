use axum::{extract::State, Json};
use proto::ctl::deployer::{
    DeployServiceReq, DeployServiceRes, ReportDeployInstanceStatusReq,
    ReportDeployInstanceStatusRes, TerminateServiceReq, TerminateServiceRes,
};
use utils::http;

use crate::http::HttpState;

pub async fn deploy_service(
    State(state): State<HttpState>,
    Json(DeployServiceReq {
        service_spec,
        // TODO: Use redeployment policy
        redeployment_policy: _,
    }): Json<DeployServiceReq>,
) -> http::Result<Json<DeployServiceRes>> {
    let res = state.deployer.deploy_service(service_spec).await?;
    Ok(Json(res))
}

pub async fn terminate_service(
    State(state): State<HttpState>,
    Json(TerminateServiceReq { service_id }): Json<TerminateServiceReq>,
) -> http::Result<Json<TerminateServiceRes>> {
    state.deployer.terminate_service(service_id).await?;
    Ok(Json(TerminateServiceRes {}))
}

pub async fn report_instance_status(
    State(state): State<HttpState>,
    Json(ReportDeployInstanceStatusReq {
        instance_id,
        status,
    }): Json<ReportDeployInstanceStatusReq>,
) -> Json<ReportDeployInstanceStatusRes> {
    state
        .deployer
        .report_instance_status(instance_id, status)
        .await;
    Json(ReportDeployInstanceStatusRes {})
}
