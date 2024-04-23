use std::sync::Arc;

use reqwest;
use tokio::time::sleep;

use crate::{args::WorkerArgs, monitor::collector::MetricsCollector};

pub async fn start_pusher(args: Arc<WorkerArgs>) {
    let mut metrics_report: MetricsCollector = MetricsCollector::new();

    let client = reqwest::Client::new();

    loop {
        sleep(args.metrics_report_interval).await;
        let metrics = metrics_report.get_metrics();

        let _ = client
            .post("http://localhost:3000/worker/metrics")
            .json(&metrics)
            .send()
            .await;
    }
}
