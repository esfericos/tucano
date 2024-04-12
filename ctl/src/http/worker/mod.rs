use axum::Json;
use proto::common::node::Metrics;

pub async fn push_metrics(Json(payload): Json<Metrics>) {
    println!("{payload:#?}");
}
