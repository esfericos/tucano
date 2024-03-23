use std::{thread::sleep, time::Duration};
use sysinfo::System;

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


#[derive(Debug)]
pub struct Metrics {
    free_memory: f64,
    cpu_usage: f32,
}

pub struct MetricsReport {
    system: System
}

impl MetricsReport {
    pub fn new() -> Self {
        Self {
            system: System::new_all()
        }
    }

    pub fn get_metrics(&self, measure: SpaceUnit) -> Metrics {
        Metrics {
            cpu_usage: get_cpu_usage(),
            free_memory: get_memory(&measure)
        }
    }
}

fn get_memory(measure: &SpaceUnit) -> f64 {
    get_memory_as_byte() / measure.byte_conv_factor()
}

fn get_memory_as_byte() -> f64 {
    let mut system = System::new_all();
    system.refresh_memory();

    (system.total_memory() - system.used_memory()) as f64
}

fn get_cpu_usage() -> f32 {
    let mut system = System::new_all();

    system.refresh_cpu();
    sleep(Duration::from_millis(500));
    system.refresh_cpu();

    let all_cpus_usages: Vec<f32> = system
        .cpus()
        .iter()
        .map(|cpu| cpu.cpu_usage())
        .collect();
    let cpu_usage = all_cpus_usages.iter().sum::<f32>();

    cpu_usage / all_cpus_usages.len() as f32
}
