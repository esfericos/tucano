use axum::Json;
use proto::{common::node::Metrics, ctl::worker::PushWorkerMetricsRes};
use tracing::info;

pub async fn push_metrics(Json(payload): Json<Metrics>) -> Json<PushWorkerMetricsRes> {
    info!("{payload:#?}");
    todo!()
}
