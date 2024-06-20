use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::node::Metrics;

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
pub struct PushWorkerMetricsRes {}
