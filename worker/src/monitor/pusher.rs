use std::sync::Arc;

use tokio::time::sleep;

use crate::{args::WorkerArgs, monitor::collector::MetricsCollector};

pub async fn start_pusher(args: Arc<WorkerArgs>) {
    let mut metrics_report: MetricsCollector = MetricsCollector::new();

    loop {
        sleep(args.metrics_report_interval).await;
        let metrics = metrics_report.get_metrics();
        println!("{metrics:#?}");
    }
}
