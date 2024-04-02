use std::{thread::sleep, time::Duration};

use sysinfo::System;

use proto::common::node::Metrics;

const CPU_DELAY_IN_MILLIS: u64 = 500;

/// Struct containing system's information such as CPU and Memory.
pub struct MetricsReport {
    system: System,
}

impl MetricsReport {
    /// Instantiates a new MetricsReport.
    ///
    /// Responsable for collecting metrics about the System, such as
    /// CPU Percentage Usage, Free Memory Space
    ///
    /// **⚠ Instantiate as mutable**
    ///
    /// | Keep in mind, you only need to instantiate ONE `MetricsReport`
    ///
    /// ```
    /// mod metrics;
    /// use crate::metrics::{MetricsReport::new()};
    ///
    /// fn main(){
    ///     let mut metrics_report = MetricsReport::new();
    /// }
    /// ```
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
        }
    }

    /// Get the `cpu_usage` and `free_memory` metrics from the current system.
    ///
    /// When calling this method, will sleep the thread for 500ms, customizable
    /// in the `CPU_DELAY_IN_MILLIS` constant
    pub fn get_metrics(&mut self) -> Metrics {
        Metrics {
            cpu_usage: self.get_cpu_usage(),
            mem_total_mib: self.get_total_memory(), 
            mem_used_mib: self.get_used_memory(),
        }
    }

    /// Returns the CPU Usage in percentage
    fn get_cpu_usage(&mut self) -> f64 {
        self.system.refresh_cpu();
        sleep(Duration::from_millis(CPU_DELAY_IN_MILLIS));
        println!("REFRESHED!");
        self.system.refresh_cpu();

        let all_cpus_usages: Vec<f64> = self
            .system
            .cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage() as f64)
            .collect();

        let cpu_usage = all_cpus_usages.iter().sum::<f64>();

        cpu_usage / all_cpus_usages.len() as f64
    }

    /// Returns the total_memory size in MiB
    fn get_total_memory(&mut self) -> f64 {
        (self.get_total_memory_as_byte() / 1024 / 1024) as f64
    }

    // Returns the total_memory in bytes
    fn get_total_memory_as_byte(&mut self) -> u64 {
        self.system.refresh_memory();

       self.system.total_memory()
    }

    // Returns the used_memory in MiB
    fn get_used_memory(&mut self) -> f64 {
        (self.get_used_memory_as_byte() / 1024 / 1024) as f64
    }

    // Returns the used_memory in bytes
    fn get_used_memory_as_byte(&mut self) -> u64 {
        self.system.used_memory()
    } 
}
