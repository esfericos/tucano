mod metrics;

use crate::metrics::{MetricsReport, SpaceUnit};

fn main() {
    let metrics_report = MetricsReport::new();
    let metric = metrics_report.get_metrics(SpaceUnit::GiB);
}
