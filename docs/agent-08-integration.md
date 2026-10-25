# Agent 08 integration — CS35L41 HALO WMFW

**This is a standalone branch and a merge-ready patch series, not a replacement
for checkpoint 50 or for a compiler of proprietary HALO source.**

## Repository provenance

The requested checkpoint-50 bundle (`b7ed4b5`, Venus service region 33,872
bytes) was not mounted in this agent's session. The available repository
archive `recompile-firmware.bundle` instead pointed to main commit
`69ebf0d1c50859189a9196a21155c388ac914d7b`. All agent-08 commits were
made on branch `agent/08-cirrus-wmfw` from **that exact ancestor**, using
both author and committer `Codex <codex@openai.com>`.

**Do not merge the old `main` ref from the agent-08 bundle into checkpoint 50**;
instead replay the agent's commits with the provided
`agent-08-wmfw-changes-for-checkpoint50.mbox` patch series, or cherry-pick
commit IDs from `69ebf0d..agent/08-cirrus-wmfw` after fetching the bundle.
On checkpoint-50 sources the common CLI (`main.rs`) and module declarations
(`lib.rs`) may require minor conflict resolution. The agent's WMFW-specific
source names are `wmfw_plan.rs`, `wmfw_algorithms.rs`, `wmfw_halo_memory.rs`,
`wmfw_memory_map.rs`, `wmfw_control_ranges.rs`, `wmfw_program_patterns.rs`,
`wmfw_cs35l41_bus.rs`, `wmfw_sha256.rs`, and `wmfw_external_payload.rs`.

Example import on a clean checkpoint-50 checkout:

```sh
# Create an integration branch from checkpoint 50.
git switch -c integrate/agent-08
# git am applies agent patches in original Codex commit order.
git am --3way /path/to/agent-08-wmfw-changes-for-checkpoint50.mbox
# If shared CLI/module changes conflict, resolve manually and git am --continue.
cargo test --workspace --offline --locked
```

## Verified outputs

| Source | Independent bytes | Source status |
|---|---:|---|
| WMFW v3 header + eleven typed descriptors | 128 | Exact |
| Human-readable build information | 48 | Exact |
| HALO framework named controls | 948 | Exact |
| CSPL named controls | 2,100 | Exact |
| Event logger named controls | 760 | Exact |
| Seven XM/YM/PM packed DSP payloads | **0 / 30,252** | External only |
| Total file bytes independently generated | **3,984 / 34,236** | 11.64% |

The 30,252 external bytes are cryptographically pinned by **seven region SHA-256
values**; the full 34,236-byte repack equals the reference SHA-256
`d1bea27e41001024eb3878276292b5bcc0cda4484ffced0880e12c03ec9bac2c`
when those external payloads are supplied. Such a repack is **not** an
independent rebuild of HALO machine instructions or runtime behavior.

## Reproducible audit

```sh
# Supply the original privately held and exact-revision reference file.
PATH=/path/to/rust/bin:$PATH \
  tools/verify-agent08-wmfw.sh /private/cirrus/cs35l41-dsp1-spk-prot.wmfw
```

The audit runs offline Rust tests and release build, Python tests, checks WMFW
metadata and DSP memory ranges, builds a complete file in a *temporary*
directory from independently generated metadata plus explicitly extracted,
hash-validated proprietary segments, compares the full output and destroys
the temporary original firmware payload files. Do not commit the temporary
files or export the complete binary as independently authored source.

## Remaining independent decompilation work

The HALO packed program is **4,056 40-bit words** in file range
`0x278c–0x76c4`, with **no independently authored HALO opcode implementation**.
The other six DSP segments are XM/YM machine state/program data and are not
independently reconstructed. Linux supplies WMFW container definitions,
register layouts and memory-mapping rules, but not the required complete HALO
ISA decoder/encoder or Cirrus CSPL original algorithm implementation. A future
agent requires independently validated HALO ISA semantics and original
algorithm code (or a complete legitimate source build chain) before counting
these missing bytes as recompiled source. Byte-identical external repacking is
already available as a distinct, accurately labeled workflow.

## Checkpoint 08-12 — extraction changes

External-payload extraction now requires a **non-existing** destination folder
and commits only after writing and syncing all seven SHA-pinned inputs to a
private staging directory. Existing folders (including empty ones) and
symlinks are rejected. Failed extraction does not leave a partially populated
OEM payload directory. This is a protective behavior change to the external
repack workflow, not a reconstruction of proprietary HALO source.
