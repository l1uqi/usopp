use sysinfo::System;

use crate::dto::SystemInfo;

pub fn get_system_info() -> SystemInfo{
  let mut sys = System::new_all();
  sys.refresh_cpu();
  sys.refresh_memory();

  let mut total_cpu_usage = 0f32;
  let total_mem_info = sys.total_memory();
  let used_men = sys.used_memory();
  // 内存使用占比
  let used_percent = &(used_men as f64 / total_mem_info as f64) * 100.0 as f64;
  for cpu in sys.cpus() {
    total_cpu_usage += cpu.cpu_usage();
  }
  let cpu_count = sys.cpus().len() as f32;
  println!("cpu usage: {}%", total_cpu_usage);
  println!("cpu count: {}", cpu_count);
  let total_cpu_usage = total_cpu_usage / cpu_count;
  SystemInfo {
    cpu_usage: total_cpu_usage,
    memory_usage: used_percent,
  }
}
