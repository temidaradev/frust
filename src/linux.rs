use std::fs::{self, File};
use std::io::{BufRead, BufReader};

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

    Some(format!(
        "{}",
        model.unwrap_or("Unknown".into()),
    ))
}

fn get_mem_info() -> Option<String> {
    let file = std::fs::read_to_string("/proc/meminfo").ok()?;

    let mut mem = None;

    for line in file.lines() {
        if line.starts_with("MemTotal") && mem.is_none() {
            mem = line.split(':').nth(1).map(|s| s.trim().to_string());
        }
    }
    
    if let Some(mem_str) = mem {
        let kb_value = mem_str.split_whitespace().next()?.parse::<f64>().ok()?;

        let gb_value = kb_value / 1_048_576.0;

        return Some(format!("{:.2} GiB", gb_value));
    }

    Some("Memory: Unknown".into())
}

pub fn show_info() {
    if let Ok(os) = fs::read_to_string("/etc/hostname") {
        println!("\x1b[34mHost:\x1b[0m {}", os.trim());
    }
    if let Some(os) = get_os_info() {
        println!("\x1b[34mOS:\x1b[0m {}", os);
    }
    if let Some(os) = get_cpu_info() {
        println!("\x1b[34mCPU:\x1b[0m {}", os);
    }
    if let Some(os) = get_mem_info() {
        println!("\x1b[34mMemory:\x1b[0m {}", os);
    }
}
