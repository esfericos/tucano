use std::net::IpAddr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::node::Metrics;

#[derive(Debug, Serialize, Deserialize)]
pub struct HelloReq {}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelloRes {
    pub status: HelloStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HelloStatus {
    Ok,
    AlreadyRegistered,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ByeReq {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ByeRes {}

/// Pushes new metrics of a given worker node.
///
/// The controller associates the provided metrics to the node that sent them,
/// using its peer IP address.
///
/// The controller server *may* ignore older requests that are received
/// out-of-order with respect to the `recorded_at` field.
#[derive(Debug, Serialize, Deserialize)]
pub struct PushWorkerMetricsReq {
    pub metrics: Metrics,
    pub recorded_at: DateTime<Utc>,
}

/// Response for [`PushWorkerMetricsReq`].
#[derive(Debug, Serialize, Deserialize)]
pub struct PushWorkerMetricsRes {
    pub status: PushMetricsStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PushMetricsStatus {
    /// Acknowledged.
    Ack,
    /// The worker has been removed from the cluster (at some moment in the
    /// past), and this metrics call is refused.
    Removed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryWorkersReq {}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryWorkersRes {
    pub workers: Vec<IpAddr>,
}
