use axum::{extract::State, Json};
use proto::worker::runner::{DeployInstanceReq, DeployInstanceRes};

use crate::http::HttpState;

pub async fn new_instance(
    State(state): State<HttpState>,
    Json(_payload): Json<DeployInstanceReq>,
) -> Json<DeployInstanceRes> {
    _ = state.runner;
    todo!();
}
