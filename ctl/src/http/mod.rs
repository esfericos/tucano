use axum::{routing::post, Router};
use proto::well_known::CTL_HTTP_PORT;
use utils::server;

use crate::discovery::DiscoveryHandle;

pub mod deployer;
pub mod worker;

#[derive(Clone)]
pub struct HttpState {
    pub discovery: DiscoveryHandle,
}

pub async fn run_server(state: HttpState) {
    let app = Router::new()
        .route("/worker/push-metrics", post(worker::push_metrics))
        .nest(
            "/deployer",
            Router::new()
                .route("/deploy-service", post(deployer::deploy_service))
                .route("/terminate-service", post(deployer::terminate_service))
                .route("/status", post(deployer::report_instance_status)),
        )
        .with_state(state);

    server::listen("controller http", app, ("0.0.0.0", CTL_HTTP_PORT)).await;
}
