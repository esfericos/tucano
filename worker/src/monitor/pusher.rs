use eyre::Result;
use tokio::time::sleep;

use crate::{args::WorkerArgs, monitor::collector::MetricsCollector};

/// Request [`MetricsCollector`] to collect [`Metrics`] within a given time
/// interval.
pub async fn push(args: WorkerArgs) -> Result<()> {
    let mut metrics_report: MetricsCollector = MetricsCollector::new();

    loop {
        sleep(args.metrics_report_interval).await;
        let metrics = metrics_report.get_metrics();
        println!("{metrics:#?}");
    }
}
