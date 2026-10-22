# Nabu exact reconstruction: work begun, remaining steps (2026-10-22)

## Local verified progress (not executable firmware)

* All nine original extracted regular files were verified against existing APK SHA256 records.
* 713-byte `modemuw.jsn` is regenerated from semantic Rust data, unchanged.
* `src/elf32_prefix.rs` and `src/elf_specs.rs` now independently generate exactly **4,096 bytes** of the first page of each Qualcomm/ARM ELF32 file. Six `verify-elf-prefix` checks compare equal to original bytes. **No executable code or cryptographic material is in that output.**
* `src/wmfw_metadata.rs` builds the 40-byte Cirrus WMFW header and all 11 eight-byte region descriptors from typed declarations. All **128 metadata bytes** compare byte-for-byte equal to the local original, but zero HALO DSP payload bytes are reconstructed.
* `scripts/elf-byte-budget.py` classifies every original ELF file offset without exporting vendor content. Every unmapped/padding byte of all six MBNs is zero. `evidence/elf-byte-budget.json` shows auth bytes and executable-flagged bytes separately. PF_X ranges are **not necessarily all instructions**, so treating them as disassembly input blindly is wrong.
* Both Hexagon V66 and Hexagon V4 are present (A640 = `e_flags 0x3`). Existing bare-metal Rust probe proves V60 instruction generation and supports experimenting with `hexagonv66`, **not** V4. LLVM v6 historical target is the next candidate for V4 disassembly.

## Tooling handoff

Execute `scripts/codex-download-next-toolchain-ubuntu24.04.sh` on Ubuntu **24.04 amd64** with Internet access. This script **has been checked with `bash -n` on Debian; not executed on Ubuntu yet**. It fetches the missing libc++/ABI/unwind runtime for the existing LLVM22 binaries; builds Rizin v0.9.1 if possible; downloads and attempts an LLVM6 Hexagon-only build (old V4 ISA), and saves the V4-capable IDA processor's source plus a standalone V5 disassembler for cross-checking.

On Debian, test new LLVM6 tools using `scripts/probe-hexagon-v4.sh` with the original `a640_zap.mbn` **outside this repository**. Do not regard a nonempty disassembly as a proof that instructions are decoded correctly: inspect against documented V4 encodings/packet bits and confirm control-flow.

## Next executable reconstruction milestones

1. Obtain an independently **validated V4 disassembly** of all 1,968 A640 executable-flagged bytes; identify real entry, functions and ROM/CSR accesses. V4 plugin or old LLVM6 is a *candidate*, not yet a working result.
2. For V66 images, improve reliable function-boundary and physical/virtual address mapping, and reconcile decompiler coverage with segment maps and function calls. Existing Ghidra decompilation output is OEM-derived research, not distributable GPL3 Rust.
3. Identify ISA and boot interfaces for Cirrus HALO and Novatek NT36523; parse file payload structures accurately before claiming executable decompilation.
4. Draft new Rust from **behavioral specifications** independently corroborated by public documentation and tests, not bulk-translated vendor pseudocode. Identify replacement ABI and linker layout constraints. LLVM-generated instruction bytes will likely differ from original even if behavior matches.
5. Compare each candidate output with SHA-256 plus `cmp`, record per-segment mismatches and test independently in emulation/hardware only after authentication constraints are resolved. No firmware image here is fit for flashing.
6. Secure boot: identical signed image can retain original valid signatures, but without the private signing key **independently generating byte-identical proprietary signatures is not generally feasible**. Do not embed original vendor machine-code blobs, present copied signatures as GPL source, or bypass authorization claims without evidence.

## Licensing nuance

Rewrite from a decompiler does not necessarily create independently copyright-owned GPL code. For redistributable GPL software, new code must not copy protectable expression from proprietary sources, and one should obtain legal/permission review. Independently authored code generated from public algorithms is the safer route; whether it can emit the **same** machine-code bits is a separate technical problem. Record provenance per function.
