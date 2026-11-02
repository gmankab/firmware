# Agent 20-15 progress — HALO program-memory reverse engineering

**Branch:** `agents20/15-halo-pm` **starting Git commit:**
`5699b32ff9acfdf4b7837b392374a8b80666a948` (Agent 08 checkpoint 18).
The checkpoint-50-rooted integrated branch is **not available here**;
this is a portable independent child of the previous agent's branch.

## Checkpoint 01 — checked PM transport and reproducible audit

* Original CS35L41 WMFW matched the pinned SHA-256, **34,236 bytes**.
* Owned PM file range `[0x278c,0x76c4)` independently verified as
  **20,280 externally supplied bytes**.
* 4,056 packed 40-bit storage words, 1,014 twenty-byte blocks, 5,070
  aligned AHB register writes `[0x03800000,0x03804f38)`. No out-of-window
  or overlapping registers; reference round-trip has **zero byte mismatches**.
* Public Linux `cs_dsp` regmap transport formula and CS35L41 PMEM register
  end bounds reproduced with typed checked Rust and independent Python.
* New tests cover bitwise transport (all 160 block bit positions), register
  alignment and lane offsets, last-word bounds, whole-device PM write list,
  patched-value bounds/atomicity, malformed WMFW, wrong SHA revision,
  and input-file overwrite prevention.
* Full standalone Rust and Python audits produce *identical JSON evidence*
  containing only counts/hashes and no OEM code bytes or instruction listing.
* No HALO instruction-set decoder can be validated with the provided ISA
  evidence: **0 independently rebuilt executable PM bytes**. This is not
  an authenticated or executable image.

Reproduction with private reference external to Git:

```sh
bash tools/halo/verify-halo-pm.sh /private/cs35l41-dsp1-spk-prot.wmfw
# Standalone non-mutating counts/hashes:
cargo run --offline --locked --example halo_pm_audit -- /private/cs35l41-dsp1-spk-prot.wmfw
python3 tools/halo/pm_transport.py /private/cs35l41-dsp1-spk-prot.wmfw
```

Required source files are isolated to `wmfw_halo_pm_transport.rs`,
`examples/halo_pm_audit.rs`, two new test/audit scripts and this
agent-specific documentation. The only shared change is the one-line
`pub mod wmfw_halo_pm_transport;` registration in `lib.rs`, committed
separately for merge review.

## Remaining

A CS35L41-specific HALO ISA source/assembler oracle is required to identify
program operations and regenerate bytes from legitimate source. The original
PM starts at file `0x278c`; **that is still the first not-source-recovered
byte**. Do not confuse independently verified transport with independent
firmware decompilation. Agent 16 owns XM/YM data and metadata.

## Checkpoint 02 — fail-closed candidate instruction-span coverage gate

* Added `wmfw_halo_pm_oracle.rs`, a bounded **bit-field acceptance gate**
  for future CS35L41-specific *independently source-assembled* 20-/40-bit
  instruction candidates. It does **not** decode the ISA or prove that
  arbitrary halves of a 40-bit storage word are instructions.
* Candidate record includes the storage word index, high/low 20-bit position,
  width, numeric candidate bits and human-audited source-evidence identifier.
  Empty or anonymous evidence, overwide encodings, overlap, unaligned long
  claims and out-of-range word indices all fail closed.
* The reader verifies each candidate bit-field against the original PM
  reference, returning a count of verified half-slots, complete storage words,
  and the **first unaccounted half-slot**. No OEM instruction bits are
  incorporated into the repository.
* The standalone source-candidate assembler requires **all 8,112 half-slots**
  to be explicitly provided and rejects gaps rather than inventing zeros.
  Complete reconstruction is exercised on a deterministic **synthetic**
  4,056-word fixture, not any reconstructed Cirrus opcode.
* Rust/Python pinned-reference audit now reports
  `source_verified_20bit_half_slots: 0`, `first_uncovered_pm_word: 0`,
  `first_uncovered_pm_half: 0`, and
  `independently_rebuilt_executable_bytes: 0` identically.
* This new gate does not change the source-derived firmware-byte count;
  until a valid CS35L41 HALO assembler/ISA oracle or source program is
  supplied, PM executable source reconstruction remains **0 / 20,280**.
