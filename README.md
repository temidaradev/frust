# fust

A minimal system fetch tool written in Rust. Shows your system info next to your distro's ascii art. That's it.

<img width="819" height="272" alt="Screenshot_20260421_154924" src="https://github.com/user-attachments/assets/6e7b0479-e901-4c2d-b08c-536ce0f61efc" />

## What it shows

- Hostname and kernel version
- OS name, uptime, shell
- CPU model
- GPU(s) via `pci-ids`
- RAM usage and swap
- Disk usage for `/`
- How long it took to run and how much memory it used

## Supported distros

Arch, Alpine, Asahi, CachyOS, EndeavourOS, Ubuntu, Debian, Fedora, NixOS — anything else falls back to a generic Linux logo.

## Building

You need Rust installed. Then:

```sh
git clone https://github.com/temidaradev/fust
cd fust
cargo build --release
```

Binary ends up at `target/release/fust`. Move it wherever you want it.

```sh
sudo cp target/release/fust /usr/local/bin/
```

## Running

```sh
fust
```

No flags, no config file. It reads everything straight from `/proc` and `/etc`.

## Dependencies

- `libc` — for the `statvfs` disk stat call
- `pci-ids` — bundled PCI ID database for GPU detection (no external tools needed)

## How GPU detection works

Reads `/sys/class/drm/*/device/uevent`, parses `PCI_ID=VENDOR:DEVICE`, looks up vendor and device name via the `pci-ids` crate. No `lspci` or `pciutils` required.

