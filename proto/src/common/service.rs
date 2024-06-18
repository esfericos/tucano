use serde::{Deserialize, Serialize};

/// The service ID (i.e., its name).
///
/// Is unique in the cluster.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ServiceId(pub String);

/// The service image.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ServiceImage(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceSpec {
    /// The service domain.
    pub service_id: ServiceId,
    pub image: ServiceImage,
    /// Whether this service is visible to the public load balancer.
    pub public: bool,
    /// The maximum number of instances that Tucano is allowed to run for this
    /// service.
    pub concurrency: u32,
    pub resource_config: ResourceConfig,
}

/// The allocation of resources for a Service.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceConfig {
    pub cpu_shares: i64,
    pub memory_limit: i64,
}
