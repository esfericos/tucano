use axum::{extract::State, response::IntoResponse, Json};
use proto::worker::runner::{DeployInstanceReq, DeployInstanceRes, TerminateReq, TerminateRes};
use reqwest::StatusCode;
use setup::http;

use crate::http::HttpState;

pub async fn deploy_instance(
    State(state): State<HttpState>,
    Json(payload): Json<DeployInstanceReq>,
) -> http::Result<impl IntoResponse> {
    state.runner.deploy_instance(payload.instance_spec).await?;
    Ok((StatusCode::ACCEPTED, Json(DeployInstanceRes {})))
}

pub async fn terminate_instance(
    State(state): State<HttpState>,
    Json(payload): Json<TerminateReq>,
) -> http::Result<impl IntoResponse> {
    state.runner.terminate_instance(payload.instance_id).await?;
    Ok((StatusCode::ACCEPTED, Json(TerminateRes {})))
}
