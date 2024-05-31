use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkSpec {
    pub expose_ports: Vec<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceSpec {
    /// The service domain.
    pub id: String,
    pub image: String,
    pub network: NetworkSpec,
    /// Whether this service is visible to the public load balancer.
    ///
    /// Only for port 80 (HTTP traffic).
    pub public: bool,
    /// The maximum number of instances that Tucano is allowed to run for this
    /// service.
    pub concurrency: u32,
}
