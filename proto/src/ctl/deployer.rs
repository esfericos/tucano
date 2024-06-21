use std::{collections::HashMap, fmt, net::IpAddr};

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

impl fmt::Display for DeploymentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Starts a new deploy in the system.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeployServiceReq {
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
pub struct DeployServiceRes {
    pub deployment_id: DeploymentId,
    pub instances: HashMap<InstanceId, IpAddr>,
}

/// Stops a given service from running in the system.
#[derive(Debug, Serialize, Deserialize)]
pub struct TerminateServiceReq {
    pub service_id: ServiceId,
}

/// Response for [`TerminateReq`].
#[derive(Debug, Serialize, Deserialize)]
pub struct TerminateServiceRes {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportDeployInstanceStatusReq {
    pub instance_id: InstanceId,
    pub status: instance::Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportDeployInstanceStatusRes {}
