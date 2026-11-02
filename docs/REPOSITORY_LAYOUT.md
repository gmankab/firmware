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
