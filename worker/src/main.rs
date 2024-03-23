mod metrics;

use crate::metrics::{MetricsReport, SpaceUnit};

fn main() {
    let mut metrics_report = MetricsReport::new();
    let metric = metrics_report.get_metrics(SpaceUnit::GiB);
    println!("{metric:?}");
}
