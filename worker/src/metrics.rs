use sysinfo::System;

pub enum MemoryMeasure {
    Megabyte,
    Gigabyte,
}

#[derive(Debug)]
pub struct Metric {
    free_memory: f64,
    cpu_usage: f64,
}

impl Metric {
    pub fn new(measure: MemoryMeasure) -> Metric {
        Metric {
            free_memory: get_memory(measure), 
            cpu_usage: 10.0
         }
    }
}

fn get_memory(measure: MemoryMeasure) -> f64 {
    match measure {
        MemoryMeasure::Gigabyte => get_memory_as_gigabyte(),
        MemoryMeasure::Megabyte => get_memory_as_megabyte(),
    }
}

fn get_memory_as_byte() -> f64 {
    let mut system = System::new_all();
    system.refresh_all();

    (system.total_memory() - system.used_memory()) as f64
}

fn get_memory_as_gigabyte() -> f64 {
    get_memory_as_byte() / 1024.0 / 1024.0 / 1024.0
}

fn get_memory_as_megabyte() -> f64 {
    get_memory_as_byte() / 1024.0 / 1024.0
}