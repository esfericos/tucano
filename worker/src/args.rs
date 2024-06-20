use std::{net::IpAddr, time::Duration};

use clap::Parser;

#[derive(Debug, Parser)]
pub struct WorkerArgs {
    /// Controller's HTTP address.
    #[arg(short, long)]
    pub controller_addr: IpAddr,

    /// Interval at which metrics are pushed to the controller.
    ///
    /// Time in seconds. Must be greater than 1.
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
