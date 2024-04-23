use axum::{extract::State, Json};
use proto::ctl::deployer::{DeployId, DeployReq, DeployRes, RevisionId};

use crate::discovery::DiscoveryHandle;

pub async fn deploy(
    State(discovery_handle): State<DiscoveryHandle>,
    Json(payload): Json<DeployReq>,
) -> Json<DeployRes> {
    let revision_id = RevisionId::now_v7();

    let mut deploys_id: Vec<DeployId> = Vec::new();
    for _i in 0..payload.service_spec.concurrency {
        deploys_id.push(discovery_handle.schedule_deploy(revision_id).await);
    }

    tokio::spawn(async move {
        let _workers = discovery_handle.query_worker().await;
        // TODO: Select worker
        // TODO: Start deployment on runner
    });

    Json(DeployRes {
        revision_id,
        deploy_ids: deploys_id,
    })
}
