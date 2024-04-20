use axum::Json;
use proto::{common::node::Metrics, ctl::deployer::DeployReq};

pub async fn deploy(Json(payload): Json<DeployReq>) {
    println!("{payload:#?}");
}
