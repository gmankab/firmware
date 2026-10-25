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
