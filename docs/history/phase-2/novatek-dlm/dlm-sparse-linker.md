# Agent 20-19 — Sparse conditional Novatek DLM assembler

## Precise scope

This is an **executable build path** for the entire **49,552-byte** DLM region
`[0x22000,0x2e190)` of the pinned postmarketOS Novatek Nabu image (SHA-256
`8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5`).
It does **not** recover unknown firmware algorithm code or proprietary
calibration bytes. An exact output can be produced only if **30,522 bytes** of
external primary DLM input are separately supplied. No raw OEM bytes are
compiled into the Rust/Python implementation or checked into this branch.

| Mutually exclusive physical category | Bytes | Source ownership |
| --- | ---: | --- |
| Independently generated index schedules, Gaussian coefficients, pointers and state | **9,353** | Reconstructed source DLM data |
| Reconstructed from earlier *externally supplied* DLM values by checked bank relocations and copies | **9,677** | Dependency only; **not** independent source |
| Externally required seed bytes, in **60 exact disjoint ranges** | **30,522** | Not independently reconstructed |
| **Complete DLM output, conditional on all external seeds** | **49,552** | Not a complete firmware image |

Checkpoint 16 adds the independently checked **96-byte record suffix**
relationship: 23 record pairs have equal bytes +9..+95, while the first
nine bytes remain externally supplied. The original two other records
already fall inside the seven earlier alias spans and are excluded to avoid
double accounting. These suffixes are **reference-dependent data copies**;
the controller command semantics of their primary fields are not proven.

Checkpoint 17 additionally copies the first 24 bytes and last nine bytes of
30 paired **40-byte descriptor records**; seven intervening bytes in each
record differ between banks. The relationship adds 990 dependent bytes.
Checkpoint 18 now derives **all seven mutable middle bytes** of each upper
record from the externally supplied lower-record values, using checked
16-bit index increments of `0x10`, `0x64`, or `0xc8` (selected by record
position) and a conditional selector flag bit on records 12–19. There are
**210 additional reference-dependent bytes**, 0 new authored bytes. Every
one of the 30 transforms matches the pinned reference, and synthetic
overflow/malformed-selector tests fail closed. Controller semantics remain
unproven, so this is not source authorship.

The descriptor relocation source operation touches **2,940 target bytes**, but
**40** are already covered by independently generated schedule-link fields.
Those 40 are checked for equality with the independently derived relocation,
not counted a second time. The dependency total consists of:

| Dependent transform destination | Physical bytes | Overlap with source | Unique dependent |
| --- | ---: | ---: | ---: |
| 21 relocated `0x8c`-byte descriptor records | 2,940 | 40 | 2,900 |
| Seven exact lower/upper bank aliases | 3,136 | 0 | 3,136 |
| 23 repeated 96-byte-record suffixes, 87 bytes each | 2,001 | 0 | 2,001 |
| 30 lower/upper paired 40-byte records (24-byte head plus nine-byte tail) | 990 | 0 | 990 |
| 30 dependent 40-byte descriptor mutable fields (seven bytes each) | 210 | 0 | 210 |
| Duplicated lower calibration/profile block | 428 | 0 | 428 |
| Repeated 12-byte profile signature | 12 | 0 | 12 |
| **Total** | **9,717** | **40** | **9,677** |

Existing bank aliases, calibration/profile copies, 96-byte record suffixes and descriptor relocations
were previously independently tested as *relationships*, not source
reconstruction. The new `novatek_dlm_sparse` builder composes them to
produce a finished **DLM payload** while rejecting missing or overlapping
externally supplied material, index relocation overflow, and disagreement
between authored and derived values.

## Reproduction (private original stays outside Git)

```sh
export CARGO_TARGET_DIR=/tmp/agent19-build
cargo test --workspace --offline --locked
cargo build --workspace --release --offline --locked
python3 -m unittest discover -s tests -p 'test_*.py'

# Public metadata only; does not read a firmware reference or include OEM bytes.
"$CARGO_TARGET_DIR/release/novatek-dlm" sparse-plan > /tmp/novatek-dlm-seed-plan.json

# Compare an in-memory build against an original image you already possess;
# this uses 30,522 private external bytes and does not export them.
"$CARGO_TARGET_DIR/release/novatek-dlm" verify-sparse-rebuild /private/novatek_nt36523_fw.bin

# Independent Python verification against the same original/typed fragments.
"$CARGO_TARGET_DIR/release/novatek-dlm" emit /tmp/agent19-authored-fragments
python3 tools/verify-novatek-dlm-sparse.py \
  --reference /private/novatek_nt36523_fw.bin \
  --emitted-dir /tmp/agent19-authored-fragments \
  --sparse-plan /tmp/novatek-dlm-seed-plan.json
```

For a private build without the original full reference file, place the 60
**externally supplied** seed blobs at the exact ranges listed by `sparse-plan`
using filenames `dlm-external-<six-digit-start>-<six-digit-end>.bin`, then run:

```sh
"$CARGO_TARGET_DIR/release/novatek-dlm" assemble-sparse /private/seed-directory /private/novatek-dlm-only.bin
```

The command strictly requires every seed at its exact length and position,
rejects unexpected seed blob names, and refuses data that conflict with the
independent 40-byte cross-checks. It does not require a copy of the original
whole firmware to execute. **Do not publish reference-derived seeds or the
assembled payload as newly compiled independent source.** In particular,
`0x2e190` is the DLM payload end, not a complete/flashable NVT image end;
independent ILM source, metadata/vector values, stored CRCs, and device
behavior remain separate integration tasks.

## Verification/evidence

- Rust and an independently written Python linker rebuild the exact DLM with
  zero differences against the SHA-256-pinned original, given only generated
  source fragments and private seed values; neither reads private firmware
  during the builder's normal `sparse-plan` or `assemble` paths.
- Python compares the complete 60-range plan, counted source vs dependent
  bytes, and all 40 cross-checked bytes to Rust output; its test suite includes
  negative conditions for missing/reordered seeds, malformed pointers and
  source-vs-dependent conflicts.
- A synthetic all-zero seed test illustrates that relocations are genuinely
  executed (21 records × 9 pointer fields are incremented) rather than
  copying the later bank's original bytes.
- The preexisting source provenance ledger is not changed. It still records
  **9,353 independently generated DLM bytes** and **40,199 unresolved
  original DLM bytes**, of which 9,677 can now be built conditionally from
  earlier externally supplied unknown values. This is not source credit.

## Dependencies and merge notes

This checkpoint derives from the previously saved Agent 19 checkpoint 14,
which itself is based on the *older-base* integrated preview `941c805…`, not
checkpoint 50 `b7ed4b5…`. Keep this as a portable branch, apply it to the
verified newer integration only after the assigned overlap/ownership checks,
and rerun all gates on the actual integrated HEAD.
