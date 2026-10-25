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
