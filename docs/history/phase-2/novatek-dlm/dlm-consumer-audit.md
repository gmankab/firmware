# Agent 20-19 — DLM GP/EX9 consumer map

This audit is a **read-only cross-agent analysis**, not a recovered firmware
algorithm and not an expansion of independently authored DLM bytes. It joins
Agent 18 checkpoint 02's typed NDS32 upper-ILM operations with Agent 19
checkpoint 09's explicit DLM provenance ledger. Neither original payload nor
Agent 18's source is copied into the Git branch.

## Reproduction

```sh
cargo run --offline --locked -p firmware-tools --bin novatek-dlm -- ledger > /tmp/dlm-ledger.json
python3 tools/novatek/audit-novatek-dlm-consumers.py \
  --upper-ilm-rs /path/to/agent18/novatek_upper_ilm.rs \
  --upper-ex9-rs /path/to/agent18/novatek_upper_ex9.rs \
  --ledger /tmp/dlm-ledger.json --verify-checkpoint02-snapshot \
  --output /tmp/agent19-consumers.json
```

The scanner verifies each NDS32 operation's mapped SRAM address using the
postmarketOS GP base `0x2d9e0` and the `+0x2000` DLM SRAM-to-file delta.
Widths are 1, 2 or 4 bytes for known GP operations, and each `EX9.IT` is
expanded against the supplied Agent 18 source table. Unknown GP operations,
truncated accesses, missing EX9 entries, duplicate instruction addresses and
ledger overlap or provenance tampering are rejected. The optional pinned
checkpoint-02 gate rejects a shortened input with silently missing accesses.

## Verified scope and uncredited observations

| Measurement | Count |
| --- | ---: |
| GP access instruction sites intersecting the DLM | 79 |
| Sites through expanded EX9 slots | 10 |
| Read sites | 45 |
| Write sites | 34 |
| Distinct DLM file locations | 37 |
| 8-bit / 16-bit / 32-bit access sites | 35 / 23 / 21 |
| Sites touching previously source-authored DLM fields | 17 |
| Sites touching still-unknown DLM fields | 62 |
| Independently authored DLM bytes **added by this audit** | **0** |

There are 24 unique **unresolved read addresses**, including:

- DLM packet/header fields: `0x22002`, `0x22003`, `0x2202f`,
  `0x22032–0x22034`, `0x2209a`, `0x22120`, `0x22126–0x22129`,
  `0x22194`, `0x223e8`.
- A calibration/control offset: `0x2d5ac`.
- Tail globals: `0x2e102–0x2e10c` (six distinct halfword addresses),
  `0x2e128`, `0x2e178`, and `0x2e184`.

The branch at ILM `0x10aa8/0x10aaa` writes literal `4` to DLM file
`0x2244a`, and `0x11c06/0x11c1a` can write `1` to file `0x2e128`;
both static reference fields match those values. **Neither store is an
unconditional startup initializer.** The first is conditional packet parsing
with an alternate code `0x14`; the second is a command/event branch. These
bytes remain `unknown_dlm`. Runtime writes do not prove source-owned static
initializers, a central distinction from the four reset-compatible fields
accepted in checkpoint 07.

The direct GP map is **not** a full memory-reference graph. Indirect pointer
arithmetic (`Movi` + indexed loads), unimplemented ILM instructions and runtime
firmware activity are out of scope; generated counts must not be interpreted
as total read/write traffic. Neither source-level compiler behavior nor
hardware semantics are thereby established.

## Next reconstruction targets

Source-derive static values only when the consuming ILM routine establishes
more than a runtime mutation, e.g. a reproducible startup default or a
mathematically specified parameter table. In priority order investigate the
unresolved `0x2e102–0x2e10c` halfwords, `0x22000–0x22194` header fields,
and the `0x2e184` counter whose initial value remains unproven. Preserve
separate `source_dlm_data`, `unknown_dlm`, `derived_mirror`, and externally
supplied records in every checkpoint ledger.
