use axum::{extract::State, Json};
use proto::{common::node::Metrics, ctl::deployer::DeployReq};

use crate::discovery::DiscoveryHandle;

pub async fn deploy(State(state): State<DiscoveryHandle>, Json(payload): Json<DeployReq>) {
    println!("{payload:#?}");
}
