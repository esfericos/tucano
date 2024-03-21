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
pub struct MetricsReport {
    free_memory: f64,
    cpu_usage: f64,
}

impl MetricsReport {
    pub fn new(measure: SpaceUnit) -> MetricsReport {
        MetricsReport {
            free_memory: get_memory(measure),
            cpu_usage: 10.0,
        }
    }
}

fn get_memory(measure: SpaceUnit) -> f64 {
    get_memory_as_byte() / measure.byte_conv_factor()
}

fn get_memory_as_byte() -> f64 {
    let mut system = System::new_all();
    system.refresh_all();

    (system.total_memory() - system.used_memory()) as f64
}