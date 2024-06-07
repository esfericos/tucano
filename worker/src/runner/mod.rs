use std::collections::HashMap;

use proto::worker::runner;

use get_port::{tcp::TcpPort, Range};

pub struct UsedPorts {
    pub in_use: HashMap<uuid::Uuid,
}

pub fn deploy(instanceReq: &runner::DeployInstanceReq) -> Result<()> {
    let tcp_port = TcpPort::in_range(
        "127.0.0.1",
        Range {
            min: 6000,
            max: 7000,
        },
    )
    .unwrap();

    Ok(())
}

pub fn terminate(instance: &InstanceSpec) {}
