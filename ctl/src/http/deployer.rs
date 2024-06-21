use axum::{extract::State, Json};
use proto::ctl::deployer::{
    DeployServiceReq, DeployServiceRes, ReportDeployInstanceStatusReq,
    ReportDeployInstanceStatusRes, TerminateServiceReq, TerminateServiceRes,
};

use crate::http::HttpState;

pub async fn report_instance_status(
    State(_state): State<HttpState>,
    Json(_payload): Json<ReportDeployInstanceStatusReq>,
) -> Json<ReportDeployInstanceStatusRes> {
    todo!();
}

pub async fn deploy_service(
    State(_state): State<HttpState>,
    Json(_payload): Json<DeployServiceReq>,
) -> Json<DeployServiceRes> {
    todo!();
}

pub async fn terminate_service(
    State(_state): State<HttpState>,
    Json(_payload): Json<TerminateServiceReq>,
) -> Json<TerminateServiceRes> {
    todo!();
}
