use serde::{Deserialize, Serialize};

use crate::common::service::{ServiceName, ServiceSpec};

/// Starts a new deploy in the system.
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
    /// Whether to completely remove the service from the system, calling the
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
