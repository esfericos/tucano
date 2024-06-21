use std::sync::Arc;

use chrono::Utc;
use eyre::Context as _;
use proto::{clients::CtlClient, ctl::worker::PushMetricsStatus};
use tokio::time::sleep;
use tracing::{error, trace};

use crate::{args::WorkerArgs, monitor::collector::MetricsCollector};

pub async fn start_pusher(args: Arc<WorkerArgs>, ctl_client: CtlClient) -> eyre::Result<()> {
    let mut metrics_report: MetricsCollector = MetricsCollector::new();
    trace!("pusher started");

    // Try to join the cluster
    ctl_client
        .hello()
        .await
        .wrap_err("worker failed to join the cluster")?;

    loop {
        trace!("sending metrics");
        let metrics = metrics_report.get_metrics();
        let now = Utc::now();

        let result = ctl_client
            .push_metrics(metrics, now)
            .await
            .map(|r| r.status);
        match result {
            Ok(PushMetricsStatus::Ack) => (),
            Ok(PushMetricsStatus::Removed) => {
                eyre::bail!("worker was removed from cluster");
            }
            Err(error) => {
                error!(?error, "failed to send metrics to ctl");
            }
        }

        sleep(args.metrics_report_interval).await;
    }
}
