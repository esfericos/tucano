use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::service::ServiceSpec;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RevisionId(pub Uuid);

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeployId(pub Uuid);

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum DeployStatus {
    /// The deployment process is in progress (e.g. running the build script).
    InProgress,
    /// The deployment is finished and the service is running.
    Running,
    /// The service has gracefully stopped.
    Stopped,
    /// The service build script has failed.
    BuildFailed,
    /// The service has abruptly crashed.
    Crashed,
}

/// Starts a new deploy in the system.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeployReq {
    pub service_spec: ServiceSpec,
}

/// Response for [`DeployReq`].
#[derive(Debug, Serialize, Deserialize)]
pub struct DeployRes {
    pub revision_id: RevisionId,
    pub deploy_ids: Vec<DeployId>,
}

/// Stops a given service from running in the system.
#[derive(Debug, Serialize, Deserialize)]
pub struct StopReq {
    pub service_name: String,
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
