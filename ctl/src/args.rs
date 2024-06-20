use std::time::Duration;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct CtlArgs {
    /// Interval after which a worker that hasn't send any metrics *can be*
    /// considered dead, after which it will be removed from the controller's
    /// workers pool.
    ///
    /// Notice that this interval MUST be greater than the value configured for
    /// **each** worker's `--metrics_report_interval` parameter.
    ///
    /// Time in seconds. Should be greater than 1.
    #[arg(
        long,
        default_value = "10",
        value_parser = parse_duration
    )]
    pub worker_liveness_timeout: Duration,
}

fn parse_duration(arg: &str) -> eyre::Result<Duration> {
    let s = arg.parse()?;
    Ok(Duration::from_secs(s))
}
