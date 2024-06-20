use axum::{routing::post, Router};

use crate::worker_mgr::WorkerMgrHandle;

pub mod deployer;
pub mod worker;

#[derive(Clone)]
pub struct HttpState {
    pub worker_mgr: WorkerMgrHandle,
}

pub fn mk_app(state: HttpState) -> Router {
    Router::new()
        .nest(
            "/worker",
            Router::new()
                .route("/hello", post(worker::hello))
                .route("/bye", post(worker::bye))
                .route("/push-metrics", post(worker::push_metrics)),
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
