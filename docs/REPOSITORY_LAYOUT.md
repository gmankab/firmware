# Repository layout and maintenance rules

The repository groups firmware tools by device, purpose and architecture.
Contributor names and checkpoint numbers are not implementation identifiers.

| Purpose | Location |
| --- | --- |
| ARM32/QRISC/IR3 encoder crates | `crates/arm32`, `crates/qrisc`, `crates/ir3` |
| Firmware formats and builders | `crates/firmware-tools` |
| Low-memory Venus controllers and tail | `crates/venus-controller`, `crates/venus-tail` |
| Source firmware assembly | `sources/xiaomi-nabu/<firmware>/` |
| Device-specific tooling | `tools/{adsp,cdsp,modem,wlanmdsp,venus,halo,novatek}` |
| Common firmware tooling | `tools/build`, `tools/verify`, `tools/analysis`, `tools/integration` |
| Firmware provenance and exact reference hashes | `reference/xiaomi-nabu/` |
| Regression tests | `tests/test_<firmware>_<function>.py` |
| Historical reports/evidence | `docs/history`, `docs/evidence`, `verification` |

### Function-bounded Venus tail source

`crates/venus-tail/src/venus_tail_program.rs` keeps the public module namespace
and common ARM32 emitter. Its 92 ordered `include!` sections in `parts/` are
split at complete Rust item boundaries, without changing the emitted firmware
instruction stream. `parts/manifest.json` records the exact original
63,602-line source SHA-256 and each chunk SHA-256. The regression test
`tests/test_venus_tail_slices.py` reconstructs the original source bytes and
validates the inclusion order. All 64 Venus-tail unit tests and the direct
124,332-byte reference comparison pass after the split.

### Assembly and device metadata ownership

The 140 Qualcomm modem `.s` fragments now live in
`sources/xiaomi-nabu/modem/`, rather than among Rust module files. Their
original filenames and bytes are unchanged. The Novatek DLM seed table is
co-located with `novatek/dlm/novatek_dlm_seed_manifest.rs`, preserving the
original `include_str!` lookup. The one-to-one source migration and SHA-256
ledger is `docs/maintenance/assembly-paths.json`; the regression test
`tests/test_firmware_assembly_sources.py` checks every original byte.

Source is kept in the owning crate. The original A32 instruction encoders and
source-derived data are not duplicated. Keep new APIs narrow and prefer
`pub(crate)` for internal functions. Preserve the architecture's memory map,
literal pools and data alignment, and verify every generated region against
the pinned original reference before claiming byte equality.

`docs/maintenance/path-migrations.json` records 1,029 migrated paths and the
16 unavailable or superseded historical paths from the recovered artifacts.
Do not add filename aliases or symlinks for the old contributor-specific names.

Use `./tools/check-local.sh` for fast offline quality gates, and
`./tools/check-local.sh --full` for the complete low-memory workspace build.
No GitHub Actions are used.
