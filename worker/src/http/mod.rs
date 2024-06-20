use axum::{routing::post, Router};

use crate::runner::RunnerHandle;

mod runner;

#[derive(Clone)]
pub struct HttpState {
    pub runner: RunnerHandle,
}

pub fn mk_app(state: HttpState) -> Router {
    Router::new()
        .nest(
            "/runner",
            Router::new()
                .route("/deploy-instance", post(runner::deploy_instance))
                .route("/terminate-instance", post(runner::terminate_instance)),
        )
        .with_state(state)
}
