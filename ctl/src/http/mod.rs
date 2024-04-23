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
        .route("/worker/metrics", post(worker::push_metrics))
        .route("/deploy", post(deployer::deploy))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("HTTP listening at port 3000");
    axum::serve(listener, app).await.unwrap();
}
