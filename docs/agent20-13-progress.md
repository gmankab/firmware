# Agent 20-13 — WLAN early Hexagon continuation

## Provenance and exclusivity

- Assigned repository integration base: checkpoint 50 SHA `b7ed4b587633b9b475a40f5a3ace5cc90db620ff`, **not present** in the supplied files. The auditable agent-7 continuation is rooted at agent-7 verified HEAD `d6104bb257ad60aa52f36eb91f338ef3e08586c5`; branch `agents20/13-wlan-early-provisional`. Publish a portable patch and verified branch, not a false checkpoint-50-rooted integration.
- Owned WLAN executable ELF file offsets `[0x42708,0x100000)`; Agent 7 owns `[0x40000,0x42708)` and Agent 14 owns `[0x100000,0x353d38)`. Virtual address = file offset minus `0x40000` plus `0xb0000000` for this executable `PT_LOAD` (paddr equal to vaddr). Do not touch agent 7 source modules.
- Private reference is `wlanmdsp.mbn` SHA-256 `87c2c7bb1fe97eaf007a947832feba0898644b4d6626ac8ef49d8b94dd82604b`; extracted in the correct multipart order and **never committed**.
- Hexagon ELF e_machine 164 and e_flags `0x66`; input instructions checked with LLVM22 Hexagon V66. LLVM6 V65 is used for the previously reconstructed Agent7 prefix.

## Checkpoint 01 — register-record validation helper

- File `[0x42708,0x427a0)`; VMA `[0xb0002708,0xb00027a0)`; **152 independently authored Hexagon instruction bytes** (38 encoded words including duplex/immext and hardware-loop packets). No padding or external bytes within the new region.
- Source: `sources/xiaomi-nabu/wlanmdsp/agent13-runtime-record-check.S`. Implements record-table search using 7-iteration `loop0`, validates a caller-provided record/status value, emits diagnostic messages and returns via `dealloc_return` after restoring saved register pair.
- Correct source packet ordering matters. Extended literal packets at `0x2728` and `0x2788` must put the `r0=##...` instruction before `r2=r16` in assembly text to reconstruct original duplex placement. Correct absolute Hexagon `call` PC-relative displacements were verified against reference; wrong displacement assembles but differs in opcode bytes.
- First assembler comparison initially found 22 differing bytes. Correcting two packet orders, one extended address and two call displacements yields **0 byte differences**. Rebuilding with supplied LLVM22 V66 matches all 152 reference bytes exactly; SHA-256 `5e86e28fded96cd5fe19b39410884ce9727e58dbe26446b21a1bc67723a5c23e`.
- Full executable-layout prefix, *only when rebuilt from Agent7 independent sources*, is `[0x40000,0x427a0)` = **10,144 matching bytes**, SHA-256 `d486b130c9c4da569e37e0e2d0dde127b25e298795a69b5b41c35e110d608096`. Components: **6,368 independently authored instruction bytes** (6,216 earlier +152 new), **3,776 identified linker-fill/alignment bytes** (not source code). There are no unknown bytes in this prefix.
- Deterministic `tools/build-wlanmdsp-agent13.py` assembles new code, optionally independently rebuilds and joins Agent7 code (`--join-agent07`), and validates exact pinned firmware SHA, ELF32 entry, executable `PT_LOAD` and first differing byte, with optional JSON provenance manifest. It refuses implicit holes, overlapping fragments, missing assembler, and embedded OEM `.incbin`, `.word`, `.long`, `.quad`, `.byte`, `.fill` directives.
- `cargo test --workspace --offline --locked -q`: **160 Rust tests passed**. `python3 -m unittest discover -s tests -p 'test_*.py'`: **22 cases, 20 passed, 2 optional skipped**. `cargo build --workspace --release --offline --locked`: passed. Direct strict reference checks (standalone and combined) passed. `git diff --check`: passed.
- **Next executable function begins at file `0x427a0` / VMA `0xb00027a0`.** Remaining owned interval `[0x427a0,0x100000)` is not yet source-rebuilt. Signed firmware is not produced; exact instruction encodings do not prove complete high-level source recovery or runtime behavior.

### Reproduction (private reference must be outside Git)

```sh
LD_LIBRARY_PATH=/private/nabu-next-toolchain/ubuntu-runtime/flat \
python3 tools/build-wlanmdsp-agent13.py /tmp/agent13-wlan.bin \
  --llvm22 /private/llvm22/bin/llvm-mc \
  --reference /private/wlanmdsp.mbn --manifest /tmp/agent13-wlan.json

LD_LIBRARY_PATH=/private/nabu-next-toolchain/ubuntu-runtime/flat \
python3 tools/build-wlanmdsp-agent13.py /tmp/wlan-prefix-through-27a0.bin \
  --join-agent07 --llvm6 /private/llvm6/bin/llvm-mc \
  --llvm22 /private/llvm22/bin/llvm-mc \
  --reference /private/wlanmdsp.mbn
```

## Checkpoint 02 — classify WLAN record format and runtime diagnostic status

- Appends `[file 0x427a0,0x42960)` = **448 source-authored Hexagon V66 bytes** to the previous 152-byte record lookup; Agent13 now owns **600 contiguous instruction bytes** `[0x42708,0x42960)` / VMA `[0xb0002708,0xb0002960)`.
- Source file `agent13-runtime-record-classify.S` models the whole classifier: QuRT runtime state reads, format tag handling (including bitfield extract and static mode tables), signed/unsigned status reporting, and shared fallback/diagnostic code. It has conditional new-value packet branches and an extended far tail jump to the platform handler.
- All **448 original bytes** match independently emitted LLVM22 V66 code, preserving 112 encoded words with packet-accurate order. The far jump uses a packet-base displacement explicitly documented in source. Incorrect packet order and an incorrect immediate originally produced mismatches; both were corrected before this checkpoint.
- Agent13 generated code now has SHA-256 `d6fce6e6ce1c796c7ff47499a0869ae25900bb6a7c25ad28af19f52b4dd1e82d`. Independently joining Agent7's verified code yields `[0x40000,0x42960)` = **10,592 matching layout bytes**, SHA-256 `e15f567f6364cea88c166665903f03319b93d14b358357614b9a582071700623`. Distinguish **6,816 executable instruction bytes** (6,216 prior +600 new) from **3,776 generated linker-fill bytes**.
- Verified against the exact SHA-256-pinned `wlanmdsp.mbn`; zero differences over both the new fragment and entire joined prefix. Repository Rust suite: 160 tests passed. Python suite: 22 discovered, 20 passed, 2 skipped. Offline release build: passed.
- **Next uncovered function begins at file `0x42960`, runtime VMA `0xb0002960`**. Owned reservation still ends at file `0x100000`, so no overlap with Agent 14.

## Checkpoint 03 — record-list aggregate and wrappers

- Added **324 independently written Hexagon instruction bytes** in three adjacent routines after Checkpoint 02:
  - `[file 0x42960,0x42a14)` / VMA `[0xb0002960,0xb0002a14)`: **180-byte** record-list traversal and status aggregation; bounded by the record count at object offset `0x140`, branches through status 5/6/255 classes.
  - `[file 0x42a14,0x42a54)`: **64-byte** diagnostic wrapper with QuRT frame restoration and tail branch to `0xb0002708`.
  - `[file 0x42a54,0x42aa4)`: **80-byte** auxiliary logging wrapper computing a record pointer using a table index, then tail-branched back to `0xb0002a14`.
- Zero byte differences for **all 324 new bytes** under independent LLVM22 Hexagon V66 compilation, with correct packet ordering for two `combine(#0,#0)` compound initialization bundles. These are complete functions, not padded gaps.
- Total Agent13 new executable coverage: `[0x42708,0x42aa4)` **924 bytes**, SHA-256 `bce70806209bef5e137e41938ae45c564cd4dbc30abd33e67e83072993212a42`.
- The full prefix rebuilt from Agent7 and Agent13 source is `[0x40000,0x42aa4)` = **10,916 layout bytes**, SHA-256 `69e77fe4dc061d8f69b49e0ff6e1ff4dacc0817a2a60730be62c623536ab2b43`, all reference-identical. Of these, **7,140 independently re-encoded instructions** and **3,776 prior linker fill**.
- `cargo test --workspace --offline --locked -q`: **160 Rust tests passed**. Python test discovery: **22 tests, 20 passed, 2 skipped**. Offline release build: passed. Direct reference SHA, ELF segment and full-region comparison: passed.
- Next function begins at file `0x42aa4`, VMA `0xb0002aa4`. No bytes above this boundary are claimed.

## Checkpoint 04 — frame dispatch, array walk, metadata and session guards

- Added **704 independently re-encoded executable bytes** `[file 0x42aa4,0x42d64)`, corresponding to VMA `[0xb0002aa4,0xb0002d64)`, across six complete Hexagon V66 routines:
  - 296-byte record-type dispatcher (`0x42aa4–0x42bcc`), including a bounded indirect function-table lookup from pre-existing controller state and logging for type 18/19; no table content copied from OEM.
  - 56-byte bounded array walker (`0x42bcc–0x42c04`), conditional diagnostic execution on each record.
  - 76-byte metadata-report wrapper and platform-jump thunk (`0x42c04–0x42c50`).
  - 60-byte auxiliary metadata report (`0x42c50–0x42c8c`).
  - 164-byte two-table runtime-session seed (`0x42c8c–0x42d30`), including conditional index wiring.
  - 52-byte null-record guard (`0x42d30–0x42d64`).
- Each code interval matches exact original bytes, checked independently with LLVM22 `hexagonv66`, original firmware checksum/profile comparison and segment mapping. Source uses named packet semantics, never opaque instruction arrays.
- Total Agent13 independently assembled code `[0x42708,0x42d64)` = **1,628 instruction bytes** SHA-256 `9fffffcb5f33321138dd7c98c1c2a7aa2967f71f650050c4ba766e0b2e80292e`.
- Source-built joined prefix `[0x40000,0x42d64)` = **11,620 matching layout bytes**, SHA-256 `03111a4e49b3a0583b7564e020d1ac064ec6ee2f718cd095ef693df1a03d2920`; composed of **7,844 instruction bytes** (6,216 Agent7 + 1,628 Agent13) plus **3,776 linker fill**.
- 160 offline Rust tests passed; 22 Python tests discovered (20 passed, 2 skipped), offline release build passed. Direct reference hash+ELF+byte comparison passed. Next function begins at file `0x42d64`, VMA `0xb0002d64`.

## Checkpoint 05 — record exchange, two-slot update, and byte-table seeding

- Three consecutive independently written Hexagon v66 routines add **436 source-generated instruction bytes**:
  - `[file 0x42d64,0x42e20)` = 188-byte record-field exchange, using packet-local conditional operations, `:mem_noshuf` ordering and an explicit far handler jump.
  - `[file 0x42e20,0x42ed4)` = 180-byte bounded two-slot hardware-loop updater with packet-local predicate and log handling.
  - `[file 0x42ed4,0x42f18)` = 68-byte byte-table seeder, with a two-member hardware loop and `:mem_noshuf` on the combined byte-load/store packet.
- Agent 13 now independently reassembles **2,064 executable bytes** `[file 0x42708,0x42f18)` (VMA `[0xb0002708,0xb0002f18)`), SHA-256 `caf44fb8f8258bf8a44ad1a4d548dfec8adc8f3325189daf5231394350a6efa5`.
- Joined with independently reassembled Agent 7 code, the source-built executable **layout** `[file 0x40000,0x42f18)` contains **12,056 byte-identical bytes**, SHA-256 `e3c6f2320eff8de7a9247f7f9552b0190066e2ebe7fa4ee7f63d3ed79532d230`. Ledger: **8,280 independently authored instruction bytes** (6,216 Agent 7 + 2,064 Agent 13), **3,776 earlier linker fill** (not source code).
- Strict firmware reference SHA/ELF/byte comparison reports **zero mismatches** for both the new independent fragment and complete joined prefix. No source file includes opaque `.word`, `.incbin`, `.byte`, or proprietary opcode imports.
- Offline Rust suite: **160 passing tests**. Python discovery with LLVM22/reference configured: **22 tests discovered, 21 passing, 1 skipped**; the offline release build and `git diff --check` pass.
- **Next uncovered function starts at file offset `0x42f18`, VMA `0xb0002f18`**. Remaining assigned interval is `[0x42f18,0x100000)`. Nothing beyond the next boundary is claimed as rebuilt.
- **Provenance limitation:** branch is based on Agent 7's checkpoint (`d6104bb`) because the original checkpoint-50 merged history is unavailable. This is an integration-ready **portable branch**, not a checkpoint-50-descendant or a hardware-signed firmware.

## Checkpoint 06 — record readiness and callback guard

- Added **148 independently written Hexagon V66 instruction bytes**, covering two entire adjacent record-helper functions:
  - `[file 0x42f18,0x42f74)` = 92-byte record readiness/format checks, handler fallback, `memuh` guard, and a normalized status return.
  - `[file 0x42f74,0x42fac)` = 56-byte callback guard, conditional record-mode dispatch, saved-register restoration, and early exits.
- New Agent 13 source-built code `[0x42708,0x42fac)` is **2,212 bytes**, SHA-256 `9e747db9fbf9b1d5c29b6aa7274ecd9077ba881cacfa459f4ae82d3075b4f065`.
- Joined source-built prefix `[file 0x40000,0x42fac)` is **12,204 reference-exact layout bytes**, SHA-256 `d5abd5b035deadf4575556ea11c89472a0f0e32276cfffd8f5c817206dfc3516`, comprising **8,428 executable-instruction bytes** and **3,776 independently represented linker-fill bytes**.
- All new instructions are written as Hexagon packet syntax; zero OEM opcode arrays or unknown/padded bytes were introduced. Direct strict reference-byte comparison is zero-difference in both standalone and Agent 7–joined modes.
- Offline tests: **160 Rust**, Python discovery **22 cases, 21 passing and 1 skipped** with LLVM22/reference configured; release build passes.
- Next uncovered routine starts at **file `0x42fac`, VMA `0xb0002fac`**. The owned reservation remains `[0x42fac,0x100000)`; this is not a complete image or a newly authenticated WLAN firmware.
- Provenance: portable Agent 7–derived branch; a genuine checkpoint-50-descendant integrated branch remains a separate task.

## Checkpoint 07 — WLAN incoming record validation and stream-state dispatch

- New **1,124 source-assembled Hexagon V66 instruction bytes**, file `[0x42fac,0x43410)` / VMA `[0xb0002fac,0xb0003410)`, in complete packet-level assembly `agent13-runtime-record-processing.S`. The next routine begins with a `call/allocframe` packet at file `0x43410`; none of that packet is borrowed or emitted in this fragment.
- Reconstructed header unpacking, capacity/format checks, index/record state transitions, conditional diagnostic callbacks, pointer safety validation, and stream bookkeeping with packet-local `.new` predicates. The four-operation packet at VMA `0x2fb4` is reordered to reproduce the original duplex selection, and the `0x3394` far jump is explicitly extended as `jump ##0x2a24c0` (packet-relative target `0x2a5854`). No proprietary opcode/data words appear in the source.
- **Zero byte mismatches** for 1,124 newly generated bytes and the whole prior+new executable-layout prefix. Agent 13's total independent instruction coverage is now **3,336 bytes** `[file 0x42708,0x43410)`; joined source-built prefix is `[0x40000,0x43410)` = **13,328 bytes**: **9,552 independently authored instruction bytes** (Agent 7: 6,216; Agent 13: 3,336) and **3,776 separate linker-fill bytes**. The whole layout SHA-256 is `5561a711119f8f73ebd044340f8009ac14fa034d810f6de0038923e724aef600`.
- Rebuild command: `LD_LIBRARY_PATH=/private/ubuntu-runtime/flat python3 tools/build-wlanmdsp-agent13.py /tmp/agent13-prefix.bin --join-agent07 --llvm6 /private/llvm6/bin/llvm-mc --llvm22 /private/llvm22/bin/llvm-mc --reference /private/wlanmdsp.mbn --manifest /tmp/agent13-prefix.json`.
- Isolated Rust workspace tests **160 pass**; Python discovery **22 tests, 21 pass, 1 skip**; LLVM22 optional packet-roundtrip checked with reference; offline release build passed; `git diff --check` clean.
- Source ancestry is still Agent 7's `d6104bb`, **not** missing checkpoint-50. Whole firmware and OEM signing are not reproduced; later WLAN Hexagon executable remains unimplemented from **file `0x43410` / VMA `0xb0003410`** to this agent's reserved upper bound `0x100000`.

## Checkpoint 08 — record-context allocation and state initialization

- Added **464 source-assembled Hexagon V66 bytes** across three complete adjacent routines:
  - `[file 0x43410,0x434a8)` = 152-byte allocator/record context constructor; initializes command blocks and a mapped record pointer.
  - `[file 0x434a8,0x434b4)` = 12-byte allocator tail-call wrapper; separate entrypoint, not padding.
  - `[file 0x434b4,0x435e0)` = 300-byte record state constructor that configures object byte/word metadata and registers controller callbacks.
- LLVM22 packet encoding required preserving a compound duplex grouping around VMA `0x3480`, a packet-relative extended far jump at `0x34a0`, another correct far jump at `0x35d8`, and `:mem_noshuf` ordering of an ordered doubleword store and word load at `0x3534`. All are expressed as Hexagon V66 assembly operations, no imported instruction words.
- Total Agent 13 independently assembled code is **3,800 bytes** `[file 0x42708,0x435e0)`. Combined with Agent 7, full independently reconstructed executable layout `[file 0x40000,0x435e0)` has **13,792 exact bytes**: **10,016 encoded instruction bytes** + **3,776 linker-fill/layout bytes**. SHA-256 `9f7552d0888952b6d89ba761d607f2b2b54a3f3837a9f48a7f3c451338c9c6cb`.
- Real LLVM22 assembler and strict pinned reference check passed for all three new functions and the complete joined prefix, with zero byte mismatches. Offline Rust suite **160 pass**. Python discovery **22 tests, 21 pass, 1 skip**. Offline release build and `git diff --check` passed.
- Next code begins **file `0x435e0` / VMA `0xb00035e0`**. The owner boundary is still file `0x100000`, and the branch remains Agent 7-based, not checkpoint-50-descended. Signed/flashable firmware is not produced.

## Checkpoint 09 — bounded WLAN record controller and bit-state transitions

- Independently reconstructed **528 Hexagon V66 instruction bytes** at file `[0x435e0,0x437f0)` / VMA `[0xb00035e0,0xb00037f0)` in a complete routine. It dispatches record classes 7, 18, and 19, normalizes record state flags, counts set bits in a bounded eight-slot hardware `loop0`, and calls preexisting platform routines on state transitions.
- Exact packet-level encoding includes deliberate four-operation `r19:18`/`r20`/`r5`/`r17` duplex selection at runtime `0x35e8`, an extended `0xe000ffff` bit mask with scheduled register move at `0x374c`, and the original `}:endloop0` packet ending at `0x3714`. All are source-authored assembly instructions rather than vendor instruction word arrays.
- Total new Agent13 code `[file 0x42708,0x437f0)` **4,328 source-assembled instruction bytes**. Joined with Agent 7 source the layout prefix is `[0x40000,0x437f0)` = **14,320 reference-exact bytes**, **10,544 instruction bytes** and **3,776 linker-fill bytes**. SHA-256 `9cb96bc6ffa35277012456125a3cfd4f23c0b084e56f63bf8c1fa6dc39264fa4`.
- Direct strict reference check, LLVM22 assembler reconstruction, 160 Rust tests, 22 Python test cases (21 pass, 1 skip), release build and Git diff checks passed. Next Hexagon routine begins at file `0x437f0` / VMA `0xb00037f0`, still below Agent13's upper file bound `0x100000`.
- Portable Agent7-based branch only; no checkpoint-50 ancestry, new Qualcomm signature, complete WLAN executable, or hardware execution has been established.

## Checkpoint 10 — stream control, bounded counter, and record query

- Added **772 reference-exact source-authored Hexagon V66 instruction bytes** in three adjacent completed routines:
  - `[file 0x437f0,0x43984)` = **404 bytes**: stream/control-format unpacking, optional status checks, record-list handling and controller callbacks.
  - `[file 0x43984,0x43a18)` = **148 bytes**: bounded record/count normalization with conditional callback return.
  - `[file 0x43a18,0x43af4)` = **220 bytes**: record lookup and query-format dispatch with restored frames and common diagnostics.
- The first routine preserves the original compound call/zero-branch packet at `0x3830` (`r0 = #0; jump ##0x2a1fe8`) and a deliberate duplex grouping of `r0/r1/r2/r3` around `0x38e0`. All 772 bytes independently LLVM22-reassemble, with zero OEM copied word arrays.
- Source-generated Agent13 code is now `[file 0x42708,0x43af4)` **5,100 bytes**; combined Agent7+13 executable layout prefix `[0x40000,0x43af4)` is **15,092 byte-identical bytes**, including **11,316 independently encoded instruction bytes** and **3,776 separate linker-fill bytes**. Whole-prefix SHA-256 `0fd8383ee9ffa3ab7b15b9688382770734597198b3b7d28df4d61c8994aa3f8d`.
- Full strict firmware checksum and direct byte verification, LLVM22 assembly, **160 Rust tests**, **21 passing Python tests with 1 optional skip**, release build and `git diff --check` all passed. The next function begins at **file `0x43af4` / VMA `0xb0003af4`**. Agent13's owned upper bound remains `0x100000`; the file is still partial and not OEM-signed.
- Git branch remains an Agent7-based portable continuation; integration against absent checkpoint-50 requires an independent merge and full retest.

## Checkpoint 11 — record unlinking, popcount and session traversal

- Three complete Hexagon V66 routines add **784 independently assembled instruction bytes** contiguous with checkpoint 10:
  - `[file 0x43af4,0x43c30)` **316 bytes**: unlinks session/record chains, runs callbacks, and releases record state. Its conditional store/load uses `:mem_noshuf`; the three-operation packet at runtime `0x3b88` has explicit source ordering to preserve the original immediate extension and duplex load.
  - `[file 0x43c30,0x43c8c)` **92 bytes**: callback/count wrapper, eight-entry bounded popcount `loop0` and tail return. Source operand ordering preserves a conditional duplex packet at `0x3c58`.
  - `[file 0x43c8c,0x43e04)` **376 bytes**: linked session traversal, bounded record cleanup and predicate/status guards.
- Agent13 code `[file 0x42708,0x43e04)` totals **5,884 independently encoded instruction bytes**. Joined Agent7+13 executable layout `[file 0x40000,0x43e04)` totals **15,876 byte-identical bytes**: **12,100 instruction bytes** + **3,776 separately modeled linker fill**, zero difference from pinned `wlanmdsp.mbn`.
- `--join-agent07 --reference`, LLVM22 V66 packet assembler, `cargo test --workspace --offline --locked` (**160 passing Rust tests**), Python discovery (**22 passed, 1 optional skip**), release build and `git diff --check` all passed. Fragment sources contain no `.incbin` or opaque opcode-word directives.
- Next function begins **file `0x43e04` / VMA `0xb0003e04`**. Independent firmware completion and OEM signing remain out of scope; branch is still based on Agent7 `d6104bb`, not checkpoint50.

## Checkpoint 12 — record increment and controller plane processing

- Three complete adjacent Hexagon V66 routines add **888 independently assembled bytes**:
  - `[file 0x43e04,0x43f54)` = **336 bytes**: increment ring slot, update wide metadata and invoke optional command reporting; four `:mem_noshuf` packets preserve original dependent memory instruction order.
  - `[file 0x43f54,0x43f88)` = **52 bytes**: controller readiness checks and fast guarded tail call/return.
  - `[file 0x43f88,0x4417c)` = **500 bytes**: output-plane metadata updates, 64-bit masks and optional control callbacks; three more `:mem_noshuf` groups needed for exact packet ordering.
- Agent 13's independently assembled instruction count: **6,772** bytes `[file 0x42708,0x4417c)`. Joined Agent7+13 prefix `[file 0x40000,0x4417c)` is **16,764 bytes**, consisting of **12,988 independent Hexagon instructions** and **3,776 noninstruction linker-fill bytes**. Strict pinned-reference comparison reports zero mismatches.
- LLVM22 V66 reassembly, 160 Rust tests, Python discovery 24 cases (**23 passed, 1 optional skipped**), offline release build, and Git diff checks passed. No original binary byte literals were embedded in the assembly sources.
- Next full routine begins at file `0x4417c` / VMA `0xb000417c`; Agent13 reservation ends at `0x100000`. Branch is still Agent7-derived and not merged atop checkpoint50; firmware authentication, further code, and hardware execution remain unverified.

## Checkpoint 13 — record control, refresh, and status thunks

- Seven adjacent complete Hexagon V66 routines add **736 independent instruction bytes**:
  - `[file 0x4417c,0x441cc)` 80 bytes, controller callback gate;
  - `[file 0x441cc,0x44244)` 120 bytes, controller notification and resource release;
  - `[file 0x44244,0x44314)` 208 bytes, record mode/field marking and cleanup;
  - `[file 0x44314,0x443e4)` 208 bytes, record-refresh and packet serialization;
  - `[file 0x443e4,0x4443c)` 88 bytes, record status handler;
  - `[file 0x4443c,0x4444c)` and `[0x4444c,0x4445c)` 16 bytes each, state-notification wrappers.
- The original refresh packet at runtime `0x4314` requires forcing an extended packet-local jump (`r1=#2; jump ##0xc`) to retain original immediate-extension placement and duplex selection; no raw instruction word import was used. Packet `0x43d8` preserves memory dependence with `:mem_noshuf`.
- Agent13 source assembled `[file 0x42708,0x4445c)` = **7,508 instruction bytes**. Joined Agent7+13 `[file 0x40000,0x4445c)` is **17,500 exact layout bytes**, comprising **13,724 genuine instruction bytes** and **3,776 separately modeled linker fill**. Direct pinned revision compare: zero mismatched bytes; all source modules use semantic Hexagon packet syntax.
- LLVM22 V66 assembler, 160 Rust tests, Python discovery 25 cases (**24 passed, 1 skipped**), release build and Git diff checks passed. Next routine starts file `0x4445c`, VMA `0xb000445c`; Agent13 exclusive ownership ends at file `0x100000`.
- Portable Agent7 branch only, not checkpoint50-descended or signed/flashable WLAN firmware. Full executable and on-device behavioral verification remain unfinished.

## Checkpoint 14 — controller command, notification thunks, bitmap shift

- Four contiguous original Hexagon V66 routine encodings add **440 byte-identical instruction bytes**:
  - `[file 0x4445c,0x445b0)` = **340 bytes**, controller mode processing, conditional descriptor/record operations, and callback dispatch.
  - `[file 0x445b0,0x445b8)` and `[0x445b8,0x445c0)` = **8 bytes each**, distinct state-notification tail thunks with extended packet-relative jumps (`jump ##0x1ce980` and `jump ##0x1ce978`) targeting the same platform address.
  - `[file 0x445c0,0x44614)` = **84 bytes**, bounded three-entry hardware-loop bitmap shift and result return.
- Newly authored Agent13 Hexagon source totals **7,948 instruction bytes** `[file 0x42708,0x44614)`. Joined Agent7+13 executable layout `[file 0x40000,0x44614)` is **17,940 exact bytes**, made up of **14,164 independent instruction bytes** and **3,776 distinct linker-fill bytes**, with zero reference mismatches.
- Independent LLVM22 reassembly, full 160 Rust tests and Python discovery 26 cases (**25 pass, 1 optional skip**), offline release build, and `git diff --check` passed.
- The next function begins at `file 0x44614` / VMA `0xb0004614`; still below Agent13's `file 0x100000` ownership boundary. Branch remains Agent7-derived; no checkpoint50 integration, OEM signing, whole WLAN executable completion, or on-device execution established.

## Checkpoint 15 — linked-record maintenance and status-mask fields

- Six complete Hexagon V66 source routines add **844 instruction bytes**:
  - `[file 0x44614,0x446f0)` **220 bytes** controller request and record status handling;
  - `[0x446f0,0x44760)` **112 bytes** indexed record-list insertion (extended relative tail jump);
  - `[0x44760,0x447dc)` **124 bytes** linked-record removal (explicit instruction/duplex source ordering);
  - `[0x447dc,0x44804)` **40 bytes** record-list query;
  - `[0x44804,0x4492c)` **296 bytes** controller mode/mask field updates;
  - `[0x4492c,0x44960)` **52 bytes** status-bit query.
- New Agent13 source `[file 0x42708,0x44960)` totals **8,792 independently assembled instruction bytes**. Combined Agent7+13 WLAN executable layout `[file 0x40000,0x44960)` totals **18,784 exact bytes**: **15,008 instruction bytes** + **3,776 separately modeled linker-fill bytes**, with zero reference-byte mismatches.
- LLC Hexagon LLVM22 V66 source assembly, 160 Rust tests, Python discovery 27 cases (**26 passed, 1 optional skip**), release build and Git diff checks passed.
- Next routine starts at file `0x44960` / VMA `0xb0004960`. Agent13 owns to `0x100000`, and this is an Agent7-derived portable branch only, not a complete or signed WLAN firmware.

## Checkpoint 16 — session table, record integrity, statistics and reconfiguration

- Six complete contiguous original Hexagon V66 routines add **884 source-authored instruction bytes**:
  - `[file 0x44960,0x449d8)` **120 bytes**: session table reporting;
  - `[file 0x449d8,0x44a4c)` **116 bytes**: guard on session/controller mode;
  - `[file 0x44a4c,0x44b20)` **212 bytes**: record health and state checks;
  - `[file 0x44b20,0x44b7c)` **92 bytes**: record reset and callback helper;
  - `[file 0x44b7c,0x44cb4)` **312 bytes**: record stats publisher and metadata;
  - `[file 0x44cb4,0x44cd4)` **32 bytes**: reconfiguration tail handler.
- Agent 13 new source `[file 0x42708,0x44cd4)` totals **9,676 typed LLVM22-assembled instruction bytes**. Joined Agent 7 + Agent 13 `[file 0x40000,0x44cd4)` is **19,668 byte-identical layout bytes**: **15,892 source-assembled instructions** and **3,776 distinct linker-fill bytes**. Exact pinned-reference validation: zero mismatches.
- Full Cargo tests: **160 Rust passed**; Python suite: **28 total, 27 passed and 1 optional skipped**. `cargo build --workspace --release --offline --locked`, real LLVM22 v66 packet reassembly, and `git diff --check` all passed.
- Next missing routine begins at WLAN file `0x44cd4`, virtual address `0xb0004cd4`; this agent's exclusive upper bound is file `0x100000`. Portable Agent7-derived branch, not checkpoint-50 integrated. Whole WLAN firmware and OEM authentication remain unrebuilt.
