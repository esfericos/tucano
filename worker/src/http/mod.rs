use axum::{routing::post, Router};

use crate::runner::RunnerHandle;

mod runner;

#[derive(Clone)]
pub struct HttpState {
    pub runner: RunnerHandle,
}

pub async fn run_server(state: HttpState) {
    let app = Router::new()
        .nest(
            "/runner",
            Router::new()
                .route("/deploy-instance", post(runner::deploy_instance))
                .route("/terminate-instance", post(runner::terminate_instance)),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6969").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
