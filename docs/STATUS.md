# Reconstruction status

Updated: **22 October 2026**. Only results verified against the reference inputs are described as exact matches.

## Firmware inventory

| Original APK file | Current result | Remaining work |
|---|---|---|
| `modemuw.jsn` | **Complete, 713 bytes byte-identical** from Rust | No executable code in this JSON configuration |
| `a640_zap.mbn` | 4,096-byte ELF prefix exact; **740 distinct executable bytes exact** from typed Rust (356 QRISC + 384 IR3) | Remaining QRISC routines, 16 unimplemented IR3 slots, data and signed container |
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
- The Rust `nabu-qrisc` crate generates the first **336 bytes** using named operations for security dispatch, protected-register setup and polling. This includes the 16-byte preemption guard.
- A separate **20-byte** QRISC idle routine matches. The `nabu-ir3` crate generates **384 bytes** (48 consecutive IR3 slots), zero-initializing vector registers through documented category-1 immediate-move fields. These three nonoverlapping regions total **740 / 1,968 executable bytes (37.6%)**. Their choice and ordering were informed by vendor analysis, so copyright independence is **not established**.
- The conservative static control-flow analyzer visits **352 of 492** 32-bit positions as reachable QRISC words, with 15 branch destinations, 4 call destinations, and 8 distinct return locations. It avoids the shader data region. These are possible paths, not demonstrated hardware behavior.
- QRISC field decoding/re-encoding handles the known control-region instruction fields; the four unusual normal-`ret` encodings remain modeled as uninterpreted bits. The remaining 16 IR3 slots are not yet independently implemented, including one slot the Mesa decoder does not understand.

## Constraints and next milestones

1. Extend the **typed QRISC program** beyond the first 84 words, using the reference binary only for verification rather than embedding its instruction words.
2. Express the remaining **16 IR3 shader slots** as semantic Rust operations and investigate the currently unrecognized slot. The completed zero-initialization sequence does not yet establish full shader behavior.
3. Reconstruct loadable data, ELF layouts, and authentication metadata, and verify a complete image with `cmp` and SHA-256.
4. Investigate the remaining Hexagon, ARM, HALO and Novatek firmware independently.
5. Validate licensing and device authentication; modified signed Qualcomm payloads normally require valid vendor-trusted signatures. No firmware has been boot-tested.

A byte-identical derivative made from vendor instructions is a useful reproducibility check, **not automatically a permissible GPLv3 distribution**. All OEM-derived listings and raw firmware stay in separate private analysis storage.
