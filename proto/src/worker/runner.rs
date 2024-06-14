//! Very similar to [`crate::ctl::deployer`], but while the former coordinates a
//! **service deploy**, this module is concerned with the actual deployment of
//! an instance on a given worker node.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::instance::{InstanceId, InstanceSpec};

///

/// Starts a new deploy in the system
#[derive(Debug, Serialize, Deserialize)]
pub struct DeployInstanceReq {
    pub id: DeoployReqId,
    pub instance_spec: InstanceSpec,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployInstanceRes {
    pub id: DeoployReqId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeoployReqId(Uuid);

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
