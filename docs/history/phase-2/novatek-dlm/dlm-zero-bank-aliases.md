# Agent 20-19 — chained DLM bank-copy windows (checkpoint 20)

**Classification:** reference-dependent **only**; no source-authored firmware
bytes gained, no inferred zero initializers, and no new NDS32 executable.

Read-only comparison of two original 196,608-byte Novatek revisions establishes
three identical, zero-valued DLM windows in corresponding memory banks. Both
independently pinned originals agree:

- postmarketOS SHA-256
  `8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5`;
- older map220v SHA-256
  `72364eca0a91855d59e043371aca774354e5ad0885df5024edcec8f976543ad2`.

Only postmarketOS is the **rebuild target**. The older revision is used solely
as independent corroboration that the identical-bank relationship is not
an accident of one firmware build. The implementation copies **externally
provided** earlier-bank bytes, never inserts a literal zero blob; a synthetic
nonzero-source test proves that changing external input changes both outputs.

| Externally required earlier-file span | Generated later-file span | Size | Dependence |
| --- | --- | ---: | --- |
| `[0x2271c, 0x2286c)` | `[0x2671c, 0x2686c)` | 336 | Reference-observed identical bank tail |
| `[0x22875, 0x22a00)` | `[0x26875, 0x26a00)` | 395 | Reference-observed identical remaining bank tail |
| `[0x262ec, 0x26a00)` | `[0x2a2ec, 0x2aa00)` | 1,812 | Larger repeated bank window |
| **Total** | | **2,543** | No independently generated source-credit delta |

The third source interval includes the destinations of the first two copies,
so it must be constructed **after** them. The sparse linker follows this
explicit topological order and fails on unpopulated primary ranges. Its
three destinations are nonoverlapping and do not collide with previously
implemented DLM bank transforms, authored Gaussian/index fragments, or the
checkpoint-19 narrow relocation. The apparent zeros in the two originals
are **not** sufficient evidence to mark them as independently generated
source data or compiler-generated padding.

## Provenance and external-seed reduction

| DLM category | Checkpoint 19 | Checkpoint 20 | Delta |
| --- | ---: | ---: | ---: |
| Independently authored | 9,353 | 9,353 | 0 |
| Dependently generated | 10,441 | 12,984 | +2,543 |
| Explicitly external | 29,758 | 27,215 | −2,543 |
| Required seed ranges | 58 | 60 | +2 |
| **Total DLM** | **49,552** | **49,552** | **0** |

There are still **40,199 DLM bytes without independent source
reconstruction**, although conditional exact rebuilding requires only 27,215
externally supplied bytes. The DLM mirror is separately validated but never
counted twice. The first missing *independent-source* byte remains at file
`0x22000` / SRAM `0x20000`.

## Reproduce from private pinned firmware inputs

```sh
cargo test --workspace --offline --locked
cargo build --workspace --release --offline --locked
python3 -m unittest discover -s tests -p 'test_*.py'

./target/release/novatek-dlm sparse-plan
./target/release/novatek-dlm verify-sparse-rebuild /private/postmarketos/novatek_nt36523_fw.bin
./target/release/novatek-dlm verify-zero-bank-aliases /private/postmarketos/novatek_nt36523_fw.bin
python3 tools/novatek/verify-novatek-dlm-zero-bank-aliases.py /private/postmarketos/novatek_nt36523_fw.bin
python3 tools/novatek/verify-novatek-dlm-zero-bank-aliases.py /private/map220v/novatek_nt36523_fw.bin
```

The Python model reconstructs these relationships independently of the Rust
source and intentionally rejects truncated, mispaired and modified data.
Git contains neither OEM DLM seeds nor complete reference firmware images.
This branch remains rooted in the **older** merged preview and still needs
a verified checkpoint-50-rooted integration replay.
