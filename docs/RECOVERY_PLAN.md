# Firmware recovery and throughput plan

Updated: 2026-11-07.

## Goal and accounting

Produce byte-identical source builds for all 19 pinned Xiaomi Nabu firmware files. Do not count copied reference payloads, opaque opcode arrays, or unverifiable derivations as recovered source.

Track three separate outcomes:

1. Verified byte coverage from reviewed source emitters.
2. Semantic recovery: understood algorithms, data structures, and behavior, distinct from typed instruction transcription.
3. Complete images reproduced from source.

The last committed, verified checkpoint is `9e2adeda`: 956,689 of 35,409,209 bytes (2.7018%), with six complete-source images. Venus accounts for 646,504 bytes and GMU for 2,584 bytes. The working ledger includes another 960 GMU bytes, but the parent verification was interrupted; do not treat that increment as verified yet.

The previous 167-minute run added a net 74,955 verified bytes, or 0.2117 percentage points (0.0761 percentage points per hour). This is byte coverage, not a measure of semantic recovery. There is no measured breakdown of compiler, analysis, or coordination time and no demonstrated speedup for the changes below.

## 1. Recover interrupted work before adding new work

Preserve existing changes, including user-owned configuration. Lost background tasks are not assumed to remain running, and their lost validation results are not proof of success.

Recover these outputs in order:

| Work | Runtime range | State and next action |
| --- | --- | --- |
| GMU stream operations | `[0xc38, 0xff8)` | Modified existing module; independently review and rerun focused and authenticated reference tests before crediting 960 bytes. |
| Venus buffer acquire | `[0x7f7bc, 0x7fd1c)` | Inspect persisted untracked emitter; verify completeness and exact output. |
| Venus buffer accessors | `[0x7fd1c, 0x804fc)` | Same recovery and verification gate. |
| Venus dispatch allocator | `[0x804fc, 0x80c18)` | Same recovery and verification gate. |
| Venus runtime utilities | `[0x80c18, 0x817b8)` | Same recovery and verification gate. |
| Venus VPX probability initialization | `[0xc65ac, 0xc6e54)` | Reported isolated success; still requires parent review, integration, and verification. |
| Venus VPX context tail | `[0xc6e54, 0xc77a4)` | Inspect persisted output; prior task completion is unknown. |

The four utility ranges total 8,188 bytes; the two VPX ranges total 4,600 bytes. Neither group is currently integrated or credited. Inspect actual files before relying on these planned extents.

Use runtime/file mappings explicitly: Venus file offset is runtime plus `0x4000`; GMU's main code segment uses runtime minus `0x260`, not a universal mapping for every segment. Preserve the A32/Thumb boundary at Venus runtime `0x818a0`; do not extend a decoder mode across it without evidence.

## 2. Cache analysis and prepare bounded work packets

Create reusable analysis artifacts keyed by the authenticated image hash, tool/version, processor mode, load mapping, and analysis configuration. Reuse an existing cache location where possible; do not commit reference firmware or large generated analysis dumps.

Each bounded work packet should contain:

- Exact runtime and file extents, entry points, and processor modes.
- Decoded instructions, control-flow edges, cross-references, and available decompiled C.
- Reviewed code/data boundaries, literal pools, jump tables, alignment, and unresolved ambiguities.
- Expected output extent and the focused comparison command.

Use a persistent whole-image analysis database and export packets from it. Reanalyze ambiguous regions rather than repeatedly decoding unchanged ranges. Earlier work primarily used Capstone/cstool disassembly, not whole-image Ghidra decompilation; establishing a decompiler cache remains work to do.

Decompiled C is supporting evidence, not a byte-identity recipe. Translating C to Rust does not preserve the original compiler's instruction selection, register allocation, or layout automatically.

## 3. Automate mechanical translation without hiding bytes

Pilot an authenticated-bytes → reviewed code/data map → decoded IR → typed semantic encoder pipeline on representative, already understood functions.

Requirements:

- Reject unsupported instructions; never fall back to opaque instruction words or reference copying.
- Preserve flags, predication, writeback, wrapping arithmetic, cold blocks, literal pools, jump tables, and mode transitions.
- Keep encoder semantics review separate from generated call-site review.
- Compare exact emitted bytes against the authenticated reference.
- Label typed transcription separately from high-level semantic recovery.

Expand automation only after the pilot passes independent review and demonstrates useful accepted output per minute.

## 4. Use exact-model subagents with bounded ownership

Exact-model dispatch was smoke-tested successfully for all three models below. These tests establish dispatch only, not comparative quality, latency, or backend identity attestation.

| Exact Maki model spec | Initial assignment |
| --- | --- |
| `openai/gpt-6.1-sol` | Reverse engineering, ambiguous mode/data boundaries, shared encoder semantics, independent correctness review. |
| `openai/gpt-6.1-terra` | Prepared, bounded implementation and test work; benchmark before assigning complex recovery. |
| `openai/gpt-6-luna` | Bounded searches, inventories, work-packet checks, and simple mechanical work with deterministic validation. |

The task tool now exposes `model` after enabling `allow_model = true`. Prefer exact names over tier requests for this experiment: tier assignments can be overridden and do not uniquely identify a model. Prior firmware tasks did not explicitly select models or tiers, so the earlier run is not evidence of mixed-model performance.

Benchmark 8–12 representative tasks using accepted, independently verified bytes or completed checks per minute, total token cost where available, rework, and defect rate. Model output price alone is not a throughput metric. Escalate ambiguous work instead of allowing repeated speculative retries.

Give agents disjoint files and ranges. Workers must not edit shared exports, manifests, coverage ledgers, or status documents. The integration owner handles those changes and any requested commits. Agent reports must include changed paths, verified extents, tests actually run, and unresolved risks; a self-reported pass still requires the parent gate.

## 5. Coordinate builds and freeze integration batches

Use one build coordinator. Workers should not launch competing broad Cargo builds or mutate shared integration files during validation.

Start with coherent integration batches of roughly 16–32 KiB where natural function boundaries permit; adjust using measured turnaround and defect rates. Recovery batches may be smaller. Freeze source, exports, coverage, and generated status for the entire validation window. New worker output waits for the next batch.

Run Maki and Rust tooling directly inside the Fedora container. Build only the needed target, for example:

```sh
cargo build -p firmware-tools --bin firmware-tools
```

Reuse Cargo artifacts and a focused test harness. Measure cache misses before adding compiler-cache infrastructure. Do not confuse analysis caching, Cargo incremental artifacts, and clean reproducibility builds: each serves a different purpose.

## 6. Apply tiered verification gates

1. **Worker gate:** focused emitter/encoder tests, extent checks, and authenticated exact comparison for the assigned range.
2. **Frozen-batch gate:** independent review; parent focused tests; shared-plan/export integration; manifest union and overlap checks; relevant CLI and Python coverage tests.
3. **Milestone gate:** full relevant Rust and Python suites after shared encoder/build changes or substantial integrated tranches.
4. **Reproducibility gate:** two fresh builds when complete-image builders change and at explicit checkpoints. Report this as same-toolchain reproducibility unless different toolchains were actually tested.

GMU's pinned comparison is ignored by default and must be explicitly invoked:

```sh
env GMU_REFERENCE=<pinned-gmu-path> cargo test -p firmware-tools --lib \
  devices::xiaomi_nabu::a640_gmu_prefix::tests::modelled_spans_match_reference \
  -- --ignored --exact
```

Venus optional tests use `NABU_VENUS_REFERENCE`, falling back to `VENUS_REFERENCE`. Authenticate the full reference's size and hash, then compare the same loaded buffer using the project's SHA-256 implementation. Normal tests must not depend on private absolute paths, external disassemblers, or shell hashing tools. Never execute firmware.

Only after a frozen batch passes should the integration owner finalize credited coverage and regenerate status with `python3 -m tools.firmware status --write`. Credit the deduplicated emitted union, not summed overlapping declarations. Record the exact tested tree and commands; any subsequent change invalidates the affected gate.

## 7. Continue toward complete images without overstating feasibility

After recovering the pending GMU, utilities, and VPX work, recompute the remaining gaps from the verified plan and prepare the next bounded packets. Continue executable recovery while separately tracking unrecovered tables, configuration, calibration, signatures, and certificates.

OEM authentication material and device-specific calibration remain provenance blockers. Typed DER or tag representations do not derive the original signature bytes or settings. Do not claim complete-source reproduction until every required byte has an acceptable source derivation and the entire image matches.

Commit only when explicitly authorized, using author and committer `Codex <codex@openai.com>` for tested firmware commits. Never push. This plan does not itself authorize a commit.
