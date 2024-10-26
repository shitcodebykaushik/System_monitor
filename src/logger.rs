use std::fs::OpenOptions;
use std::io::Write;
use sysinfo::{System, SystemExt, CpuExt}; // Import SystemExt and CpuExt for method access
use chrono::Local;

pub fn log_metrics(system: &System) -> std::io::Result<()> {
    // Open the log file in append mode, create if it doesn't exist
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("metrics_log.txt")?;

    // Gather CPU and memory information
    let cpu_usage: f32 = system.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / system.cpus().len() as f32;

    // Alternative memory methods based on possible updates
    let total_memory = system.total_memory() / 1024;     // in MB
    let used_memory = (system.total_memory() - system.available_memory()) / 1024; // in MB

    // Format the log entry
    let log_entry = format!(
        "Timestamp: {}, CPU Usage: {:.2}%, Memory Usage: {} MB / {} MB\n",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        cpu_usage,
        used_memory,
        total_memory,
    );

    // Write to the file
    file.write_all(log_entry.as_bytes())?;

    Ok(())
}
