use axum::{extract::State, Json};
use proto::worker::runner::{DeployInstanceReq, DeployInstanceRes, TerminateReq, TerminateRes};
use setup::http;

use crate::http::HttpState;

pub async fn deploy_instance(
    State(state): State<HttpState>,
    Json(payload): Json<DeployInstanceReq>,
) -> http::Result<Json<DeployInstanceRes>> {
    state.runner.deploy_instance(payload.instance_spec).await?;
    Ok(Json(DeployInstanceRes {}))
}

pub async fn terminate_instance(
    State(state): State<HttpState>,
    Json(payload): Json<TerminateReq>,
) -> http::Result<Json<TerminateRes>> {
    state.runner.terminate_instance(payload.instance_id).await?;
    Ok(Json(TerminateRes {}))
}
