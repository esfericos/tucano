use axum::{extract::State, Json};
use proto::ctl::deployer::{DeployId, DeployReq, DeployRes, RevisionId};
use uuid::Uuid;

use crate::http::HttpState;

pub async fn deploy(
    State(state): State<HttpState>,
    Json(payload): Json<DeployReq>,
) -> Json<DeployRes> {
    let revision_id = RevisionId(Uuid::now_v7());

    let mut deploys_id: Vec<DeployId> = Vec::new();
    for _ in 0..payload.service_spec.concurrency {
        deploys_id.push(state.discovery.schedule_deploy(revision_id).await);
    }

    tokio::spawn(async move {
        let _workers = state.discovery.query_worker().await;
        // TODO: Select worker
        // TODO: Start deployment on runner
    });

    Json(DeployRes {
        revision_id,
        deploy_ids: deploys_id,
    })
}
