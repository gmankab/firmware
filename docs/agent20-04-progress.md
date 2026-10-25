# Agent 20-04 — Venus controller and middle-code reconstruction

**Intended integration base:** `checkpoint-50-venus-33872.bundle`, commit `b7ed4b587633b9b475a40f5a3ace5cc90db620ff`.

**Available actual parent:** `a3ffc58da64d8f96a65b9a48f4bed24592ba0ac1` from the uploaded Agent 03 late-Venus bundle. Checkpoint 50 itself is not mounted and is **not** this branch's ancestor. This work is therefore a **provisional portable extension**, not a checkpoint-50-rooted integrated release.

**Owned reservation:** runtime `[0x40000,0x68000)` in the first executable `venus.mbn` `PT_LOAD`, mapping to file `[0x44000,0x6c000)`. The executable `PT_LOAD` starts at file offset `0x4000`, vaddr zero, filesz `0x1029f4`; do not apply the ELF physical address as an additive offset.

## Checkpoint 01 — signed dimension normalization and short return

- The reservation begins **in the middle** of a previous large function: runtime `[0x40000,0x40a2c)` is **not** covered by this implementation. It is not declared as rebuilt, not filled with implicit zeros and not copied from the original into the generator.
- Source-derived executable span **runtime `[0x40a2c,0x40c14)`**, file `[0x44a2c,0x44c14)`, exactly **488 bytes = 122 A32 instructions**. No literal-data words in this initial interval.
- Functions: the genuine saved-register-return dimension/classification routine `[0x40a2c,0x40c0c)`, followed by the two-instruction `return 0` stub `[0x40c0c,0x40c14)`.
- The first routine normalizes signed 16-bit input dimensions, calls helpers at `0x81838`, `0x53e8c`, `0x53edc`, and `0x53f98`, handles conditional clipping, and returns a two-bit small-dimension status mask. That is an interpretation of decoded control flow, **not** independently recovered high-level source semantics.
- `cargo run -p firmware-tools --example agent20_04_venus_middle -- verify /private/venus.mbn` compares every emitted byte against the separately extracted, pinned `venus.mbn` reference (SHA-256 `9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085`). **Zero differences**.
- Independently assembled **all 122 ARM mnemonic instructions** with Clang/LLVM, resolving all relative PC+8 branches through linker symbols, and compared its 488-byte result to the Rust emitter and original. All agree exactly. This strengthens instruction encoding verification but does not prove high-level behavioral equivalence or new OEM signing.
- The next verified function entry is `0x40c14`. Do not extend into Agent 05 ownership (runtime at or above `0x68000`).

## Build and verification

```sh
cargo test --workspace --offline --locked
cargo build --workspace --release --offline --locked
python3 -m unittest discover -s tests -p 'test_*.py'
cargo run --offline --locked -p firmware-tools --example agent20_04_venus_middle -- verify /private/venus.mbn
cargo run --offline --locked -p firmware-tools --example agent20_04_venus_middle -- emit /tmp/venus-middle.bin
git diff --check
```

**Integration changes:** the only shared file touched is the single `mod.rs` module-registration line; the new source and CLI example have unique Agent 20-04 filenames. There are no original OEM binaries, signed images, executable opcode arrays or authentication materials in the Git commits. All tests require an agent-isolated `CARGO_TARGET_DIR` to prevent stale `arm32` artifacts from other agent branches.

## Checkpoint 02 — codec controller capability flags

- Independently source-emitted second **disjoint** function and literal pool at runtime `[0x424c0,0x4272c)`, original file `[0x464c0,0x4672c)`.
- Exact coverage: **620 bytes**, of which **616 bytes are executable A32 instructions** (154 instructions) and **4 bytes are structured non-executable capability-mask data** (`0xfff801ff` at runtime `0x42728`).
- Semantics recovered at instruction level: branch among codec command modes, assemble quantization/capability bit fields from controller descriptors, write one or more typed command records and adjust output pointer. This is not a complete high-level rewrite or proof of device execution.
- Source verification through the standalone CLI covers both the original 488-byte fragment and the new 620-byte fragment; they remain **separate**. Never sum them as a contiguous span across uncovered runtime `[0x40c14,0x424c0)`.
- Clang/LLVM independently reassembled every one of the 154 instructions using original assembly mnemonics and a linker; all 616 instruction bytes matched the ARM32 Rust emitter and pinned MBN. The trailing four data bytes were compared separately, with zero mismatches.
- The only shared source change remains registration of uniquely named modules in `devices/xiaomi_nabu/mod.rs`. All firmware originals stay outside Git.

## Checkpoint 03 — complete codec format register command writer

- Third independent **disjoint** A32 function and data pool at runtime `[0x412b8,0x416c0)`, MBN file `[0x452b8,0x456c0)`, length **1,032 bytes**: **254 original ARM32 instructions = 1,016 executable bytes**, plus **four explicit literal words = 16 data bytes** at `0x416b0–0x416c0`.
- This codec format/control routine selects packet bit depth, channel flags, signed-byte controller fields and output command words. Two signed-byte register-indexed loads require a separately tested, checked **module-local** public ARM LDRSB encoding; the shared `arm32` crate remains unmodified to avoid conflicts with other agents.
- Independent Clang/LLVM reassembly of all 254 mnemonics exactly matches the 1,016 typed Rust instruction bytes; the four-word literal pool matches independently. The complete private-reference verifier now checks **three separate fragments** with zero differences.
- Verified source-derived total through this checkpoint is **2,140 bytes** over three disjoint intervals (488 + 1,032 + 620): **2,120 A32 executable instruction bytes** (530 instructions), **20 known data bytes** (five literal words), no padding or external code. The remaining uncovered intervals within the first group are runtime `[0x40000,0x40a2c)`, `[0x40c14,0x412b8)` and `[0x416c0,0x424c0)`.
- The next confirmed function entry after the third reconstructed interval is `0x4272c`; its later ranges remain pending. The owned exclusive upper boundary is still `0x68000`.
