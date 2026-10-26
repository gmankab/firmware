# Agent 20-06 — Venus executable tail (checkpoint 01)

**Scope:** first Venus executable `PT_LOAD`, ARM runtime `[0xa8000,0x1029f4)`, MBN file offset `runtime+0x4000`. Do not edit any earlier ARM runtime address; the next writable `PT_LOAD` at file offset `0x10b000` is NOT part of the executable segment. Disassembly must be reviewed for code/data rather than treating every aligned word as instruction.

**Starting-repository constraint:** checkpoint 50 SHA `b7ed4b587633b9b475a40f5a3ace5cc90db620ff` and Agent 20-01's merged descendant were **not present** in this session. The preparatory branch `agents20/06-venus-tail-preparatory` descends from Agent 03's final older-base commit `a3ffc58da64d8f96a65b9a48f4bed24592ba0ac1`, which itself does not contain checkpoint 50. Treat this as a **portable, disjoint, old-base implementation**; cherry-pick or patch onto a verified future checkpoint-50 descendant and test again. No checkpoint-50 integration is claimed.

## Checkpoint 01 — first executable interval

- **ARM runtime:** `[0xa8000,0xa83f8)`, length `0x3f8` = **1,016 bytes**.
- **Original MBN file offsets:** `[0xac000,0xac3f8)`, pinned `venus.mbn` SHA-256 `9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085`.
- **Executed A32 instructions:** 224 typed operations, each produced by a checked public ARM32 constructor.
- **Non-executable literal/data:** 30 words (120 bytes), modeled separately as controller/error addresses, inline diagnostic values and status flags; located at `[0xa8114,0xa8124)`, `[0xa835c,0xa8360)`, `[0xa838c,0xa8390)`, `[0xa8398,0xa83f8)`.
- **Independent Clang mnemonic assembly:** ARM-mode Clang/LLVM verifies the instruction encodings and distinct data pools; **zero mismatches** across all 1,016 bytes.
- **Reference comparison:** `cargo run --offline --locked --bin agent20_06_venus_tail -- verify /private/venus.mbn` returns exact, no firmware payload is needed to **emit** the rebuilt bytes.
- **Function boundary caveat:** `0xa8000` is inside an earlier controller function starting *below* Agent 20-06's exclusive range. The first new function boundary is `0xa83f8`. Because this fragment references several earlier conditional branches and the source address `0xa8000` is not an independent entrypoint, it is **not independently callable**. Future merged `ImagePlan` must join this to Agent 20-05's preceding code only after byte and control-flow verification.
- **Next uncovered ARM runtime address:** `0xa83f8` (MBN file `0xac3f8`).

## Validation and restrictions

```sh
cargo test --workspace --offline --locked
cargo build --workspace --release --offline --locked
python3 -m unittest discover -s tests -p 'test_*.py'
cargo run --offline --locked --bin agent20_06_venus_tail -- verify /private/venus.mbn
cargo run --offline --locked --bin agent20_06_venus_tail -- emit /tmp/venus-tail-agent20-06.bin
git diff --check
```

The fragment is **instruction-level, byte-identical re-encoding**, not proof of full high-level semantic decompilation, a hardware-tested firmware, an OEM-signed image, or completion of the giant first `PT_LOAD`. The reference is kept outside Git; static data pools are clearly classified and never counted as executable instructions.

**Integration ownership:** New `venus_runtime_tail_agent20_06.rs`, unique CLI binary `agent20_06_venus_tail.rs`, progress notes; one explicit export in `devices/xiaomi_nabu/mod.rs`. No edits to Agent 20-01/03/05 source regions or shared MBN/ELF code. Keep module-export change distinct for conflict resolution during the real checkpoint-50 integration.

## Checkpoint 02 — controller allocator and callback table

- Byte-exact ARM32 runtime `[0xa8000,0xa8cbc)`, MBN file `[0xac000,0xaccbc)`, total **3,260 consecutive bytes**.
- Newly implemented `[0xa83f8,0xa8cbc)` = **2,244 bytes** (499 typed instructions + 62 non-executable literal words).
- The function starts at `0xa83f8` with its original stack-frame prologue, allocates/initializes controller channel buffers, stores codec callback pointers and routes allocation failures through the original logging/status handling. It returns by `pop {...,pc}` at `0xa8944` and contains branch-only error paths through `0xa8bc0`.
- Three separate tables/data pools: `[0xa8bc4,0xa8c24)` 24 controller constants; `[0xa8c24,0xa8c90)` 27 callback/global-binding values; `[0xa8c90,0xa8cbc)` 11 diagnostic pointers. **Never count these 62 words as independently reconstructed ARM instructions.**
- Independent Clang assembly reconstructed mnemonic instructions and explicitly identified data tables; **zero differences** on all 2,244 bytes. The full 3,260-byte Rust fragment also compares equal to original Venus reference.
- Next uncovered runtime `0xa8cbc` (`venus.mbn` offset `0xaccbc`), exactly a new `push` function prologue.

## Checkpoint 03 — codec record-slot initialization

- Cumulative contiguous byte-verified interval `[0xa8000,0xa8ea0)` = **3,744 bytes**; MBN file `[0xac000,0xacea0)`.
- Newly reconstructed `[0xa8cbc,0xa8ea0)` = 484 bytes: **117 typed A32 instructions** and 4 non-instruction diagnostic addresses at `[0xa8e90,0xa8ea0)`.
- Function boundaries at `0xa8cbc`, `0xa8d50`, `0xa8db4` and `0xa8de4`; the last function returns through `bx lr` at `0xa8e8c` and preserves its diagnostic literal pool.
- Independent Clang/LLVM mnemonic reassembly and direct Rust byte comparison both match the entire newly added 484-byte interval exactly.
- The next uncovered ARM32 runtime address is `0xa8ea0` (file offset `0xacea0`), a new codec-buffer helper.

## Checkpoint 04 — indexed slot accessors and verifier

- Byte-exact total ARM32 runtime `[0xa8000,0xa92c8)` = **4,808 bytes**, private original file `[0xac000,0xad2c8)`.
- Newly independently built `[0xa8ea0,0xa92c8)` = **1,064 bytes**: 263 typed ARM32 instructions (1,052 bytes), and three separately identified repeated diagnostic pointers (12 bytes) at `0xa9034`, `0xa90d8`, `0xa911c`.
- Function-level behavior: clear flags on codec command slots, derive indexed records using 84-byte strides, validate pointer initialization, update per-record metadata and report invalid use. Several helpers return via `bx lr` and others use guarded stack epilogues.
- The new checked architecture encoder `arm32::transfer_byte_postindexed_register` uses public A32 register-offset addressing and rejects unsafe PC or load-into-base; committed separately from firmware work to isolate shared changes.
- A new agent-local independent LLVM/Clang mnemonic round-trip helper `tools/agent20_06_a32_roundtrip.py` requires **explicit classification of every data pool** and errors on unsupported or unclassified instructions, using private reference bytes only for the validation process. It verifies all completed intervals individually with zero mismatches.
- Next uncovered runtime address: `0xa92c8`, a new `push`-framed controller routine. The current source interval remains independently disjoint from other agents and has NOT been integrated atop checkpoint 50.

```sh
python3 tools/agent20_06_a32_roundtrip.py /private/venus.mbn 0xa8ea0 0xa92c8 \
  --data-range 0xa9034:0xa9038 --data-range 0xa90d8:0xa90dc \
  --data-range 0xa911c:0xa9120
```

## Checkpoint 05 — indexed codec map and packed hardware fields

- Total Venus tail byte-identical runtime `[0xa8000,0xa9578)` = **5,496 bytes**, original MBN file `[0xac000,0xad578)`.
- Added `[0xa92c8,0xa9578)` = 688 bytes consisting of **171 ARM32 instructions (684 bytes)** and **one global-pointer data word** (`0xa9514`, value `0x0010dd64`).
- Controller mapping calculates bit-packed codec indices and three register fields from 84-byte per-slot structures; short resolver functions publish allocated slot pointers and a hardware fallback.
- The Rust emitter and independent Clang/LLVM assembly both match the private `venus.mbn` exactly, with no copied instruction opcode words in the source. The verifier was updated to normalize ARM's `BLO` (carry clear) branch alias, ensuring the independently assembled branch at `0xa9504` is checked correctly.
- Next uncovered ARM runtime address `0xa9578` (MBN file offset `0xad578`), a new `push`-framed function.

## Checkpoint 06 — next controller codec format prefix

- Cumulative verified runtime `[0xa8000,0xa9800)` = **6,144 byte-identical bytes**, original file `[0xac000,0xad800)`.
- Added `[0xa9578,0xa9800)` = **648 typed A32 instruction bytes**, no literal data in this interval. The next long codec controller routine begins with its original stack prologue at `0xa9578`, copies format metadata to hardware buffer control fields, and checks incoming stream flags.
- This *function is incomplete*: it continues beyond the checkpoint boundary `0xa9800`. Later branches and literal pools must be reconstructed before claiming the complete routine.
- Independently verified 162 instructions by Clang/LLVM mnemonic assembly and direct typed Rust emitter reference comparison; zero differing bytes.
- Exact next missing ARM address: `0xa9800` (original `venus.mbn` file offset `0xad800`).

## Checkpoint 07 — fast codec plane scaling and blending

- Runtime interval `[0xa8000,0xa9cf0)` now yields **7,408 consecutive bytes**; private original MBN file offsets `[0xac000,0xadcf0)`. The added 1,264 bytes at `[0xa9800,0xa9cf0)` consist of 316 typed ARM32 instructions with **zero** literal words.
- The code prepares scaling/quantization metadata and updates codec buffer accounting. It returns at `0xa9cec` on a fast path, but the enclosing routine still has later error paths beginning at `0xa9cf0`, so this checkpoint is not a complete function.
- The shared checked public A32 encoder `signed_multiply_accumulate_bottom_halves` (`SMLABB`) is new. It encodes the low signed halfword multiply/accumulate at `0xa9bcc` and `0xa9bdc`, and rejects PC operands. Encoder source and tests are committed separately for integration review.
- Direct source-generated Rust output matches all 7,408 reference bytes (SHA-256 `5a1a09cd923b2b5a624e36031c901f1120aa83cd2a39336df98f64a41ec40737`); independent Clang/LLVM reassembly of the new 316 instructions exactly matches original. All 189 Rust tests, 13 Python tests and offline release build passed.
- Next missing runtime `0xa9cf0` (`venus.mbn` offset `0xadcf0`). No external OEM data is copied into the emitter. Remains provisional relative to unavailable checkpoint 50 integration base.

## Checkpoint 08 — complete codec plane blending with error handling

- Byte-identical ARM runtime `[0xa8000,0xaa22c)`: **8,748** consecutive bytes, MBN `[0xac000,0xae22c)`; SHA-256 `1d5215b5b71021ad2995b3d90ce4d823f3fea97056f94ab5e4cf301609b45a4b`.
- Reconstructed additional `[0xa9cf0,0xaa22c)` = **1,340 bytes**: 321 typed ARM32 instructions (1,284 bytes), followed by 14 independently classified non-executable literal/data words (56 bytes) at `[0xaa1f4,0xaa22c)`. The data include codec status addresses and `0x77777777` patterns which the disassembler misreads as opcodes. No OEM bytes were copied into the generator.
- The session begun at `0xa9578` now includes both its fast-path return at `0xa9cec` and its fallback paths and end-of-function data pool at `0xaa22c`.
- Added checked `arm32` halfword load with pre-indexed **register** writeback, pre-indexed immediate writeback, and immediate post-indexing; dedicated ISA tests check exact independently decoded opcodes and reject unsafe register combinations. These changes are isolated in a shared-encoder commit.
- Independent Clang/LLVM ARM mnemonic reassembly: 1,284 code bytes and 56 classified data bytes, zero differences. Full offline cargo test: 190 Rust tests passed, 13 Python tests passed; offline release build succeeded.
- Next uncovered VMA `0xaa22c`, the next ARM32 function. This is still on a provisional branch derived from older Agent 03 commit `a3ffc58...`, not the inaccessible checkpoint-50 base.

## Checkpoint 09 — controller output descriptor and quantization buffer

- Source-built Venus runtime `[0xa8000,0xaa6f0)` now covers **9,968 consecutive reference-exact bytes**. Original file offsets `[0xac000,0xae6f0)`. Whole fragment SHA-256 `1ebd61d7464516c47b31099e6f4b42c5a039dc5f6cb03e53938b7f15d6efb235`.
- Additional routine `[0xaa22c,0xaa6f0)` = 1,220 bytes: **303 ARM32 instructions** (1,212 bytes), and two separate global-pointer data words at `[0xaa6e8,0xaa6f0)` (8 bytes). Builds per-field codec-mode flags, packs controller quantization records, checks buffer limits and dispatches an optional callback. Original return is a register-indirect `BX r3` at `0xaa6e4`; no synthetic return is emitted.
- The reusable checked ISA adds `transfer_halfword_register` (LDRH/STRH, positive register offset without writeback) and `transfer_byte_register_preindexed_writeback` (STRB/LDRB register preindex writeback), with focused architectural tests and unsafe-PC/repeated-base checks. Encoder changes are an independent commit for integration review.
- Both generated Rust and independent Clang/LLVM reconstruction match the original, 1,212 instruction bytes and 8 data bytes with zero differences. Cargo workspace passes 191 Rust tests, 13 Python tests pass, and the offline release build succeeded.
- Next uncovered ARM runtime `0xaa6f0` (`venus.mbn` file offset `0xae6f0`) is a new controller function. The work is a typed instruction-level rebuilding only, not source-level decompilation/hardware verification or an OEM-signable image. It remains a provisional patch rooted in the older Agent 03 branch due to inaccessible checkpoint-50 base.

## Checkpoint 10 — codec channel command queue and aux quality tables

- Total private-reference-exact Venus runtime `[0xa8000,0xab04c)` = **12,364 bytes**, MBN offsets `[0xac000,0xaf04c)`. SHA-256 of source-generated fragment `e8ece6a6aceda58c7f04a07c515aa1b9c8dd4ac07878e44fc46078be37be55d7`.
- Newly reconstructed complete function `[0xaa6f0,0xab04c)` = **2,396 bytes**: 592 typed A32 instructions (2,368 bytes) and seven separately identified literal addresses (28 bytes) at `[0xab030,0xab04c)`. The original return is `pop {..., pc}` at `0xab02c`; no synthetic return, OEM opcode arrays or implicit zero fill are used.
- The command queue initializes controller MMIO packet fields, updates quality/cached buffer tables, handles special codec mode paths and optional diagnostics. The independent Clang/LLVM assembler reassembled 2,368 new code bytes plus the explicit 28 data bytes with zero differences.
- All 191 Rust tests, 13 Python tests and offline release build passed. The next missing runtime address is `0xab04c` (file `0xaf04c`), starting a new function. This remains an **instruction-level** byte-matched reconstruction based on the older provisional Agent 03 branch, not checkpoint 50 integration.

## Checkpoint 11 — controller FIFO, DMA and per-frame registers

- Total source-generated `[0xa8000,0xab784)` = **14,212 byte-identical bytes**, file `[0xac000,0xaf784)`; SHA-256 `780988a1737209ac7885f5d05632f909218777276519d23c5b59e583507adfa1`.
- Added complete function `[0xab04c,0xab784)` = **1,848 bytes**: 455 public-ARM32-encoder instructions (1,820 bytes), seven separately classified diagnostic/global words (28 bytes) at `[0xab768,0xab784)`. Its actual final `pop {...,pc}` occurs at `0xab764`.
- Handles controller state preparation, DMA parameters, register FIFO sequences, and conditional branches. No original firmware instructions are embedded as data arrays.
- Independent LLVM/Clang ARM assembler verifies all 1,820 code bytes and 28 data bytes exactly. Full cargo workspace 191 Rust tests, 13 Python tests, release build pass. Next uncovered runtime address: `0xab784` (`venus.mbn` file offset `0xaf784`).
- The standalone branch is still provisional, descending from older Agent 03, not checkpoint 50. Instruction identity does not establish semantic source reconstruction, emulated behavior, or OEM signing.

## Checkpoint 12 — three codec slot and parameter helpers

- Source-authored typed Venus runtime `[0xa8000,0xaba80)` = **14,976 contiguous byte-identical bytes**, private original MBN offsets `[0xac000,0xafa80)`; SHA-256 `6e69c8e6503313558daf0a4bfecdf29be0b4fd5b65a6c5c3b7b89649a74b5873`.
- Newly added `[0xab784,0xaba80)` = **764 bytes**: three ARM32 helpers with 189 typed instructions (756 bytes) and two global data-address words (8 bytes) at `[0xab8e4,0xab8ec)`. The helpers remap controller slot pointers, link a selected codec slot by tag and publish mode/parameter defaults. The original returns at `0xab8dc`, `0xab960`, and `0xaba7c`; no synthetic epilogues or filler.
- Independent Clang/LLVM reconstructed all 756 code and 8 explicitly typed data bytes with zero differences. Full 191 Rust tests, 13 Python tests and release build passed.
- Next uncovered runtime `0xaba80` (private MBN offset `0xafa80`). Work is still provisional, not based on inaccessible checkpoint 50 and not independently reverse engineered to high-level functional semantics.

## Checkpoint 13 — codec event forwarding and per-mode register packets

- The byte-identical source-generated Venus fragment is now `[0xa8000,0xac750)` = **18,256 contiguous bytes**, original MBN `[0xac000,0xb0750)`; SHA-256 `f67d0842a6efb6dcc843b02d2060b168917a0e35a5a674bd3293b1dee91f6479`.
- New `[0xaba80,0xac750)` = **3,280 bytes**, containing **806 typed A32 instructions** (3,224 bytes), plus 14 separately identified data words (56 bytes) in two pools: `[0xabbe4,0xabbf8)` five words and `[0xac72c,0xac750)` nine words. No raw OEM instructions in Rust generator.
- First routine dispatches codec events via a logger tail-branch; second programs multiple codec register streams and handles mode-specific diagnostics and fallback branches. The original main return is `pop {...,pc}` at `0xac5ec`; extended alternative paths occupy `0xac5f0–0xac72c`.
- Added checked `arm32::load_signed_halfword_register` (A32 LDRSH unshifted register offset), with opcode and PC-rejection tests. Independent Clang/LLVM ARM assembler matched 3,224 instruction bytes plus 56 classified data bytes exactly.
- 192 offline Rust tests, 13 Python tests, and full release build all passed. Next uncovered VMA `0xac750` (original file `0xb0750`), a new ARM32 function. Checkpoint remains provisional relative to inaccessible checkpoint-50 integration branch.

## Checkpoint 14 — frame release and scaled codec allocator

- Contiguous byte-identical ARM runtime `[0xa8000,0xace00)` = **19,968 bytes**, MBN file offsets `[0xac000,0xb0e00)`. The prior 18,256 bytes remain unchanged.
- Newly reconstructed `[0xac750,0xace00)` = 1,712 bytes, comprising **425 ARM32 instructions** (1,700 bytes) and three trailing diagnostic-address words (12 bytes) at `[0xac9e4,0xac9f0)`.
- First complete function `[0xac750,0xac9f0)` serializes codec session release, input dimensions, callback release, and diagnostic reporting. The second, larger codec allocator begins at `0xac9f0` and remains incomplete at `0xace00`.
- Added a checked ARM32 `UMULL`/`SMULL` encoder with explicit low/high destinations and PC/alias rejection, unit-tested against the reference instruction encodings.
- Independent Clang/LLVM round-trip verified `[0xac750,0xace00)` using `tools/agent20_06_a32_roundtrip.py` with `--data-range 0xac9e4:0xac9f0`, **zero byte differences**.
- No original OEM opcode blobs are embedded; this is byte-identical **instruction-level** re-encoding and not a high-level source reimplementation, hardware test, newly signed MBN, or completed assigned tail.
- Next uncovered ARM runtime address `0xace00`, original MBN offset `0xb0e00`.

## Checkpoint 15 — scaled codec allocation and callback getter

- Exact source-generated runtime `[0xa8000,0xad728)` = **22,312 bytes**, reference file offsets `[0xac000,0xb1728)`; original Venus SHA-256 remains pinned.
- Newly implemented `[0xace00,0xad728)` = 2,344 bytes, with **578 typed ARM32 instructions (2,312 bytes)** and 8 separately identified constant/address words (32 bytes): arithmetic/diagnostic pool `[0xad6f0,0xad70c)` (7 words), callback pointer `[0xad724,0xad728)` (1 word).
- Completes the large allocator started at `0xac9f0`, including codec plane-size calculations, DMA/frame buffers and state/diagnostic updates. A six-instruction callback-tail helper `[0xad70c,0xad724)` follows.
- New independently tested architectural `SMULBB` instruction encoder lives in shared `crates/arm32`; signed bottom-half multiplication is validated against Clang. No embedded OEM executable opcode arrays.
- Independent Clang/LLVM A32 mnemonic assembly matches 2,312 instruction bytes and 32 data bytes exactly using `--data-range 0xad6f0:0xad70c --data-range 0xad724:0xad728`.
- Full Rust/Python suites and offline release build pass. Next uncovered runtime address `0xad728`, original file offset `0xb1728`.
- This remains instruction-level byte-identity only; behavior, OEM signatures, and checkpoint-50-based integration are not established.

## Checkpoint 16 — codec configuration descriptor and bounded signed-byte field

- Independently emitted `[0xa8000,0xadc14)` = **23,572 contiguous byte-identical bytes**; MBN `[0xac000,0xb1c14)`.
- Added complete record initialization function `[0xad728,0xadc14)`, 1,260 bytes: **314 ARM32 instructions (1,256 bytes)** and one diagnostic pointer word at `[0xadc10,0xadc14)`.
- The function configures codec mode fields and input dimensions, copies and normalizes up to five resource words with `LDM/STM`, and populates the conditional auxiliary plane descriptor fields before returning via original `pop {...,pc}` at `0xadb84`. It retains actual later error branches `0xadb88–0xadc0c` rather than inventing sequential control flow.
- Fixed one ISA distinction: `LDRSB` signed byte register-offset at `0xad920` must sign-extend rather than use unsigned `LDRB`; added checked `arm32::load_signed_byte_register` encoder and unit tests.
- LLVM/Clang round-trip matches the full interval using `--data-range 0xadc10:0xadc14`, **zero differences**.
- Full Rust/Python tests pass; release build result is recorded separately. Next uncovered address `0xadc14` (MBN file `0xb1c14`). No OEM signature, complete image or semantic high-level equivalence is asserted.

## Checkpoint 17 — controller channel activation, reset and deactivation

- Contiguous source-built Venus runtime `[0xa8000,0xae020)` = **24,608 byte-identical bytes**, MBN file `[0xac000,0xb2020)`.
- Newly constructed `[0xadc14,0xae020)` = 1,036 bytes: **255 typed A32 instructions (1,020 bytes)** and four diagnostic data words (16 bytes), two at `[0xadf10,0xadf18)` and two at `[0xae018,0xae020)`.
- Includes complete surface/slot initialization `[0xadc14,0xadcbc)`, guarded buffer activation `[0xadcbc,0xadf18)` and guarded buffer deactivation `[0xadf18,0xae020)`; original conditional branches and returns preserved.
- Independent LLVM/Clang round-trip passes with `--data-range 0xadf10:0xadf18 --data-range 0xae018:0xae020` and zero differing bytes.
- Full Rust/Python suites pass; release build result is separately recorded. Next uncovered ARM runtime address `0xae020` (file `0xb2020`). This remains partial instruction re-encoding; no behavioral/hardware validation or newly OEM-signed image is claimed.

## Checkpoint 18 — controller register plane initialization and buffer publication

- Cumulative exact Venus runtime `[0xa8000,0xae748)` = **26,440 byte-identical bytes**, MBN original file `[0xac000,0xb2748)`.
- New interval `[0xae020,0xae748)` = 1,832 bytes: **456 ARM32 instructions** (1,824 bytes) and **two data words** (8 bytes) at `0xae58c` and `0xae744`. Correct count: code intervals `[0xae020,0xae58c)` and `[0xae590,0xae744)`, data at `[0xae58c,0xae590)` and `[0xae744,0xae748)`.
- Implemented codec channel buffer retirement/clear, mode-specific controller register encoding and per-channel output buffer publishing. At `0xae2f0` the conditional `ANDEQ` is real instruction, not assumed zero data. Original final returns at `0xae104`, `0xae588`, `0xae644`, `0xae740`.
- Independently verified by Clang/LLVM with `--data-range 0xae58c:0xae590` and `--data-range 0xae744:0xae748`; **zero mismatches**. Literal pool regression tests include the new pools.
- Rust and Python suites pass. Offline release-build status tracked separately. Next uncovered ARM address `0xae748` (MBN file `0xb2748`). No high-level behavioral equivalence, OEM signing, or checkpoint-50 integration claimed.

## Checkpoint 19 — codec stream-event dispatch with table and diagnostics

- Cumulative exact source-generated runtime `[0xa8000,0xaf290)` = **29,328 byte-identical bytes**, reference MBN `[0xac000,0xb3290)`.
- New interval `[0xae748,0xaf290)` = 2,888 bytes: **664 ARM32 instructions (2,656 bytes)**, **41 handler-address words (164 bytes)** at `[0xae984,0xaea28)` and **17 diagnostic/control data words (68 bytes)** at `[0xaf24c,0xaf290)`.
- Implements the codec stream-event scheduling paths, callback validation, command queue, hardware status responses and a 41-target dispatch table containing repeated target addresses that must not be interpreted as executable opcodes.
- Independent LLVM/Clang reassembly exactly reproduced all instruction/data words; explicit `--data-range 0xae984:0xaea28 --data-range 0xaf24c:0xaf290` flags were required.
- Added an independent Rust regression checking legal dispatch destinations, frequency of repeated branch targets and selected non-code diagnostic values.
- Rust/Python test suites pass. Next uncovered runtime address `0xaf290` (file `0xb3290`), a separate short controller helper. This remains provisional old-base instruction-level reconstruction, not independently signed or hardware-validated firmware.

## Checkpoint 20 — codec format lookup, reverse map and allocator helpers

- Verified contiguous Venus runtime `[0xa8000,0xaf6e0)` = **30,432 exact bytes**, MBN original file `[0xac000,0xb36e0)`.
- Added `[0xaf290,0xaf6e0)` = 1,104 bytes, comprising **263 A32 instructions (1,052 bytes)** and **13 words of explicit non-executable data (52 bytes)**: eight-jump format table `[0xaf370,0xaf390)`, two scaling/data constants `[0xaf5b8,0xaf5c0)`, and three diagnostic words `[0xaf688,0xaf694)`.
- Implements codec parameter extraction, 8-way format-to-controller-code mapping, reverse codec format lookup, scaling prediction and a bounded allocator helper. All functions preserve their actual `BX LR` or `POP ... PC` return instructions and conditional error branches.
- Independent LLVM/Clang round-trip with all three `--data-range`s reconstructs exactly 1,052 instruction and 52 data bytes; **zero byte differences**.
- Rust/Python tests pass; next uncovered address `0xaf6e0` (original file `0xb36e0`). Checkpoint remains provisionally based on older Agent 03 rather than checkpoint-50 merge, and demonstrates ISA-level byte identity, not semantic or OEM-signed firmware rebuild.

## Checkpoint 21 — codec command buffer publication and diagnostics

- Cumulative independently source-encoded Venus runtime `[0xa8000,0xb02b0)` = **33,456 contiguous byte-identical bytes**, original MBN file `[0xac000,0xb42b0)`.
- Newly reconstructed `[0xaf6e0,0xb02b0)` = 3,024 bytes comprising **746 typed ARM32 instructions** (2,984 bytes), followed by **ten explicitly classified literal/data words** (40 bytes) `[0xb0288,0xb02b0)`.
- The routine prepares and publishes codec controller command packets and serialized buffer accounting, including error dispatch, optional message callbacks and hardware status updates. Its original `POP {...,pc}` returns at `0xb0284` before the literal data pool. No proprietary machine-opcode slices are pasted into Rust.
- Independent Clang/LLVM reconstruction of `[0xaf6e0,0xb02b0)`, with `--data-range 0xb0288:0xb02b0`, matches the reference *exactly*; the whole generated fragment also compares equal. A Rust regression test now asserts the literal-pool words and their order.
- Next unimplemented runtime address `0xb02b0` (original file offset `0xb42b0`), a new small controller helper.
- The branch remains provisional, rooted in older Agent 03 history. This is typed instruction-level reassembly, **not** high-level source recovery, OEM signing, or working hardware validation.

## Checkpoint 22 — codec record setup and guarded controller callbacks

- Total contiguous original Venus runtime `[0xa8000,0xb0748)` = **34,632 byte-identical bytes**, original MBN file `[0xac000,0xb4748)`.
- Newly reconstructed `[0xb02b0,0xb0748)` = 1,176 bytes: **292 typed ARM32 instructions (1,168 bytes)** and two data words (8 bytes) at `[0xb0444,0xb0448)` and `[0xb06d8,0xb06dc)`. The first pointer resolves to the adjacent helper entry `0xb0448`, and the second is a diagnostic address.
- Captures codec record allocation, initialization of tagged byte/halfword fields, input/output size bounds, and guarded controller notification. Includes three separate helper/function entries at `0xb02b0`, `0xb0448` and `0xb06dc`. Returns retain the original `POP {...,PC}`.
- Direct Rust reference comparison and independent Clang/LLVM round-trip (explicit `--data-range 0xb0444:0xb0448 --data-range 0xb06d8:0xb06dc`) have **zero byte differences**. Rust regression checks both data words.
- Next missing runtime address `0xb0748`, MBN file `0xb4748`, begins a larger codec controller read/modify loop. Still a provisional old-base source-encoded fragment, not high-level source recovery or newly signed firmware.

## Checkpoint 23 — controller FIFO, codec records and IRQ response

- Verified contiguous runtime `[0xa8000,0xb0c64)` = **35,940 byte-identical bytes**, original file `[0xac000,0xb4c64)`.
- Newly source-encoded `[0xb0748,0xb0c64)` = 1,308 bytes: **322 ARM32 instructions** (1,288 bytes) and **five separately classified literal words** `[0xb0c50,0xb0c64)` (20 bytes). The instruction stream implements queue/plane reassignment, status comparison and codec IRQ notification; retains branches back into the preceding initialization path.
- New checked A32 `transfer_word_register_offset` supports **negative** shifted-register offsets without writeback. Unit tests validate exact architectural opcodes and reject unsafe PC operands; byte transfer reuses existing signed-index checked encoder. Shared encoder change committed separately for integration review.
- Direct typed Rust bytes match reference exactly. Clang/LLVM independently reassembles 1,288 instruction bytes and 20 data bytes (`--data-range 0xb0c50:0xb0c64`) with no differences. Regression test checks the five diagnostic literal words.
- Next uncovered VMA `0xb0c64`, MBN file `0xb4c64`, a short guarded codec helper. Work remains provisional from older Agent 03, not checkpoint-50 merge; is byte-identical instruction encoding, not independent high-level source reconstruction or OEM-authenticated image.

## Checkpoint 24 — guarded plane sizes, codec descriptors, stream line accounting

- Total contiguous byte-identical ARM runtime `[0xa8000,0xb11b4)` = **37,300 bytes**, MBN original `[0xac000,0xb51b4)`.
- New interval `[0xb0c64,0xb11b4)` = 1,360 bytes: **331 typed ARM32 instructions (1,324 bytes)** and 9 non-instruction data words (36 bytes) at `[0xb0d00,0xb0d0c)`, `[0xb0f40,0xb0f50)`, `[0xb11ac,0xb11b4)`.
- Functions verify codec plane-length limits, correct out-of-range counts with hardware diagnostics, update buffer-command checksums and dispatch callbacks. A conditional `STRH [R7],#2` is encoded using an existing checked halfword-postindex architecture helper; the retained after-return error branch is not discarded.
- Direct generated Rust comparison and independent Clang/LLVM round-trip of all 1,360 bytes give **zero mismatches**. Rust regression covers all three new diagnostic data pools.
- Next missing VMA `0xb11b4` (MBN file `0xb51b4`). Still a provisional old-branch instruction-byte re-encoding, not a fully recovered high-level source, not OEM signed.

## Checkpoint 25 — codec stream state, buffer registration and reporting

- Cumulative runtime `[0xa8000,0xb1b44)` = **39,748 consecutive reference-identical bytes**, corresponding MBN file `[0xac000,0xb5b44)`.
- Rebuilt new `[0xb11b4,0xb1b44)` = 2,448 bytes: **601 ARM32 instructions (2,404 bytes)** and **11 non-executable literal words (44 bytes)** in two separately classified pools at `[0xb1a08,0xb1a2c)` (nine words) and `[0xb1b3c,0xb1b44)` (two words).
- Reconstructs stream hardware initialization, data-plane/controller slot updates, callback/error handling and codec register reports. The original loop and conditional branches into earlier status transitions are retained. An original `POP {...,PC}` at `0xb1b38` returns before the second pool; no invented return is added.
- Direct Rust bytes match pinned original exactly. Independent Clang/LLVM round-trip for `[0xb11b4,0xb1b44)` verifies 2,404 typed instruction bytes and 44 data bytes with **zero mismatches**. Literal regression tests cover both pools.
- Next unimplemented VMA `0xb1b44` (file `0xb5b44`) begins the next codec controller helper. This remains provisional source-level A32 *instruction encoding*, not a behavioral decompilation or newly signed OEM image.

## Checkpoint 26 — codec command queue and frame-plane stream prefix

- Contiguous source-built Venus runtime `[0xa8000,0xb2000)` = **40,960 reference-identical bytes**, MBN file `[0xac000,0xb6000)`.
- Added `[0xb1b44,0xb2000)` = **1,212 typed ARM32 executable instruction bytes (303 instructions)**, with no inline literal/data words. It processes controller plane-size metadata, codec stream command/queue fields and status checks.
- The routine beginning at `0xb1b44` has an original branch/return at `0xb1de4`, but additional fallback branches continue afterward; it is **incomplete at `0xb2000`**. Do not describe it as a finished semantic function. The following first unimplemented runtime address is `0xb2000` (file `0xb6000`).
- Full generated fragment and independent Clang/LLVM mnemonic reassembly match the original exactly, **zero mismatches**. Still provisional, based on older Agent 03, with no hardware behavior/OEM signing claims.

## Checkpoint 27 — completed frame/plane controller command queue

- Cumulative runtime `[0xa8000,0xb2a80)` = **43,648 consecutive byte-identical bytes**, original MBN file `[0xac000,0xb6a80)`.
- New interval `[0xb2000,0xb2a80)` adds **2,688 bytes**, 657 typed A32 instructions (2,628 bytes) plus **15 independently classified data words** `[0xb2a44,0xb2a80)` (60 bytes). These include diagnostic strings, codec mask/controller global addresses and a pointer back to helper `0xb0448`.
- Completes controller frame-queue processing: command handling, slot format transitions, optional buffer expansion, register readback, error callbacks, and packet publication. The final branch at `0xb2a40` returns into existing command logic; next distinct routine begins `0xb2a80`.
- Direct generated-byte comparison and independent Clang/LLVM assembler round-trip pass with zero mismatches, with the data region explicitly excluded from instruction decoding. Rust regression checks all 15 words.
- The assigned range after this checkpoint still has unimplemented bytes; next uncovered VMA `0xb2a80` (file `0xb6a80`). Provisional old-base branch only; not a new signed/flashable firmware or behavioral proof.

## Checkpoint 28 — command-output readback and codec response arrays

- Verified cumulative runtime `[0xa8000,0xb2f08)` = **44,808 contiguous byte-identical bytes** (reference `venus.mbn` file `[0xac000,0xb6f08)`). The complete emitted fragment is independent of reading the private OEM reference; the optional `verify` command performs the comparison.
- New `[0xb2a80,0xb2f08)` = **1,160 bytes**, consisting of **284 typed ARM32 instruction encodings (1,136 bytes)** and six explicitly named data words (24 bytes) at `[0xb2d30,0xb2d48)`. The first routine initializes codec report records, loops through hardware response slots and updates completion/status fields. The second reads controller channel responses into record-plane status arrays, with a conditional early `POP {...,PC}` at `0xb2e58` and final return at `0xb2f04`.
- The diagnostic/data words are pointer-like values (`0x000f89a4`, `0x000f89ce`, `0x000e2788`, `0x000f88c7`, `0x000f7058`, `0x000f7095`), not unclassified A32 instructions.
- Independent Clang/LLVM `tools/agent20_06_a32_roundtrip.py` with `--data-range 0xb2d30:0xb2d48` reconstructs all 1,160 bytes with zero differences. Rust direct verifier and fragment-specific tests also pass; the full offline Rust and Python suites pass. Release build status is logged separately; do not claim success if incomplete.
- Next uncovered ARM runtime address is **`0xb2f08`** (original file `0xb6f08`). Work remains a *provisional older-base typed instruction re-encoding* rather than checkpoint-50 integration, independent behavioral decompilation, hardware testing, complete firmware or OEM signing.

## Checkpoint 29 — queued codec-frame reconciliation

- Contiguous byte-identical runtime `[0xa8000,0xb3bdc)` = **48,092 bytes**, original MBN file `[0xac000,0xb7bdc)`.
- Added `[0xb2f08,0xb3bdc)` = **3,284 bytes**, comprising **816 typed ARM32 instructions (3,264 bytes)** and **five data words (20 bytes)** `[0xb3bc8,0xb3bdc)`, with values `0x000f8c24`, `0x000e2788`, `0x000f8be8`, `0x000f8c4b`, `0x000f8c72` classified as diagnostics, not executable opcodes.
- The function handles active and queued codec-plane records, branch-back resource status loops, signed byte loads, bounded descriptor scans and callback notification. The last branch at `0xb3bc4` returns to an earlier status path; `0xb3bdc` is a new function entry.
- The private-reference compare and the independent Clang/LLVM mnemonic assembly check **both report zero byte mismatches**. The full Rust workspace (199 tests) and 13 Python tests pass. No release-build success is claimed for this checkpoint.
- The branch is still *provisional*, rooted in the older Agent 03 history. This is byte-identical typed ARM32 encoding, **not** high-level source reconstruction, authenticated firmware, or on-device execution.

## Checkpoint 30 — controller slot compaction and live buffer setup

- Verified cumulative ARM runtime `[0xa8000,0xb40ac)` = **49,324 contiguous byte-identical bytes**, original MBN `[0xac000,0xb80ac)`.
- Added `[0xb3bdc,0xb40ac)` = **1,232 bytes**: **305 typed ARM32 instructions** (1,220 bytes) and **three explicitly classified non-code global/diagnostic words** `[0xb40a0,0xb40ac)` (12 bytes): `0x0010b654`, `0x0010b698`, `0x000f8be8`.
- The two functions `[0xb3bdc,0xb3d04)` and `[0xb3d04,0xb40a0)` compact per-channel buffer slots, publish hardware descriptor selections and reset controller state. The second original return is `pop {...,pc}` at `0xb409c` immediately before the data words; no synthetic return or opaque code blob is used.
- Added a checked `arm32::load_signed_byte_postindexed` encoder in a separate commit because the real A32 `LDRSB Rt,[Rn],#imm` at `0xb3e0c` requires sign extension *and* post-incremented addressing. Tests compare independent opcode encodings and reject unsafe register aliasing.
- Full Rust emitter and independently assembled Clang mnemonic source both match pinned `venus.mbn` **exactly**; Rust 201 tests and Python 13 tests pass. No release-build pass is claimed in this checkpoint.
- Next uncovered runtime `0xb40ac` (reference file `0xb80ac`). The branch remains provisional older-base instruction-level re-encoding, not functional decompilation, re-signed firmware or checkpoint-50 integration.

## Checkpoint 31 — controller slot lifecycle and record ordering

- Cumulative runtime `[0xa8000,0xb46cc)` = **50,892 byte-identical bytes**, original MBN file `[0xac000,0xb86cc)`.
- Added `[0xb40ac,0xb46cc)` = **1,568 bytes**, including **388 ARM32 instructions (1,552 bytes)** and four separately distinguished data words (16 bytes): one warning pointer at `[0xb41c0,0xb41c4)` and three controller/global references `[0xb46c0,0xb46cc)`.
- Five controller functions manage frame-slot retirement, sorted record insertion, remaining-record compaction and channel-state notification. The function at `[0xb42dc,0xb43c4)` has a branch-only tail beginning at `0xb43c4`, and the final routine uses its original tail branch at `0xb46bc` prior to the data pool. No synthetic return instructions are inserted.
- New shared architecture encoder `arm32::transfer_word_postindexed_scaled_register` supports `[Rn], Rm, LSL #amount`, required by the real hardware loop at `0xb40d4` and a later record scan. It has a separate focused ISA commit and tests rejecting unsafe PC/alias/shift arguments.
- Whole generated fragment and independent Clang/LLVM mnemonic reassembly with two `--data-range` flags match the pinned reference with zero mismatches. **203 Rust tests** and **13 Python tests** passed; the release-build check is not claimed as passed for this checkpoint.
- Next uncovered runtime is `0xb46cc` (file `0xb86cc`). Source is provisional older-base ARM32 instruction-level encoding and not high-level semantic decompilation, on-device proof, authenticated firmware or checkpoint-50 merge.

## Checkpoint 32 — codec record ordering, stream feedback and controller flush

- Full contiguous Venus executable-tail region `[0xa8000,0xb5008)` = **53,256 exactly matching bytes**, original MBN file `[0xac000,0xb9008)`.
- Newly source-encoded `[0xb46cc,0xb5008)` = **2,364 bytes**, consisting of **577 typed A32 instructions (2,308 bytes)** and **14 separately classified non-code data words (56 bytes)**. Literal pools are `[0xb48ac,0xb48bc)` (four), `[0xb4e0c,0xb4e24)` (six), and `[0xb4ff8,0xb5008)` (four); no OEM instruction words are copied into Rust.
- Rebuilt record-index compaction, active-record registration, two plane/line byte-count helpers, controller flush, and stream-feedback state updates. Branches into previous routines and the original return paths are preserved.
- Direct `agent20_06_venus_tail verify` and independent Clang/LLVM `tools/agent20_06_a32_roundtrip.py` with the **three declared data ranges** both report **zero differing bytes**. Full offline Rust suite **204 tests**, Python suite **13 tests** pass; release-build status not claimed here.
- Next uncovered ARM runtime is `0xb5008` (reference file offset `0xb9008`), beginning another controller routine. The branch remains provisional old-base A32 instruction-level reconstruction, not genuine high-level semantic decompilation, newly signed OEM firmware, on-device execution, or a checkpoint-50-rooted integration.

## Checkpoint 33 — controller stream-class jump table and first record checks

- Cumulative reference-exact ARM runtime `[0xa8000,0xb5600)` = **54,784 contiguous bytes**, source MBN file `[0xac000,0xb9600)`.
- Newly reconstructed `[0xb5008,0xb5600)` = **1,528 bytes**: **368 typed A32 instructions (1,472 bytes)** plus **14 explicitly classified jump-table words (56 bytes)** at `[0xb5054,0xb508c)`. The table repeats the five target handler addresses exactly, and the original computed `LDR PC,[R7,R6]` at `0xb5050` remains a register-indexed PC destination rather than being rewritten as an ordinary load.
- Encodes classification of fourteen controller stream command types, associated error/suspend status conditions, buffer-slot flags and per-record metadata. This larger function is still incomplete at `0xb5600`; its later error branches remain beyond the checkpoint.
- Optional private-original verification and independent Clang/LLVM assembly (`--data-range 0xb5054:0xb508c`) both report **zero differing bytes** over the new interval and cumulative region. **205 Rust tests and 13 Python tests pass**. The prior checkpoint's data-pool test now checks the original milestone boundary independently of the extending `END` value.
- Next uncovered ARM runtime `0xb5600` (MBN file `0xb9600`). This remains provisional old-base instruction encoding rather than high-level decompilation, OEM signature recovery or device execution.

## Checkpoint 34 — complete codec stream-classification and controller diagnostics

- Complete cumulative typed Venus region `[0xa8000,0xb60ec)` = **57,580 exactly matching bytes**, original `venus.mbn` file `[0xac000,0xba0ec)`.
- The new `[0xb5600,0xb60ec)` = **2,796 bytes**: **681 independently typed ARM32 instructions (2,724 bytes)** and **18 explicitly classified non-instruction data words (72 bytes)** in the controller/diagnostic pool `[0xb60a4,0xb60ec)`.
- Completes the codec stream-classification function begun at `0xb5008`: command-state loops, slot class selection, session event/retry handling and controller buffer status. Its original branch-only terminal path at `0xb60a0` returns to an earlier controller state instead of inventing a new return instruction.
- Direct Rust verifier and Clang/LLVM independent reconstruction pass with zero differing bytes, using `--data-range 0xb60a4:0xb60ec`. Unit test validates all 18 controller data values separately. **206 Rust and 13 Python tests pass.** Release build not reported as passed here.
- The next function begins at runtime `0xb60ec` (MBN file `0xba0ec`). Work is still provisional older-base typed ARM32 instruction re-encoding, not a high-level behavioral equivalent, complete OEM-authenticated image or on-device proof.

## Checkpoint 35 — codec plane initialization and default controller fields

- Contiguous Venus ARM32 tail `[0xa8000,0xb62d0)` = **58,064 byte-identical bytes**, original MBN file `[0xac000,0xba2d0)`.
- New interval `[0xb60ec,0xb62d0)` = **484 bytes**: **118 typed A32 instructions (472 bytes)** plus three distinct data words (12 bytes) `[0xb6258,0xb6264)`. The first function initializes codec controller plane fields and validates frame sizes; the second short function writes default input/descriptor values and returns via the original `BX LR` at `0xb62cc`.
- Independent Clang/LLVM reassembly and complete typed-Rust emitter match pinned reference with zero differences; a Rust unit test validates the data pool. **207 Rust tests and 13 Python tests passed**. Release build is not claimed as passed for this checkpoint.
- Next missing runtime address is `0xb62d0` (MBN file `0xba2d0`). Branch remains provisional older-base typed A32 instruction encoding, not high-level behavioral equivalent or signed firmware.

## Checkpoint 36 — codec descriptor allocation and controller wiring

- Cumulative typed A32 tail `[0xa8000,0xb65e4)` = **58,852 reference-identical bytes**, file `[0xac000,0xba5e4)` in the original `venus.mbn`.
- New `[0xb62d0,0xb65e4)` = **788 bytes**: **185 ARM32 instructions (740 bytes)** and 12 named controller globals/diagnostic constant words (48 bytes) `[0xb65b4,0xb65e4)`.
- Rebuilds device descriptor allocation, checks allocation and controller state, initializes shared metadata, invokes caller callback hooks, and preserves original cleanup/return flow. The twelve-word literal pool includes the reference function address `0xb60ec` and controller-global pointers, not executable instructions.
- Direct typed Rust emission and independent Clang/LLVM reassembly match original **with zero differences**; a Rust regression validates all 12 words. **208 Rust tests**, **13 Python tests** pass. Offline release build is not claimed in this checkpoint.
- Next runtime address `0xb65e4` (`venus.mbn` file `0xba5e4`) begins a small callback thunk; no behavior/hardware/OEM-signing validation has been established. Work is provisional old-base instruction re-encoding only.

## Checkpoint 37 — controller context thunks and codec descriptor registration

- Cumulative exact Venus runtime `[0xa8000,0xb68bc)` = **59,580 bytes**, original `venus.mbn` file `[0xac000,0xba8bc)`.
- New `[0xb65e4,0xb68bc)` = **728 bytes**: **173 typed ARM32 instructions (692 bytes)** and nine literal/controller data words (36 bytes) `[0xb6898,0xb68bc)`.
- Rebuilt the small controller forwarding thunks `[0xb65e4,0xb6654)` and the descriptor-registration function `[0xb6654,0xb68bc)`, preserving its conditional branches, controller callbacks and source register-allocation semantics. Data words include controller globals and diagnostic addresses, not instructions.
- Independent Clang/LLVM assembler matches all 692 instruction and 36 data bytes (`--data-range 0xb6898:0xb68bc`), zero differences; full typed emitter verifies the entire cumulative fragment. **209 Rust and 13 Python tests pass.** Release build not claimed at this checkpoint.
- Next runtime `0xb68bc` (file `0xba8bc`) begins a new codec descriptor size/offset calculation routine. This remains a provisional old-base typed ARM32 reconstruction, not a full semantic source rebuild, signed OEM firmware, or on-device verification.

## Checkpoint 38 — codec plane layout and six-way event dispatch

- Contiguous exact Venus ARM runtime `[0xa8000,0xb6edc)` = **61,148 bytes**; reference MBN file `[0xac000,0xbaedc)`.
- New `[0xb68bc,0xb6edc)` = **1,568 bytes**, comprising **368 typed ARM32 instructions (1,472 bytes)** and 24 separately classified data words (96 bytes). Constant pools are `[0xb6ab4,0xb6ac4)`, `[0xb6c34,0xb6c4c)`, `[0xb6d28,0xb6d34)`, `[0xb6e2c,0xb6e44)` (the six-way dispatch table) and `[0xb6ec8,0xb6edc)`.
- Reconstructs controller buffer dimensions/offsets, allocation checks, callbacks and a six-way mode dispatch. The computed `LDR PC,[R0,R1]` at `0xb6e28` is a valid control transfer, so it uses the PC-capable ARM register transfer encoder rather than the guarded data-only helper.
- Direct source-built output comparison against `venus.mbn` and independent Clang/LLVM reassembly both show **zero byte differences**. All **210 Rust** and **13 Python** tests pass. Offline release build is not claimed here.
- Next uncovered runtime `0xb6edc` (reference file offset `0xbaedc`); this remains a provisional older-base instruction-level A32 implementation, not a behavioral high-level decompilation or a newly signed/flashable image.

## Checkpoint 39 — release codec context and reset active queues

- Contiguous typed Venus runtime `[0xa8000,0xb71b8)` = **61,880 byte-identical bytes**, original MBN file `[0xac000,0xbb1b8)`.
- New `[0xb6edc,0xb71b8)` = **732 bytes**, split into **174 typed ARM32 instructions (696 bytes)** and **nine explicitly classified data words (36 bytes)**. Literal pools: five-entry mode dispatch table `[0xb6f48,0xb6f5c)` and four controller diagnostic pointers `[0xb71a8,0xb71b8)`.
- Release codec session callbacks, reset per-resource state, traverse outstanding response slots and return via the original POP/PC; no fabricated return or copied machine-code words.
- Full source-generated fragment verified against the pinned Venus MBN. Independent Clang/LLVM reconstruction reassembled all 696 instruction bytes and classified 36 data bytes with zero differences; **211 Rust** and **13 Python tests** pass. The offline release build exceeded this session's execution limit and is **not claimed as passing**.
- First uncovered runtime address `0xb71b8` (MBN file `0xbb1b8`). Work remains provisional, on the older agent branch, not merged onto checkpoint 50. Instruction-byte equality does not establish high-level behavioral equivalence or OEM authentication.

## Checkpoint 40 — close active codec slot contexts and drain callbacks

- Cumulative source-encoded Venus runtime `[0xa8000,0xb76f4)` = **63,220 bytes**; pinned MBN file `[0xac000,0xbb6f4)` with zero differing bytes.
- New `[0xb71b8,0xb76f4)` = **1,340 bytes**: 327 explicitly encoded A32 instructions (1,308 bytes) and eight named literal diagnostic/controller words (32 bytes) `[0xb76d4,0xb76f4)`.
- Covers channel close/reset state transitions, draining remaining codec slots, freeing individually owned controller buffers, and optional callback notifications; original return/tail branches retained.
- Direct whole-fragment verification, independent Clang/LLVM reassembly, 212 Rust and 13 Python tests all pass. No release build claimed for this checkpoint (the earlier attempt exceeded the execution limit).
- Next uncovered ARM runtime `0xb76f4`, original MBN file `0xbb6f4`. Provisional older-base instruction re-encoding; no high-level semantic equivalence or new OEM authentication established.

## Checkpoint 41 — codec lifecycle state and buffer record maintenance

- Contiguous Venus runtime `[0xa8000,0xb7bd8)` = **64,472 reference-identical bytes**, MBN file `[0xac000,0xbbbd8)`.
- New `[0xb76f4,0xb7bd8)` = **1,252 bytes**: 305 A32 instructions (1,220 bytes) and eight named literal event/diagnostic words (32 bytes). Data pools: `[0xb7938,0xb794c)` (five), `[0xb79e4,0xb79e8)` (one), `[0xb7b0c,0xb7b10)` (one), `[0xb7b94,0xb7b98)` (one).
- Implements codec context activation, repeated frame packet publication, initial stream record setup, stop/suspend state and callbacks across five original function entrypoints. Separate unit test verifies data words explicitly.
- Full fragment and independent Clang/LLVM assembler check match the pinned reference with **zero differing bytes**. **213 Rust tests and 13 Python tests pass**. Full release build is **not** claimed here.
- Next uncovered runtime `0xb7bd8`, original MBN file `0xbbbd8`. This is a provisional older-base instruction-level A32 representation rather than high-level firmware semantics or newly OEM-authenticated code.

## Checkpoint 42 — table-based CRC32 and controller-output messages

- Total reconstructed Venus runtime `[0xa8000,0xb80d8)` = **65,752 byte-identical bytes**, private reference MBN file `[0xac000,0xbc0d8)`.
- Newly typed `[0xb7bd8,0xb80d8)` = **1,280 bytes**, comprising 307 ARM32 instructions (1,228 bytes) and thirteen separately classified literal words (52 bytes). Data pools `[0xb7ca8,0xb7cb4)`, `[0xb7d90,0xb7d98)`, `[0xb7f84,0xb7f94)`, `[0xb80c8,0xb80d8)`.
- The first routine performs MSB-first table-based CRC32 using polynomial `0x04c11db7`, and later routines publish codec descriptors, handle deferred output callbacks, and prepare controller packet fields.
- Direct byte comparison and independent Clang/LLVM round-trip verify with zero differences. **214 Rust tests and 13 Python tests passed**. No full release build is claimed.
- Next uncovered runtime `0xb80d8`, MBN file `0xbc0d8`. Provisional old-base branch, instruction-byte reconstruction only; remaining Venus firmware and OEM signing are not reconstructed.

## Checkpoint 43 — codec input-plane dimensions and four-family dispatch

- Contiguous Venus runtime `[0xa8000,0xb84fc)` = **66,812 byte-identical bytes**, original MBN file `[0xac000,0xbc4fc)`.
- New `[0xb80d8,0xb84fc)` = **1,060 bytes**: **253 typed A32 instructions (1,012 bytes)** and twelve separately identified dispatch/logging words (48 bytes), with table `[0xb82cc,0xb82dc)` and diagnostic pool `[0xb84dc,0xb84fc)`.
- Reconstructed four codec-plane buffer-size formulas, controller record preparation, input stream parameter propagation, warnings for invalid format values and guarded channel-status updates.
- Direct byte comparison, independent Clang/LLVM reassembly, **215 Rust tests and 13 Python tests** all passed with zero differences. Release build not claimed.
- First uncovered runtime `0xb84fc`, file `0xbc4fc`. Provisional older-base typed A32 encoding only, not source-level semantic equivalence or new OEM authentication.

## Checkpoint 44 — controller input records and fallback state

- Cumulative verified Venus runtime `[0xa8000,0xb87d8)` = **67,544 exact bytes**, original MBN file `[0xac000,0xbc7d8)`.
- New `[0xb84fc,0xb87d8)` = **732 bytes**: 173 typed ARM32 instructions (692 bytes) and ten separately modeled controller/diagnostic words (40 bytes). Pools `[0xb86b0,0xb86c8)` and `[0xb87a4,0xb87b4)`.
- Implements input frame-plane control, buffer subscription/allocation, codec error feedback and short status inspection. Original conditional branches and return forms preserved; no OEM instruction-array imports.
- Rust whole-region byte comparison, independent Clang/LLVM assembler check, **216 Rust tests**, **13 Python tests** all pass with zero differences. Offline release build not claimed.
- Next runtime `0xb87d8`, firmware offset `0xbc7d8`; provisional older-base A32 instruction encoding, not a new signed or flashable firmware image.

## Checkpoint 45 — codec frame-descriptor dispatch, first portion

- Cumulative independently generated Venus runtime `[0xa8000,0xb8c00)` = **68,608 byte-identical bytes**, pinned `venus.mbn` file `[0xac000,0xbcc00)`.
- New typed span `[0xb87d8,0xb8c00)` = **1,064 bytes**: 258 ARM32 instructions (1,032 bytes) and two four-entry data/target tables (eight words, 32 bytes). The mode tables reside at `[0xb88b8,0xb88c8)` and `[0xb8ab0,0xb8ac0)` and have dedicated Rust unit checks.
- The function begins at `0xb87d8`; the end of this checkpoint at `0xb8c00` is an **incomplete function boundary**, not an invented return. The rest of the large descriptor state machine will be reconstructed in subsequent checkpoints.
- Full Rust fragment compares byte-for-byte with the pinned original reference. Independent ARM-mode Clang/LLVM mnemonic reassembly shows **zero mismatches**, explicitly treating the jump tables as data, not executable opcodes. **217 Rust tests and 13 Python tests pass.** Offline release build not claimed.
- This is a provisional older-base ARM32 instruction-level reconstruction; high-level behavior and OEM authentication remain unproven. Next uncovered runtime `0xb8c00`, MBN file `0xbcc00`.

## Checkpoint 46 — frame command writer and third four-way jump table

- Total Venus runtime `[0xa8000,0xb9208)` = **70,152 byte-identical source-encoded bytes**, file `[0xac000,0xbd208)`.
- Newly encoded `[0xb8c00,0xb9208)` = **1,544 bytes**, consisting of 382 ARM32 instructions (1,528 bytes) and a four-entry handler address table `[0xb91f8,0xb9208)` (16 bytes).
- Continues the large command-buffer routine, including conditionally packed controller descriptors, frame-size accounting, output callback dispatch and layout classification. Original PC-indexed branch into the handler table is encoded explicitly.
- Entire Rust fragment compares equal to the pinned reference, with zero differing bytes. Independent ARM-mode Clang/LLVM reassembles the new 1,528 instruction bytes, with separate 16-byte data pool; zero differences. **218 Rust tests and 13 Python tests pass.** Offline release build not claimed.
- Next missing runtime `0xb9208`, MBN file `0xbd208`, still inside this large function; this is provisional old-base instruction-level reconstruction, not semantic completeness, OEM-signature generation, or hardware execution proof.

## Checkpoint 47 — complete frame descriptor dispatch and error tails

- Entire generated Venus runtime `[0xa8000,0xb9904)` = **71,940 consecutive byte-identical bytes**, file `[0xac000,0xbd904)`.
- New `[0xb9208,0xb9904)` = **1,788 bytes**: **428 typed ARM32 instructions** (1,712 bytes) and 19 data words (76 bytes) across four explicitly classified pools: singleton `[0xb9800,0xb9804)`, diagnostics `[0xb9830,0xb9838)`, MMIO mode constants `[0xb9860,0xb986c)`, final diagnostics `[0xb98d0,0xb9904)`.
- Finishes the large frame-descriptor state machine that begins at `0xb87d8`, including its original `pop {...,pc}` return at `0xb9768`, optional recovery and error tails, resource metadata and input format control. The next function begins at `0xb9904` with a PUSH frame prologue.
- Direct whole-fragment Rust byte verification and independent Clang/LLVM mnemonic reconstruction report **zero differences**. **219 Rust tests and 13 Python tests passed.** No release build claimed.
- Still an older-base, provisional ARM32 instruction-level re-encoding, not a signed/flashable OEM firmware image or evidence of high-level source semantics equivalence.

## Checkpoint 48 — frame-controller status and optional input sources

- Entire Venus tail `[0xa8000,0xb9c64)` = **72,804 consecutive byte-identical bytes**, MBN file `[0xac000,0xbdc64)`.
- New `[0xb9904,0xb9c64)` = **864 bytes**, comprising 209 typed ARM32 instructions (836 bytes) and seven literal diagnostic words (28 bytes), two non-instruction data pools `[0xb9b6c,0xb9b78)` and `[0xb9c54,0xb9c64)`.
- Rebuilds the controller-status and optional-source cleanup functions, preserving their original `pop {...,pc}` returns, conditional `BLNE` subroutine call, error reporting, and buffer-slot pointer resets. Both functions are complete at this boundary.
- Entire generated Rust fragment matches pinned original Venus reference, **zero byte differences**. An independent Clang/LLVM round-trip also matches all 836 instruction bytes and 28 data bytes. The verifier was corrected to distinguish conditional `BLNE` from a nonrelocated absolute target. **220 Rust tests and 13 Python tests passed**; no release build claimed.
- Next uncovered runtime `0xb9c64` (file `0xbdc64`) begins a new codec-register setup routine. Reconstruction is provisional older-base instruction-level encoding, not OEM signing or high-level semantic equivalence.

## Checkpoint 49 — frame-register initialization and optional source cloning

- Complete generated Venus runtime `[0xa8000,0xba210)` = **74,256 consecutive byte-identical bytes**, original MBN offsets `[0xac000,0xbe210)`.
- New `[0xb9c64,0xba210)` = **1,452 bytes**, comprising 353 A32 instructions (1,412 bytes) and ten distinguished non-code words (40 bytes): `[0xb9eb4,0xb9eb8)` (one pointer), `[0xb9f78,0xb9f80)` (two messages), `[0xba1f4,0xba210)` (seven callback/error addresses).
- Three complete original functions: frame-register configuration and sizing, attachment of optional input sources with guarded callbacks, and cloning controller buffer/session records with original return and cleanup branches.
- Full typed Rust fragment has **zero differing bytes** against pinned `venus.mbn`. Independent Clang/LLVM reassembly matches all 1,412 ARM32 instruction bytes and 40 explicitly classified data bytes. **221 Rust tests and 13 Python tests pass.** Release build not claimed.
- First uncovered runtime `0xba210` (file `0xbe210`) begins another codec resource helper. This is provisional older-base instruction-level encoding, not signed or hardware-tested firmware or high-level semantic decompilation.

## Checkpoint 50 — codec DMA cleanup and controller reset

- Cumulative reconstructed Venus runtime `[0xa8000,0xba40c)` = **74,764 byte-identical bytes**, pinned MBN file `[0xac000,0xbe40c)`.
- New `[0xba210,0xba40c)` = **508 bytes**, comprising **120 typed ARM32 instructions (480 bytes)** and **7 independently classified data words (28 bytes)** in the literal pools `[0xba294,0xba2a0)` and `[0xba3fc,0xba40c)`.
- Three original functions: select/finish codec stream end state, settle pending DMA resource statuses, and reset controller session/command slots. Original ARM return sequences and controller callbacks are retained; the pools contain device-global and diagnostic pointers, not executable code.
- Full Rust-emitted region and independent mnemonic-only Clang/LLVM reassembly show **zero byte mismatches** with the pinned reference. **222 Rust tests, 13 Python tests** pass. The attempted offline release build **exceeded the execution limit and is not claimed as passed**.
- Next uncovered ARM runtime `0xba40c`, reference file offset `0xbe40c`. The branch remains provisional on older Agent 03 ancestry, **not** checkpoint-50-based integrated Git history; this is ISA-level re-encoding, not a source-level behavioral rebuild or an OEM-authenticated image.

## Checkpoint 51 — codec buffer window layout, reconciliation and DMA state

- Contiguous independently encoded ARM runtime `[0xa8000,0xbb078)` = **77,944 bytes**; matching `venus.mbn` file `[0xac000,0xbf078)` (exclusive endpoints).
- Added full controller resource layout and DMA-window state routine `[0xba40c,0xbb078)` = **3,180 bytes**. This is **767 typed ARM32 instructions (3,068 bytes)** followed by **28 separately classified non-executable controller/literal words (112 bytes)** at `[0xbb008,0xbb078)`.
- It synchronizes controller resource pointers, buffer plane sizes, queued DMA metadata and fallback status, then preserves the original return at `0xbaf6c` and error handler branch tail through `0xbb004`.
- Direct typed-Rust fragment verification against pinned Venus SHA-256 `9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085` passes with **0 differing bytes**. Independent Clang/LLVM mnemonic reassembly passes with 0 differences when `[0xbb008,0xbb078)` is explicitly classified as data.
- **223 Rust and 13 Python tests passed**. The offline release build has not finished within this environment's execution limit, so it is not claimed as passed for this checkpoint.
- First uncovered runtime address: `0xbb078` (original MBN offset `0xbf078`). The branch is still provisional on the older Agent 03 base and not integrated atop the unavailable checkpoint-50 history. Byte identity at the instruction level does not prove high-level semantics or independent OEM signing.

## Checkpoint 52 — codec frame publication and controller events

- Contiguous Venus runtime `[0xa8000,0xbb278)` = **78,456 source-encoded byte-identical bytes**, private MBN `[0xac000,0xbf278)`.
- Newly reconstructed `[0xbb078,0xbb278)` = **512 bytes**, comprising **119 typed ARM32 instructions (476 bytes)** plus **9 non-executable literal/data words (36 bytes)** at `[0xbb254,0xbb278)`.
- The function publishes active codec records to a controller, updates outstanding frame FIFO counts, queues frame/metadata messages, checks optional callback routes and restores the original stack state through POP/PC.
- The complete typed-Rust emitter and independent Clang/LLVM reassembly match the pinned Venus original with **zero different bytes**. All **224 Rust** and **13 Python tests** pass; a completed offline release build is not claimed.
- Next uncovered runtime address is `0xbb278`, original MBN file offset `0xbf278`. This remains provisional old-base typed re-encoding, not source-level behavioral recompilation, OEM signing or a hardware-tested image.

## Checkpoint 53 — auxiliary codec record and DMA descriptor synthesis

- Cumulative typed-Rust Venus runtime `[0xa8000,0xbb570)` = **79,216 consecutive reference-identical bytes**, MBN file offsets `[0xac000,0xbf570)`.
- Newly reconstructed `[0xbb278,0xbb570)` = **760 bytes**, comprising **189 typed ARM32 instructions (756 bytes)** and a one-word packed codec-record format constant `0x01010200` at `[0xbb56c,0xbb570)`; this word is explicitly data, not decoded ARM instructions.
- Builds codec-frame auxiliary records from controller buffer offsets, updates DMA command lengths, transfers stream payloads, compacts record slots, and preserves the actual POP/PC return at `0xbb568`.
- Full typed-Rust emitter exactly matches the pinned `venus.mbn` and independent Clang/LLVM mnemonic reassembly returns **zero differences** for 756 code bytes and 4 classified data bytes.
- **225 Rust tests, 13 Python tests passed**. No successful offline release build is claimed for checkpoint 53.
- Next uncovered runtime address `0xbb570`, file offset `0xbf570`; still provisional older Agent 03 ancestry, not the unavailable checkpoint-50-based integration.

## Checkpoint 54 — retire codec session and update external input references

- Cumulative source-encoded Venus runtime `[0xa8000,0xbb6d8)` = **79,576 byte-identical bytes**, pinned MBN file `[0xac000,0xbf6d8)`.
- New `[0xbb570,0xbb6d8)` = **360 bytes**: **85 independently re-encoded ARM32 instructions (340 bytes)** and **five structured literal words (20 bytes)**. The four-word status/diagnostic pool is `[0xbb644,0xbb654)` and the single warning address is `[0xbb6d4,0xbb6d8)`.
- Two complete controller helpers and a diagnostic tail: retire a session/record, invoke error logging and cleanup callbacks, and hand off updated external codec references. Preserve `BXEQ LR` and the original return/branch targets.
- Full typed-Rust generated tail and independent Clang/LLVM reassembly match all byte positions against the pinned reference (0 mismatches). **226 Rust and 13 Python tests pass**. A successful release build is not claimed for this checkpoint.
- Next uncovered runtime `0xbb6d8`, original MBN file offset `0xbf6d8`. This branch still descends from the older Agent 03 base and must be revalidated on a genuine checkpoint-50 integration.

## Checkpoint 55 — program encoded codec controller registers

- Cumulative typed-Rust runtime `[0xa8000,0xbbabc)` = **80,572 byte-identical Venus bytes**, original MBN `[0xac000,0xbfabc)`.
- Newly reconstructed `[0xbb6d8,0xbbabc)` = **996 bytes**: **245 typed ARM32 instructions (980 bytes)** and **four separately classified diagnostic data words (16 bytes)** at `[0xbbaac,0xbbabc)`.
- Rebuilds encoded codec frame/output pipeline registers, validates active frame mode, sets mask bits and issues controller-write packets; original saved-register return occurs at `0xbbaa8`.
- Full Rust source-generated fragment comparison reports **zero byte differences**. Independent Clang/LLVM reassembly matches all 980 instruction bytes plus the four identified data words. **227 Rust tests and 13 Python tests passed**. No completed release-build result is claimed.
- First uncovered runtime `0xbbabc` (MBN file offset `0xbfabc`). Still provisional older Agent 03 branch, not checkpoint-50-based integrated history. Byte identity is not high-level semantic decompilation or OEM authentication.

## Checkpoint 56 — codec status register readback and stream command programming

- Complete source-emitted Venus runtime `[0xa8000,0xbbcd4)` = **81,108 consecutive reference-identical bytes**, original MBN file `[0xac000,0xbfcd4)`.
- Added `[0xbbabc,0xbbcd4)` = **536 bytes**, comprising **134 typed ARM32 instructions** and **zero separately classified data words**. Three source-level groups: conditional controller readback, stream status register programming and auxiliary register configuration. The first group falls through to the next routine rather than inventing a return.
- Full fragment byte equality against the pinned `venus.mbn` is exact, **zero differing bytes**. Independent Clang/LLVM ARM32 mnemonic reassembly matches all 536 bytes exactly.
- **228 Rust and 13 Python tests passed**. A successful release build is not claimed at this checkpoint.
- First missing runtime `0xbbcd4`, file offset `0xbfcd4`. The next ARM32 controller function starts here. This remains provisional older-Agent-03 ancestry and not a source-level semantic rebuild, full firmware, or new OEM-authenticated image.

## Checkpoint 57 — codec MMIO controller and frame-register programming

- Continuous independently encoded Venus ARM32 region `[0xa8000,0xbc598)` = **83,352 reference-identical bytes**, original MBN `[0xac000,0xc0598)`.
- Newly reconstructed `[0xbbcd4,0xbc598)` = **2,244 typed ARM32 instruction bytes** (561 instructions), without embedded data pools. The function sets up a controller MMIO and frame-resource state, copies optional tuning blocks, writes output-plane command fields and queues frame metadata.
- The PC-relative reference constants for this function are **outside** the emitted end boundary; no OEM data bytes were inserted to fill the future pool. This is a **partial function**, with the next missing instruction at runtime `0xbc598` (MBN offset `0xc0598`).
- Exact full-fragment reference compare passed (0 mismatching bytes). Independent Clang/LLVM mnemonic reassembly for the entire new interval passed (0 differences). **229 Rust and 13 Python tests passed**. No completed release-build result is claimed.
- This Git branch remains a provisional descendant of older Agent 03 history, not of the missing checkpoint-50-based integrated Git branch. Byte-identical ISA encoding is not source-level behavioral decompilation or OEM signing.

## Checkpoint 58 — complete controller command queue and adjacent MMIO helpers

- Continuous typed ARM32 region `[0xa8000,0xbc7dc)` = **83,932 pinned-reference-equal bytes**, original Venus file `[0xac000,0xc07dc)`.
- Newly reconstructed `[0xbc598,0xbc7dc)` = **580 bytes**, with 138 ARM32 instructions (552 bytes) and 7 separately classified literal/data words (28 bytes) at `[0xbc78c,0xbc7a8)`. The full controller command routine ends via its original POP/PC or conditional tail at `0xbc788`. Three adjacent short helpers implement controller MMIO register writes and original BX LR returns.
- Whole Rust-generated region compares against pinned original Venus with **zero mismatches**; independent Clang/LLVM ARM32 instruction reassembly also matches all code bytes and classified data. **230 Rust and 13 Python tests pass**; release build not claimed.
- Next uncovered VMA is `0xbc7dc`, file offset `0xc07dc`; a separate codec controller function begins there. Still provisional older Agent-03 branch, not checkpoint-50-rooted integrated Git history, nor a high-level decompiled/OEM-signed firmware image.

## Checkpoint 59 — complete hardware codec profile programming

- Entire emitted Venus runtime `[0xa8000,0xbd000)` = **86,016 consecutive byte-identical bytes**, original MBN file `[0xac000,0xc1000)`.
- Added the full codec hardware-profile controller routine `[0xbc7dc,0xbd000)` = **2,084 bytes**, comprising **511 independently re-encoded ARM32 instructions** (2,044 bytes) and **ten separately modeled non-instruction literal words** (40 bytes) at `[0xbcfd8,0xbd000)`. Contains codec status/MMIO writes, structured frame-row registers, channel geometry, fallback reports and the original POP/PC return.
- Entire generated output exactly matches pinned `venus.mbn` (zero different bytes). Independent Clang/LLVM mnemonic reconstruction matches all code bytes, with the literal pool classified as data. **231 Rust + 13 Python tests passed**; no release build claimed.
- Next uncovered runtime `0xbd000` (file `0xc1000`), next function prologue begins here. Work stays on the provisional old-Agent-03 Git branch, not a checkpoint-50-based integration. Instruction-byte equality does not establish recovered high-level source semantics or OEM authentication.

## Checkpoint 60 — controller record initialization and packed-word normalizer

- Continuous typed Venus runtime `[0xa8000,0xbd648)` = **87,624 byte-identical bytes**, original MBN file `[0xac000,0xc1648)`.
- Added `[0xbd000,0xbd648)` = **1,608 bytes**, comprising **394 ARM32 instructions** (1,576 bytes) and **8 separate data words** (32 bytes) at `[0xbd540,0xbd560)`.
- Rebuilds the controller's stream-record lifecycle initialization, source buffer accounting, conditional cleanup, 16-word packed register-field normalizer and short command dispatch helper. The original POP/PC returns are preserved; the diagnostic and mode table are modeled as data.
- Entire source-emitted fragment matches the pinned `venus.mbn` (0 mismatches); independent Clang/LLVM round-trip matches all 1,576 instruction bytes and 32 explicit data bytes. **232 Rust tests and 13 Python tests passed**. No successful release build claimed at this checkpoint.
- Next missing VMA `0xbd648`, MBN file offset `0xc1648`, begins another resource update function. Provisional old-Agent-03 Git base, not checkpoint-50-integrated; instruction equality is not high-level decompilation, hardware proof or OEM signature generation.

## Checkpoint 61 — codec buffer reconciliation and computed dispatch

- Contiguous Venus runtime `[0xa8000,0xbdc00)` = **89,088 source-encoded reference-identical bytes**, original MBN `[0xac000,0xc1c00)`.
- Newly reconstructed `[0xbd648,0xbdc00)` = **1,464 bytes** consisting of **359 typed ARM32 instructions (1,436 bytes)** and a **seven-address branch table (28 data bytes)** at `[0xbd850,0xbd86c)`.
- Initializes codec buffer record state, reconciles stream planes, computes active command status, and enters a longer codec controller packet sequence. The function is **not complete** at this boundary; its branches reference addresses above `0xbdc00`.
- During direct validation, one word at `0xbd854` differed because a jump-table handler address was initially treated as a `MULEQ` instruction. The whole seven-word table was reclassified as named data, and the full byte comparison then passed exactly. Independent Clang/LLVM reassembly matches all 1,436 instruction bytes plus the seven explicit handler targets. **233 Rust and 13 Python tests passed**; no release build claimed.
- First missing runtime address `0xbdc00`, file offset `0xc1c00`. Work remains provisional on older Agent-03 Git ancestry, without OEM signing or evidence of high-level behavioral decompilation.

## Checkpoint 62 — eight-way codec controller frame-mode dispatcher

- Typed Venus runtime `[0xa8000,0xbe000)` = **90,112 exact bytes**, original file `[0xac000,0xc2000)`.
- New interval `[0xbdc00,0xbe000)` = **1,024 bytes**, comprising **248 typed ARM32 instructions (992 bytes)** and **eight explicit branch-table pointers (32 data bytes)** at `[0xbdc84,0xbdca4)`. Retains the original computed `LDR PC` dispatch, resource pointers, and MMIO programming paths. This is a partial function, still continuing at `0xbe000`.
- Full source-emitted fragment matches original reference with zero differing bytes, and independent Clang/LLVM reassembly matches the 992 instruction bytes plus eight data words. **234 Rust and 13 Python tests passed**; no release build claimed.
- Next missing VMA `0xbe000`, MBN offset `0xc2000`. Older Agent-03-derived provisional history; not high-level semantic decompilation, new OEM authentication, or an integrated checkpoint-50 build.

## Checkpoint 63 — four-way codec output-plane controller policy

- Continuous Venus runtime `[0xa8000,0xbe7dc)` = **92,124 byte-identical source-encoded bytes**, original firmware `[0xac000,0xc27dc)`.
- New interval `[0xbe000,0xbe7dc)` = **2,012 bytes**, **499 typed ARM32 instructions (1,996 bytes)** and **four handler-address words (16 bytes)** at `[0xbe44c,0xbe45c)`.
- Resolves and formats codec output-plane metadata and queue mode, preserving the original `LDR PC` control-flow dispatch and status fallback instructions. This remains part of the long function starting `0xbd648`; the return is not in the current range.
- Full emitted region compares equal to pinned original with **0 differing bytes**, and independent Clang/LLVM reconstruction reproduces all 1,996 code bytes and 16 classified data bytes. **235 Rust tests and 13 Python tests passed.** Release build not claimed.
- Next uncovered runtime VMA `0xbe7dc`, MBN file offset `0xc27dc`. Older-base provisional Git ancestry, not checkpoint-50 integrated or OEM-signed high-level source reconstruction.

## Checkpoint 64 — frame queue and pending output

- Continuous typed ARM32 region `[0xa8000,0xbeb70)` = **93,040 byte-identical bytes**, original MBN `[0xac000,0xc2b70)`.
- New continuation `[0xbe7dc,0xbeb70)` = **916 bytes**, **227 typed ARM32 instructions (908 bytes)** and two *separately classified non-instruction data words* (8 bytes) at `[0xbe9e8,0xbe9ec)` (`0x000fa36c`) and `[0xbe9f8,0xbe9fc)` (`0x000fa5a8`). These are diagnostic pointer constants; the disassembler misleadingly prints them as ANDEQ.
- Reconciles queue records and stream dimensions, bounds active controller state, expands codec control flags and copies per-frame status. This is a **partial function**; the next code begins `0xbeb70` (original MBN offset `0xc2b70`). No bytes outside agent's `[0xa8000,0x1029f4)` reservation were added.
- Independently reassembled using:

  ```sh
  python3 tools/agent20_06_a32_roundtrip.py /private/venus.mbn 0xbe7dc 0xbeb70 \
    --data-range 0xbe9e8:0xbe9ec --data-range 0xbe9f8:0xbe9fc
  ```

  Clang/LLVM confirms all 908 instruction bytes and 8 literal bytes, **zero differences**. Entire Rust fragment also matches the pinned original SHA-256 `9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085` slice.
- `cargo test --workspace --offline --locked`: **236 Rust tests passed**. `python3 -m unittest discover -s tests -p 'test_*.py'`: 13 tests passed. This is still a provisional old-Agent-03-base bundle and does not claim checkpoint-50 integration, high-level decompilation or OEM-authenticated output.

## Checkpoint 65 — queued stream records, byte shuffle, and plane offsets

- Contiguous typed Venus ARM32 runtime `[0xa8000,0xbeea0)` = **93,856 byte-identical bytes**, original reference file `[0xac000,0xc2ea0)`; compared directly to pinned Venus SHA-256 `9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085`.
- New continuation `[0xbeb70,0xbeea0)` = **816 bytes** consisting of **180 A32 instructions (720 bytes)** and **24 explicitly classified non-executable words (96 bytes)**. Distinct pools at `[0xbeb70,0xbeb78)`, `[0xbec30,0xbec78)`, `[0xbed60,0xbed64)` and `[0xbee94,0xbeea0)`.
- Includes the outgoing pending-record status update, a 64-entry indexed byte shuffle, and recalculation of input/optional output plane offsets. Two routines use original stack epilogues and do not invent new returns.
- Full Rust emission equals original reference (zero byte mismatches). Independent Clang/LLVM ARM instruction mnemonic reconstruction confirms **720 code bytes + 96 classified data bytes**, zero differences using:

  ```sh
  python3 tools/agent20_06_a32_roundtrip.py /private/venus.mbn 0xbeb70 0xbeea0 \
    --data-range 0xbeb70:0xbeb78 --data-range 0xbec30:0xbec78 \
    --data-range 0xbed60:0xbed64 --data-range 0xbee94:0xbeea0
  ```

- **237 Rust tests and 13 Python tests passed**; fresh full release build exceeded the execution limit and is **not claimed to have passed**.
- Next uncovered address is `0xbeea0`, original MBN file offset `0xc2ea0`. This is still a provisional old-Agent-03-derived branch (not checkpoint-50-integrated), not an OEM-authenticated high-level decompilation or flashable firmware image.

## Checkpoint 66 — codec output-mode descriptor setup

- Continuous Venus runtime `[0xa8000,0xbf400)` = **95,232 byte-identical bytes**, original MBN file `[0xac000,0xc3400)`.
- Added `[0xbeea0,0xbf400)` = **1,376 bytes**, with **340 typed ARM32 instructions** and **four non-executable dispatch-table words** `[0xbefdc,0xbefec)` (1,360 instruction bytes + 16 table bytes). Code sets up codec plane sizes, controller-mode dispatch and output descriptor offsets. The enclosing function continues past this boundary.
- Direct whole-fragment comparison against the pinned original SHA-256 `9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085` is exact (zero mismatches). Independent Clang/LLVM reassembly of the new code region passed with the branch table classified as data:

  ```sh
  python3 tools/agent20_06_a32_roundtrip.py /private/venus.mbn 0xbeea0 0xbf400 \
    --data-range 0xbefdc:0xbefec
  ```

- **237 Rust and 13 Python tests pass**. A successful new release build is not claimed. The next uncovered ARM runtime address is `0xbf400` (file `0xc3400`). All work remains provisional on old-Agent-03 ancestry and does not independently reproduce OEM authentication or high-level algorithm semantics.
