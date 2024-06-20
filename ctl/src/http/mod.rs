use axum::{routing::post, Router};
use tracing::info;

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

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("HTTP listening at port 3000");
    axum::serve(listener, app).await.unwrap();
}
