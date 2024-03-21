//! Very similar to [`crate::ctl::deployer`], but while the former coordinates a
//! **system deploy**, this module is concerned with the actual deployment of a
//! service on a given worker node.

use serde::{Deserialize, Serialize};

use crate::common::service::{ServiceName, ServiceSpec};

/// Starts a **single** deploy of the given service spec.
///
/// The worker server doesn't follow the concurrency limit for the service as
/// defined in [`ServiceSpec`].
#[derive(Debug, Serialize, Deserialize)]
pub struct DeployReq {
    pub service_spec: ServiceSpec,
}

/// Response for [`DeployReq`].
#[derive(Debug, Serialize, Deserialize)]
pub struct DeployRes {
    // ???
}

/// Stops a given service from running in the system.
#[derive(Debug, Serialize, Deserialize)]
pub struct StopReq {
    pub service_name: ServiceName,
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
