# stage3

Stage3 tarball builder for LevitateOS. Creates the base system archive extracted during installation.

## Usage

```bash
cargo run -- build --source /path/to/rocky --output ./stage3.tar.zst
cargo run -- list ./stage3.tar.zst
cargo run -- verify ./stage3.tar.zst
```

## What's Included

- Bash shell
- Coreutils binaries
- Systemd init system
- PAM authentication
- System configuration (/etc)
- Recipe package manager

## What's NOT Included

- Kernel (installed separately)
- Bootloader (configured by installer)

## Development

```bash
cargo build        # Build
cargo test         # Run unit tests
cargo clippy       # Lint
```

## License

MIT
