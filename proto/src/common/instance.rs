use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::service::{ResourceConfig, ServiceImage};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InstanceId(pub Uuid);

impl fmt::Display for InstanceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for InstanceId {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(InstanceId(value.parse().map_err(|_| ())?))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstanceSpec {
    pub instance_id: InstanceId,
    pub image: ServiceImage,
    pub public: bool,
    pub resource_config: ResourceConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    /// The instance has successfully started.
    Started,
    /// The instance has gracefully terminated.
    Terminated,
    /// The instance stopped due to an abrupt error.
    Crashed { error: String },
    /// The instance was killed by the System due to an error.
    Killed { reason: String },
    /// The instance failed during attempted execution.
    FailedToStart { error: String },
}
