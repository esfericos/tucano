mod monitor;
use std::{thread::sleep, time::Duration};

use eyre::Result;
use monitor::{MetricsCollector, MetricsSender};

/// `clap` crate report interval placeholder
/// The value 500 is for quick visualization purposes.
const REPORT_INTERVAL_IN_MILLIS: u64 = 500;

/// `clap` crate url placeholder
const AGT_MGR_REQUEST_URL: &str = "http://localhost:8080/http/agt_mgr";

#[tokio::main]
async fn main() -> Result<()> {
    let mut metrics_sender = MetricsSender::new(AGT_MGR_REQUEST_URL)?;
    let mut metrics_report: MetricsCollector = MetricsCollector::new();

    loop {
        let metrics = metrics_report.get_metrics();
        metrics_sender.send_request(&metrics).await?;
        sleep(Duration::from_millis(REPORT_INTERVAL_IN_MILLIS));
    }
}
