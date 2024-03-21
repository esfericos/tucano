use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::{
    node::{Metrics, NodeName},
    service::ServiceName,
};

/// Pushes new metrics of a given **worker** node.
///
/// The server must validate whether the node name corresponds to the
/// appropriate node address. If they don't match, the operation fails.
///
/// The server *may* ignore older requests that are received out-of-order with
/// respect to the `recorded_at` field.
#[derive(Debug, Serialize, Deserialize)]
pub struct PushWorkerMetricsReq {
    pub node_name: NodeName,
    pub metrics: Metrics,
    /// The number of services that are being executed on the node.
    pub services: HashMap<ServiceName, u32 /* todo: more info? */>,
    pub recorded_at: DateTime<Utc>,
}

/// Response for [`PushWorkerMetricsReq`].
#[derive(Debug, Serialize, Deserialize)]
pub struct PushWorkerMetricsRes {}
