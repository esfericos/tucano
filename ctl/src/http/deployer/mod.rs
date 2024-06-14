use axum::{extract::State, Json};
use proto::ctl::deployer::{DeployReq, DeployRes};

use crate::http::HttpState;

pub async fn deploy(
    State(state): State<HttpState>,
    Json(_payload): Json<DeployReq>,
) -> Json<DeployRes> {
    _ = state.discovery;
    todo!();
}
