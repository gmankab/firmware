# Firmware implementation status

Updated **22 October 2026**. The project's practical goal is working firmware built from auditable Rust source for multiple devices, using byte-identical reconstruction as the first acceptance test. Xiaomi Nabu is the first target.

## Verified outputs

| Firmware | Independently generated, byte-identical content | Remaining work |
| --- | --- | --- |
| `modemuw.jsn` | **713 / 713 bytes**, complete | No executable code (text configuration) |
| `a640_zap.mbn` | **1,156 / 1,968 executable-segment bytes**; 4,096-byte ELF metadata prefix | Unimplemented QRISC and IR3, other data, authentication |
| `adsp.mbn`, `cdsp.mbn`, `modem.mbn`, `wlanmdsp.mbn` | 4,096-byte ELF metadata prefix each | Hexagon V66 programs, data, authentication |
| `venus.mbn` | 4,096-byte ELF metadata prefix | ARM32 program, data, authentication |
| `cs35l41-dsp1-spk-prot.wmfw` | 128 bytes of container metadata | HALO DSP executable and payloads |
| `novatek_nt36523_fw.bin` | Reference revision and checksum verified | Identify the actual MCU ISA and firmware layout |

**Complete byte-identical files: 1 / 9.** No executable image has been built or boot-tested. A byte match below means a direct comparison with the SHA-256-verified reference, not just that a decoder accepted the instructions. The typed Rust implementations are analysis-informed; byte equality does not by itself establish independent licensing provenance.

## A640 coverage

| Nonoverlapping implemented region | Offset(s) within executable segment | Exact bytes |
| --- | --- | ---: |
| QRISC security dispatch | `0x000–0x150` | 336 |
| QRISC protected shader register setup | `0x150–0x1d4` | 132 |
| QRISC compute register configuration | `0x1d4–0x278` | 164 |
| IR3 shader, 63 independently encoded slots | Scattered in `0x440–0x640` | 504 |
| QRISC idle/return routine | `0x79c–0x7b0` | 20 |
| **Total** | | **1,156 / 1,968 (58.7%)** |

The embedded IR3 shader has **63 of 64 slots** encoded from typed Rust operations. The remaining slot (`61`) looks like a category-7 barrier, but fails Mesa's documented bit-pattern requirements. Its behavior is **not established**, and the source does not embed an unexplained vendor instruction in place of an implementation.

The generic `ImagePlan` refuses to emit an image with missing bytes, reports overlap and out-of-bounds errors, and compares implemented regions independently. Its current missing ranges are:

```text
0x278–0x440  456 bytes — QRISC control program
0x628–0x630    8 bytes — unknown IR3 instruction at slot 61
0x640–0x79c  348 bytes — QRISC control program
```

The three gaps total **812 bytes**. Even after they are implemented, the full `.mbn` includes other segments, offsets, metadata, and signed content that must be accounted for separately.

Reproduce the progress and source comparisons with:

```sh
cargo run --offline --locked -p firmware-tools -- a640-coverage
cargo run --offline --locked -p firmware-tools -- \
  verify-a640-routines /private/a640_zap.mbn
```

## Validation and limits

- **47 Rust tests, 6 Python tests**, and an offline release build pass.
- All nine reference files match the postmarketOS package. The Novatek revision must be the one from the package, not a different upstream checkout.
- Six ELF prefixes, the Cirrus WMFW metadata and `modemuw.jsn` match. Typed A640 executable fragments match at their exact offsets.
- Mesa's private OEM-derived assembly round-trip reproduced the entire A640 executable segment, but it is only a reference experiment, **not a Rust rebuild or redistributable clean-room source**.
- The `a640_zap.mbn` ELF header identifies machine 164 (Hexagon), but this executable segment contains Adreno QRISC plus embedded IR3. Do not feed the segment to a Hexagon decompiler.
- Secure firmware authentication, device boot, GPU operation and the legal status of decompilation-derived code are not validated.

## Next engineering steps

1. Determine the purpose of the nonstandard IR3 slot 61 and implement its *documented or independently verified* semantics. Keep an explicit gap until then.
2. Implement the remaining QRISC control regions from named operations, with validated branch/call targets and GPU register semantics. The `ImagePlan` catches overlaps and omissions.
3. Construct the full, deterministic container from independent executable and data generators. Identify all hashed/signed inputs before attempting a complete binary comparison.
4. Implement other Nabu processors in separate architecture-specific crates and add more devices through scoped device modules.
5. Validate a complete image and authentication behavior on a recoverable test device before considering it usable firmware.

Nothing here is a replacement for signed OEM executable firmware yet.
