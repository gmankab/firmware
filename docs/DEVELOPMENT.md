# Development and verification

## Structure and code boundaries

The root manifest is a virtual [Cargo workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html) with two members:

- `crates/nabu-firmware-research` — CLI, firmware parsing, source-level reconstruction of the JSON configuration, structured ELF and WMFW metadata generators, and A640 container analysis.
- `crates/nabu-qrisc` — reusable typed QRISC word encoding, symbolic assembler, conservative control-flow validator and A640 routine definitions.

The specialized `experiments/hexagon-probe` is deliberately **excluded** from ordinary workspace builds. It remains useful for later Hexagon V66 investigation but requires the corresponding Rust source, `core` libraries and vendored compiler dependencies. Download scripts and legacy decompilation reports have been removed from Git; the private research corpus stays separate.

The root `Cargo.lock` is committed. Add new reusable crates under `crates/` and list them in the root `[workspace]` manifest. Use dependency paths rather than copying modules between packages.

## Verification commands

With the reference postmarketOS APK extracted to your own firmware directory (the `cargo fmt` command additionally needs the `rustfmt` component installed):

```sh
cargo fmt --all --check
cargo test --workspace --offline --locked
cargo build --workspace --release --offline --locked
python3 -m unittest discover -s tests -p 'test_*.py'

# Complete text configuration:
cargo run -p nabu-firmware-research --offline --locked -- \
  verify-modemuw /path/to/firmware/qcom/sm8150/Xiaomi/nabu/modemuw.jsn

# A640 security-control routines and possible branch/call edges:
cargo run -p nabu-firmware-research --offline --locked -- \
  verify-a640-routines /path/to/firmware/qcom/sm8150/Xiaomi/nabu/a640_zap.mbn
cargo run -p nabu-firmware-research --offline --locked -- \
  inspect-a640-flow /path/to/firmware/qcom/sm8150/Xiaomi/nabu/a640_zap.mbn

# External package provenance; do not commit extracted payloads:
(cd /path/to/firmware && sha256sum -c /path/to/repo/reference/firmware.sha256)
```

To compare the synthetic QRISC demo against Mesa's public assembler, supply the tool and its register database:

```sh
tools/verify-qrisc-against-mesa.sh /path/to/qrisc-asm \
  /path/to/mesa/share/freedreno/registers
```

`tools/decompile-all.sh` and `tools/analyze-a640-mesa.sh` can regenerate **private**, OEM-derived analysis outside the source checkout. Their output must not be merged into GPLv3 Rust code or checked into history without a separate rights assessment.

## QRISC design notes

- `codec` converts documented, typed fields to and from 32-bit little-endian QRISC words. Lossless bit-field roundtrips do **not** prove that the device behavior is understood.
- `program` resolves symbolic branch and call targets, checks relative displacements and emits a program deterministically.
- `a640` describes the entry/security-dispatch prefix and the idle routine through structured instruction constructors. This code remains an unfinished, analysis-informed reimplementation.
- `flow` follows possible QRISC branches/calls and blocks transfers into a separately identified IR3 data range. It does not simulate secure GPU operations, registers, scheduling or memory effects.

Use semantic names for known public registers and mark unknown fields explicitly. Do not add vendor opcode arrays or an OEM-derived IR3 payload to the public source. New output matches should have reference comparisons and synthetic unit tests.

## Git authorship and evidence

Every commit in the exported history, including the nine rewritten historical commits, uses:

```sh
git config user.name codex
git config user.email codex@localhost
```

History rewriting changes commit hashes; existing clones should start from the new bundle or carefully migrate branches. The author and committer fields are repository metadata and **do not change underlying copyright ownership, legal provenance, or past factual authorship**.

The old `evidence/` directory is removed. Useful stable information is condensed into the documentation and `reference/firmware.sha256`; stale toolchain download logs, duplicate status reports and proprietary pseudocode are not retained in the public tree. Store disassemblies, source APKs, reference firmware, private assembler reproductions and signatures outside Git.

GPL-3.0-only applies to the original code written for this repository, not to OEM executable bytes. Reimplementation informed by reverse engineering may still require a legal review before redistribution or use in Fedora/Pocketblue.
