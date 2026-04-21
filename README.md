# frust

A minimal system fetch tool written in Rust. Shows your system info next to your distro's ascii art. That's it.

![]("https://github.com/user-attachments/assets/ced1b260-83ea-4b2f-8e82-f39cd1ab957b">)

## What it shows

- Hostname and kernel version
- OS name, uptime, shell
- CPU model
- GPU(s) via `lspci`
- RAM usage and swap
- Disk usage for `/`
- How long it took to run and how much memory it used

## Supported distros

Arch, Alpine, Asahi, CachyOS, EndeavourOS, Ubuntu, Debian, Fedora, NixOS — anything else falls back to a generic Linux logo.

## Building

You need Rust installed. Then:

```sh
git clone https://github.com/temidaradev/frust
cd frust
cargo build --release
```

Binary ends up at `target/release/frust`. Move it wherever you want it.

```sh
sudo cp target/release/frust /usr/local/bin/
```

## Running

```sh
frust
```

No flags, no config file. It reads everything straight from `/proc` and `/etc`.

## Dependencies

- `libc` — for the `statvfs` disk stat call
- `lspci` — needs to be installed for GPU detection (part of `pciutils`)

