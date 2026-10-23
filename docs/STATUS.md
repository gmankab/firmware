# Firmware implementation status

Updated **23 October 2026**. The goal is reproducible, working firmware from auditable Rust implementations for multiple devices. Xiaomi Nabu is the first target; **byte-identical builds** are the initial acceptance test, before authentication and runtime verification on hardware.

## Verified outputs

| Reference file | Rebuilt from Rust and directly verified | Remaining work |
| --- | --- | --- |
| `modemuw.jsn` | **713 / 713 bytes**, complete configuration | No executable code; full reference file rebuilt |
| `a640_zap.mbn` | **1,968 / 1,968 executable bytes**; **7,856 / 14,256 complete-file bytes (55.1%)** | OEM signature (256 bytes) and certificate field (6,144 bytes) remain external inputs; no independent signing, verified shader semantics or device testing |
| `adsp.mbn`, `cdsp.mbn`, `modem.mbn`, `wlanmdsp.mbn` | Exact 4 KiB ELF metadata prefix; SHA-384 reference segment hashes independently validated | Hexagon V66 programs, additional data and cryptographic authentication |
| `venus.mbn` | Exact 4 KiB ELF metadata prefix; **27,904 contiguous ARM32 boot/runtime bytes** at file offset `0x4000` (including a 4,096-byte page with verified zero-fill); SHA-384 table and OEM signature checked | Remaining ARM32 program, data and independent authentication |
| `cs35l41-dsp1-spk-prot.wmfw` | 128 bytes of container metadata | Cirrus HALO DSP program and payloads |
| `novatek_nt36523_fw.bin` | Verified source-file revision and SHA-256 | Identify processor ISA and reconstruct program and container |

**Complete byte-identical reference files: 1 / 9.** The A640 executable is generated from typed Rust QRISC/IR3 operations, not embedded OEM opcode arrays. Its exact match is a byte-encoding result, not a validation of executable behavior, independent copyright provenance, or hardware support.

## A640 executable and container

The 1,968-byte segment now has no missing byte ranges:

| Executable component | Relative offset | Bytes |
| --- | --- | ---: |
| QRISC security dispatch | `0x000–0x150` | 336 |
| QRISC protected shader setup | `0x150–0x1d4` | 132 |
| QRISC compute-register configuration | `0x1d4–0x278` | 164 |
| QRISC compute command pipeline | `0x278–0x440` | 456 |
| IR3 shader (all 64 slots) | `0x440–0x640` | 512 |
| QRISC final security and event routines | `0x640–0x79c` | 348 |
| QRISC idle and return routine | `0x79c–0x7b0` | 20 |
| **Total** | | **1,968** |

**IR3 slot 61:** A typed encoder matches the original 8-byte category-7-like synchronization instruction, but the public Mesa decoder rejects this encoding. Its operational semantics remain **provisional**. Exact byte matching does not independently establish that the barrier is understood.

The independently generated container gap is **`0x1138–0x2a38` (6,400 bytes)**. It comprises the image's original 256-byte OEM signature followed by a 6,144-byte certificate-chain field. Rust regenerates the ELF prefix, MBNv6 header, named 120-byte OEM image-policy metadata, three SHA-384 hash entries, executable code and zero alignment padding. A complete `.mbn` is emitted **only if those OEM bytes are supplied externally**. On the exact reference version, the result is SHA-256-identical; this does not replace vendor signing credentials or prove hardware trust. See [Building](BUILDING.md).

## Verification

The Rust workspace has **122 unit tests** and the Python suite has **13 tests**; perform the full local check with:

```sh
cargo test --workspace --offline --locked
cargo build --workspace --release --offline --locked
python3 -m unittest discover -s tests -p 'test_*.py'

cargo run --offline --locked -p firmware-tools -- \
  emit-a640-executable /tmp/a640-executable.bin
cargo run --offline --locked -p firmware-tools -- \
  verify-a640-routines /private/a640_zap.mbn
cargo run --offline --locked -p firmware-tools -- a640-container-coverage
cargo run --offline --locked -p firmware-tools -- \
  verify-mbnv6-hashes /private/a640_zap.mbn
```

The generic `verify-mbnv6-hashes` also succeeds on `adsp.mbn`, `cdsp.mbn`, `modem.mbn`, `venus.mbn` and `wlanmdsp.mbn` from the verified postmarketOS package. It checks segment-data integrity using SHA-384; **it does not verify signatures or hardware trust**. Separately, `tools/verify-mbn-signatures.py` verifies the OEM RSA-PSS/SHA-256 signature and internal certificate chain on all six Nabu MBNs, and supports future compatible ELF32/MBNv6 images. All six share the same embedded Xiaomi root. Hardware trust remains unverified. `tools/verify-xiaomi-nabu.sh /private/firmware-tree` performs all six SHA-384 table checks, nine file hashes, generated metadata and executable comparisons, and the 27,904-byte Venus boot/runtime comparison. When OpenSSL is available it also verifies each OEM RSA-PSS signature and embedded X.509 chain.

## Next engineering priorities

1. Validate the exceptional IR3 synchronization instruction's actual behavior using the public ISA and, eventually, hardware traces. Preserve the current encoding-exact but semantics-unverified label.
2. Determine whether a redistributable replacement can satisfy independent firmware authentication requirements. **Regenerating an existing OEM signature requires its signing key or the original signed data.** Merely regenerating the hash table does not suffice. Do not put original OEM certificates or signatures into GPLv3 source and claim that they were relicensed.
3. Continue Venus ARM32 boot and runtime reconstruction beyond the first verified 27,904 contiguous bytes; establish each new code/data boundary using multiple disassemblers and direct byte comparisons. The latest contiguous region ends at ARM32 address `0x6d00`, corresponding to file offset `0xad00`.
4. Add other devices through separate device modules, reusing the generic MBNv6 verification, image-layout builder and appropriate ISA encoders.
5. Test a fully authenticated firmware image on recoverable hardware before describing it as usable firmware.

Neither successful byte reconstruction nor matching a vendor-derived assembly listing alone guarantees a clean-room license result. The program has not been flashed or boot-tested.
