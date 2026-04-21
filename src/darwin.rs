use std::env;
use std::io::StdoutLock;
use std::io::{BufWriter, Write};
use std::process::Command;

pub fn get_os_info() -> Option<String> {
    let output = Command::new("sw_vers").arg("-productName").output().ok()?;

    let product_name = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let output = Command::new("sw_vers")
        .arg("-productVersion")
        .output()
        .ok()?;

    let product_version = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let output = Command::new("sw_vers").arg("-buildVersion").output().ok()?;

    let build_version = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let output = Command::new("uname").arg("-m").output().ok()?;

    let machine = String::from_utf8_lossy(&output.stdout).trim().to_string();

    Some(format!(
        "{} {} ({}) {}",
        product_name, product_version, build_version, machine
    ))
}

fn get_host_info() -> Option<String> {
    let user = env::var("USER").unwrap_or_else(|_| "user".to_string());
    let output = Command::new("hostname").output().ok()?;
    let host = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let hostname = format!("{}@{}", user, host);

    Some(hostname)
}

fn cpu_info() -> Option<String> {
    let out = std::process::Command::new("sysctl")
        .args(["-n", "machdep.cpu.brand_string"])
        .output()
        .unwrap();

    let raw = String::from_utf8_lossy(&out.stdout).trim().to_string();

    let out = Command::new("sysctl")
        .arg("-n")
        .arg("hw.physicalcpu")
        .output()
        .expect("Failed to execute command");

    let cores = String::from_utf8_lossy(&out.stdout).trim().to_string();

    Some(format!("{} ({})", raw, cores))
}

fn gpu_info() -> Option<String> {
    let output = Command::new("system_profiler")
        .arg("SPDisplaysDataType")
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut model = String::new();
    let mut cores = String::new();

    for line in stdout.lines() {
        let line = line.trim();
        if line.starts_with("Chipset Model:") {
            model = line.replace("Chipset Model:", "").trim().to_string();
        } else if line.starts_with("Total Number of Cores:") {
            cores = line
                .replace("Total Number of Cores:", "")
                .trim()
                .to_string();
        }
    }

    Some(format!("{} ({})", model, cores))
}

pub fn show_info(out: &mut BufWriter<StdoutLock>) {
    let hostname = get_host_info();

    if let Some(hostname) = &hostname {
        println!("\x1b[0m {}", hostname);
    }

    println!("\x1b[34m=========================\x1b[0m");

    if let Some(os) = get_os_info() {
        println!("\x1b[34mOS:\x1b[0m {}", os);
    }

    if let Some(hostname) = hostname {
        println!("\x1b[34mHost:\x1b[0m {}", hostname);
    }

    if let Some(cpu) = cpu_info() {
        println!("\x1b[34mCPU:\x1b[0m {}", cpu);
    }

    if let Some(gpu) = gpu_info() {
        println!("\x1b[34mGPU:\x1b[0m {}", gpu);
    }
}
