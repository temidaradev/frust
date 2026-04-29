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

    let pid = std::process::id();
    let ram = std::process::Command::new("ps")
        .args(["-o", "rss=", "-p", &pid.to_string()])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().parse::<u64>().unwrap_or(0))
        .unwrap_or(0);

    println!("\ntime: {:.2?} | ram: {} KB", elapsed, ram);
}
