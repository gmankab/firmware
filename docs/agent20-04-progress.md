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

## Checkpoint 08 — controller format initialization (partial function)

- Added `[0x43ff0,0x45000)`: **4,112 typed executable bytes = 1,028 A32 instructions**, MBN file `[0x47ff0,0x49000)`; the source directly encodes A32 operand forms including conditional flag-setting arithmetic, signed-byte register loads, indexed byte loads, and conditional controller writes. The function continues **past 0x45000**; no return or adjacent implementation is implied.
- Independent Clang/LLVM reassembly of all 1,028 mnemonic instructions from ARM architecture operands, using same-section branch labels and relative external targets, agrees byte-for-byte with the typed Rust output and original pinned `venus.mbn`.
- New **contiguous** source-derived fragment `[0x40a2c,0x45000)` = **17,876 byte-identical bytes**, of which **4,443 ARM32 instructions (17,772 bytes)** and 26 literal-data words (104 bytes). No OEM executable words, unknown gaps or original-file payload arrays are imported into source. The bytes before runtime `0x40a2c` remain unknown in this agent's reservation.
- Testing: normal offline workspace Rust and Python suites pass. The default optimized release compile exceeded this execution session's per-command time limit; `CARGO_PROFILE_RELEASE_OPT_LEVEL=0 cargo build --workspace --release --offline --locked` succeeds. Do **not** claim default optimized release compilation passed at this checkpoint.
- Next uncovered runtime `0x45000`, file offset `0x49000`. Agent 04 still owns through `0x68000` exclusively.

## Checkpoint 09 — codec command-mode and register capability fields

- New typed region `[0x45000,0x45800)` = **2,048 bytes**: 511 executable A32 instructions (2,044 bytes) and one independently classified 4-byte inline data word `0xfe0f83e0` at runtime address `0x4509c`. Its predecessor branches explicitly skip the word; blindly interpreting the word as `CDP2` would be incorrect.
- Clang/LLVM independently reassembled the two code intervals `[0x45000,0x4509c)` and `[0x450a0,0x45800)`. Their 511 instructions and the separately compared literal word match both the typed Rust emitter and original `venus.mbn` with zero differences.
- Contiguous generated Venus region `[0x40a2c,0x45800)` now comprises **19,924 byte-identical bytes**: **4,954 A32 instructions (19,816 bytes) plus 27 non-executable literal words (108 bytes)**. The enclosing controller format routine still has branches beyond `0x45800`, and this is **partial-function** coverage.
- Repository tests passed, including offline Rust/Python. The release profile built successfully with `CARGO_PROFILE_RELEASE_OPT_LEVEL=0`; do not imply a default optimized release build completed. No proprietary raw executable bytes are embedded in any source module.
- Next runtime address is `0x45800` (MBN file `0x49800`); Agent 04's upper reservation boundary remains `0x68000`.

## Checkpoint 10 — controller capability checks and literal pools

- Added runtime `[0x45800,0x46000)` = **2,048 exact bytes** (original MBN `[0x49800,0x4a000)`). Correct classification is **509 typed A32 instructions (2,036 bytes)** and **three inline literal words (12 bytes)** at `0x45c80`, `0x45c84`, and `0x45f14`.
- The two words `[0x45c80,0x45c88)` are branch-skipped data (`0x00ff03ff`, `0x40820826`), not the apparent invalid `LDRSHT PC` ARM opcode. The third word `0x807fffff` at `0x45f14` follows a branch to `0x4600c` and is likewise non-executable data. Source code names these values explicitly and tests their positions.
- Independent Clang/LLVM assembler checks of three executable intervals, plus separate checks of all literal words, reproduce **all 2,048 bytes**. The typed Rust output matches original `venus.mbn` with zero byte differences.
- New verified contiguous Agent 04 output `[0x40a2c,0x46000)` = **21,972 bytes**: **5,463 A32 instructions (21,852 bytes)** and **30 non-executable data words (120 bytes)**. No proprietary opcode bytes or unknown zero-filled fragments are in the source. The enclosing controller configuration function continues past `0x46000`.
- Python/offline Rust tests pass. Release-profile compilation passes with `CARGO_PROFILE_RELEASE_OPT_LEVEL=0`; default optimized release still has not completed within the available execution limit. Next uncovered runtime `0x46000` (MBN offset `0x4a000`).

## Checkpoint 11 — controller command capability continuation

- Added contiguous A32 source-generated interval `[0x46000,0x46800)`, original MBN file `[0x4a000,0x4a800)`, **2,048 bytes** consisting of **511 typed instructions** (2,044 bytes) and an explicitly classified runtime function callback-address word at `0x460b8` (`0x000ee028`, 4 bytes). This word is skipped by an unconditional branch, so it must not be disassembled as an ordinary AND instruction.
- New isolated module `venus_agent20_04_feature_extension.rs` independently encodes controller bit-field packing, descriptor pointer adjustment, conditional capability flags and register-based controller index access. The original enclosing controller function is not finished at `0x46800`; do not fabricate an epilogue here.
- Reference byte comparison of the entire 12-module region now verifies **24,020 contiguous bytes** `[0x40a2c,0x46800)` (original MBN `[0x44a2c,0x4a800)`). Exactly **5,974 A32 instructions**, 31 distinguished data words, no copied original instruction words or unknown fill. Uncovered prefix `[0x40000,0x40a2c)` remains outside this reconstruction.
- Independent `clang --target=armv7a-none-eabi`/`llvm-objcopy` assembled 511 parsed mnemonic instructions and separately modeled literal pointer, and the full resulting 2,048 bytes equal both the typed Rust emitter and original MBN. Reproduce using `tools/agent20_04_roundtrip.py` with `--disasm`, `--start 0x46000 --end 0x46800`, `--data '0x460b8:0x460bc:0x000ee028'` and private `--reference` MBN and `--file-offset 0x4a000`. This corroborates instruction encodings; it is **not** independent recovered high-level source semantics.
- The checkpoint still branches from older Agent 03 history rather than checkpoint 50, whose raw Git bundle has not been made available here. Treat as provisional for later source integration.

## Checkpoint 12 — controller packet-source masks and parameter serialization

- Added exact original A32 interval `[0x46800,0x47000)` (MBN `[0x4a800,0x4b000)`) of **2,048 bytes**: **510 typed A32 instructions** and **two genuine four-byte inline controller masks** (`0x3ffc0fc0` at `0x46a18`, `0x007fffff` at `0x46a30`). The disassembler spuriously displays these as `SVCLO` and `LDRSHT PC`; the previous conditional branch flow bypasses each data word. They are never emitted as A32 instructions in the typed source.
- This continues the existing controller parameter writer without fabricating a return; serialization includes mode-dependent stream fields and fast register-indexed buffer lookups. The next uncovered instruction is `0x47000`.
- Agent 04's generated contiguous output `[0x40a2c,0x47000)` = **26,068 bytes**: **6,484 A32 instructions (25,936 bytes)** + **33 identified data words (132 bytes)**. All assembled source output matches the pinned `venus.mbn` with zero differences, without imported original opcode bytes or layout fill.
- The independent Clang/LLVM round-trip verified all 510 instructions and both data words using `tools/agent20_04_roundtrip.py`, disassembly from `llvm-objdump -d --adjust-vma=0x46000`, `--start 0x46800 --end 0x47000 --data 0x46a18:0x46a1c:0x3ffc0fc0 --data 0x46a30:0x46a34:0x007fffff`, `--file-offset 0x4a800`, with the private pinned reference. This is instruction-level corroboration, not high-level behavioral recovery.
- Base ancestry remains explicitly provisional (Agent 03 predecessor, not checkpoint 50). Agent 20-04 owns only runtime addresses below `0x68000` and does not attempt to change the incomplete earlier `[0x40000,0x40a2c)` routine.

## Checkpoint 13 — codec frame component dimensions and packet controls

- Rebuilt `[0x47000,0x47800)` = **2,048 bytes** of typed A32 instruction-level code, original MBN `[0x4b000,0x4b800)`; 510 instructions and two branch-skipped data words `0x2dc0ffff` at runtime `0x4755c` and `0x7fff8000` at runtime `0x4774c` (4 bytes each). The linear disassembler's `STCLHS`/`SVCVC` mnemonics are implausible because those words are reached only as data, not via fallthrough.
- This continues the same codec/controller function from `0x46000` and serializes signed size normalization, packed codec component width/height and mode-dependent stream flags. The larger controller function still continues after `0x47800`; no return instruction is invented.
- Verified full contiguous Agent 04 interval `[0x40a2c,0x47800)` = **28,116 bytes**: 6,994 ARM32 instructions (27,976 bytes) and 35 explicitly classified data words (140 bytes). All generated bytes equal the pinned reference with no unknown fill or OEM opcode copying.
- Independent Clang/LLVM reassembly of all 510 A32 instructions plus independently identified literal data reproduces the same 2,048 bytes (`tools/agent20_04_roundtrip.py --start 0x47000 --end 0x47800 --data '0x4755c:0x47560:0x2dc0ffff' --data '0x4774c:0x47750:0x7fff8000' --file-offset 0x4b000`). Data/instruction classification is based on preceding branch targets; assembler comparison is not behavioral proof.
- Provisional old Agent 03 ancestry (not checkpoint 50); the Agent 20-04 reservation still ends at runtime `0x68000`, and `[0x40000,0x40a2c)` remains outside current source coverage.

## Checkpoint 14 — codec stride and plane size calculations

- New `[0x47800,0x48000)` (MBN `[0x4b800,0x4c000)`), **2,048 bytes**, constructed as 511 typed ARM32 instructions and one explicit four-byte negative stride literal `0xfffffefc` at runtime `0x47e48`. The unconditional branch at `0x47e44` skips the constant, which LLVM lists as an unknown instruction; the original routine falls through at `0x47e4c`.
- Encoding includes UMULL high:low arithmetic, SMULBB fixed-point signed low-halfword multiply, conditional multiply/accumulate, and codec buffer descriptor stride computation. Checked local opcode generators for UMULL/SMULBB have independent operand-word tests; no original instruction words are pasted into Rust.
- Reassembled all 511 instructions from ARM mnemonics using Clang/LLVM and combined the independently identified data word. All **2,048** bytes match both the typed Rust emitter and reference MBN (`tools/agent20_04_roundtrip.py --start 0x47800 --end 0x48000 --data 0x47e48:0x47e4c:0xfffffefc`, file offset `0x4b800`).
- New continuous Agent 04 coverage `[0x40a2c,0x48000)` = **30,164 bytes** comprising 7,505 ARM32 instructions (30,020 bytes) and 36 distinct literal/data words (144 bytes), without padding or OEM-copy fallback. The next byte is runtime `0x48000`. Function continues; no epilogue inferred.
- The repository remains provisional because checkpoint 50 history was not supplied. Keep the preceding `[0x40000,0x40a2c)` code outside this agent's measured source coverage and stop at `0x68000` when reached.

## Checkpoint 15 — codec command masks, packed dimensions and signed DSP multiply

- Added `[0x48000,0x49000)` = **4,096 exact typed ARM32 bytes**, original MBN `[0x4c000,0x4d000)`; **1,020 A32 instructions** and four separately classified inline data words at `[0x48a20,0x48a30)` (`0xffffc000`, `0x80003fff`, `0x000ee028`, `0xaaaaaaab`). These are branch-skipped constants; disassembling them as `STRDHI`, `BGE` or undefined instructions would be incorrect.
- The instruction stream emits codec/controller command flags, 32-bit block strides, fixed-point SMULBB with LR destination (permitted; SP/PC forbidden) and conditional register writes. The updated local SMULBB operand validator is tested against the explicitly encoded LR-destination word `0xe16e0382` and remains isolated from shared `arm32` crate sources.
- Full comparison of all Agent 04 fragments verifies one continuous `[0x40a2c,0x49000)` region = **34,260 bytes**, consisting of **8,525 ARM32 instructions** (34,100 bytes) and **40 non-executable data words** (160 bytes). No ELF/linker padding, vendor instruction words or unknown bytes were copied into the emitter. Runtime prefix `[0x40000,0x40a2c)` remains intentionally uncovered.
- Independent Clang/LLVM reconstruction through the mnemonic-and-symbol script verifies all 1,020 new instructions and the four named data words, reproducing the exact 4,096-byte source/emitter reference (`--start 0x48000 --end 0x49000 --file-offset 0x4c000 --data '0x48a20:0x48a30:0xffffc000,0x80003fff,0x000ee028,0xaaaaaaab'`). This is byte-encoding corroboration, not behavioral equivalence on hardware.
- The current enclosing function still continues at `0x49000`; Agent 20-04's exclusive upper bound remains `0x68000`. Git base remains older Agent 03 history (checkpoint 50 unavailable), so this is a provisional portable branch.

## Checkpoint 16 — codec-class table and extended command packet layout

- New exact `[0x49000,0x4a000)` (MBN `[0x4d000,0x4e000)`) = **4,096 bytes**: **1,019 ARM32 instructions (4,076 bytes)** and five branch-skipped, PC-referenced codec-class data words (`0x01000302, 0x00011111, 0x02020202, 0x03030302, 0x000ee028`) at `[0x49e54,0x49e68)`. These are **not** the `<unknown>`, `MOVWEQ` or strange ARM instructions shown by a linear disassembler; original BL/PC LDR reaches into the table and the preceding branch bypasses it.
- Typed original ARM architectural operations rebuild packet-buffer loops, class indexing and mode-specific flags. The controller function continues beyond `0x4a000`, with no invented stack return or source-level behavioral assertion.
- Full gapless Agent 04 fragment `[0x40a2c,0x4a000)` = **38,356 bytes**, comprised of **9,544 A32 instructions (38,176 bytes)** plus **45 non-instruction literal/data words (180 bytes)**. Every byte matches the private pinned Venus MBN reference; no vendor opcode/unknown hole copied or zero-filled.
- Independent Clang/LLVM reassembly confirms **1,019** original mnemonic instructions plus five separated literal values equal the Rust emitter and reference, using the checked `tools/agent20_04_roundtrip.py`, `--start 0x49000 --end 0x4a000 --data '0x49e54:0x49e68:0x01000302,0x00011111,0x02020202,0x03030302,0x000ee028' --file-offset 0x4d000`.
- Source tree is still a portable provisional descendant of older Agent 03 Git history rather than checkpoint 50, which is not available in this session. Agent 04 owns runtime `[0x40000,0x68000)` only; prefix `[0x40000,0x40a2c)` remains uncovered and the next missing byte is `0x4a000`.

## Checkpoint 17 — codec stream-class controller packet writer

- Added source-generated ARM32 `[0x4a000,0x4c000)` = **8,192 bytes** corresponding to original Venus MBN file `[0x4e000,0x50000)`. Exactly **2,047 typed ARM32 instructions** and **one non-instruction PC-referenced diagnostic pointer**, `0x000ee028` at runtime `0x4ba08` (read from `0x4b9d4`, skipped by unconditional branch at `0x4ba04`).
- Independent Clang/LLVM mnemonic rebuild of all 2,047 instructions and separate diagnostic literal reproduces the Rust output and the original file with zero differences; invocation: `tools/agent20_04_roundtrip.py --disasm /private/next2.txt --start 0x4a000 --end 0x4c000 --data 0x4ba08:0x4ba0c:0x000ee028 --reference /private/venus.mbn --file-offset 0x4e000 --compare /private/check17-4a000.bin`.
- Gapless Agent 04 coverage `[0x40a2c,0x4c000)` = **46,548 bytes** containing **11,591 ARM32 instructions** and **46 classified data words**. The parent codec routine continues at `0x4c000`; no invented return or zero-filled padding was added. Uncovered original prefix `[0x40000,0x40a2c)` remains excluded.
- Offline Rust suite: 207 tests passed. Python suite: 13 tests passed. The branch still derives from the older Agent 03 history rather than checkpoint 50, and is marked provisional.

## Checkpoint 18 — controller diagnostic pointer and stream-length setup

- Added ARM32 instructions for runtime `[0x4c000,0x4d000)` = **4,096 exact bytes**, original MBN `[0x50000,0x51000)`. This interval contains **1,023 typed ARM32 instructions** plus **one PC-referenced literal** `0x000ee028` at runtime `0x4cb70`.
- The word at `0x4cb70` is loaded from **eleven** PC-relative instruction sites and is bypassed by unconditional branch at `0x4cb6c`. A linear disassembler incorrectly displays the constant as `ANDEQ LR,...,LSR #32`; the encoder rejects that instruction form and the word is deliberately emitted as a named data value instead.
- Independent Clang/LLVM mnemonic assembly verifies the new 1,023 instructions and one data word match both the Rust emitter and the reference: `tools/agent20_04_roundtrip.py --disasm /private/fwd.txt --start 0x4c000 --end 0x4d000 --data '0x4cb70:0x4cb74:0x000ee028' --reference /private/venus.mbn --file-offset 0x50000 --compare /private/check18-4c000.bin`.
- Full contiguous agent-generated executable interval `[0x40a2c,0x4d000)` is **50,644 bytes** = **12,614 ARM32 instructions** and **47 non-instruction data words**. The original 0x40000–0x40a2c prefix remains uncovered. The enclosing controller routine continues beyond `0x4d000`; no return fabricated.
- 208 Rust tests and 13 Python tests passed in this environment. Branch ancestry is provisional from the older Agent 03 repository, not checkpoint 50.

## Checkpoint 19 — signed frame records, carry-aware arithmetic and controller diagnostics

- Added Venus runtime `[0x4d000,0x4e000)` = **4,096 byte-identical bytes**, original MBN `[0x51000,0x52000)`. This code encodes **1,017 typed ARM32 instructions** and **seven explicit literal words**: a branch-skipped logging pointer at runtime `0x4d8fc`, and six diagnostics/capabilities at `[0x4dd54,0x4dd6c)` following an ARM return.
- ARM32 `ADC Rt,Rn,#0`, signed `LDRSH Rt,[Rn],#imm`, and unsigned `LDRH Rt,[Rn],#imm` postindexed forms have local checked encoders with exact instruction tests. Reuses existing typed register-indexed LDRH/LDRSH shared with Agent04 controller modules. These are not copied OEM instruction bytes.
- The linear disassembler misleadingly shows `ANDEQ LR,...,LSR #32` at `0x4d8fc`, `STRDEQ` inside the literal pool and an `<unknown>` constant; none is emitted as an executable instruction. Independent Clang/LLVM mnemonic reconstruction of all 1,017 instructions with both separately identified literal ranges reproduces the original exactly.
- Verified cumulative Agent04 contiguous range `[0x40a2c,0x4e000)` = **54,740 bytes**: **13,631 ARM32 instructions** and **54 classified non-instruction data words**. The next routine begins at `0x4dd6c` inside this interval and continues after `0x4e000`; no invented function epilogue.
- Offline Rust workspace 209 tests and Python 13 tests passed. Source branch is provisional older Agent03 history, not checkpoint50; prefix `[0x40000,0x40a2c)` remains uncovered.

## Checkpoint 20 — controller command list, mode buffers and next function prologue

- Added exact runtime `[0x4e000,0x4f000)` = **4,096 bytes**, MBN file `[0x52000,0x53000)`, re-encoded as **1,021 A32 instructions** and **3 separately identified non-instruction words** `[0x4eb54,0x4eb60)`: width mask `0xffc03f0f`, signed mask `0x7fff8000`, coefficient `0x0548eba0`, after a real POP-PC at `0x4eb50`.
- New module `venus_agent20_04_controller_span_4e000.rs` partitions controller packet construction and follow-up descriptor serialization into four instruction-checked functions, with explicit `Code::check` boundaries and tests. It does not import raw OEM executable bytes.
- `tools/agent20_04_roundtrip.py` with the original reference and `--data 0x4eb54:0x4eb60:0xffc03f0f,0x7fff8000,0x0548eba0` independently reassembled all 1,021 instructions with Clang/LLVM, matching the generated Rust bytes and original Venus MBN with zero differences.
- Contiguous Agent04 coverage now `[0x40a2c,0x4f000)` = **58,836 bytes**, comprising **14,652 instructions**, **57 distinct non-instruction words**; first missing runtime address `0x4f000`. The earlier `[0x40000,0x40a2c)` gap remains outside measured coverage, and the whole branch is **provisional old Agent03 history**, not checkpoint 50.
- Offline Rust workspace: 210 tests; Python: 13 passed; `CARGO_PROFILE_RELEASE_OPT_LEVEL=0 cargo build --workspace --release --offline --locked` passed. Default optimized release not claimed tested for this checkpoint.

## Checkpoint 21 — codec normalization and next controller list

- New exact Venus `[0x4f000,0x50000)` = **4,096 bytes**, MBN `[0x53000,0x54000)`, with **1,019 typed A32 instructions** and **five separately identified post-return data words**: diagnostic pointer `0x000ee028`, `0xffc03f0f`, `0x7fff8000` at `[0x4f3b4,0x4f3c0)`, and `0xffc03f0f`, `0x0548eba0` at `[0x4f924,0x4f92c)`.
- The first function ends at `0x4f3b0`; the next ends at `0x4f920`. Neither data pool is executed. `venus_agent20_04_controller_span_4f000.rs` explicitly models both pools with structural tests and rebuilds every other word with public typed A32 encoders.
- `tools/agent20_04_roundtrip.py` reassembled the original ARM mnemonics with Clang/LLVM (1,019 instructions), preserving the classified pools, and matched both independently generated Rust bytes and the original MBN exactly (zero byte differences).
- Agent04 continuous source-generated region `[0x40a2c,0x50000)` is now **62,932 bytes**, **15,671 ARM32 instructions and 62 data words**, no source-unknown fill or original executable-byte imports. Prior `[0x40000,0x40a2c)` remains excluded. The next byte is `0x50000`, inside a function.
- Offline Rust workspace **211 tests** and Python **13 tests** passed; `CARGO_PROFILE_RELEASE_OPT_LEVEL=0 cargo build --workspace --release --offline --locked` passed. Default `--release` optimization is not asserted passed.
- This branch derives from older Agent 03 ancestry, not the still-unavailable checkpoint-50 commit; merge as a provisional patch only.

## Checkpoint 22 — controller buffer transfer and auxiliary packet descriptors

- Added **4,096 exact bytes** at runtime `[0x50000,0x51000)` (original MBN `[0x54000,0x55000)`) rebuilt as **1,017 A32 instructions** and **seven named data words** at `0x502b4`, `0x504dc`, `[0x50ae8,0x50af8)`, and `0x50c6c`. Pool contents are individually verified; the disassembler's `CDPEQ` at `0x50af4` is part of non-executable pointer `0x0ee6b280`, not executable coprocessor code.
- Original returns at `0x502b0`, `0x504d8`, `0x50aa8`, `0x50c68` bound separate codec routines; data words immediately after these returns are deliberately not encoded as A32 instructions. The next function continues through `0x51000`.
- Independent Clang/LLVM reassembly using `tools/agent20_04_roundtrip.py` validates all 1,017 instruction bytes and seven data words with zero differences against the typed Rust emitter and pinned Venus reference. See `--data 0x502b4:0x502b8:0x000ee028`, `--data 0x504dc:0x504e0:0x000ee028`, `--data 0x50ae8:0x50af8:0x000ee028,0x000ee12a,0x000e2788,0x0ee6b280`, `--data 0x50c6c:0x50c70:0x000ee028`.
- Verified continuous Agent04 span `[0x40a2c,0x51000)` = **67,028 bytes**, comprising **16,688 typed A32 instructions** and **69 separate literal/data words**, none vendor-executable bytes copied or implicit zeros.
- Offline Rust workspace **212 tests**, Python **13 tests** and release build with `CARGO_PROFILE_RELEASE_OPT_LEVEL=0` all passed. Default optimized release not claimed. Provisional older Agent03 ancestry, **not** checkpoint50; still no coverage for `[0x40000,0x40a2c)`.

## Checkpoint 23 — frame-parameter command packets and seven-way format selector

- Added runtime `[0x51000,0x52000)` = **4,096 exact bytes** (MBN file `[0x55000,0x56000)`) as **1,008 typed A32 instructions** and **16 individually labeled data words**, including a seven-way format jump table at `[0x51df4,0x51e10)` targeting `0x51e10,0x51ea8,0x51f2c,0x51fac,0x52030,0x520b4,0x52138`.
- The four-word logging/controller pool at `[0x513a8,0x513b8)` occurs after a POP-LR followed by a branch; the five-word MMIO/diagnostic pool `[0x51bb0,0x51bc4)` follows an unconditional branch. The linear disassembler displays `LDRDEQ`/`STRHEQ` for some of these data words, but control-flow confirms they are **non-instruction** values.
- Independent `tools/agent20_04_roundtrip.py` using Clang/LLVM verifies all 1,008 A32 instructions and 16 named data words exactly. Full `agent20_04_venus_middle -- verify` passes against private pinned Venus MBN with zero byte mismatches.
- Contiguous Agent04 coverage `[0x40a2c,0x52000)` now totals **71,124 source-reencoded bytes**, **17,696 typed A32 instructions** and **85 separate literal/data words**, without using original executable opcode bytes as data arrays or inserting unknown fill. First uncovered runtime byte is `0x52000`; the current function continues beyond it. `[0x40000,0x40a2c)` remains unimplemented.
- Offline Rust workspace **213 tests**, Python **13 tests**, release with `CARGO_PROFILE_RELEASE_OPT_LEVEL=0` passed. Default optimized release not claimed. Still **provisional old-Agent03 ancestry**, not checkpoint 50.

## Checkpoint 24 — frame geometry, controller MMIO and halfword record operations

- Added **4,096 reference-exact bytes** `[0x52000,0x53000)` (firmware MBN file `[0x56000,0x57000)`), modeled as **1,009 typed ARM32 instructions plus 15 separately identified non-instruction data words**. The 7-word post-return frame diagnostics `[0x522b4,0x522d0)`, flag `[0x524d4,0x524d8)`, logging pointer `[0x52508,0x5250c)`, and 6-word resource/control pool `[0x527a4,0x527bc)` are explicitly emitted as data.
- Added a *local* checked architecture encoder for negative register-offset LDRH/STRH (U=0, I=0) with instruction-value tests against independently generated A32 instructions. No vendor executable words are copied into Rust source.
- `tools/agent20_04_roundtrip.py` independently reassembled the 1,009 instructions with Clang/LLVM, modeled the 15 literal words separately, and compared **all 4,096 bytes** to both the source-built Rust emitter and the original pinned `venus.mbn`, yielding **zero differences**.
- Full standalone `agent20_04_venus_middle -- verify` passed all 25 adjacent fragments. Continuous Agent 04 coverage `[0x40a2c,0x53000)` is **75,220 source-reencoded bytes**, **18,705 typed ARM32 instructions**, **100 non-instruction literal/data words**, with no OEM opcode arrays or implicit fill. The earlier `[0x40000,0x40a2c)` is outside verified coverage, and the next uncovered runtime address is `0x53000`.
- Offline workspace **215 Rust tests**, **13 Python tests**, and `CARGO_PROFILE_RELEASE_OPT_LEVEL=0 cargo build --workspace --release --offline --locked` passed. Default full-opt release is not asserted tested.
- Branch remains a provisional descendant of the older Agent03 history, not checkpoint50. An independent, tested merge onto checkpoint50's actual commit is still needed before release.

## Checkpoint 25 — completed format update and indexed controller setup prefix

- Generated **1,024 exact bytes** at runtime `[0x53000,0x53400)` (MBN `[0x57000,0x57400)`) using **252 typed A32 instructions and four named non-instruction literal values** at `[0x53254,0x53264)` immediately after a saved-register return.
- Independent Clang/LLVM reassembly matches all 252 instructions and four literal words; the standalone emitter compares all 26 existing fragments to the pinned original `venus.mbn` with zero mismatches.
- Contiguous Agent 04 coverage `[0x40a2c,0x53400)` is **76,244 source-reencoded bytes**, comprising **18,957 ARM32 instructions and 104 data words**. The newly started indexed controller function continues beyond `0x53400`; do not claim it is complete.
- Offline Rust workspace **216 tests**, Python **13 tests**, and opt0 release build pass. Default full optimization is not claimed; prior no-reference range `[0x40000,0x40a2c)` remains unimplemented.
- This provisional Git branch still descends from old Agent03 rather than the missing checkpoint-50 Git SHA; integrate by cherry-pick/rebase only after obtaining the exact base.

## Checkpoint 26 — indexed codec controller and plane-count scanner

- Source-reencoded **1,348 exact bytes** at `[0x53400,0x53944)` (MBN `[0x57400,0x57944)`) as **321 typed A32 instructions plus 16 literal data words**. Two source functions have post-return data pools `[0x53708,0x5373c)` (13 words) and `[0x53938,0x53944)` (three words), including codec identifiers, controller-address displacements, diagnostic pointers and the `0xcccccccd` constant used in reciprocal scaling.
- Includes checked typed ARM32 `UMULL` low/high-result encoding and architecture doubleword memory transfers, with a result-register distinctness test. No proprietary instruction-word arrays or implicit zeros in the generator.
- Independent Clang/LLVM reassembly matched all **321 instructions and 16 data words** to both Rust emitter and private reference with **zero differences**. Standalone `agent20_04_venus_middle -- verify` passed its complete set of adjacent fragments.
- Continuous Agent04 source-generated interval `[0x40a2c,0x53944)` now **77,592 bytes**, **19,278 typed ARM32 instructions and 120 named data words**. First uncovered runtime address `0x53944` (function entry). Earlier `[0x40000,0x40a2c)` remains uncovered.
- Rust workspace **218 tests**, Python **13**, and opt0 release all passed. Provisional old-Agent03 ancestry, **not** original checkpoint50 integration; no flashability claim.

## Checkpoint 27 — eight-lane buffer normalization and format-dispatch helpers

- New typed A32 range runtime `[0x53944,0x53b24)` = **480 exact executable bytes**, file `[0x57944,0x57b24)`; 120 ARM32 instructions and **zero literal/data words**. The independent mnemonic assembler and the typed Rust emitter each match the same pinned `venus.mbn` bytes.
- `0x53944–0x53a28` scales up to eight controller plane words, optionally divides normalized values and compares two eight-lane arrays using signed magnitude; the genuine function returns at `0x53a24`.
- `0x53a28–0x53b24` checks the buffer's codec type and dispatches initialized frame descriptors to platform callbacks, returning through the original saved registers at `0x53b20`.
- Previous consecutive span `[0x40a2c,0x53944)` = 77,592 bytes. Together, adjacent generated code now covers `[0x40a2c,0x53b24)` = **78,072 bytes**, with no hidden fill or overlapping Agent 05 boundaries.
- Verified with `cargo run --offline --locked -p firmware-tools --example agent20_04_next -- verify /private/venus.mbn`, independent `tools/agent20_04_roundtrip.py --start 0x53944 --end 0x53b24 --file-offset 0x57944 --compare /private/new27.bin`, and repository tests.
- First uncovered runtime `0x53b24`, original MBN file offset `0x57b24`. Parent history is still **provisional**, derived from older Agent 03, *not* original checkpoint 50.

## Checkpoint 28 — packed frame buffers, codec lookups and fixed-point geometry helpers

- Added runtime `[0x53b24,0x54028)` = **1,284 exact typed ARM32 bytes**, mapped Venus file `[0x57b24,0x58028)`, containing **317 executable instructions** and **four non-instruction literal words** at `[0x53e20,0x53e2c)` and `[0x53e88,0x53e8c)`.
- Includes codec callback and stream serialization (`0x53b24–0x53e2c`), record lookup (`0x53e2c–0x53e8c`), 16-bit minimum plane-size normalization (`0x53e8c–0x53edc`), and two carry-aware 64-bit geometry routines (`0x53edc–0x54028`).
- Encoded A32 `UMULL` and `ADC Rd,Rn,#0` explicitly in a dedicated checked module-local encoder, rather than swapping carry-aware semantics for ordinary `ADD`.
- Private reference comparison via opt-in `AGENT04_PRIVATE_VENUS` test passed with **zero mismatches**, as did Clang independent mnemonic reassembly of all 317 instructions plus four literal words.
- Contiguous Agent 04 coverage `[0x40a2c,0x54028)` = **79,356 byte-identical bytes**. Uncovered reservation prefix `[0x40000,0x40a2c)` is not counted; next uncovered runtime `0x54028`. Checkpoint remains provisional old-Agent-03 ancestry, not checkpoint 50.

## Checkpoint 29 — fixed codec register and format-property initializer

- New independently generated runtime `[0x54028,0x54854)` = **2,092 exact bytes** (MBN `[0x58028,0x58854)`), consisting of **522 ARM32 instructions and one four-byte noninstruction capability word** `0x001fa3ff` at `0x54850`, immediately after the genuine saved-register return at `0x5484c`.
- The two typed sections initialize codec controller property records, multiple format/pixel layout presets, byte-sized hardware flags and default buffer values. Register-list `STM`, signed-byte register loads and conditional moves use checked A32 architectural encoders rather than copied opcode words.
- Private reference compare via `AGENT04_PRIVATE_VENUS` passes with zero differences. Independent Clang/LLVM mnemonic reconstruction using `--start 0x54028 --end 0x54854 --data 0x54850:0x54854:0x001fa3ff --file-offset 0x58028` matches all new instructions and the literal pool.
- Combined contiguous Agent 04 interval `[0x40a2c,0x54854)` now **81,448 byte-identical bytes**, exclusive of the still-unimplemented prefix `[0x40000,0x40a2c)`. The next function starts at runtime `0x54854`. The branch is still provisional old-Agent-03 ancestry, **not** checkpoint 50.

## Checkpoint 30 — controller resource initialization and per-entry registration

- New runtime `[0x54854,0x54c1c)` = **968 byte-identical generated bytes**, original Venus file `[0x58854,0x58c1c)`; **241 typed A32 instructions plus one diagnostic address** `0x000ee470` at `0x54c18`, following the real function return at `0x54c14`.
- Reconstructs controller runtime handle creation, per-slot resource registration and initialization of frame-format state, with conditional logging and a tail call to the shared diagnostic handler. Contains both `pop {...,lr}` for tail-call and `pop {...,pc}` for normal return; no invented instruction words.
- Private reference test and independent Clang/LLVM reassembly report **zero byte differences** (`--start 0x54854 --end 0x54c1c --data 0x54c18:0x54c1c:0x000ee470 --file-offset 0x58854`).
- Consecutive Agent 04 coverage `[0x40a2c,0x54c1c)` totals **82,416 bytes**. Reservation prefix `[0x40000,0x40a2c)` remains unknown; the next routine begins at `0x54c1c`. Provenance remains provisional old Agent 03 rather than checkpoint 50.

## Checkpoint 31 — codec-mode flag tables and state reset

- Added runtime `[0x54c1c,0x54cfc)` = **224 exact bytes** (MBN `[0x58c1c,0x58cfc)`), containing **52 typed A32 instructions** and **four data words** `[0x54cbc,0x54ccc)` (diagnostic pointer and three codec-mode table base addresses).
- First routine validates the selected codec class, builds and stores compact mode flags; second clears three codec boolean status fields without inventing any epilogue.
- Opt-in private reference Rust test passes and independent Clang/LLVM mnemonic reassembly matches all 224 bytes exactly (`--start 0x54c1c --end 0x54cfc --data '0x54cbc:0x54ccc:0x000ee470,0x000f16e0,0x000f16a0,0x000f1660'`).
- Verified contiguous Agent 04 source-built interval is `[0x40a2c,0x54cfc)` = **82,640 bytes**. The excluded prefix `[0x40000,0x40a2c)` and all runtime `>=0x68000` remain outside this agent's coverage. Next function starts at `0x54cfc`.

## Checkpoint 32 — controller resource policy and indexed halfword parameters

- New exact typed executable span runtime `[0x54cfc,0x55400)` = **1,796 ARM32 bytes** (449 instructions, **no inline data words**), MBN `[0x58cfc,0x59400)`.
- Emits controller resource and codec mode selection, indexed byte reads and writes and two distinct register-indexed halfword transfer forms. The latter have their own checked architectural encoder for both LDRH pre-index writeback and STRH ordinary indexed addressing, with independently tested instruction-word examples.
- The platform-resource function continues beyond `0x55400`, so the emitted range is explicitly **partial-function** code, not a standalone callable routine. A later `LDR [PC]` is allowed to reference a literal after this span without copying those future bytes.
- Optional private-reference byte verifier reports zero mismatches. Independent Clang/LLVM reassembly confirms all 449 instructions, using the original file interval and the emitted Rust `frag32.bin`.
- Consecutive Agent 04 coverage `[0x40a2c,0x55400)` is now **84,436 exact bytes**; the original `[0x40000,0x40a2c)` remains uncovered. Next missing runtime `0x55400`.

## Checkpoint 33 — recovered controller property continuation

- Checkpoint 33's original bundle was not present in this execution session; reconstructed its claimed interval independently starting from the verified checkpoint-32 Git bundle at commit `2aa732c`. This is a new, replacement checkpoint commit, not the missing earlier checkpoint-33 SHA.
- Source-generated `[0x55400,0x55800)` = **1,024 exact bytes** from 256 typed ARM32 instructions; no inline data words. Reference MBN `[0x59400,0x59800)`.
- The added instructions normalize codec input field limits, copy control fields into the active frame descriptor, set per-channel status flags and validate controller feature bits. The function from `0x54cfc` still continues beyond `0x55800`; no fabricated return is added.
- Direct private-reference comparison passed with zero differing bytes; independent Clang/LLVM mnemonic reassembly also passed. Agent 04 contiguous coverage `[0x40a2c,0x55800)` is **85,460 bytes**. Uncovered prefix `[0x40000,0x40a2c)` remains excluded.
- This branch is still derived from earlier Agent03 Git history, not checkpoint 50. Restrict all further Agent 04 bytes to ARM32 addresses strictly below `0x68000`.

## Checkpoint 34 — codec property copy and status normalization

- Continued the existing controller/property function with an independently emitted **2,048-byte** interval `[0x55800,0x56000)` (MBN `[0x59800,0x5a000)`). Every word is a typed A32 instruction: **512 instructions, zero literal/data words** in this interval. Cross-region PC-relative literal references are encoded as loads but their target pools are not silently included.
- The code copies active format descriptors and signed/unsigned status fields into codec register structures, validates input conditions, updates bounded control status and flags. This **continues** the earlier function; no return or high-level semantic proof is invented at the checkpoint boundary.
- Exact Rust output vs pinned `venus.mbn`: **zero mismatches**. Independent Clang/LLVM mnemonic reconstruction of all 512 instructions matches the reference and the Rust fragment (`tools/agent20_04_roundtrip.py --disasm /private/future.txt --start 0x55800 --end 0x56000 --reference /private/venus.mbn --file-offset 0x59800 --compare /private/55800-56000.bin`).
- Full contiguous Agent 04 region now `[0x40a2c,0x56000)` = **87,508 byte-identical source-derived bytes**. Prior reservation prefix `[0x40000,0x40a2c)` remains unimplemented; `[0x68000,...)` belongs to Agent 05.
- Offline test suite: **237 Rust tests** and **13 Python tests** passed. Offline release build passed with `CARGO_PROFILE_RELEASE_OPT_LEVEL=0`. Shared `mod.rs` adds only one dedicated module, and standalone `agent20_04_continue` example includes the new adjacent fragment. This remains **provisional older ancestry** (checkpoint 50 unavailable).

## Checkpoint 35 — indexed codec controller classes, dimensions, and dispatch tables

- Extended the adjacent Venus typed output with runtime `[0x56000,0x56800)`, private MBN file `[0x5a000,0x5a800)`: **2,048 exact bytes**, **497 executable A32 instructions (1,988 bytes)** and **15 separately modeled data words (60 bytes)**.
- PC/indirect-branch-related non-executable data pools are `[0x561b0,0x561c0)` (4 handler addresses, selected by `ldr pc,[r0,r1]`), `[0x56304,0x56310)` (3 negative field offsets), and `[0x56778,0x56798)` (8 table destinations selected from `ldr pc,[r1,r2]`). These are not imported as OEM opcode words. The controller function still continues past this block, so the chunk is **partial function** code only.
- The source builds bounded per-plane sizes, propagates size/stride limits, computes packet callback masks and dispatches original class-specific control paths through checked typed ARM32 operations. The signed indexed-byte load uses the existing shared checked encoder.
- Direct generated-fragment comparison against revision-pinned original `venus.mbn`: **0 mismatches**. Independent Clang/LLVM verification matches **all 497 instruction encodings and all 15 data words** using the three `--data` ranges listed above and source `tools/agent20_04_roundtrip.py`.
- Complete contiguous Agent 04 interval is `[0x40a2c,0x56800)` = **89,556 byte-identical source-derived bytes**; prefix `[0x40000,0x40a2c)` is intentionally excluded. Reservation ends at `0x68000` (exclusive).
- Workspace: **239 Rust tests** and **13 Python tests** passed. Release build uses `CARGO_PROFILE_RELEASE_OPT_LEVEL=0`. Branch remains **provisional** because checkpoint 50's commit is not an ancestor of this repository.

## Checkpoint 36 — controller class validation and multiple dispatch pools

- Continued source-generated Venus A32 runtime `[0x56800,0x57000)` = **2,048 byte-identical bytes**, private MBN `[0x5a800,0x5b000)`; **481 executable ARM32 instructions** and **31 separately modeled data words**. The enclosing controller function continues across the module boundary.
- Non-executable data, each verified separately from instructions: negative parameter offsets `[0x568b0,0x568bc)`, `[0x56918,0x56920)`, `[0x56930,0x56934)`, `[0x56944,0x5694c)`; diagnostic and eight-target jump table `[0x56978,0x5699c)`; format offset and second eight-target jump table `[0x56a60,0x56a84)`; pointer/flag pools `[0x56c6c,0x56c78)`, `[0x56e10,0x56e14)`, `[0x56fb0,0x56fb4)`. The jumping `ldr pc, [r1,r2]` at `0x56974` and `ldr pc, [r0,r2]` at `0x56a5c` confirm the arrays are handler pointers, not executable code.
- Exact byte comparison to the pinned `venus.mbn`: **0 differences**. Independent Clang/LLVM A32 reconstruction verifies all 481 instructions and 31 words using nine manually classified `--data` ranges. The module does **not** copy opaque OEM instruction words or pad unknown bytes.
- Combined contiguous Agent 04 region `[0x40a2c,0x57000)` now totals **91,604 exact bytes**. Reservation prefix `[0x40000,0x40a2c)` stays unimplemented; ownership ends before `0x68000`.
- Offline **241 Rust tests**, **13 Python tests** pass; optimized release is built with `CARGO_PROFILE_RELEASE_OPT_LEVEL=0`, and this provisional branch retains older Agent 03 ancestry instead of checkpoint 50.

## Checkpoint 37 — controller stream dimensions and class arithmetic

- Exact standalone source reconstruction `[0x57000,0x58000)` = **4,096 bytes**, private pinned `venus.mbn` file `[0x5b000,0x5c000)`; **1,020 typed ARM32 instructions and four explicitly identified literal words**. The computed output has zero differences from the original.
- Four PC-relative, branch-skipped diagnostic constants are explicitly modeled as data at `[0x57284,0x57288)` (`0x000ee57e`), `[0x57304,0x57308)` (`0x000ee5c3`), `[0x57480,0x57484)` (`0x000ee470`), and `[0x5765c,0x57660)` (`0x000ee5e6`). The conditional multiply at `0x57ccc` is actual executable A32, **not** a literal word.
- Includes codec plane-size normalization, class arithmetic and controller command flag propagation; the surrounding function remains partial. Independent Clang/LLVM mnemonic reconstruction confirms all **1,020 instructions** and four data words, in addition to the complete generated-fragment comparison.
- Total contiguous source-generated interval `[0x40a2c,0x58000)` is **95,700 exact bytes**; the uncovered reservation prefix `[0x40000,0x40a2c)` remains excluded, and Agent 04 must not emit at or above `0x68000`.
- All **243 Rust tests**, **13 Python tests** and release build with `CARGO_PROFILE_RELEASE_OPT_LEVEL=0` pass. Git base remains provisional from older Agent 03 history, not checkpoint 50.

## Checkpoint 38 — fixed-point codec buffer calculations and cleanup paths

- Source-generated Venus A32 intervals `[0x58000,0x59000)` and `[0x59000,0x5a000)` add **8,192 exact bytes** in MBN file `[0x5c000,0x5e000)`: **2,007 typed A32 instructions and 41 distinguished non-executable data words**. Each individual 4,096-byte emitter matches the pinned `venus.mbn` with zero differing bytes; independent Clang/LLVM reconstruction confirms both sections.
- First half `[0x58000,0x59000)` contains **996 instructions plus 28 literal words** in seven data pools, including multiplication-based stride/capacity normalization, controller status, callbacks, and record clearing. The signed and unsigned 64-bit multiply forms use distinct checked local ARM32 encoders, not copied OEM instruction bytes. Pools: `[0x58084,0x58088)`, `[0x58104,0x58130)`, `[0x587f8,0x58814)`, `[0x58860,0x5886c)`, `[0x588fc,0x58900)`, `[0x58c2c,0x58c38)`, `[0x58f58,0x58f60)`.
- Second half `[0x59000,0x5a000)` contains **1,011 instructions plus 13 literal words**, in two data pools `[0x595f8,0x59604)` and `[0x59b28,0x59b50)`. These pools are referenced by nearby `LDR [pc,#imm]` instructions, and a branch/return explicitly skips each pool. Device cleanup, resource allocation and mode-dependent controller packets are represented as typed operations.
- Complete Agent 04 contiguous coverage is now **103,892 byte-identical bytes** at runtime `[0x40a2c,0x5a000)`. The private-reference aggregate verifier for the newer continuation also passes **19,456 bytes** `[0x55400,0x5a000)`; the omitted prior-prefix `[0x40000,0x40a2c)` remains unimplemented. The exclusive ownership boundary remains `0x68000`.
- Initial workspace test invocation briefly failed due to registration of the second module while the Cargo build was already underway. After source edits completed, a clean rerun passed **247 Rust tests**, **13 Python tests** and the offline release build with `CARGO_PROFILE_RELEASE_OPT_LEVEL=0` (all exit codes zero). This branch is still provisional old-Agent-03 ancestry, not checkpoint-50-rooted.

## Checkpoint 39 — codec controller cleanup, resource allocation and indexed descriptors

- Independently emitted adjacent runtime intervals `[0x5a000,0x5b000)` and `[0x5b000,0x5c000)` = **8,192 reference-identical bytes**, original pinned `venus.mbn` file `[0x5e000,0x60000)`: **2,014 executable typed ARM32 instructions** and **34 explicitly modeled literal data words**. Clang/LLVM rebuilt every instruction byte independently; direct Rust comparison and combined continuation verifier report zero mismatches.
- `[0x5a000,0x5b000)` has **1,016 instructions plus 8 literal words**, with PC-referenced diagnostic pools `[0x5a648,0x5a64c)`, `[0x5a708,0x5a710)`, `[0x5a964,0x5a968)`, `[0x5ab40,0x5ab4c)` and `[0x5ac28,0x5ac2c)` after real returns/branches. Code prepares codec buffer metadata, performs resource cleanup, allocates protected descriptors and dispatches logging callbacks.
- `[0x5b000,0x5c000)` has **998 instructions plus 26 literal words**. Pools at `[0x5b018,0x5b01c)`, `[0x5b468,0x5b490)`, `[0x5b5f0,0x5b610)`, `[0x5b778,0x5b794)` contain branch-excluded addresses and frame-controller format parameters. Post-indexed `STRH` and `LDRB` are encoded through separately checked standard public ARM32 methods, not OEM instruction arrays.
- Complete contiguous Agent 04 interval `[0x40a2c,0x5c000)` now contains **112,084 byte-identical source-generated bytes**. The independent aggregate continuation CLI verifies the latest **27,648 bytes** `[0x55400,0x5c000)`; exclusive upper ownership boundary remains `<0x68000`, and initial reservation prefix `[0x40000,0x40a2c)` is still incomplete.
- First concurrent Cargo build exceeded the 4-GiB memory cgroup and its Rust jobs were killed with SIGKILL; **not a source mismatch**. The completed source was rebuilt with `CARGO_BUILD_JOBS=1`, `CARGO_PROFILE_DEV_DEBUG=0` and `CARGO_PROFILE_TEST_DEBUG=0`. That clean rerun passed **251 Rust tests**, **13 Python tests** and the offline release build (`CARGO_PROFILE_RELEASE_OPT_LEVEL=0`), all exit codes zero. Branch ancestry remains provisional (older Agent 03, not checkpoint 50).

## Checkpoint 40 — wide controller switch tables and resource flags

- Independently emitted adjacent Venus runtime `[0x5c000,0x5e000)` = **8,192 exact bytes**, private original MBN `[0x60000,0x62000)`. The first 4,096-byte chunk reconstructs **883 executable A32 instructions** and **141 explicitly distinguished data words** (including two indexed dispatch tables spanning 56 and 21 entries). The second reconstructs **965 instructions** and **59 named data words**, including PC-relative diagnostics, controller masks, and shorter dispatch tables.
- Independent Clang/LLVM reassembly of both mixed code/data chunks matches the original precisely, using the data-pool boundaries declared as `INLINE_DATA_*` constants. A separate typed Rust harness emitted each chunk byte-identically and direct `cmp` against the original reference passed. The compiled aggregate example is extended to include both; no fabricated zeros or borrowed OEM executable opcode arrays appear in the committed source.
- Contiguous Agent 04 region becomes **120,276 bytes** `[0x40a2c,0x5e000)`. The assigned interval remains `<0x68000` and the prefix `[0x40000,0x40a2c)` is still outside proven coverage. The function at `0x5e000` is unfinished; do not claim this interval alone represents a new deployable image.
- Git ancestry remains provisional from older Agent 03, not checkpoint 50. Full offline Rust/Python tests and release build will be recorded after verification. RAM-capped compiler runs use `CARGO_BUILD_JOBS=1`, `CARGO_PROFILE_TEST_DEBUG=0`, `CARGO_PROFILE_DEV_DEBUG=0`; release build uses `CARGO_PROFILE_RELEASE_OPT_LEVEL=0` to bound memory/time.

## Checkpoint 41 — codec configuration dispatch, mask tables, and descriptor updates

- Two new adjacent source-generated runtime blocks `[0x5e000,0x5f000)` and `[0x5f000,0x60000)` contribute **8,192 byte-identical bytes**, firmware MBN file `[0x62000,0x64000)`. The first has **925 typed ARM32 instructions** plus **99 explicitly modeled data words** (large 44-word diagnostic table, two smaller jump tables, and branch-skipped parameter pointers). The second has **1,014 instructions** and **10 literal data words** after a return.
- Distinct packed opcode data is tracked via `INLINE_DATA_*` arrays, with data ranges inspected for PC-relative load references, unconditional branch skips, and jump-table control flow. Each 4,096-byte region matches the pinned original reference with `cmp` and independent Clang/LLVM reconstruction; neither contains a copied OEM instruction-word array or inferred padding.
- Combined Agent 04 contiguous region is **128,468 bytes** `[0x40a2c,0x60000)`. The prefix `[0x40000,0x40a2c)` remains unknown and the assigned upper limit is exclusive `0x68000`.
- The interval spans portions of multiple controller functions and is not standalone executable or a complete/signed firmware image. All tests and aggregate checks are recorded in the checkpoint-41 logs after a memory-limited build.

## Checkpoint 42 — controller resource tables and indexed dispatch continuation

- Added adjacent generated A32 fragments `[0x60000,0x61000)` and `[0x61000,0x62000)` = **8,192 byte-identical bytes**; pinned reference Venus `venus.mbn` offsets `[0x64000,0x66000)`. Independent typed Rust emitters and Clang/LLVM assembly reproduce all bytes, with no OEM instruction arrays or fabricated fill.
- The first block contains **958 typed ARM32 instructions plus 66 separately classified data words** across 9 pools. The second has **973 instructions and 51 structured data words** in 9 pools, including a branch-skipped reciprocal constant `0xaaaaaaab` at runtime `0x617e8`. Source-level classification requires control-flow/literal validation and is not demonstrated merely by opcode decoding.
- Combined contiguous Agent 04 interval is **136,660 exact bytes**, runtime `[0x40a2c,0x62000)`. Reservation prefix `[0x40000,0x40a2c)` remains unknown and exclusive upper bound is `<0x68000`. Aggregate CLI continuity from `[0x55400,0x62000)` requires its separate repository test.
- Shared compiler state is constrained to single-job builds to fit the 4 GiB memory cgroup. The routine at the upper boundary `0x62000` may continue into the next function; this checkpoint is not a standalone firmware image or newly signed executable. Git ancestry remains provisional from the older Agent 03 branch rather than checkpoint 50.

## Checkpoint 43 — controller state callbacks and 10-target dispatch

- Contiguous typed A32 fragments `[0x62000,0x63000)` and `[0x63000,0x64000)` add **8,192 reference-identical bytes**, pinned Venus MBN file offsets `[0x66000,0x68000)`. Independent Clang/LLVM reassembly of each mixed code/data block reproduces every byte; the source doesn't import opaque OEM instruction words.
- `[0x62000,0x63000)` has **1,018 ARM32 instructions** and six separately classified diagnostic pointers, in four pools; includes checked register-indexed signed-halfword loads at `0x620a0` and `0x620a8`.
- `[0x63000,0x64000)` has **1,008 ARM32 instructions** and 16 named data words in four pools: diagnostics at `0x63588`, `0x636f8–0x63704`, `0x63834–0x6383c`, and a ten-target controller dispatch table at `0x63b00–0x63b28`, selected by indexed branch-load logic.
- Combined Agent 04 uninterrupted region `[0x40a2c,0x64000)` grows to **144,852 bytes**; the original reservation prefix `[0x40000,0x40a2c)` is unimplemented, and the upper limit `0x68000` remains exclusive. These fragments are not independent executable firmware images and their byte equality does not establish higher-level behavioral equivalence.
- Full offline workspace Rust/Python tests, aggregate `agent20_04_continue` verifier, and opt0 release-build results are recorded separately in the checkpoint logs, avoiding falsely reporting tests not actually run.
