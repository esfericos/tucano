//! Very similar to [`crate::ctl::deployer`], but while the former coordinates a
//! **service deploy**, this module is concerned with the actual deployment of
//! an instance on a given worker node.

use serde::{Deserialize, Serialize};

use crate::common::instance::{InstanceId, InstanceSpec};

/// Starts a new deploy in the system.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeployReq {
    pub instance_spec: InstanceSpec,
}

/// Stops a given service from running in the system.
#[derive(Debug, Serialize, Deserialize)]
pub struct TerminateReq {
    pub instance_id: InstanceId,
}
