use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::process::Command;

fn get_os_info() -> Option<String> {
    let file = File::open("/etc/os-release").ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(l) = line
            && l.starts_with("PRETTY_NAME=")
        {
            let name = l.replace("PRETTY_NAME=", "").replace("\"", "");
            return Some(name);
        }
    }
    None
}

fn get_cpu_info() -> Option<String> {
    let file = std::fs::read_to_string("/proc/cpuinfo").ok()?;

    let mut model = None;

    for line in file.lines() {
        if line.starts_with("model name") && model.is_none() {
            model = line.split(':').nth(1).map(|s| s.trim().to_string());
        }
    }

    Some(format!("{}", model.unwrap_or("Unknown".into()),))
}


fn get_mem_info() -> Option<String> {
    let file = std::fs::read_to_string("/proc/meminfo").ok()?;

    let parse_kb = |prefix: &str| -> Option<f64> {
        file.lines()
            .find(|line| line.starts_with(prefix))?
            .split(':')
            .nth(1)?
            .split_whitespace()
            .next()?
            .parse::<f64>()
            .ok()
    };

    let mem_total  = parse_kb("MemTotal:")?;
    let mem_free   = parse_kb("MemFree:")?;
    let buffers    = parse_kb("Buffers:")?;
    let cached     = parse_kb("Cached:")?;

    let mem_used = mem_total - mem_free - buffers - cached;

    let to_gib = |kb: f64| kb / 1_048_576.0;

    Some(format!(
        "{:.2} GiB / {:.2} GiB",
        to_gib(mem_used),
        to_gib(mem_total)
    ))
}

fn get_uptime_info() -> Option<String> {
    let seconds = fs::read_to_string("/proc/uptime").ok()?
        .split_whitespace()
        .next()?
        .parse::<f64>().ok()?;

    let hours = seconds / 3600.0;
    Some(format!("{:.2} hours", hours))
}


fn get_shell_info() -> Option<String> {
    std::env::var("SHELL").ok()
}

pub fn show_info() {
    if let Ok(os) = fs::read_to_string("/etc/hostname") {
        println!("\x1b[34mHost:\x1b[0m {}", os.trim());
    }
    if let Ok(kernel) = fs::read_to_string("/proc/sys/kernel/osrelease") {
        println!("\x1b[34mKernel:\x1b[0m Linux {}", kernel.trim());
    }
    if let Some(up) = get_uptime_info() {
        println!("\x1b[34mUptime:\x1b[0m {}", up);
    }
    if let Some(sh) = get_shell_info() {
        println!("\x1b[34mShell:\x1b[0m {}", sh);
    }
    if let Some(os) = get_os_info() {
        println!("\x1b[34mOS:\x1b[0m {}", os);
    }
    if let Some(cpu) = get_cpu_info() {
        println!("\x1b[34mCPU:\x1b[0m {}", cpu);
    }
    if let Some(mem) = get_mem_info() {
        println!("\x1b[34mMemory:\x1b[0m {}", mem);
    }
}
