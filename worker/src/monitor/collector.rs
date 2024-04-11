use proto::common::node::Metrics;
use sysinfo::System;

/// Responsable for collecting metrics about the System, such as
/// CPU Percentage Usage, Free Memory Space.
#[derive(Default)]
pub struct MetricsCollector {
    system: System,
}

impl MetricsCollector {
    /// Instantiates a new [`MetricsCollector`].
    #[must_use]
    pub fn new() -> Self {
        MetricsCollector::default()
    }

    /// Retruns the [`Metrics`] struct for the current system.
    #[must_use]
    pub fn get_metrics(&mut self) -> Metrics {
        self.system.refresh_memory();
        self.system.refresh_cpu();

        Metrics {
            cpu_usage: self.get_cpu_usage(),
            mem_total: self.get_total_memory(),
            mem_used: self.get_used_memory(),
        }
    }

    fn get_cpu_usage(&mut self) -> f64 {
        let amount_cpus = self.system.cpus().len();

        let sum_cpus_usages: f64 = self
            .system
            .cpus()
            .iter()
            .map(|cpu| f64::from(cpu.cpu_usage()))
            .sum();

        sum_cpus_usages / amount_cpus as f64
    }

    fn get_total_memory(&mut self) -> u64 {
        self.system.total_memory()
    }

    fn get_used_memory(&mut self) -> u64 {
        self.system.used_memory()
    }
}
