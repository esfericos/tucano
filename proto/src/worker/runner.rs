//! Very similar to [`crate::ctl::deployer`], but while the former coordinates a
//! **system deploy**, this module is concerned with the actual deployment of a
//! service on a given worker node.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Specification of a service's instance
#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceSpec {
    id: Uuid,
    image: String,
}

/// Starts a **single** deploy of the given service spec.
///
/// The worker server doesn't follow the concurrency limit for the service as
/// defined in [`InstanceSpec`].
#[derive(Debug, Serialize, Deserialize)]
pub struct DeployInstanceReq {
    pub instances: Vec<InstanceSpec>,
}

/// Stops a given instance from running in the system.
#[derive(Debug, Serialize, Deserialize)]
pub struct StopReq {
    pub instance: InstanceSpec,
    /// Whether to completely remove the service from the node, calling the
    /// teardown script, if any.
    pub remove: bool,
}

/// Response for [`StopReq`].
#[derive(Debug, Serialize, Deserialize)]
pub struct StopRes {
    /// Whether the service was removed.
    ///
    /// Only returns `true` if the service has a teardown script and it was
    /// successfully executed.
    pub removed: bool,
}
