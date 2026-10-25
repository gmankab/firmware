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
bash tools/verify-agent15-halo-pm.sh /private/cs35l41-dsp1-spk-prot.wmfw
# Standalone non-mutating counts/hashes:
cargo run --offline --locked --example agent15_pm_audit -- /private/cs35l41-dsp1-spk-prot.wmfw
python3 tools/agent15_pm_python.py /private/cs35l41-dsp1-spk-prot.wmfw
```

Required source files are isolated to `wmfw_halo_pm_transport.rs`,
`examples/agent15_pm_audit.rs`, two new test/audit scripts and this
agent-specific documentation. The only shared change is the one-line
`pub mod wmfw_halo_pm_transport;` registration in `lib.rs`, committed
separately for merge review.

## Remaining

A CS35L41-specific HALO ISA source/assembler oracle is required to identify
program operations and regenerate bytes from legitimate source. The original
PM starts at file `0x278c`; **that is still the first not-source-recovered
byte**. Do not confuse independently verified transport with independent
firmware decompilation. Agent 16 owns XM/YM data and metadata.
