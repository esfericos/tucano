use axum::{routing::post, Router};

use crate::runner::RunnerHandle;

mod runner;

#[derive(Clone)]
pub struct HttpState {
    pub runner: RunnerHandle,
}

pub async fn run_server(state: HttpState) {
    let app = Router::new()
        .route("/instance/new", post(runner::new_instance))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6969").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
