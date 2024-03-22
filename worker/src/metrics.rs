use sysinfo::{CpuRefreshKind, System};

pub enum SpaceUnit {
    MiB,
    GiB,
}

impl SpaceUnit {
    pub fn byte_conv_factor(&self) -> f64 {
        get_cpu_usage();
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
            cpu_usage: get_cpu_usage(),
        }
    }
}

fn get_memory(measure: SpaceUnit) -> f64 {
    get_memory_as_byte() / measure.byte_conv_factor()
}

fn get_memory_as_byte() -> f64 {
    let mut system = System::new_all();
    system.refresh_memory();

    (system.total_memory() - system.used_memory()) as f64
}

fn get_cpu_usage() -> f64 {
    let mut system = System::new_all();
    system.refresh_cpu_specifics(CpuRefreshKind::everything());

    let all_cpus_usages: Vec<f64> = system.cpus().iter().map(|cpu| cpu.cpu_usage() as f64).collect();
    let cpu_usage = all_cpus_usages.iter().sum::<f64>();

    cpu_usage / all_cpus_usages.len() as f64
}
