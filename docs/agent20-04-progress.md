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

## Checkpoint 04 — contiguous fixed-point codec controller writer

- New source-derived span `[0x40c14,0x412b8)` = **1,700 bytes**, MBN `[0x44c14,0x452b8)`: **422 typed ARM32 instructions** `[0x40c14,0x412ac)` and **three literal data words** `[0x412ac,0x412b8)`.
- Full original function dispatches codec/controller formats, normalizes signed-byte parameters, emits input descriptor records and calls fixed-point ARM-to-Thumb helpers; it uses public SMULL/SMULBB/SMLABB architecture encodings with module-local operand checks. These encoders have tests against independent Clang/LLVM assembly, including signed multiply-accumulate operands and expected instruction words.
- Independent 422-instruction LLVM mnemonic reassembly agrees byte-for-byte with the source-generated code; original reference comparison of every 1,700 bytes likewise passes. Three named literal constants (`0x26201c16`, `0x88888889`, `0xfffffdda`) are verified independently.
- The previously separate first and third fragments now form **3,220 consecutive bytes** `[0x40a2c,0x416c0)`. The additional capability function `[0x424c0,0x4272c)` is still **disjoint**; the next uncovered region is `[0x416c0,0x424c0)`.
- Source-derived total through this checkpoint = **3,840 bytes**, of which **3,808 executable bytes** (952 instructions) and **32 literal data bytes** (eight words), without inferred padding, externally borrowed code or implicit original-byte copying.

## Checkpoint 05 — contiguous controller packet processing

- Fully source-generated and reference-verified the gap function at runtime `[0x416c0,0x424c0)`, file `[0x456c0,0x464c0)` (**3,584 bytes**). It has 886 executable A32 instructions, six explicitly typed scaler-class jump-table words at `0x41cb4–0x41ccc`, and four trailing constant words at `0x424b0–0x424c0`.
- Encodes multiple runtime buffer commands, conditional device-specific feature flags, scaled widths/strides, register-indexed halfword/signed-halfword loads, signed and unsigned 64-bit multiply, and exact BLX behavior. No OEM instruction words or implicit padding appear in the generated source.
- Verified the two executable chunks independently using Clang/LLVM: 381 instructions at `[0x416c0,0x41cb4)` and 505 instructions `[0x41ccc,0x424b0)`. Conditional `BLO`/`BHS` targets must be symbolically resolved in the independent linker; comparing all original instruction bytes now passes.
- **The five preceding source-built fragments now form one gapless `[0x40a2c,0x4272c)` output of 7,424 bytes**, composed of **1,838 A32 executable instructions** (7,352 bytes) and 18 explicitly classified literal data words (72 bytes). Full concat of generated fragments matches the pinned firmware exactly with zero differences. The fragment at runtime `[0x40000,0x40a2c)` remains unimplemented because the reservation begins inside a prior function.
- Next confirmed function boundary after this contiguous block is `0x4272c`.

## Checkpoint 06 — codec mode packet writer

- Fully typed, reference-verified source interval `[0x4272c,0x43188)` = **2,652 bytes**, file `[0x4672c,0x47188)`: 658 ARM32 instructions (2,632 bytes), plus five explicitly named non-instruction literal constants `[0x43174,0x43188)`.
- The original function begins with saved-register prologue, computes codec/controller mode flags and descriptor positions, includes signed-byte and signed-halfword indexed reads and looped packet flag calculation, and returns through `pop {...,pc}` at `0x43170`.
- Independent Clang/LLVM assembler reconstruction of the 658 original A32 mnemonic instructions reproduces every source-generated instruction byte; five trailing data words were checked independently. `cargo run --example agent20_04_venus_middle -- verify /private/venus.mbn` compares **all six registered fragments** with the pinned reference, with zero mismatches.
- Contiguous, source-emitted region now `[0x40a2c,0x43188)` = **10,076 bytes**: 2,496 A32 instructions (9,984 bytes) plus 23 literal data words (92 bytes). Remaining reservation prefix `[0x40000,0x40a2c)` is still untouched, since it begins inside an earlier routine. Next verified function begins at `0x43188`.

## Checkpoint 07 — full codec command serializer, contiguous through 0x43b50

- Added original controller function `[0x43188,0x43b50)`, file `[0x47188,0x47b50)` = **2,504 source-generated bytes**: 623 ARM32 instructions including two return/side-entry instructions `[0x43b3c,0x43b44)`, and three named literal data words `[0x43b44,0x43b50)`.
- Implemented checked module-local ARM32 `LDRSH Rt,[Rn],#imm` postindexed addressing after an initial comparison found one erroneous word. Decoder, Rust emitter and independent Clang/LLVM assembler now agree exactly on every byte, including `BLX`/callback branches.
- Combined adjacent source-built interval `[0x40a2c,0x43b50)` = **12,580 contiguous byte-identical bytes**, 3,119 A32 instructions (12,476 bytes) and 26 independently identified literal data words (104 bytes). Verified by concatenating each independently generated module and comparing the *entire* result against original pinned `venus.mbn`, with no implicit fill or copied OEM opcode words.
- First unavailable function starts at runtime `0x43b50`, MBN file `0x47b50`; Agent 04 reservation upper bound remains `0x68000`.

## Checkpoint 07 — complete controller packet-list constructor

- Added separately typed instruction-level A32 function `[0x43b50,0x43ff0)`, corresponding to firmware file `[0x47b50,0x47ff0)`. **1,184 executable bytes = 296 ARM instructions**, without a literal pool or inferred fill.
- The routine builds controller packet headers from byte-sized codec flags, dispatches multiple Y/UV plane records through the existing `0x3e02c` callback, adjusts command buffer pointers and finishes by emitting 96/100/104/108 control records. The genuine return is the final `pop {...,pc}` at `0x43fec`.
- Independent Clang/LLVM reassembly of the **296 mnemonics**, with PC-relative branches and external BL targets resolved using expression-relative offsets, reproduces every instruction byte. The typed Rust emitter and the pinned `venus.mbn` also match; no reference OEM instruction array was imported into the Rust module.
- Combined contiguous typed reconstruction `[0x40a2c,0x43ff0)` is **13,764 bytes**, comprising **3,415 executable A32 instructions = 13,660 bytes** and **26 separately identified literal data words = 104 bytes**. Full original-reference check compares all eight adjacent regions and reports zero differences.
- Next confirmed function entry at `0x43ff0` (file offset `0x47ff0`), inside Agent 04 reservation `[0x40000,0x68000)`. Prefix `[0x40000,0x40a2c)` remains unimplemented, as previously documented.
