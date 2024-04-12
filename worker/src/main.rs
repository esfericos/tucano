use eyre::Result;
use tokio::time::sleep;

use crate::{args::WorkerArgs, monitor::collector::MetricsCollector};

mod args;
mod monitor;

#[tokio::main]
async fn main() -> Result<()> {
    let args = WorkerArgs::parse();

    let mut metrics_report: MetricsCollector = MetricsCollector::new();

    loop {
        sleep(args.metrics_report_interval).await;
        let metrics = metrics_report.get_metrics();
        println!("{metrics:#?}");
    }
}
