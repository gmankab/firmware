# Firmware Builder

**Goal:** produce working, redistributable firmware binaries from auditable GPLv3 Rust implementations, beginning with **byte-for-byte reproductions** of reference firmware. Xiaomi Pad 5 (`nabu`) is the first device; CPU, GPU and DSP instruction encoders and inspection tools are intended to be shared with other devices.

The long-term deliverable is usable firmware, **not** a collection of disassembly reports. The current project is incomplete: its generated microcode fragments must **not** be flashed as full firmware images. Byte equality of a fragment does not establish that the reconstructed code is independently licensable or that a changed, signed image will pass hardware authentication.

## Get started

Requires Rust 1.99.1 or newer, Python 3, and a local copy of the device's reference firmware for comparison. All Cargo dependencies are local workspace crates.

```sh
cargo test --workspace --offline --locked
cargo build --workspace --release --offline --locked
python3 -m unittest discover -s tests -p 'test_*.py'

# Inspect any supported ELF32, Cirrus WMFW or Novatek-candidate image:
cargo run --offline --locked -p firmware-tools -- inspect-image /path/to/image.mbn
```

Generate **partial** A640 firmware components without reading an OEM executable:

```sh
cargo run --offline --locked -p firmware-tools -- \
  emit-a640-fragments /tmp/a640-parts
```

Generate Nabu's fully reconstructed text configuration:

```sh
cargo run --offline --locked -p firmware-tools -- \
  rebuild-modemuw /tmp/modemuw.jsn
```

Verify generated executable fragments against your *own* copy of A640 firmware:

```sh
cargo run --offline --locked -p firmware-tools -- \
  verify-a640-routines /path/to/a640_zap.mbn
```

View exact coverage and the three remaining microcode gaps without using a vendor image:

```sh
cargo run --offline --locked -p firmware-tools -- a640-coverage
```

**Current result:** the 713-byte text configuration matches completely, and 1,156 of the 1,968 bytes in the A640 executable-marked segment match across independently encoded QRISC and IR3 fragments. Neither partial GPU code nor metadata is a working firmware replacement. See [current status](STATUS.md) for the precise remaining gaps.

## Guides

- [Decompilation handbook](DECOMPILATION.md) — thorough, reproducible workflow for new firmware files and new devices: triage, processor identification, Ghidra, LLVM, Rizin, Freedreno, custom containers, interpretation and verification.
- [Building and validating](BUILDING.md) — Rust targets, reproducible builds, checksum checks, tests, adding support for a device, and device authentication considerations.
- [Current status](STATUS.md) — measured rebuild coverage, blocked components, and the next implementation milestones.

## Source and distribution

The original code in this repository is licensed **GPL-3.0-only** (see `LICENSE`). Reference OEM firmware, private disassemblies, vendor-authored programs and cryptographic signing material are **not part of the source tree** and are not covered by the project's GPL license. Keep them in private directories outside the checkout.

This project uses firmware disassembly to understand formats and behavior. Rewriting code in Rust, or obtaining identical executable bytes, does not automatically eliminate copyright, licensing, patent or authentication constraints. Assess these separately before distributing images as GPLv3 binaries.
