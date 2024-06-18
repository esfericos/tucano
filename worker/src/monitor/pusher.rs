use std::sync::Arc;

use tokio::time::sleep;

use crate::{args::WorkerArgs, monitor::collector::MetricsCollector, sender};

pub async fn start_pusher(args: Arc<WorkerArgs>, sender: Arc<sender::Sender>) {
    let mut metrics_report: MetricsCollector = MetricsCollector::new();
    loop {
        sleep(args.metrics_report_interval).await;
        let metrics = metrics_report.get_metrics();
        let _ = sender.send_metrics(metrics).await;
    }
}
