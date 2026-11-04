# Repository migration evidence

Current implementation paths are described in [Repository layout](../REPOSITORY_LAYOUT.md).
This directory retains verified **one-way migration ledgers**. Its JSON files
record where source files were before a reorganization; historic names are
provenance, **not supported imports or command-line locations**.

For bounded archive inspection, low-memory compilation, ChatGPT session
resets, and checkpoint recovery, see [Long-running firmware sessions](long-running-sessions.md).

## Source and tooling migrations

| Scope | Ledger | Validation |
| --- | --- | --- |
| Original contributor-specific filenames | `path-migrations.json` | `tests/test_repository_layout.py` |
| Initial incomplete-history recovery | `RECOVERY-CHECKPOINT.md`, `pending-path-migrations.json` | Historical only; not the current module tree |
| Rust source modules | `xiaomi-nabu-module-paths.json` | Rust workspace compilation and tests |
| Initial modem assembly and Novatek data | `assembly-paths.json` | `tests/test_firmware_assembly_sources.py` |
| ADSP assembly | `adsp-assembly-relocation.json`, `adsp-assembly-paths.json` | `tests/test_assembly_source_ownership.py` |
| CDSP assembly | `cdsp-assembly-relocation.json` | `tests/test_assembly_source_ownership.py` |
| Device build entry points | `build-entrypoint-relocation.json` | `tests/test_build_entrypoint_ownership.py` |
| CDSP verification utilities | `cdsp-helper-relocation.json` | `tests/test_verifier_component_ownership.py` |
| Novatek verification utilities | `novatek-verifier-relocation.json` | `tests/test_verifier_component_ownership.py` |
| Shared MBN signing and hash inspection | `mbn-verifier-relocation.json` | `tests/test_mbn_verifier_ownership.py` |
| ELF/MBN build and analysis utilities | `common-tool-relocation.json` | `tests/test_common_tool_layout.py` |
| Decompilation and QRISC shell utilities | `shell-tool-relocation.json` | `tests/test_shell_tool_layout.py` |
| WLAN V66 packet parser | `wlan-packet-parser-relocation.json` | `tests/test_wlan_packet_tool_location.py` |
| ADSP shared V66 encoder and emitters | `adsp-tool-ownership.json` | `tests/test_adsp_tool_ownership.py` |

Dated `docs/history/**/progress.md` reports are being retired from HEAD:
Git history retains their exact bytes, and the `retired_historical_files`
SHA-256 ledger distinguishes archived destinations from missing migrations.
Technical research and verification evidence remain in the working tree.

To prevent another proliferation of wrapper scripts, new command-line
entry points should live in the owning firmware/tool directory. Only stable
public entry points remain directly under `tools/`. Changes must update actual
call sites and regression tests, not create symlinks or backwards-compatibility
copies under the old name.

If a source file must move again, add an immutable before/after hash record.
If executable source text must change because its root path moved, distinguish
original-source hash from installed-source hash. Keep reference OEM firmware
outside Git, and never imply that a matching container using external vendor
payload is independently reconstructed code.

For CI-like checks run `./tools/check-local.sh`; pass `--full` to run the
memory-bounded Rust workspace tests and release-profile build. The project
intentionally does **not** use GitHub Actions.
