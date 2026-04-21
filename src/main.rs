use std::io::Write;

mod ascii;
#[cfg(target_os = "macos")]
mod darwin;
#[cfg(target_os = "linux")]
mod linux;

fn main() {
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());

    #[cfg(target_os = "linux")]
    linux::show_info(&mut out);

    #[cfg(target_os = "macos")]
    darwin::show_info(&mut out);
}
