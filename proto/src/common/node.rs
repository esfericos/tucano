use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

bty::brand!(
    pub type NodeName = String;
);

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub name: NodeName,
    pub addr: SocketAddr,
    pub kind: NodeKind,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NodeKind {
    Ctl,
    Worker,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metrics {
    /// The average CPU usage.
    pub cpu_usage: f64,
    /// The total memory, in bytes.
    pub mem_total: u64,
    /// The used memory, in bytes.
    pub mem_used: u64,
}
