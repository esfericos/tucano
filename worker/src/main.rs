use sysinfo::System;
mod metrics;
use crate::metrics::{Metric, MemoryMeasure};

fn main() {
  let metrics = Metric::new(MemoryMeasure::Gigabyte);
  println!("{:?}", metrics);

}
