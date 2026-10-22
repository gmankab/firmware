# Building and validating firmware

## Development environment

Install Rust 1.99.1 or newer, Cargo, Python 3, GNU coreutils, and Git. `rustfmt` is recommended for formatting checks. The current workspace has **no third-party Cargo dependencies** and can be built offline once the Rust standard library is present.

```sh
cargo test --workspace --offline --locked
cargo build --workspace --release --offline --locked
python3 -m unittest discover -s tests -p 'test_*.py'
# When rustfmt is installed:
cargo fmt --all --check
```

`tools/verify.sh` runs the build and synthetic tests. These do not require OEM firmware and are appropriate for CI. Rust's high-level code and encoder test results should remain reproducible without depending on private files.

## Generate and verify outputs

**Device-independent inspection:**

```sh
cargo run --offline --locked -p firmware-tools -- inspect-image /path/to/firmware
cargo run --offline --locked -p firmware-tools -- inspect-elf32 /path/to/elf32
```

**Current Xiaomi Nabu target:**

```sh
cargo run --offline --locked -p firmware-tools -- \
  emit-a640-fragments /tmp/firmware-output
cargo run --offline --locked -p firmware-tools -- \
  rebuild-modemuw /tmp/firmware-output/modemuw.jsn
cargo run --offline --locked -p firmware-tools -- \
  verify-a640-routines /private/firmware/a640_zap.mbn
cargo run --offline --locked -p firmware-tools -- \
  verify-modemuw /private/firmware/modemuw.jsn
```

The firmware-tools crate also provides a **device-independent checked image planner** (`ImagePlan`). The current A640 plan inserts each generated QRISC/IR3 fragment at its verified offset, rejects overlap, compares only implemented regions and refuses to emit a full binary when any byte is unresolved:

```sh
cargo run --offline --locked -p firmware-tools -- a640-coverage
```

The result is a coverage report, **not** a flashable file. Implemented fragments must not be padded with zeroes or original vendor bytes to bypass the completeness check. A complete plan is a necessary, not sufficient, requirement for a valid signed firmware container.

The A640 command emits partial `.bin` files named by the starting IR3 shader slot. Their positions matter: **do not concatenate those fragment files and flash them**. The IR3 start and end offsets are determined by the image's QRISC jump-over-shader layout, and intervening shader instructions have not all been implemented.

For an extracted postmarketOS firmware tree with the expected `qcom`, `cirrus`, and `novatek` paths, run the full device-specific comparison:

```sh
tools/verify-xiaomi-nabu.sh /private/firmware-tree
```

This script checks all nine packaged hashes and the already implemented fragments, metadata prefixes and text configuration. It does **not** certify hardware functionality. Keep `/private/firmware-tree` outside the project checkout.

## Target selection and new devices

A device must provide its own input inventory, architecture information, memory map and build parameters. Nabu's reference values are intentionally scoped to the `xiaomi_nabu` device module; the `qrisc` and `ir3` crates have no device identity requirement.

For another device:

1. Identify the original image and exact firmware revision, then make a hash manifest. See [Decompilation](DECOMPILATION.md).
2. Establish whether it actually uses a known ISA or container. Do not infer the processor solely from the filename or ELF `e_machine` field. Reuse `qrisc` or `ir3` for supported Adreno chips; use a separate backend for unrelated architectures.
3. Add a new device module under `crates/firmware-tools/src/devices/` for product-specific register settings, metadata, and executable implementations. Shared encoders remain hardware-family-specific rather than device-specific.
4. Add synthetic tests that build without reference firmware, followed by external byte-comparison tests against the exact target image. Include reference hashes, memory offsets, and hardware version in the verification report.
5. Work toward a complete, deterministic build. Do not claim executable compatibility solely because headers, instruction fields, or small slices match.

The existing `tools/generate-elf-specs.py` can produce **metadata-only** typed Rust headers from another ELF32 firmware set. Review the generated content before including it in a device module; executable payloads and signatures are not imported.

```sh
python3 tools/generate-elf-specs.py \
  /private/new-device/firmware /tmp/new-device-elf-specs.rs
```

## What byte-identical actually means

Verify both the full file and the interpretation of its contents:

```sh
cmp -- /private/reference.bin /tmp/rebuilt.bin
sha256sum /private/reference.bin /tmp/rebuilt.bin
wc -c /private/reference.bin /tmp/rebuilt.bin
# If they differ, inspect the first offset(s):
cmp -l /private/reference.bin /tmp/rebuilt.bin | head
```

A successful `cmp` means all bytes match for that exact firmware revision. For a signed or hashed container, record executable and data segment hashes and verify headers, alignment, trailing padding, authentication regions and all sidecar files. An identical image retains its identical signature bytes; it does not imply that newly modified code can be signed with a trusted key.

Verification tiers should be reported separately: **(1)** independently generated metadata; **(2)** assembler round-trip using source-derived instruction listings; **(3)** typed, authored implementation fragments; **(4)** complete byte-identical file; **(5)** authenticated boot and runtime testing on the hardware. A fragment must never be described as passing a later tier.

## Limitations

Many Qualcomm images are authenticated by processors or firmware services outside the Linux kernel, and a rebuilt but modified payload may fail signature verification. Secure-boot keys and acceptance policy are device-specific. Hardware validation should use a documented recovery procedure and a device you own; no fragment in this workspace should be installed as a full replacement.

Licensing must be reviewed independently of binary equality. Rewriting vendor-derived control flow from pseudocode is not automatically clean-room source, and shipping signed vendor bytes under a GPL label would not relicense them.
