# Agent 20-19 — Reference-dependent DLM bank-copy map

The pinned postmarketOS Novatek firmware contains seven **exact-copy**
dependencies from an unknown lower-bank DLM region to an upper-bank region
exactly `0x4000` bytes higher in file address. The initial upstream content
must be supplied externally; matching the second bank is **not independently
recovered firmware source**. Existing source-generated packed-index schedules,
Gaussian kernels, descriptor-bank relocated word indices and repeated 428-byte
profile are outside these seven intervals. Both sides of every interval remain
`unknown_dlm` in the canonical Agent 19 ledger.

| Primary DLM file range | Dependent DLM file range | Bytes | Label |
| --- | --- | ---: | --- |
| `[0x2387c,0x23a12)` | `[0x2787c,0x27a12)` | 406 | codec-bank-staging |
| `[0x23df8,0x23edc)` | `[0x27df8,0x27edc)` | 228 | format-index-scratch |
| `[0x24186,0x2423c)` | `[0x28186,0x2823c)` | 182 | frame-bank-defaults |
| `[0x245a6,0x24904)` | `[0x285a6,0x28904)` | 862 | pre-wide-schedule |
| `[0x24fc4,0x25220)` | `[0x28fc4,0x29220)` | 604 | post-wide-schedule |
| `[0x2574c,0x259b4)` | `[0x2974c,0x299b4)` | 616 | input-ring-metadata |
| `[0x25c5e,0x25d4c)` | `[0x29c5e,0x29d4c)` | 238 | pre-narrow-schedule |
| **Total** | | **3,136** | **0 new independent bytes** |

The names are descriptive placeholders for observed buffer regions, **not**
verified device-controller semantics. The Rust module exposes checked
`duplicate_from_external_primary` and `verify_reference_assisted_aliases`,
which require the full externally supplied input or caller-provided primary
bytes. `assert_uncredited_on_ledger` fails if either alias overlaps source
coverage. The separate Python model enforces the same bounds and copy
relationships. The main reference verifier now runs this check too.

```sh
cargo run --offline --locked -p firmware-tools --bin novatek-dlm -- \
  verify-bank-aliases /private/novatek_nt36523_fw.bin
python3 tools/verify-novatek-dlm-bank-aliases.py \
  /private/novatek_nt36523_fw.bin --ledger /tmp/dlm-ledger.json
```

**Critical distinction:** 3,136 bytes can be reproduced *conditional on 3,136
externally supplied primary bytes*. Both sides remain unknown source; the
ledger does not gain 1,432 bytes, and the image cannot be compiled wholly from
independent source by performing these copies. This is useful future
relocation/copy knowledge, not algorithm or signing recovery.
