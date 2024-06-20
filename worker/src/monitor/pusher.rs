use std::sync::Arc;

use chrono::Utc;
use proto::clients::CtlClient;
use tokio::time::sleep;
use tracing::error;

use crate::{args::WorkerArgs, monitor::collector::MetricsCollector};

pub async fn start_pusher(args: Arc<WorkerArgs>, ctl_client: CtlClient) {
    let mut metrics_report: MetricsCollector = MetricsCollector::new();
    loop {
        sleep(args.metrics_report_interval).await;
        let metrics = metrics_report.get_metrics();
        let now = Utc::now();
        if let Err(error) = ctl_client.push_metrics(metrics, now).await {
            error!(?error, "failed to send metrics to ctl");
        }
    }
}
