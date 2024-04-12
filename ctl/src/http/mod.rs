use axum::{routing::post, Router};
use tracing::info;

pub mod worker;

pub async fn run_server() {
    let app = Router::new().route("/worker/metrics", post(worker::push_metrics));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("HTTP listening at port 3000");
    axum::serve(listener, app).await.unwrap();
}
