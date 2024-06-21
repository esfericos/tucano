use axum::{routing::post, Router};

use crate::{deployer::DeployerHandle, worker_mgr::WorkerMgrHandle};

pub mod deployer;
pub mod worker_mgr;

#[derive(Clone)]
pub struct HttpState {
    pub worker_mgr: WorkerMgrHandle,
    pub deployer: DeployerHandle,
}

pub fn mk_app(state: HttpState) -> Router {
    Router::new()
        .nest(
            "/worker",
            Router::new()
                .route("/hello", post(worker_mgr::hello))
                .route("/bye", post(worker_mgr::bye))
                .route("/push-metrics", post(worker_mgr::push_metrics))
                .route("/query", post(worker_mgr::query_workers)),
        )
        .nest(
            "/deployer",
            Router::new()
                .route("/deploy-service", post(deployer::deploy_service))
                .route("/terminate-service", post(deployer::terminate_service))
                .route("/status", post(deployer::report_instance_status)),
        )
        .with_state(state)
}
