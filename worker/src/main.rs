use eyre::Result;
use tokio::time::sleep;
use tracing::info;

use crate::{args::WorkerArgs, monitor::collector::MetricsCollector};

mod args;
mod monitor;

#[tokio::main]
async fn main() -> Result<()> {
    setup::tracing();

    let args = WorkerArgs::parse();
    info!(?args, "started worker");

    let mut metrics_report: MetricsCollector = MetricsCollector::new();

    loop {
        sleep(args.metrics_report_interval).await;
        let metrics = metrics_report.get_metrics();
        println!("{metrics:#?}");
    }
}
