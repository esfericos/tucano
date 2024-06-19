use std::time::Duration;

pub const GRACEFUL_SHUTDOWN_DEADLINE: Duration = Duration::from_secs(20);

pub const PROXY_INSTANCE_HEADER_NAME: &str = "X-Tuc-Inst";

pub const WORKER_PROXY_PORT: u16 = 8080;
