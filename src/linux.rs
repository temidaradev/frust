use crate::ascii::{colorize, get_ascii_art, visible_width};
use std::fs;
use std::io::BufWriter;
use std::io::StdoutLock;
use pci_ids::Device;

const BLUE: &str = "\x1b[34m";
const RESET: &str = "\x1b[0m";

use crate::ascii::Distro;

fn label(key: &str, value: &str) -> String {
    format!("{BLUE}{key}:{RESET} {value}")
}

fn detect_distro() -> Distro {
    let content = fs::read_to_string("/etc/os-release").unwrap_or_default();
    let id = content
        .lines()
        .find(|l| l.starts_with("ID="))
        .and_then(|l| l.split('=').nth(1))
        .unwrap_or("")
        .trim_matches('"')
        .to_lowercase();

    match id.as_str() {
        "arch" => Distro::Arch,
        "alpine" => Distro::Alpine,
        "asahi" => Distro::Asahi,
        "cachyos" => Distro::CachyOS,
        "endeavouros" => Distro::Endeavour,
        "ubuntu" => Distro::Ubuntu,
        "debian" => Distro::Debian,
        "fedora" => Distro::Fedora,
        "nixos" => Distro::NixOS,
        _ => Distro::Unknown,
    }
}

fn get_os_info() -> Option<String> {
    let content = fs::read_to_string("/etc/os-release").ok()?;
    let name = content
        .lines()
        .find(|l| l.starts_with("PRETTY_NAME="))?
        .strip_prefix("PRETTY_NAME=")?
        .trim_matches('"')
        .to_string();
    Some(label("\u{f17c} OS", &name))
}

fn get_cpu_info() -> Option<String> {
    let content = fs::read_to_string("/proc/cpuinfo").ok()?;
    let model = content
        .lines()
        .find(|l| l.starts_with("model name"))?
        .split(':')
        .nth(1)?
        .trim()
        .to_string();
    Some(label("\u{f2db} CPU", &model))
}


fn get_gpu_info() -> Option<String> {
    let entries = fs::read_dir("/sys/class/drm").ok()?;

    let gpus: Vec<String> = entries
    .filter_map(|e| e.ok())
    .filter(|e| {
        let n = e.file_name();
        let s = n.to_string_lossy();
        s.starts_with("card") && !s.contains('-')
    })
    .filter_map(|e| {
        let uevent = fs::read_to_string(
            e.path().join("device/uevent")
        ).ok()?;

        let pci_id = uevent.lines()
        .find(|l| l.starts_with("PCI_ID="))?
        .strip_prefix("PCI_ID=")?
        .trim();

        let mut parts = pci_id.split(':');
        let vendor_id = u16::from_str_radix(parts.next()?, 16).ok()?;
        let device_id = u16::from_str_radix(parts.next()?, 16).ok()?;

        Device::from_vid_pid(vendor_id, device_id)
        .map(|d| format!("{} {}", d.vendor().name(), d.name()))
    })
    .collect();

    if gpus.is_empty() { return None; }

    Some(
        gpus.iter()
        .map(|g| label("\u{f26c} GPU", g))
        .collect::<Vec<_>>()
        .join("\n"),
    )
}

fn get_mem_info() -> Option<String> {
    let content = fs::read_to_string("/proc/meminfo").ok()?;

    let parse_kb = |prefix: &str| -> Option<f64> {
        content
            .lines()
            .find(|l| l.starts_with(prefix))?
            .split(':')
            .nth(1)?
            .split_whitespace()
            .next()?
            .parse::<f64>()
            .ok()
    };

    let mem_total = parse_kb("MemTotal:")?;
    let mem_used = mem_total - parse_kb("MemFree:")? - parse_kb("Buffers:")? - parse_kb("Cached:")?;
    let swap_total = parse_kb("SwapTotal:")?;
    let swap_used = swap_total - parse_kb("SwapFree:")?;

    let gib = |kb: f64| kb / 1_048_576.0;

    Some(label(
        "\u{f233} Memory",
        &format!(
            "{:.2} / {:.2} GiB\n{BLUE}\u{f0ec} Swap:{RESET} {:.2} / {:.2} GiB",
            gib(mem_used),
            gib(mem_total),
            gib(swap_used),
            gib(swap_total),
        ),
    ))
}

fn get_uptime_info() -> Option<String> {
    let seconds = fs::read_to_string("/proc/uptime")
        .ok()?
        .split_whitespace()
        .next()?
        .parse::<f64>()
        .ok()?;
    Some(label("\u{f017} Uptime", &format!("{:.2} hours", seconds / 3600.0)))
}

fn get_shell_info() -> Option<String> {
    Some(label("\u{f120} Shell", &std::env::var("SHELL").ok()?))
}

fn get_disk_info() -> Option<String> {
    let path = c"/";
    let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
    
    if unsafe { libc::statvfs(path.as_ptr(), &mut stat) } != 0 {
        return None;
    }

    let gib = |b: u64| b as f64 / 1_073_741_824.0;
    let total = stat.f_blocks * stat.f_frsize;
    let avail = stat.f_bavail * stat.f_frsize;
    let used  = total - avail;

    Some(label(
        "\u{f0a0} Disk",
        &format!("{:.2} / {:.2} GiB", gib(used), gib(total)),
    ))
}

pub fn show_info(_out: &mut BufWriter<StdoutLock>) {
    let distro = detect_distro();
    let colored_art = colorize(get_ascii_art(&distro));
    let art: Vec<&str> = colored_art.lines().collect();
    let mut infos: Vec<String> = Vec::new();

    if let Ok(h) = fs::read_to_string("/etc/hostname") {
        infos.push(label("\u{f108} Host", h.trim()));
    }
    if let Ok(k) = fs::read_to_string("/proc/sys/kernel/osrelease") {
        infos.push(label("\u{f17c} Kernel", &format!("Linux {}", k.trim())));
    }

    for line in [
        get_os_info(),
        get_uptime_info(),
        get_shell_info(),
        get_cpu_info(),
        get_gpu_info(),
        get_mem_info(),
        get_disk_info(),
    ].into_iter().flatten() {
        for sub in line.lines() {
            infos.push(sub.to_string());
        }
    }

    let art_width = art.iter().map(|l| visible_width(l)).max().unwrap_or(0);
    let max_lines = art.len().max(infos.len());

    for i in 0..max_lines {
        let art_line = art.get(i).copied().unwrap_or("");
        let info_line = infos.get(i).map(|s| s.as_str()).unwrap_or("");
        let pad = art_width.saturating_sub(visible_width(art_line));
        println!("{}{}  {}", art_line, " ".repeat(pad), info_line);
    }
}
