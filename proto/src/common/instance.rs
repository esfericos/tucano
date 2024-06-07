use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::service::ServiceImage;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InstanceId(Uuid);

#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceSpec {
    pub instance_id: InstanceId,
    pub image: ServiceImage,
    pub public: bool,
}
