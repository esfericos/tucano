use eyre::Result;
use tokio::time::{sleep, Duration};

use crate::monitor::collector::MetricsCollector;

pub mod monitor;

/// `clap` crate report interval placeholder
/// The value 500 is for quick visualization purposes.
const REPORT_INTERVAL_IN_MILLIS: u64 = 500;

#[tokio::main]
async fn main() -> Result<()> {
    let mut metrics_report: MetricsCollector = MetricsCollector::new();

    loop {
        sleep(Duration::from_millis(REPORT_INTERVAL_IN_MILLIS)).await;
        let metrics = metrics_report.get_metrics();
        println!("{metrics:#?}");
    }
}
