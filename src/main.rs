mod ascii;
#[cfg(target_os = "macos")]
mod darwin;
#[cfg(target_os = "linux")]
mod linux;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());

    #[cfg(target_os = "linux")]
    linux::show_info(&mut out);

    #[cfg(target_os = "macos")]
    darwin::show_info(&mut out);

    drop(out);

    let elapsed = start.elapsed();

    let ram = std::fs::read_to_string("/proc/self/status")
        .unwrap_or_default()
        .lines()
        .find(|l| l.starts_with("VmRSS:"))
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0);

    println!("\ntime: {:.2?} | ram: {} KB", elapsed, ram);
}
