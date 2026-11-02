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

### ADSP and CDSP semantic assembly sources

Source-authored ADSP RWE assembly now lives at
`sources/xiaomi-nabu/adsp/rwe/` (121 files) and CDSP late-controller
assembly at `sources/xiaomi-nabu/cdsp/late/` (51 files). The runnable
assemblers and verifiers remain under `tools/adsp/` and `tools/cdsp/`.
All 172 assembly files were renamed without changing their content; SHA-256
checks are recorded in `docs/maintenance/{adsp,cdsp}-assembly-relocation.json`
and enforced by `tests/test_assembly_source_ownership.py`.
Never add symlinks or duplicate firmware source into executable tool directories.

### Public entry points versus implementation tools

Only the stable public entry points remain directly in `tools/`:
`firmware.py`, `check-local.sh`, `low-memory-build.sh`, `verify.sh`, and
`verify-xiaomi-nabu.sh` (`__init__.py` maintains Python module loading).
All component-specific implementation tools live in the `tools/<component>/`
package/directory. Shared ELF and MBN inspection tools live under
`tools/analysis/`; metadata and A640 assemblers under `tools/build/`;
certificate, MBN and QRISC verifiers under `tools/verify/`.

The Hexagon V66 instruction encoder and ADSP entry/late emitters are now
co-located in `tools/adsp/`, while WLAN late packet parsing is in
`tools/wlanmdsp/`. The exact old-to-new paths and SHA-256 provenance are
recorded in:

- `docs/maintenance/common-tool-relocation.json` (five ELF/MBN utilities)
- `docs/maintenance/shell-tool-relocation.json` (four shell tools)
- `docs/maintenance/wlan-packet-parser-relocation.json`
- `docs/maintenance/adsp-tool-ownership.json`

`tests/test_public_tool_entrypoints.py` rejects stray implementation files
at the tools root and checks that no firmware assembly has returned under
`tools/`. Do not create alias scripts with obsolete locations: update callers
and their tests together.

### Avoiding unbounded reads and compilation freezes

For large A32 generators and reference binaries, inspect metadata before
reading content. Use the bounded helper instead of `cat`, whole-file regex
replacement or copying decompiler listings into a tool response:

```sh
python3 tools/analysis/bounded-inspect.py sizes path/to/large.rs
python3 tools/analysis/bounded-inspect.py lines path/to/large.rs 1200 60
python3 tools/analysis/bounded-inspect.py bytes /private/venus.mbn 0x4000 0x100
```

The helper reads at most 120 source lines or 4 KiB of binary data per call,
with a strict 64 KiB output cap. Use `rg -l` and `rg -n` with a targeted
file glob, and record large test/build output in bounded log files rather than
printing megabytes to the terminal. For multipart `.zst` archives stream the
parts into `tar -I zstd`, extracting only needed entries; never load the
compressed archive into Python memory. Keep Cargo to one job using
`tools/low-memory-build.sh`, disable debug information/incremental builds,
and do not overlap competing cargo/rustc processes under the 4 GiB cgroup.

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
