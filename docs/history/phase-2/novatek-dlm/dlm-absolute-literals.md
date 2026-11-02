# Agent 20-19 — Absolute DLM address anchors in upper NDS32 ILM and EX9

This read-only audit checks the typed upper ILM and EX9 instruction-table
sources from Agent 18 checkpoint 02, without adding them or the OEM firmware
to Agent 19's Git branch. It extracts `Movi(register, SRAM_address)` for
SRAM `[0x20000,0x2c190)` (DLM file `[0x22000,0x2e190)`), converts the
value to a file offset by adding `0x2000`, validates EX9 slot/file geometry,
then looks up the location in Agent 19's canonical source-ownership ledger.

**Important:** a `Movi` address constant is **not** itself a memory access.
No register dataflow, indirect lookup or runtime operation is inferred from
these results. Existing GP read/write sites remain separately audited in
`docs/history/phase-2/novatek-dlm/dlm-consumer-audit.md`. The new report must not be used
to award source-byte coverage or infer startup initializer values.

| Quantity | Verified |
| --- | ---: |
| Upper-ILM DLM SRAM immediate loads | 28 |
| EX9 table DLM SRAM immediate loads | 6 |
| Total absolute DLM address references | **34** |
| Absolute-address loads pointing into `unknown_dlm` | 27 |
| Distinct unresolved file target locations | 17 |
| Address loads pointing into source-authored DLM structures | 7 |
| New independently authored DLM data bytes | **0** |

Representative unresolved *targets* include DLM header/configuration
addresses `0x22000`, `0x2200c`, `0x22060`, `0x220c0`, `0x22180`,
`0x22320`, `0x22360`, `0x223c0` and `0x22420`; the paired-bank
record address `0x28238`; and runtime tail profile addresses `0x2d5b0`,
`0x2d5d8`, `0x2d5e4`, `0x2d64c`. Addresses are evidence of compiled
pointer constants only. They are not evidence of initialized *contents*.

The parser rejects inconsistent EX9 slot/file offsets, repeated source-PCs,
missing or invalid DLM ledger ownership, incomplete snapshots, and altered
source-credit totals. The optional Agent 18 checkpoint-02 snapshot gate
requires 28 upper ILM and six EX9 matches, but can be disabled when
working on a newer independently verified Agent 18 source revision.

```sh
cargo run --offline --locked -p firmware-tools --bin novatek-dlm -- ledger > /tmp/dlm-ledger.json
python3 tools/audit-novatek-dlm-literal-anchors.py \
  --upper-ilm-rs /path/to/agent18/novatek_upper_ilm.rs \
  --upper-ex9-rs /path/to/agent18/novatek_upper_ex9.rs \
  --ledger /tmp/dlm-ledger.json --verify-checkpoint02-snapshot \
  --output /tmp/dlm-absolute-anchors.json
```

These results are **not** a reference-assisted recompile of any of the
27 unresolved static target values. Agent 19 source coverage remains 9,353
bytes, with 40,199 DLM bytes unresolved. Firmware reference verification
and all existing bank-mirror dependencies remain separate gates.
