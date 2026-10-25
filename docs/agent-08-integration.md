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

## Checkpoint 08-13 — sparse Y-memory zero-run serialization

An alternative whole-file build consumes eight private external inputs totaling
24,807 bytes, regenerating 5,445 WMFW Y-memory zero bytes from a checked DSP
word interval `223..2038`. The original seven-payload importer is unchanged.
This zero interval was identified by comparing against the original and is
**not** independently recovered DSP program source; independent metadata
coverage remains 3,984 bytes. Run `extract-wmfw-sparse-y-inputs` and
`assemble-wmfw-with-sparse-y` for the reduced external input path. The full
SHA-256 and each external fragment hash must match the pinned reference.


## Checkpoint 08-14 — optional two-run packed-Y compaction

The compact-Y builder (`wmfw_compact_y.rs`) preserves the earlier seven-whole
and eight-sparse-input APIs, but adds an optional nine-input packaging mode:
3,984 independent metadata bytes, 5,748 reference-confirmed zero bytes, and
24,504 externally supplied DSP bytes. The zero-run positions were inferred
from the exact reference and are not HALO source recovery. The audit script
exercises all three modes, including deliberate source-corruption rejection.


## Checkpoint 08-15 — runtime HALO algorithm directory decoder

Added a strict `wmfw_halo_registry.rs` parser for **unpacked, big-endian,
32-bit** device register snapshots. It resolves named control offsets only
when the actual algorithm directory is supplied; the original packed WMFW
does not itself provide this trusted runtime registry. Synthetic fixtures
in the audit confirm 57 controls and reject malformed boundaries. There is
no increase in independently generated firmware bytes.


## Checkpoint 08-16 — non-executable loader-placement image

Added `wmfw_loader_image.rs` with a checked sparse word map of all seven
packed WMFW DSP regions; only 1,280 of the 1,452 declared X words are
initialized, while Y and P are fully initialized. The model re-encodes
*externally supplied* XM/YM/PM bytes and bounds 32-bit AHB write spans.
No HALO instructions are decoded or generated, and it cannot fill absent
file payloads without external input. `simulate-halo-wmfw-load` is included
in the audit script and uses the real reference supplied by the user.


## Checkpoint 08-17 — integration byte ledger

The `wmfw-byte-provenance` CLI emits complete JSON provenance covering all
34,236 source-file bytes, with explicit categories for authored metadata,
reference-observed zero-run serialization, and external proprietary input.
No byte is classified as independently decompiled HALO code. The audit
script validates the ledger and all three repackaging paths. Integration
should use these counts instead of adding external payloads to the author's
independent source-derived byte count.
