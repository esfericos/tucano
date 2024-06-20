use axum::Json;
use proto::ctl::worker::{PushWorkerMetricsReq, PushWorkerMetricsRes};
use tracing::info;

pub async fn push_metrics(Json(payload): Json<PushWorkerMetricsReq>) -> Json<PushWorkerMetricsRes> {
    info!("{payload:#?}");
    Json(PushWorkerMetricsRes {})
}
