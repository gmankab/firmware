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
