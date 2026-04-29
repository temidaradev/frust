const RESET: &str = "\x1b[0m";

const COLORS: [&str; 10] = [
    "\x1b[0m",  // $0
    "\x1b[34m", // $1
    "\x1b[37m", // $2
    "\x1b[35m", // $3
    "\x1b[31m", // $4
    "\x1b[33m", // $5
    "\x1b[32m", // $6
    "\x1b[36m", // $7
    "\x1b[90m", // $8
    "\x1b[0m",  // $9
];

#[allow(dead_code)]
pub enum Distro {
    Arch,
    Alpine,
    Asahi,
    CachyOS,
    Endeavour,
    Ubuntu,
    Debian,
    Fedora,
    NixOS,
    MacOS,
    Unknown,
}

pub fn get_ascii_art(distro: &Distro) -> &'static str {
    match distro {
        Distro::Arch => include_str!("../ascii/arch.txt"),
        Distro::Alpine => include_str!("../ascii/alpine.txt"),
        Distro::Asahi => include_str!("../ascii/asahi.txt"),
        Distro::CachyOS => include_str!("../ascii/cachyos.txt"),
        Distro::Endeavour => include_str!("../ascii/endeavouros.txt"),
        Distro::Ubuntu => include_str!("../ascii/ubuntu.txt"),
        Distro::Debian => include_str!("../ascii/debian.txt"),
        Distro::Fedora => include_str!("../ascii/fedora.txt"),
        Distro::NixOS => include_str!("../ascii/nixos.txt"),
        Distro::Unknown => include_str!("../ascii/linux.txt"),
        Distro::MacOS => include_str!("../ascii/macos.txt"),
    }
}

pub fn colorize(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut chars = raw.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '$'
            && let Some(&next) = chars.peek()
                && let Some(idx) = next.to_digit(10) {
                    chars.next();
                    out.push_str(COLORS[idx as usize]);
                    continue;
                }
        out.push(c);
    }
    out.push_str(RESET);
    out
}

pub fn visible_width(s: &str) -> usize {
    let mut width = 0;
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            for nc in chars.by_ref() {
                if nc == 'm' {
                    break;
                }
            }
        } else {
            width += 1;
        }
    }
    width
}
