use std::{thread::sleep, time::Duration};

use proto::common::node::Metrics;
use sysinfo::System;

const CPU_DELAY_IN_MILLIS: u64 = 500;

/// Struct containing system's information such as CPU and Memory.
pub struct MetricsCollector {
    system: System,
}
impl MetricsCollector {
    /// Instantiates a new `MetricsCollector`.
    ///
    /// Responsable for collecting metrics about the System, such as
    /// CPU Percentage Usage, Free Memory Space.
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
        }
    }

    /// Get the `cpu_usage` and `free_memory` metrics from the current system.
    ///
    /// When calling this method, will sleep the thread for
    /// `CPU_DELAY_IN_MILLIS`.
    pub fn get_metrics(&mut self) -> Metrics {
        Metrics {
            cpu_usage: self.get_cpu_usage(),
            mem_total_mib: self.get_total_memory(),
            mem_used_mib: self.get_used_memory(),
        }
    }

    fn get_cpu_usage(&mut self) -> Vec<f64> {
        self.system.refresh_cpu();
        sleep(Duration::from_millis(CPU_DELAY_IN_MILLIS));
        self.system.refresh_cpu();

        let all_cpus_usages: Vec<f64> = self
            .system
            .cpus()
            .iter()
            .map(|cpu| f64::from(cpu.cpu_usage()))
            .collect();

        all_cpus_usages
    }

    fn get_total_memory(&mut self) -> f64 {
        (self.get_total_memory_as_byte() / 1024 / 1024) as f64
    }

    fn get_total_memory_as_byte(&mut self) -> u64 {
        self.system.refresh_memory();

        self.system.total_memory()
    }

    fn get_used_memory(&mut self) -> f64 {
        (self.get_used_memory_as_byte() / 1024 / 1024) as f64
    }

    fn get_used_memory_as_byte(&mut self) -> u64 {
        self.system.used_memory()
    }
}
