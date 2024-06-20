use std::time::Duration;

pub const GRACEFUL_SHUTDOWN_DEADLINE: Duration = Duration::from_secs(20);

pub const PROXY_INSTANCE_HEADER_NAME: &str = "X-Tuc-Inst";

pub const CTL_HTTP_PORT: u16 = 6968;
pub const CTL_BALANCER_PORT: u16 = 8080;

pub const WORKER_PROXY_PORT: u16 = 8080;
pub const WORKER_HTTP_PORT: u16 = 6969;
