use sysinfo::{    Components, Disks, Networks, System,};

fn main() {
  get_memory();
}

fn get_memory() {
  let mut system = System::new_all();
  system.refresh_all();
  
  let free_memory = (system.total_memory() - system.used_memory()) as f64;
  println!("{:.2}", free_memory / 1024.0 / 1024.0 / 1024.0);
}