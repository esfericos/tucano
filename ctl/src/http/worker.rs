use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, State},
    Json,
};
use proto::ctl::worker::{
    ByeReq, ByeRes, HelloReq, HelloRes, PushWorkerMetricsReq, PushWorkerMetricsRes,
};

use crate::http::HttpState;

pub async fn hello(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<HttpState>,
    Json(HelloReq {}): Json<HelloReq>,
) -> Json<HelloRes> {
    let addr = addr.ip();
    let status = state.worker_mgr.hello(addr).await;
    Json(HelloRes { status })
}

pub async fn bye(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<HttpState>,
    Json(ByeReq {}): Json<ByeReq>,
) -> Json<ByeRes> {
    let addr = addr.ip();
    state.worker_mgr.bye(addr).await;
    Json(ByeRes {})
}

pub async fn push_metrics(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<HttpState>,
    Json(PushWorkerMetricsReq {
        metrics,
        recorded_at: _,
    }): Json<PushWorkerMetricsReq>,
) -> Json<PushWorkerMetricsRes> {
    let addr = addr.ip();
    let status = state.worker_mgr.push_metrics(addr, metrics).await;
    Json(PushWorkerMetricsRes { status })
}
