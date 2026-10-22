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

Generate the complete **1,968-byte A640 executable segment** without reading an OEM file (this is not a signed, flashable `.mbn`):

```sh
cargo run --offline --locked -p firmware-tools -- \
  emit-a640-executable /tmp/a640-executable.bin
```

Alternatively, generate separately inspectable A640 components:

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

Generate the verified **contiguous early Venus ARM32 boot and runtime region**. This is a partial image, not a complete or signed firmware file:

```sh
cargo run --offline --locked -p firmware-tools -- \
  emit-venus-contiguous-boot /tmp/venus-early.bin
cargo run --offline --locked -p firmware-tools -- \
  verify-venus-contiguous-boot /private/venus.mbn
```

For a locally reconstructed full A640 image, the original OEM authentication
region can be passed *externally* to the builder. This does not turn the OEM
bytes into GPLv3 code and cannot sign an altered payload. The generic
[signature verifier](BUILDING.md) checks the original embedded chain and RSA-PSS
signature without claiming hardware trust.

View full executable and signed-container coverage without using a vendor image:

```sh
cargo run --offline --locked -p firmware-tools -- a640-coverage
cargo run --offline --locked -p firmware-tools -- a640-container-coverage
```

**Current result:** the 713-byte text configuration, the entire 1,968-byte A640 executable segment, and the first **9,484 contiguous bytes of Venus boot/runtime content** (including the 4 KiB boot page) are byte-identical. The signed A640 container is only 7,856 of 14,256 bytes independently generated; the remaining 6,400 bytes can be supplied privately to reproduce the original complete file, but are not original GPLv3 output. No rebuilt executable has been hardware-tested. See [current status](STATUS.md) for the precise remaining gaps.

## Guides

- [Decompilation handbook](DECOMPILATION.md) — thorough, reproducible workflow for new firmware files and new devices: triage, processor identification, Ghidra, LLVM, Rizin, Freedreno, custom containers, interpretation and verification.
- [Building and validating](BUILDING.md) — Rust targets, reproducible builds, checksum checks, tests, adding support for a device, and device authentication considerations.
- [Current status](STATUS.md) — measured rebuild coverage, blocked components, and the next implementation milestones.

## Source and distribution

The original code in this repository is licensed **GPL-3.0-only** (see `LICENSE`). Reference OEM firmware, private disassemblies, vendor-authored programs and cryptographic signing material are **not part of the source tree** and are not covered by the project's GPL license. Keep them in private directories outside the checkout.

This project uses firmware disassembly to understand formats and behavior. Rewriting code in Rust, or obtaining identical executable bytes, does not automatically eliminate copyright, licensing, patent or authentication constraints. Assess these separately before distributing images as GPLv3 binaries.
