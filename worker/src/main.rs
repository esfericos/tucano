mod metrics;
use sysinfo::System;

use crate::metrics::{MetricsReport, SpaceUnit};

fn main() {
    let metrics = MetricsReport::new(SpaceUnit::MiB);
    println!("{:?}", metrics);
}
