mod metrics;

use crate::metrics::{MetricsReport};

fn main() {
    let mut metrics_report = MetricsReport::new();
    let metric = metrics_report.get_metrics();
    println!("{metric:?}");
}
