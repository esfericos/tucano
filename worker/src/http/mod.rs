use axum::{routing::post, Router};
use proto::well_known::WORKER_HTTP_PORT;
use utils::server;

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

    server::listen("worker http", app, ("0.0.0.0", WORKER_HTTP_PORT)).await;
}
