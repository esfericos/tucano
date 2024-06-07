use std::{collections::HashMap, net::SocketAddr};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::{
    instance::InstanceId,
    service::{ServiceId, ServiceSpec},
};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct RevisionId(pub Uuid);

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct DeploymentId(pub Uuid);

/// Starts a new deploy in the system.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeployReq {
    pub service_spec: ServiceSpec,
}

/// Response for [`DeployReq`].
#[derive(Debug, Serialize, Deserialize)]
pub struct DeployRes {
    pub deployment_id: DeploymentId,
    pub instances_mapping: HashMap<InstanceId, SocketAddr>,
}

/// Stops a given service from running in the system.
#[derive(Debug, Serialize, Deserialize)]
pub struct TerminateReq {
    pub service_id: ServiceId,
}

/// Response for [`TerminateReq`].
#[derive(Debug, Serialize, Deserialize)]
pub struct TerminateRes {}

pub struct ReportDeployInstanceStatusReq {}

pub enum Status {
    /// The instance has successfully started.
    Started(InstanceId),
    /// The instance failed to start.
    FailedToStart(InstanceId, /* error */ String),
    /// The instance has gracefully terminated.
    Terminated(InstanceId),
    /// The instance stopped due to an abrupt error.
    Crashed(InstanceId, /* error */ String),
    /// The instance was killed by the System due to an error.
    Killed(InstanceId, /* reason */ String),
}

pub struct ReportDeployInstanceStatusRes {}
