use std::net::IpAddr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub addr: IpAddr,
    pub kind: NodeKind,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NodeKind {
    Ctl,
    Worker,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metrics {
    /// The average CPU usage.
    pub cpu_usage: f64,
    /// The total memory, in bytes.
    pub mem_total: u64,
    /// The used memory, in bytes.
    pub mem_used: u64,
}
