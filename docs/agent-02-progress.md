# Agent 02 — Venus boot/service bridge

This branch reconstructs only Venus ARM A32 runtime addresses `[0x8490, 0xaa0c)`;
the MBN file offset for these instructions is runtime `+0x4000`. It does **not**
rewrite the forward service-region modules owned by Agent 01. Missing bytes
are never filled with reference slices or artificial padding.

## Base and integration note

Requested baseline: checkpoint 50 (`b7ed4b5`). That bundle was not mounted
in the Agent 02 environment. The available `recompile-firmware.bundle`
contains boot-prefix commit `1ef3787` and current parent `69ebf0d`. Its
verified 33,936-byte boot/runtime prefix is the same initial prefix described
at checkpoint 50. The changes here are deliberately isolated: merge them
onto checkpoint 50 with `git cherry-pick`, resolving only the addition of
`mod.rs` registration. Do **not** treat this branch as a direct descendant
of the missing checkpoint-50 commit.

## Checkpoint 01 — scoped register/table access (`0x8490–0x883c`)

- New separately authored ARM32 fragment: **940 consecutive bytes**, runtime
  `[0x8490,0x883c)` / file `[0xc490,0xc83c)`; 231 instructions and **four**
  clearly identified literal words.
- Functions: name-hash lookup and bounded index traversal, pointer resolution,
  compatibility filtering, scoped lookups, and record selection in a section.
- Source module: `venus_runtime_bridge_lookup.rs`, with the strict layout
  and boot-prefix concatenation in `venus_runtime_bridge.rs`.
- Full boot+bridge checked region: **34,876** consecutive bytes beginning at
  file `0x4000`; gap remaining until existing service start `0xaa0c` is
  `[0x883c,0xaa0c)`, **8,656 bytes**.
- No proprietary instruction bytes embedded in source; literal data words are
  `0x000e359a`, `0x000e35bd`, `0x000e35ea`, `0x001039ec`.
- Branch destinations into code outside this fragment are preserved, not
  executable in a standalone fragment. The fragment cannot be flashed.

### Reproduce exact verification

```sh
cargo run --offline --locked -p firmware-tools --bin venus_bridge -- \
  emit /tmp/venus-bridge.bin
cargo run --offline --locked -p firmware-tools --bin venus_bridge -- \
  verify /private/venus.mbn
cargo run --offline --locked -p firmware-tools --bin venus_bridge -- \
  verify-joined /private/venus.mbn
python3 tools/agent-02-a32-roundtrip.py /private/venus.mbn \
  --file-offset 0xc490 --vma 0x8490 --length 0x3ac \
  --data-range 0x8614:0x8618 --data-range 0x8684:0x8688 \
  --data-range 0x8834:0x883c
```

For all further checkpoints, continue at **exactly** `0x883c` and add only
consecutive verified instructions or named data words to bridge layout.

## Checkpoint 02 — record classification and queue state (`0x883c–0x8c00`)

- Adds **964** exact bytes (226 ARM32 instructions and 15 non-executable
  words: 8 format jump targets, 6 error/context words, 1 queue-head address).
- New region module: `venus_runtime_bridge_records.rs`, with record-header
  validation, type dispatch, attribute extraction and queue-state access.
- **Cumulative bridge:** 1,904 contiguous bytes, runtime `[0x8490,0x8c00)`,
  file `[0xc490,0xcc00)`; independently regenerated early boot+bridge:
  **35,840 contiguous bytes** (`0x4000–0xcc00` in the MBN).
- **Remaining gap:** `[0x8c00,0xaa0c)` = **7,692 bytes**. The original
  reference is only read by the opt-in verification command.

```sh
cargo run --offline --locked -p firmware-tools --bin venus_bridge -- verify /private/venus.mbn
cargo run --offline --locked -p firmware-tools --bin venus_bridge -- verify-joined /private/venus.mbn
python3 tools/agent-02-a32-roundtrip.py /private/venus.mbn \
  --file-offset 0xc83c --vma 0x883c --length 0x3c4 \
  --data-range 0x899c:0x89bc --data-range 0x8b94:0x8bac \
  --data-range 0x8bd8:0x8bdc
```
