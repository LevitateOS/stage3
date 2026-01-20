# CLAUDE.md - Stage3 Builder

## What is stage3?

Stage3 tarball builder for LevitateOS. Creates the base system archive that gets extracted during installation.

## Development

```bash
# Build
cargo build

# Run
cargo run

# Check with clippy
cargo clippy
```

## Common Mistakes

1. **Missing files** - Ensure all required base system files are included
2. **Wrong permissions** - File permissions must be preserved in the tarball
3. **Path issues** - Use relative paths within the tarball

## Output

The builder produces a compressed tarball containing the base LevitateOS system ready for extraction to a target partition.
