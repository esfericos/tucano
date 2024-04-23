use axum::{extract::State, Json};
use proto::{
    common::node::Metrics,
    ctl::deployer::{DeployId, DeployReq, DeployRes, RevisionId},
};
use uuid::Uuid;

use crate::discovery::DiscoveryHandle;

pub async fn deploy(
    State(discovery_handle): State<DiscoveryHandle>,
    Json(payload): Json<DeployReq>,
) -> Json<DeployRes> {
    println!("{payload:#?}");

    let revision_id = RevisionId::now_v7();

    discovery_handle.schedule_deploy(revision_id);

    Json(DeployRes {
        revision_id,
        deploy_ids: Vec::from([DeployId::now_v7()]),
    })
}
