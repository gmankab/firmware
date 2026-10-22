# Firmware implementation status

Updated **22 October 2026**. A byte match means an actual comparison with a SHA-256-checked reference, not merely that a decoder accepted the bytes.

## Verified outputs

| Reference component | Regenerated independently using Rust | What is not yet implemented |
| --- | --- | --- |
| Xiaomi Nabu `modemuw.jsn` | **713 / 713 bytes**, complete text configuration | No executable code in this file |
| Adreno A640 `a640_zap.mbn` | **820 / 1,968 bytes** in executable-marked segment; 4,096-byte ELF metadata prefix | Remaining QRISC control instructions, 6 uncovered IR3 shader slots, data, authentication payload |
| `adsp.mbn`, `cdsp.mbn`, `modem.mbn`, `wlanmdsp.mbn` | 4,096-byte ELF metadata prefix per file | Hexagon V66 machine code, data and authentication |
| `venus.mbn` | 4,096-byte ELF metadata prefix | ARM32 machine code, data and authentication |
| `cs35l41-dsp1-spk-prot.wmfw` | 128 bytes of WMFW container metadata | HALO DSP executable, DSP memory region payloads |
| `novatek_nt36523_fw.bin` | Reference revision/hash verified | Precise MCU ISA, container description, executable |

**Complete byte-identical files: 1 / 9.** The generated A640 bytes are disjoint fragments, **not** a single bootable executable or a signed `.mbn`.

### A640 verified byte accounting

| Region | Bytes | How generated |
| --- | ---: | --- |
| Security/command-control prefix | 336 | Typed QRISC instructions with symbolic control-flow labels |
| GPU idle routine | 20 | Typed QRISC register reads, branch, return |
| Shader register initialization bank, 48 IR3 slots | 384 | Typed `mov.u32u32` with repeat and register operands |
| Additional IR3 shader initialization/control/termination, 10 slots | 80 | Typed immediate moves, `nop`, predicate branch, backedge `jump`, `end` |
| **Total** | **820** | **41.7% of the 1,968-byte executable segment** |

The shader contains 64 eight-byte slots. **58 slots** are now generated from typed instructions; the six unimplemented positions are slots 0, 5, 8, 9, 10 and 61. Slot 61 remains unrecognized by the tested Mesa IR3 disassembler. All generated fragments pass direct byte comparisons, but the operation order was informed by OEM analysis; legal independence is not established.

### Verification distinctions

- Mesa's QRISC assembler also reproduces all 1,968 bytes in a **private, OEM-derived** assembly experiment. This establishes encoding feasibility, not source-level reimplementation and not independently licensable firmware.
- The A640 ELF wrapper has a Hexagon machine identifier, but its executable-marked segment contains Adreno QRISC control microcode with embedded IR3 shader code. Do not translate this segment as Hexagon V4.
- QRISC field round-trips and possible-control-flow analysis establish structural consistency, not execution equivalence or secure-state safety.
- All nine reference hashes are checked against the relevant postmarketOS APK revision. The Novatek image must use the packaged revision rather than a different upstream checkout.

## Next milestones

1. Implement the remaining shader slots as semantic, typed operations; investigate the unrecognized slot without inserting a raw OEM opcode.
2. Extend the A640 QRISC control implementation beyond the first 84 words, with explicit call/branch validation and verified register semantics.
3. Generate all file-backed non-authentication data and reconstruct the exact layout and build boundaries; compare complete images, not only snippets.
4. Investigate and implement Hexagon V66, ARM32, Cirrus HALO and the Novatek MCU on their respective targets, reusing shared parsers and instruction backends.
5. Evaluate signature requirements independently. A changed Qualcomm signed payload normally cannot be authenticated using an old signature. Validate any bootable build on real, recoverable hardware before claiming device support.

Nothing in the repository presently replaces signed vendor executables on a device. `modemuw.jsn` is complete; the remaining eight firmware files are incomplete.
