use std::{thread::sleep, time::Duration};

use sysinfo::System;

const CPU_DELAY_IN_MILLIS: u64 = 500;

#[derive(Clone, Copy)]
pub enum SpaceUnit {
    MiB,
    GiB,
}

impl SpaceUnit {
    pub fn byte_conv_factor(&self) -> f64 {
        match &self {
            SpaceUnit::GiB => f64::powi(1024.0, 3),
            SpaceUnit::MiB => f64::powi(1024.0, 2),
        }
    }
}

/// Struct containing the free_memory and cpu_usage on the current machine
#[derive(Debug)]
pub struct Metrics {
    free_memory: f64,
    cpu_usage: f32,
}

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
    pub fn get_metrics(&mut self, measure: SpaceUnit) -> Metrics {
        Metrics {
            cpu_usage: self.get_cpu_usage(),
            free_memory: self.get_memory(&measure),
        }
    }
}

impl MetricsReport {
    /// Returns the free_memory size with the SpaceUnit specified in params
    fn get_memory(&mut self, measure: &SpaceUnit) -> f64 {
        self.get_memory_as_byte() / measure.byte_conv_factor()
    }

    /// Returns the free_memory size in bytes
    fn get_memory_as_byte(&mut self) -> f64 {
        self.system.refresh_memory();

        (self.system.total_memory() - self.system.used_memory()) as f64
    }

    /// Returns the CPU Usage in percentage
    fn get_cpu_usage(&mut self) -> f32 {
        self.system.refresh_cpu();
        sleep(Duration::from_millis(CPU_DELAY_IN_MILLIS));
        println!("REFRESHED!");
        self.system.refresh_cpu();

        let all_cpus_usages: Vec<f32> = self
            .system
            .cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage())
            .collect();

        let cpu_usage = all_cpus_usages.iter().sum::<f32>();

        cpu_usage / all_cpus_usages.len() as f32
    }
}
