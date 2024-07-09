use std::time::Duration;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct WorkerArgs {
    /// Controller's HTTP address.
    #[arg(short, long)]
    pub ctl_addr: String,

    /// Whether the worker should run in a Docker-networking aware context.
    ///
    /// If set, must specify the name of the Docker network.
    ///
    /// In general, this option is desirable when executing all Tucano nodes
    /// in a single host via Docker containers.
    #[arg(long)]
    pub use_docker_network: Option<String>,

    /// Interval at which metrics are pushed to the controller.
    ///
    /// Notice that this interval MUST be smaller than the value configured for
    /// the controller's `--worker_liveness_timeout` parameter.
    ///
    /// Time in seconds. Should be greater than 1.
    #[arg(
    long,
    default_value = "5",
    value_parser = parse_duration
)]
    pub metrics_report_interval: Duration,
}

fn parse_duration(arg: &str) -> eyre::Result<Duration> {
    let s = arg.parse()?;
    Ok(Duration::from_secs(s))
}
