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

## Checkpoint 26 (disjoint; checkpoint 25 not available in this session) — controller initialization

- The latest available Git bundle in this session is Agent 13 checkpoint 16
  `7c730bf`; the user discussion references checkpoint 25 `3cb8bac`, but its
  actual Git bundle was **not mounted** and was not accessible from the
  current Project Library. This change is a **portable independently verified
  disjoint continuation**, not a branch/descendant of checkpoint 25.
- Owner: `wlanmdsp.mbn` executable file `[0x4568c,0x45a20)` = **916 newly
  assembled Hexagon v66 bytes**, VMA `[0xb000568c,0xb0005a20)`. No padding,
  OEM opcode arrays, or opaque copied reference bytes were emitted.
- The function initializes WLAN controller resources, record state, callback
  pointers and status flags. Rebuild preserves packet parse bits, `:mem_noshuf`
  ordering on two register-slot pairs, PC-relative calls/branches and an
  explicitly extended final `r0=#0; jump ##...` duplex packet.
- Assembly verifier: `python3 tools/build-wlanmdsp-agent13-forward.py
  --llvm6 /private/llvm6/bin/llvm-mc --reference /private/wlanmdsp.mbn
  --output /tmp/agent13-new.bin --manifest /tmp/agent13-new.json`.
- Source-only and exact-length tests are in `tests/test_agent13_wlan_forward.py`.
- **Do not sum** this 916-byte region into checkpoint 16's 19,668-byte prefix
  as contiguous: checkpoint 17–25 sources covering `[0x44cd4,0x4568c)` are
  absent from this session and must be applied first.

## Checkpoint 27 (disjoint; checkpoint25 still absent) — controller table routines

- Rebuilt controller-table creation and diagnostic fallbacks:
  `[file 0x45a20,0x45ac8)` = **168 Hexagon instruction bytes**.
- Rebuilt bounded two-word-key table search with a hardware loop:
  `[file 0x45ac8,0x45b58)` = **144 independently assembled instruction bytes**.
- These are adjacent to checkpoint 26’s original 916-byte controller
  initialization. Together, source-built `[file 0x4568c,0x45b58)` totals
  **1,228 bytes with zero byte differences** against the original SHA-256-pinned
  WLAN executable `PT_LOAD`.
- Exact packet identity required the table-create diagnostic packet to place
  `r1 = ##...; deallocframe` ahead of `r2=#25` *in source* so the assembler
  chose the original slot order. Table-search loop packet uses symbolic
  `loop0(L_00460,r5)` rather than an incorrect absolute immediate.
- Still **not a direct descendant** of missing Agent 13 checkpoint 25.
  The exclusive file region `[0x44cd4,0x4568c)` must be imported from
  checkpoints 17–25 before reporting a contiguous merged prefix.

## Checkpoint 28 (disjoint) — bitmap scan with reconstructed duplex packet

- Added `[file 0x45b58,0x45c0c)` = **180 source-generated Hexagon bytes**
  for a bounded hardware-loop scan of per-slot bitmap flags, load widths,
  unsigned max comparisons and optional result writeback.
- The original three-word packet at `[file 0x45b78,0x45b84)` has *two separate
  predicate operations* and a final **duplex** instruction comprising a register
  move and `r0=-1`. The LLVM6 canonical packet shuffle emits an equivalent
  but byte-different packet. Instead, the source generator assembles the
  two predicates with LLVM6 `-mno-pairing` and independently assembles the
  duplex from `{ r2 = r6; r0 = #-1 }`. Only those compiler-generated semantic
  instruction pieces are concatenated—**no extracted OEM instruction bytes**
  or `.word` directives are included in the build. The combined packet is
  byte-identical, and the hardware-loop target uses `loop0(L_00514,r7)`.
- Combined with checkpoints 26 and 27, the current disjoint forward fragment
  `[file 0x4568c,0x45c0c)` is **1,408 independent source-built bytes**, with
  zero mismatches against the pinned WLAN original.
- Agent13 checkpoint 25 is still not mounted. The preceding
  `[0x44cd4,0x4568c)` therefore remains **unavailable in this checkout**;
  do not claim a continuous `0x40000–0x45c0c` prefix until that parent history
  is recovered and merged.

## Checkpoint 29 (disjoint; checkpoint 25 unavailable) — snapshot and controller initialization prefix

- Added **236 independently assembled Hexagon instruction bytes** at file
  `[0x45c0c,0x45cf8)`: a **20-byte snapshot/callback wrapper** at
  `[0x45c0c,0x45c20)` followed by a **216-byte controller initialization
  prefix** `[0x45c20,0x45cf8)`.
- The generated continuation `[0x4568c,0x45cf8)` totals **1,644 source-assembled
  instruction bytes**, with zero differences against pinned reference
  `wlanmdsp.mbn` SHA-256
  `87c2c7bb1fe97eaf007a947832feba0898644b4d6626ac8ef49d8b94dd82604b`.
- The initial controller function continues after file `0x45cf8`; source only
  covers complete packets and stops before its first complex predicate jump.
  Two memory packets require `:mem_noshuf` for byte-identical source order.
- This remains a **checkpoint 16-derived disjoint continuation**, not a
  descendant of unavailable checkpoint 25. Do not count it as a contiguous
  prefix from `0x40000` until those missing Git commits are integrated.

## Checkpoint 30 (disjoint) — extended branch and controller mode fields

- Added **248 source-generated Hexagon bytes** `[file 0x45cf8,0x45df0)`.
  Their exact decomposition is a 16-byte compound predicate/branch and
  dependent load+new-value-store packet `[0x45cf8,0x45d08)`, followed by
  **232 bytes** of controller buffer-mode and attribute calculations
  `[0x45d08,0x45df0)`.
- LLVM6 cannot schedule the compound four-instruction packet as written:
  packet slots reject the jump, byte load, and dependent store together.
  The builder compiles the first two and final two operations independently
  from semantic Hexagon source, changing **only Hexagon packet parser bits
  15:14** of the separately compiled branch from `0b11` (packet end) to
  `0b01` (packet continues). The `.new` load/store pair and all instruction
  encoding bits come directly from LLVM6, not from copied OEM bytes. A unit
  test checks the parser-field-only transformation.
- In the following 232 bytes, three `:mem_noshuf` packets preserve the
  original memory-operation order. Direct pinned-firmware byte comparison
  reports **zero differences** for the entire continuation.
- Verified continuation is now **1,892 bytes** `[file 0x4568c,0x45df0)`;
  next missing packet is at `0x45df0`. This is **not** a checkpoint25
  descendant and must not be summed into the earlier full WLAN prefix as
  contiguous without the absent checkpoint 17–25 source history.

## Checkpoint 31 (disjoint) — controller stream flag packet groups

- Added **180 source-assembled Hexagon bytes** at file `[0x45df0,0x45ea4)` for controller state and format flag packing. Original WLAN vaddr interval `[0xb0005df0,0xb0005ea4)`.
- Two packets require `:mem_noshuf` to preserve the original dependent load/store order. An alternative source operand order preserves the predicated store packet at file `0x45e0c`.
- Numeric branch targets from decoder output were converted to **packet-relative displacements**, not copied machine-code words: the output assembler matches the original on every byte.
- Current **disjoint continuation** `[0x4568c,0x45ea4)` is **2,072 independently source-assembled instruction bytes**, zero OEM opcode arrays and zero linker fill in this new continuation. SHA-256 `b410164de7c9f2561c1409f35e589dbb751931cbc4712b8781aa3484f8f7af4b`.
- Private reference SHA-256 `87c2c7bb1fe97eaf007a947832feba0898644b4d6626ac8ef49d8b94dd82604b` was verified; source-only builder accepts the reference only for comparison. All 160 Rust and 33 executed Python tests passed (2 optional skips), and the offline release build passed.
- This function began at file `0x45c20` and continues after `0x45ea4`. Checkpoint 25 remains unavailable in this repository's Git ancestry; do **not** claim that `[0x40000,0x45ea4)` is a reconstructed contiguous prefix. Next missing packet starts file `0x45ea4`. Original firmware is not included in Git or the bundle.

## Checkpoint 32 (disjoint) — controller stream status and loop entry

- Added **676 source-assembled instruction bytes** across five-file controller stage-3 extension, file offsets `[0x45ea4,0x46148)` (original WLAN vaddr `[0xb0005ea4,0xb0006148)`). The previous 180-byte stage-3 flag group remains independently verified in checkpoint 31.
- Four new structured packet sections preserve status masks, input-codec callback fields, rate/pitch information and bounded session readiness checks. The last part builds the hardware-loop entry and packet-local loop-state predicates through an exact packet boundary at `0x46148`.
- LLVM6 semantic assembly emitted all new bytes; no `.word`, `.incbin`, or private OEM opcode array was used. Four pieces were assembled independently to preserve original packet grouping and numeric **packet-relative** call/branch offsets.
- Current **disjoint** source-generated continuation `[0x4568c,0x46148)` totals **2,748 bytes**, SHA-256 `6bab8cf6203a9b292124e4b442847477671e369149ba46ef995ed40c57cf11d9`, with **zero byte mismatches** against the SHA-256-pinned `wlanmdsp.mbn` original. The standalone builder is independent of the reference except during the optional comparison.
- Full 160 Rust tests passed; Python discovery 35 tests, 33 executed and passed, two optional skips; offline release build and `git diff --check` passed.
- WLAN controller initialization still continues at file `0x46148`. Checkpoint 25 Git ancestry is not available; this remains based on checkpoint 16 and must not be counted as a contiguous `0x40000` prefix without checkpoints 17–25.

## Checkpoint 33 (disjoint) — controller hardware loop and table iteration

- Added **204 independently assembled Hexagon instruction bytes** at WLAN file `[0x46148,0x46214)` (VMA `[0xb0006148,0xb0006214)`), finishing a bounded six-iteration `loop0`, bitmap predicate updates and the beginning of record-table traversal.
- Original three-operation physical packet `[0x46160,0x4616c)` comprises two full-width register operations followed by a predicate/load duplex. LLVM6 emitted exactly the correct instruction opcodes, but placed an end-of-packet marker on the second word. Checked builder changes only its Hexagon parser bits 15:14 from `0b11` (end) to `0b01` (continuation), then appends a separately source-assembled duplex. Source-only `:endloop0` tail matches without adjustment. No OEM opcode word is copied into the generator.
- The controller loop and following record-table packets assemble independently to zero byte differences against the exact pinned WLAN MBN. Cumulative **disjoint continuation** `[0x4568c,0x46214)` is **2,952 genuine Hexagon instruction bytes**, with no padding/external code in this new span. SHA-256 `365cdc36213968ffaa8e77fd13064f42b321a5a16beef49c70aaf508659d698c`.
- Full 160 Rust tests and 34 executed Python tests passed (2 skips), offline release build and `git diff --check` passed.
- Next missing instruction packet starts at file `0x46214` (vaddr `0xb0006214`), still within the same controller initialization function. Branch descended from checkpoint 16; missing checkpoint17–25 source history must be integrated before claiming a whole contiguous startup prefix.

## Checkpoint 34 (disjoint) — indexed-plane updates and controller callbacks

- Added **496 independently assembled Hexagon instruction bytes** at WLAN file offsets `[0x46214,0x46404)` (VMA `[0xb0006214,0xb0006404)`), implementing indexed-plane selection, codec-format offset calculations, controller state callbacks, error-path flags and public controller notification.
- Four fragment boundaries are explicitly asserted (`0x46214`, `0x46300`, `0x46380`, `0x463c0`, `0x46404`). An original three-instruction packet at `0x46300` required `:mem_noshuf` to preserve store/load operation order. A wide signed immediate (`-1342151444`) is emitted with native Hexagon syntax, not copied machine words.
- Cumulative **disjoint continuation** `[0x4568c,0x46404)` is **3,448 source-built Hexagon instruction bytes**, SHA-256 `30d978d596933ee81ae95bcb42859a7d6df25be324f9fa0814b4ca9ce020c616`, with **zero differences** against the pinned original. There are no external code bytes or padding in the new span.
- Full 160 Rust and 35 executed Python tests passed (2 optional skipped). Offline release build passed and Git diff is clean.
- Next missing packet starts file `0x46404` (VMA `0xb0006404`) within the controller routine. Still based on checkpoint 16; checkpoint17–25 ancestry is not included in this branch. Partial program cannot be flashed or newly authenticated.

## Checkpoint 35 (disjoint) — controller finalization and session buffer iterator

- Rebuilt **764 source-assembled instruction bytes** at `[file 0x46404,0x46700)` (VMA `[0xb0006404,0xb0006700)`), covering the completion of controller property publication, dependent controller callbacks and the beginning of a bounded session-buffer loop.
- The far tail branch at `0x4645c` and extended duplex at `0x464ec` required explicit `##` immediates so LLVM6 retained original immediate extender words; no OEM opcode bytes imported. Controller status packet `0x46300` and zero-initialization packet `0x46538` required source-order adjustments to preserve the original load/store and duplex slot selection.
- Three new packet sections `[0x46404,0x46500)`, `[0x46500,0x46600)`, `[0x46600,0x46700)` are built from original semantic Hexagon instructions, independently assembled and compared directly. Combined **disjoint continuation** `[0x4568c,0x46700)` totals **4,212 source-generated instruction bytes** and SHA-256 `8533db1c52db81827082f87c787bc667574cb46d3359f4fb48d6492abbb32fbf`; zero reference differences, zero external opaque bytes and zero filler inside this range.
- All 160 Rust tests passed, as did 36 executed Python tests with two optional skips, offline release build and Git diff checks.
- Next missing packet begins at file `0x46700`. The owning range ends at file `0x100000`, not yet complete. Branch history still starts from checkpoint 16; missing checkpoints 17–25 must be integrated before calling this a continuous source-built prefix from file `0x40000`.

## Checkpoint 36 (disjoint) — session buffer selection and plane records

- Added **776 independently source-assembled Hexagon instruction bytes** at `[file 0x46700,0x46a08)` (VMA `[0xb0006700,0xb0006a08)`). Code performs linked session-table traversal, indexed record lookup, codec mode-dependent plane selection, metadata status updates and buffer record copy preparation.
- Source grouped as three adjacent packet-encoded functions/sections at `[0x46700,0x46800)`, `[0x46800,0x46900)` and `[0x46900,0x46a08)`. The latter two require `:mem_noshuf` for five dependent data access packets; no opaque reference instructions are used.
- Full new disjoint continuation `[0x4568c,0x46a08)` totals **4,988 actual Hexagon instruction bytes**, no structural layout padding and no externally supplied code; SHA-256 `e0b26407966ce02eb735b7806d000dccfa17b87a8ba662ac45f301ab57487aea`. LLVM6 assembly compared against SHA-256 pinned `wlanmdsp.mbn`: **zero differing bytes**.
- All 160 Rust tests passed; 37 Python tests executed and passed with two optional skips; offline release build passed; `git diff --check` clean.
- First missing code now begins at file `0x46a08` (VMA `0xb0006a08`), in the continuing WLAN controller routine. This branch still descends from checkpoint16 rather than unavailable checkpoint25; provenance and whole-firmware warnings remain unchanged.

## Checkpoint 37 (disjoint) — finish session hardware loop and controller reporting

- Reconstructed **404 source-assembled Hexagon instruction bytes** at WLAN
  file offsets `[0x46a08,0x46b9c)` (VMA `[0xb0006a08,0xb0006b9c)`).
  This completes the prior session-buffer traversal hardware loop and its
  controller diagnostic/reporting path, including `:endloop0`, conditional
  callbacks and a `dealloc_return` epilogue at `0x46b98`.
- Six dependent load/store packets use `:mem_noshuf` to preserve the original
  packet slot order; the branch targets are symbolic local labels or explicit
  packet-relative displacements. LLVM6 assembles every word exactly without
  OEM instruction arrays or `.word` directives in committed source.
- Cumulative independently generated **disjoint** continuation
  `[file 0x4568c,0x46b9c)` totals **5,392 instruction bytes**,
  with no structural fill or externally supplied code. Reference SHA-256 is
  `87c2c7bb1fe97eaf007a947832feba0898644b4d6626ac8ef49d8b94dd82604b`.
- The checkpoint 25 ancestry hole remains: this branch descends from Agent 13
  checkpoint 16 and does not establish a continuous prefix from `0x40000`.
- The next source function begins with a call packet at `0x46b9c`. Its
  complete boundary and packet semantics must be checked before extension.

## Checkpoint 38 (disjoint) — session metadata initialization prefix

- Added **612 independently source-assembled Hexagon instruction bytes**
  at `[file 0x46b9c,0x46e00)` (VMA `[0xb0006b9c,0xb0006e00)`).
  This begins a larger session initialization function and constructs the
  controller session-parameter frame, 64-bit counters, data-plane extents,
  diagnostic metadata, and a callback to an external codec formatter.
- Three original packets required explicit semantic source reordering to
  retain the original duplex grouping: source packet positions
  `0x46bb8`, `0x46bc4`, `0x46bf8`. Reconstructed instructions were encoded
  entirely by LLVM6, without copying original instruction words.
- The output is a **partial function**; its next instruction at file
  `0x46e00` is not decoded by the LLVM6 disassembler and requires separate
  V66-capable decoding or alternative analysis before encoding. Do not
  assume surrounding words are data merely because this decoder reports
  `unknown`.
- Cumulative disjoint `[file 0x4568c,0x46e00)` is **6,004 matched
  source-assembled bytes**, no linker fill or opaque external code.
  The checkpoint-25 provenance gap remains as before.

## Checkpoint 39 (disjoint) — finish controller session-metadata and callback routing

- Added **284 source-assembled Hexagon bytes** at WLAN file
  `[0x46e00,0x46f1c)` (runtime VMA `[0xb0006e00,0xb0006f1c)`).
  This completes session-metadata reconciliation and callback/reporting,
  including a conditional format table, 8-bit record-count traversal,
  and external callback messages.
- LLVM6 initially decoded file `0x46e00` as unknown **only because the
  analysis scratch section ended halfway through its packet**. A larger
  scratch view decoded it correctly with both LLVM6 and LLVM22 Hexagon v66;
  no opcode has been treated as opaque data.
- One `:mem_noshuf` block at `0x46ea4` preserves the original dependent
  read/write order. At `0x46e3c` LLVM6 duplexes the two address writes in
  original order only when semantic source order is adjusted. Forced
  `jump ##0x29e92c` retains an otherwise omitted immediate extender.
- Combined disjoint source-built continuation `[0x4568c,0x46f1c)` now
  totals **6,288 exact instruction bytes**, zero padding or vendor blob reuse.
  Next function prologue starts at file `0x46f1c`.
- The checkpoint25 ancestry gap is still unresolved: do not claim a source-
  generated contiguous prefix extending back to `0x40000` on this branch.

## Checkpoint 40 (disjoint) — record flag validation and table occupancy

- Independently rebuilt **152 Hexagon instruction bytes** at WLAN file `[0x46f1c,0x46fb4)` (VMA `[0xb0006f1c,0xb0006fb4)`), including record-table occupancy updates, a bounded count scan, diagnostic callbacks, and the original function's return.
- The original four-word packet at file `0x46f9c` contains the conditional callback `call -0x1454`, a 32-bit extended signed argument, and a diagnostic selector. LLVM 6 reproduces the packet's exact parse markers only when those semantic instructions are grouped in one source packet. No opcode constants or OEM bytes were embedded.
- Cumulative *disjoint continuation* `[0x4568c,0x46fb4)` now totals **6,440 independently source-assembled instruction bytes**, SHA-256 `d9b0a2a900ffd2e6bd294a74c69543f442bcc2305307bcfdd48b2f1c5bec6f44`, with **zero differences** against the private, SHA-256-pinned `wlanmdsp.mbn` reference.
- 160 offline Rust tests passed. Python discovery ran 43 cases, **41 passed and 2 skipped**; offline Rust release build and Git whitespace checks passed. First missing packet starts at file `0x46fb4` / VMA `0xb0006fb4`, the prologue of the following controller function.
- **Provenance warning:** This branch descends from the verified Agent 13 checkpoint 16, not unavailable checkpoint 25. Earlier checkpoint 17–25 Git commits must be integrated before claiming a single source-generated prefix from file `0x40000`. Original firmware signing and hardware execution are not established.

## Checkpoint 41 (disjoint) — record status, callback thunk, state reset

- Added **168 independently assembled Hexagon instruction bytes** at file
  `[0x46fb4,0x4705c)` (runtime VMA `[0xb0006fb4,0xb000705c)`):
  - `[0x46fb4,0x47010)` **92 bytes**: compact status-flag check, record
    count report and callback tail dispatch;
  - `[0x47010,0x47028)` **24 bytes**: a call-through context thunk;
  - `[0x47028,0x4705c)` **52 bytes**: clear codec state and diagnostic registers.
- The call in the thunk is **in the same packet** as its following register
  move and frame allocation; splitting it changes only the Hexagon packet
  marker. The reset function similarly groups the call with a 32-bit
  extended store and uses `:mem_noshuf` for a paired store/load.
- The standalone source-built disjoint continuation `[0x4568c,0x4705c)` is
  **6,608 bytes** with SHA-256
  `6b6eca8d60646d37b88a099158fb7cabb79c6415dffd707923becf068a93f7c4`;
  all **6,608 bytes** match pinned `wlanmdsp.mbn` exactly, including the
  prior checkpoint40 6,440-byte prefix. No reference firmware bytes appear
  in the assembler source or Git. The next routine begins at file `0x4705c`.
- **Provenance:** this is still a disjoint checkpoint16-derived branch, not a
  checkpoint25 or checkpoint50 descendant. Missing checkpoint17–25 history
  must be applied before claiming joined WLAN executable-layout coverage.

## Checkpoint 42 (disjoint) — bounded controller record validation

- Added **300 independently assembled Hexagon instruction bytes** at WLAN
  file `[0x4705c,0x47188)` (runtime `[0xb000705c,0xb0007188)`). The routine
  checks record dimension bounds, computes normalized lengths, reports
  invalid record counts, and calls the platform controller diagnostic hooks.
- Three original packets contain a 32-bit immediate extension followed by
  a duplex `r18`/`r2` assignment. LLVM6 produces the correct three words
  only with the original semantic source order; no `.word` directives or
  copied OEM instruction fields are used. A pair of compound call/memory
  packets also preserves packet parse markers. Record-count branches use
  named in-function labels, with differing call displacements at two sites.
- Source-built disjoint continuation `[0x4568c,0x47188)` totals **6,908
  bytes**, with SHA-256
  `516480e71ad3faf72695094b769c9e2c9a88e8d36c66efcd33b3e172da1b350a`.
  The full output and the new 300-byte region both have **zero byte
  differences** from SHA-256-pinned original `wlanmdsp.mbn`. No OEM bytes
  are stored in the repository or embedded into source generators.
- The next instruction after `0x47188` begins a session/record processing
  routine (the preceding call+allocframe packet ends at this boundary).
- **History remains disjoint:** this branch descends from checkpoint16; the
  unmerged checkpoint17–25 source commits must be integrated before claiming
  a continuous source-generated prefix from WLAN file `0x40000`.

## Checkpoint 43 (disjoint) — guarded session-processing prefix

- Added **140 source-assembled Hexagon instruction bytes** at WLAN file
  `[0x47188,0x47214)` (runtime `[0xb0007188,0xb0007214)`). This begins the
  session record-processing function with bounded length checks, address
  normalization, conditional record-property update and traversal setup.
- The 4-word packet at file `0x471c0–0x471d0` carries predicate
  evaluation, conditional arithmetic, a predicated byte write and a predicated
  word read. LLVM6 canonicalized the two memory instructions in a different
  order without `:mem_noshuf`; the exact grouping is now explicit in source.
  Five packet-relative branch displacements are authored from instruction
  addresses, not copied from reference machine code. The function's later
  record loop remains **unreconstructed**; no invented return is emitted.
- The complete disjoint continuation `[0x4568c,0x47214)` now comprises
  **7,048 genuine Hexagon source-assembled bytes** with SHA-256
  `5a93bc4652620ff65e2d9c80f14f26480ecda207fff35d5549ff121d0bda9fdd`.
  The full output and new 140-byte interval are exactly equal to the pinned
  reference. No original firmware is committed to the Git history.
- The next packet begins at file `0x47214` (runtime `0xb0007214`).
- Branch is **still checkpoint16-derived**, missing the checkpoint17–25
  commits; this cannot be presented as a continuous source-generated WLAN
  prefix from file `0x40000` until that ancestry gap is integrated.

## Checkpoint 44 (disjoint) — variable-length record traversal

- Added **228 source-assembled Hexagon instruction bytes** at file
  `[0x47214,0x472f8)` (runtime `[0xb0007214,0xb00072f8)`). The code
  traverses a descriptor list, copies bounded fields into a stack frame,
  combines variable-length segments and updates the record stride.
- One instruction `r20 = add(r25,mpyi(#28,r19))` must form its own packet;
  grouping it with the following call changes only the packet parse bits.
  A later two-operation `r2` calculation must be separated from the
  dependent byte load/store to avoid two writes to `r2` in a single packet.
  All source instructions are LLVM6 assembled; no reference opcode arrays.
- Cumulative **7,276 exact source-generated instruction bytes** cover WLAN
  file `[0x4568c,0x472f8)`, SHA-256
  `789b3591aefdf44d75c8776b2ef03ae1e9f0d9eaa0c007e7e869d46cdb1e5c42`.
  Full reference comparison reports **zero byte differences**. The session
  processing routine continues beyond `0x472f8`; no return was invented.
- Git history still derives from checkpoint16 and lacks checkpoint17–25.
  Do not call this contiguous with the original Agent 7 prefix until those
  source histories are incorporated into a checkpoint-50-rooted integration.

## Checkpoint 45 (disjoint) — session record increment and guarded updates

- Added **164 source-built Hexagon instruction bytes** in WLAN file
  `[0x472f8,0x4739c)` (runtime `[0xb00072f8,0xb000739c)`). The code
  advances the per-session cursor, checks bounded record widths, updates
  a per-slot bitmap and conditionally appends a record field.
- The packet at `0x47370` must keep multiply-indexed address calculation
  with the following `lsl`/byte-count operations. Separating it changes only
  the Hexagon packet parser marker; the platform call at `0x47380` is
  itself a separate packet, not grouped with the next source load.
- The verified disjoint continuation `[0x4568c,0x4739c)` is now **7,440
  Hexagon instruction bytes**, with SHA-256
  `8b0445fc9e00e5154d9a9e30e0b10ed46ff4c3cfe9007fca19f36d33291734e2`.
  The original pinned WLAN MBN and the independently assembled bytes match
  exactly, with zero differences and no borrowed OEM instruction words.
- The record routine continues at file `0x4739c`. Missing checkpoint17–25
  source history prevents claiming a contiguous prefix from `0x40000`.

## Checkpoint 46 (disjoint) — complete session record error/report paths

- Added **260 Hexagon-assembled bytes** in WLAN file `[0x4739c,0x474a0)`
  (runtime `[0xb000739c,0xb00074a0)`), including three explicit 4-byte
  terminal NOPs `[0x47494,0x474a0)` that provide function-alignment fill.
  The remaining 248 bytes encode source-authored instructions for bounded
  error reporting, branch-specific diagnostic callbacks, record-count
  updates and a shared tail route; no OEM opcode arrays were copied.
- LLVM6 keeps the call at `0x473e0` in the same packet as the extended
  diagnostic argument; putting it in a separate block changes a parser bit.
  The three-instruction packet at `0x47448` must present the extended
  `r0` immediate before `r2=#10` to preserve the original duplex encoding.
- Full disjoint continuation `[0x4568c,0x474a0)` totals **7,700 exact
  bytes**, SHA-256
  `dd6b73020c2c629eed289f0138f5948847155c13b298603befb102646caa1dc7`.
  Original WLAN reference checks, offline Rust/Python regression tests and
  release build passed with zero byte mismatches.
- The record-processing routine is complete through its original error
  paths; the next function begins at file `0x474a0`. Git ancestry is still
  from checkpoint16; checkpoints 17–25 remain absent.

## Checkpoint 47 (disjoint) — controller session-parameter dispatcher

- Added **192 source-assembled Hexagon instruction bytes** at WLAN file
  `[0x474a0,0x47560)` (runtime `[0xb00074a0,0xb0007560)`). The code
  allocates session state, validates its mode parameter, loads a format
  handler and dispatches conditionally to further controller routines.
- The conditional jump at `0x474d0` requires a **source-level extended
  immediate**; LLVM6 otherwise chooses the shorter encoding. Its displacement
  and the following predicate-directed mode branch were verified against the
  pinned reference, without copying OEM code words. A table-indirect `jumpr`
  remains named as a register operation.
- The new bytes are a **partial function**, not a fabricated return.
  Its following controller routines and branches remain unresolved after
  `0x47560`, the next source-boundary packet.
- Combined disjoint continuation `[0x4568c,0x47560)` is now **7,892 exact
  Hexagon instruction bytes**, SHA-256
  `3c5418f40ee387b012bb56526e33fb5d5b84c29ca059236c7c645c14f6fd42d7`.
  Zero byte differences against the original WLAN MBN. Full offline
  Rust/Python tests and release build passed.
- Still not checkpoint25-descended: checkpoint17–25 source integration
  is pending, as is full firmware signing and runtime validation.

## Checkpoint 48 (disjoint) — controller parameter reports and extended branches

- Added **88 independently assembled Hexagon instruction bytes** at WLAN
  file `[0x47560,0x475b8)` (runtime `[0xb0007560,0xb00075b8)`). Three
  format-field extracts are followed by controller diagnostic calls; the
  section ends at two separate extended branch packets.
- LLVM6 requires both `extractu` instructions grouped with their following
  calls, and two explicit `##` immediate extenders for far conditional
  paths. The call displacements are original packet-relative values,
  authored in instruction source; no copied OEM opcode words or `.incbin`.
- The complete disjoint continuation `[0x4568c,0x475b8)` is **7,980 exact
  Hexagon source-assembled bytes**, SHA-256
  `bbeda15366ba6d145b4c07bf214c93e394698cdd64bfe301914f82894dc0fd1c`.
  Zero reference differences. Rust, Python and offline release-build checks
  pass; next missing packet starts file `0x475b8` (vaddr `0xb00075b8`).
- This remains a **checkpoint16-derived disjoint** contribution; absent
  checkpoint17–25 commits must be integrated before asserting a contiguous
  original WLAN prefix from file `0x40000`.

## Checkpoint 49 (disjoint) — session-state initialization and extended duplex

- Reconstructed **144 independently source-assembled Hexagon instruction bytes**
  in WLAN file `[0x475b8,0x47648)` (VMA `[0xb00075b8,0xb0007648)`).
  This is the entry and initial controller-state dispatch of an **incomplete**
  larger function; its cases remain outside this checkpoint.
- The first 40 bytes are the packet-framed state and optional callback guard.
  The following packet combines a far jump immediate extender with the
  one-word `r1=#63; jump` duplex instruction. LLVM6 can assemble each
  source-defined semantic part separately, but cannot pair them directly.
  The builder therefore combines *only* compiler-generated extender and
  duplex encodings, checks their parse fields, and never imports OEM words.
  The last 96 bytes initialize and report controller parameters; packet-
  relative call/jump displacements are expressed as original semantics.
- Total disjoint Agent 13 continuation `[file 0x4568c,0x47648)` now comprises
  **8,124 instruction bytes**, SHA-256
  `b15eb83323299877619468e54ba95ad83f6fd63976c10927e22d817d6a3ca308`.
  Original pinned `wlanmdsp.mbn` SHA-256 verified, and direct complete-range
  byte comparison reports **zero mismatches**.
- Source-only build:
  `python3 tools/build-wlanmdsp-agent13-forward.py --llvm6 /private/llvm6/bin/llvm-mc --reference /private/wlanmdsp.mbn --output /tmp/wlan-forward.bin`.
- Still based on checkpoint16 `7c730bf` without the missing checkpoint17–25
  branch ancestry; this **does not** establish a contiguous executable-layout
  prefix from `0x40000`. No complete or authenticated WLAN firmware was built.
- Next missing file offset is **`0x47648`** (VMA `0xb0007648`).

## Checkpoint 50 (disjoint) — controller format case packets

- Added **180 independently assembled Hexagon instruction bytes** at WLAN
  file `[0x47648,0x476fc)` (VMA `[0xb0007648,0xb00076fc)`). This continues
  the format/state case selector without inventing a return beyond it.
- Seven two-word packets contain signed immediate extenders and one-word
  `r2=#case; jump` duplex branch opcodes. LLVM6 independently generates the
  far-branch extender and low-immediate duplex instructions; the builder
  concatenates **compiler-generated** pieces with mandatory Hexagon packet
  marker validation and never reads private OEM bytes during construction.
- Four adjacent semantic source files implement case-flag setup, diagnostic
  routing, and controller state checks, including the original packet-relative
  branch displacements.
- Total disjoint Agent 13 continuation `[file 0x4568c,0x476fc)` is now
  **8,304 byte-identical independently generated instruction bytes**.
  SHA-256 `b1d5e11ffe2273c02dcae1b0e27f7d73b820669dbf331ec84c27ebb2882d6f8f`.
- Original `wlanmdsp.mbn` checksum/revision and full strict reference byte
  comparison pass. This is **partial function coverage** only; the next
  missing packet starts file `0x476fc`, VMA `0xb00076fc`.
- Ancestry is still checkpoint16 (`7c730bf`), not checkpoint25 or the
  absent checkpoint50 integration. OEM signing, full-image reconstruction,
  device execution and source-level functional equivalence are not claimed.

## Checkpoint 51 (disjoint) — format-9 dispatch and session status paths

- Added **92 byte-identical instruction bytes** at WLAN file `[0x476fc,0x47758)`
  (VMA `[0xb00076fc,0xb0007758)`). The first eight bytes encode a signed
  far-branch extender and `r2=#9; jump` duplex, assembled independently from
  semantic source files and combined with checked Hexagon packet parse fields.
- The adjacent 84 bytes perform guard/status checks, format-specific
  diagnostic dispatch and callback preparation, with packet-relative call
  displacements authored explicitly. No embedded reference instruction words.
- Entire current disjoint continuation `[0x4568c,0x47758)` now totals
  **8,396 independently generated Hexagon bytes**, SHA-256
  `424a8c62fab80a810bff7f473838ab50f67f7d60156c3919a1211b8e403e8513`,
  with zero byte differences against the pinned reference. The current
  function is still in progress; next missing packet begins at `0x47758`.
- Branch ancestry still traces to checkpoint16, not missing checkpoints
  17–25; complete signed WLAN firmware and device execution are not claimed.
