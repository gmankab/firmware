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
