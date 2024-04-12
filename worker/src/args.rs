use std::{net::SocketAddr, time::Duration};

use clap::{value_parser, Parser};

pub struct WorkerArgs {
    /// Controller's address.
    pub controller_addr: SocketAddr,

    /// Interval at which metrics are pushed to the controller.
    pub metrics_report_interval: Duration,
}

impl WorkerArgs {
    /// Parses the process arguments and returns a new [`Args`].
    ///
    /// Panics if missing arguments or if arguments are invalid.
    pub fn parse() -> Self {
        let raw = RawWorkerArgs::parse();
        WorkerArgs {
            controller_addr: raw.controller_addr,
            metrics_report_interval: Duration::from_secs(raw.metrics_report_interval.into()),
        }
    }
}

#[derive(Parser)]
struct RawWorkerArgs {
    /// Controller's HTTP address.
    #[arg(short, long)]
    controller_addr: SocketAddr,

    /// Interval at which metrics are pushed to the controller.
    ///
    /// Time in seconds. Must be greater than 1.
    #[arg(
        long,
        default_value = "5",
        value_parser = value_parser!(u32).range(1..)
    )]
    metrics_report_interval: u32,
}
