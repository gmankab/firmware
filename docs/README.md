# Xiaomi Nabu firmware reconstruction

GPLv3-licensed Rust tools for understanding and reconstructing firmware used by the Xiaomi Pad 5 (`nabu`). **This is a research project, not a flashable firmware replacement.** It does not contain redistributed vendor executables or authentication material.

## Repository layout

```text
Cargo.toml                        # Virtual Cargo workspace
Cargo.lock                        # One lockfile for workspace members
crates/
  nabu-firmware-research/
    Cargo.toml
    src/                          # Format readers, metadata builders, CLI
  nabu-qrisc/
    Cargo.toml
    src/                          # A6xx QRISC ISA, assembler, flow checks, typed routines
experiments/
  hexagon-probe/                  # Standalone, special-toolchain experiment
reference/
  firmware.sha256                # Hashes of the nine original APK payloads
tests/                             # Synthetic Python tests
tools/                             # Relevant verification and analysis scripts
docs/                              # All human-readable documentation
LICENSE                            # GPL-3.0-only
```

OEM firmware, generated pseudocode, and private assembler results are kept outside Git.

The workspace root has **no `src/` and no `[package]`**. Each production crate owns its own `src/` and `Cargo.toml`. Workspace members share a lockfile and build directory. The Hexagon probe has a separate Cargo manifest because its bare-metal target requires an additional, nonstandard toolchain.

## Quick start

Run with an installed Rust 1.99.1 toolchain or compatible newer version:

```sh
cargo test --workspace --offline --locked
cargo build --workspace --release --offline --locked
python3 -m unittest discover -s tests -p 'test_*.py'
```

Generate the complete **text configuration** from Rust:

```sh
cargo run --offline --locked -p nabu-firmware-research -- \
  rebuild-modemuw /tmp/modemuw.jsn
```

Verify the newly authored QRISC routines against your own reference A640 file:

```sh
cargo run --offline --locked -p nabu-firmware-research -- \
  verify-a640-routines /path/to/a640_zap.mbn
cargo run --offline --locked -p nabu-firmware-research -- \
  inspect-a640-flow /path/to/a640_zap.mbn
```

These comparisons read local OEM firmware without adding it to the repository. Do **not** flash any generated fragment as firmware.

## Read next

- [Current reconstruction status](STATUS.md) — exact matches, counts and open problems.
- [Development and verification](DEVELOPMENT.md) — architecture, reproducible checks, authorship and evidence policy.

The implementation's byte matches are engineering findings. Decompilation or source transcription does not automatically grant rights to relicense vendor firmware, and stock-device secure-boot policies are not bypassed by building unsigned code.
