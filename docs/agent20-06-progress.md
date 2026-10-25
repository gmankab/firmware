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
