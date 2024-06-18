use std::{collections::HashMap, net::SocketAddr};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::{
    instance::{self, InstanceId},
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
    pub redeployment_policy: RedeploymentPolicy,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RedeploymentPolicy {
    /// Disallow re-deployments if the service is already deployed with running
    /// instances.
    None,
    // TODO: Add more (blue green, etc)
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportDeployInstanceStatusReq {
    pub instance_id: InstanceId,
    pub status: instance::Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportDeployInstanceStatusRes {}
