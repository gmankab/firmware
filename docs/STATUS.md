# Reconstruction status

Updated: **22 October 2026**. Only results verified against the reference inputs are described as exact matches.

## Firmware inventory

| Original APK file | Current result | Remaining work |
|---|---|---|
| `modemuw.jsn` | **Complete, 713 bytes byte-identical** from Rust | No executable code in this JSON configuration |
| `a640_zap.mbn` | 4,096-byte ELF prefix exact; **140 distinct QRISC executable bytes exact** from typed Rust | Remaining QRISC routines, 512-byte embedded IR3 region, data and signed container |
| `adsp.mbn` | 4,096-byte ELF metadata prefix exact | Hexagon executable and data reconstruction; authentication |
| `cdsp.mbn` | 4,096-byte ELF metadata prefix exact | Hexagon executable and data reconstruction; authentication |
| `modem.mbn` | 4,096-byte ELF metadata prefix exact | Hexagon executable and data reconstruction; authentication |
| `wlanmdsp.mbn` | 4,096-byte ELF metadata prefix exact | Hexagon executable and data reconstruction; authentication |
| `venus.mbn` | 4,096-byte ELF metadata prefix exact | ARM executable and data reconstruction; authentication |
| `cs35l41-dsp1-spk-prot.wmfw` | 128 metadata bytes exact (WMFW header and descriptors) | Cirrus HALO DSP code and remaining container contents |
| `novatek_nt36523_fw.bin` | Packaged revision and checksums verified | Architecture, executable and container reconstruction |

**Fully rebuilt files: 1 of 9.** Metadata and snippet matches must not be counted as complete images.

## A640: what works now

- Mesa Freedreno decodes the image as **QRISC control code with an embedded IR3 shader**, despite the ELF wrapper's Hexagon machine identifier. Earlier Hexagon V4 interpretation was misleading.
- A **private, vendor-derived** assembly experiment reproduces the complete 1,968-byte executable segment; it remains outside Git and is **not** GPLv3-clean source.
- The public Rust `nabu-qrisc` crate generates the first **120 bytes** as named instruction operations for security dispatch and early status polling. This includes the earlier 16-byte preemption guard.
- A separate **20-byte** idle routine also matches. Total distinct independently *encoded* executable bytes: **140**. The opcode sequence and behavior were informed by vendor analysis, so copyright independence is **not established**.
- The conservative static control-flow analyzer visits **352 of 492** 32-bit positions as reachable QRISC words, with 15 branch destinations, 4 call destinations, and 8 distinct return locations. It avoids the shader data region. These are possible paths, not demonstrated hardware behavior.
- QRISC field decoding/re-encoding handles the known control-region instruction fields; the four unusual normal-`ret` encodings remain modeled as uninterpreted bits. IR3 instruction semantics are not yet reconstructed.

## Constraints and next milestones

1. Extend the **typed QRISC program** into additional complete security-state routines, using the reference binary only for independent verification rather than embedding its instructions.
2. Express the embedded **IR3 program** as semantic Rust instructions. A byte-for-byte shader match needs the exact encodings and instruction ordering; full semantic equivalence is not yet shown.
3. Reconstruct loadable data, ELF layouts, and authentication metadata, and verify a complete image with `cmp` and SHA-256.
4. Investigate the remaining Hexagon, ARM, HALO and Novatek firmware independently.
5. Validate licensing and device authentication; modified signed Qualcomm payloads normally require valid vendor-trusted signatures. No firmware has been boot-tested.

A byte-identical derivative made from vendor instructions is a useful reproducibility check, **not automatically a permissible GPLv3 distribution**. All OEM-derived listings and raw firmware stay in separate private analysis storage.
