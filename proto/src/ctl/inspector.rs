use std::{collections::HashMap, os::unix::net::SocketAddr};

use serde::{Deserialize, Serialize};

use crate::common::{
    node::{Metrics, Node},
    service::ServiceName,
};

/// Returns the current system's topological information (regarding its nodes).
#[derive(Debug, Serialize, Deserialize)]
pub struct InspectTopologyReq {}

/// Response for [`InspectTopologyReq`].
#[derive(Debug, Serialize, Deserialize)]
pub struct InspectTopologyRes {
    pub nodes: Vec<(Node, Metrics)>,
    // More stuff?
}

/// Returns information about the **services** that are being executed on the
/// system.
pub struct InspectServicesReq {}

/// Response for [`InspectServicesReq`].
pub struct InspectServicesRes {
    pub services: HashMap<ServiceName, ServiceInfo>,
}

pub struct ServiceInfo {
    /// The current number of service instances that are running.
    pub total: u32,
    /// Maps the node address to the number of instances that are executing on
    /// it.
    pub nodes: HashMap<SocketAddr, u32>,
}
