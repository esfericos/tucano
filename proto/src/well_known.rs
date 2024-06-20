use std::time::Duration;

pub const GRACEFUL_SHUTDOWN_DEADLINE: Duration = Duration::from_secs(20);

pub const PROXY_INSTANCE_HEADER_NAME: &str = "X-Tuc-Inst";
