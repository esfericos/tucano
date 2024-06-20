use axum::Json;
use proto::ctl::worker::{
    ByeReq, ByeRes, HelloReq, HelloRes, PushWorkerMetricsReq, PushWorkerMetricsRes,
};
use tracing::info;

pub async fn hello(Json(payload): Json<HelloReq>) -> Json<HelloRes> {
    info!("{payload:#?}");
    Json(HelloRes {})
}

pub async fn bye(Json(payload): Json<ByeReq>) -> Json<ByeRes> {
    info!("{payload:#?}");
    Json(ByeRes {})
}

pub async fn push_metrics(Json(payload): Json<PushWorkerMetricsReq>) -> Json<PushWorkerMetricsRes> {
    info!("{payload:#?}");
    Json(PushWorkerMetricsRes {})
}
