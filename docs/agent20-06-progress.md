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
