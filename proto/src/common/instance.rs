use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::service::ServiceImage;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InstanceId(Uuid);

#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceSpec {
    pub instance_id: InstanceId,
    pub image: ServiceImage,
    pub public: bool,
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
}
