# Building and validating firmware

## Development environment

Install Rust 1.99.1 or newer, Cargo, Python 3, GNU coreutils, Git, and OpenSSL 3.x for cryptographic verification. `rustfmt` is recommended for formatting checks. The current workspace has **no third-party Cargo dependencies** and can be built offline once the Rust standard library is present.

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
  emit-a640-executable /tmp/firmware-output/a640-executable.bin
cargo run --offline --locked -p firmware-tools -- \
  emit-a640-fragments /tmp/firmware-output
cargo run --offline --locked -p firmware-tools -- \
  rebuild-modemuw /tmp/firmware-output/modemuw.jsn
cargo run --offline --locked -p firmware-tools -- \
  verify-a640-routines /private/firmware/a640_zap.mbn
cargo run --offline --locked -p firmware-tools -- \
  verify-modemuw /private/firmware/modemuw.jsn
```

The firmware-tools crate also provides a **device-independent checked image planner** (`ImagePlan`). The current A640 plan inserts each generated QRISC/IR3 operation at its verified offset and rejects overlap. Its **executable segment is complete**, but the encompassing signed MBNv6 ELF has missing OEM authentication material. The full-file plan still refuses finalization:

```sh
cargo run --offline --locked -p firmware-tools -- a640-coverage
cargo run --offline --locked -p firmware-tools -- a640-container-coverage
```

**Locally rebuild A640 using privately supplied OEM authentication:**

The 6,400-byte OEM signature and certificate field must be supplied from a
legally obtained firmware file of the **same revision**. This is intended only
for private, byte-identical reproduction of an existing signed image. It is not
an independent signing method and must not be checked into Git.

```sh
# The OEM signature and certificate field occupies [0x1138, 0x2a38).
python3 - /private/a640_zap.mbn /private/a640-oem-auth.bin <<'PYCODE'
import sys
from pathlib import Path
reference = Path(sys.argv[1]).read_bytes()
if len(reference) != 14256:
    raise SystemExit('unexpected A640 reference image size')
Path(sys.argv[2]).write_bytes(reference[0x1138:0x2a38])
PYCODE

cargo run --offline --locked -p firmware-tools -- \
  assemble-a640-with-external-auth \
  /private/a640-oem-auth.bin /private/a640-rebuilt.mbn
cmp /private/a640_zap.mbn /private/a640-rebuilt.mbn
python3 tools/verify/verify-mbn-signatures.py /private/a640-rebuilt.mbn
```

The assembler checks all MBNv6 SHA-384 ELF segment hashes. The separate
Python/OpenSSL verifier checks the RSA-PSS/SHA-256 signature over the MBNv6
header, signer metadata and hash table, and verifies X.509 chain signatures
against the **embedded** root certificate. It does **not** establish that the
hardware fuse or secure monitor trusts this certificate. It accepts the
optional `--expected-root-sha256` argument for a user-pinned root certificate.
Without that argument the root is untrusted external data.

For any ELF32/MBNv6 image with a single OEM signer (including another device):

```sh
python3 tools/verify/verify-mbn-signatures.py /private/other-device/firmware.mbn
```

Images with a second QTI signature currently require additional format support;
the verifier rejects them rather than claiming to have checked all signers.

**Venus ARM32 reconstruction:**

```sh
cargo run --offline --locked -p firmware-tools -- \
  emit-venus-contiguous-boot /tmp/venus-early.bin
cargo run --offline --locked -p firmware-tools -- \
  verify-venus-contiguous-boot /private/venus.mbn
```

The generated region spans **27,388 consecutive bytes** at file offset `0x4000`. It includes 4,096 bytes of boot-page content (1,076 bytes of structured ARM32 code and constants, and 3,020 bytes of checked zero padding) plus **23,292 bytes of additional ARM32 instructions and literal pools**. The `arm32` crate encodes its supported instructions as typed operands, and `ImagePlan` enforces exact adjacency and rejects gaps. The remaining executable sections, data, signed metadata and authentication material have not been reproduced; this output is **not a runnable firmware image**.

The first command should report **1,968 / 1,968 executable bytes**. The container command reports **7,856 / 14,256 bytes**, leaving the original signature and certificate-chain bytes ungenerated. The container is **not flashable**. Do not fill those gaps with placeholder signatures, extra zero padding or vendor bytes disguised as GPLv3 source. A complete layout is necessary, not sufficient, for a valid signed firmware.

`emit-a640-fragments` emits individual `.bin` files for implementation inspection. Their positions matter: **do not concatenate those fragments and flash them**. `emit-a640-executable` emits the now-complete QRISC/IR3 segment, but this is still *not* a complete `.mbn` and must not be flashed. Slot 61's typed encoding matches the original; its precise synchronization semantics have not been established.

For an extracted postmarketOS firmware tree with the expected `qcom`, `cirrus`, and `novatek` paths, run the full device-specific comparison:

```sh
tools/verify-xiaomi-nabu.sh /private/firmware-tree
```

This script checks all nine packaged hashes, generated executable regions and metadata, the text configuration, all six Qualcomm MBNv6 SHA-384 tables, and the contiguous Venus boot region. When OpenSSL is present, it also verifies the six OEM RSA-PSS signatures and embedded certificate chains. Neither check establishes **hardware root-of-trust acceptance** or functionality on a real device. Keep `/private/firmware-tree` outside the project checkout.

For **any** compatible device with an ELF32-wrapped Qualcomm MBNv6 image:

```sh
cargo run --offline --locked -p firmware-tools -- \
  verify-mbnv6-hashes /private/other-device/firmware.mbn
```

This command checks the relationship between ELF segments and the SHA-384 digest table, and reports signed-region boundaries. It is **not a signature or certificate-chain verifier**; a modified attacker-produced image can carry internally consistent hashes yet fail device authentication.

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
