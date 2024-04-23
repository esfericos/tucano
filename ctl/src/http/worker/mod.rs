use axum::Json;
use proto::common::node::Metrics;
use tracing::info;

pub async fn push_metrics(Json(payload): Json<Metrics>) {
    info!("{payload:#?}");
}
