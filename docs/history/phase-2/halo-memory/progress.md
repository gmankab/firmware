# Agent 20-16 — HALO data memory

**Latest work: checkpoint 02.** Its ledger below supersedes the checkpoint-01
totals. Both checkpoints are portable continuations of the verified Agent 08
history, with separate integration-only commits.

## Starting point and ownership

- Branch: `agents20/16-halo-memory`.
- Verified parent: Agent 08 checkpoint 18, `5699b32ff9acfdf4b7837b392374a8b80666a948`.
- This is a **portable continuation on the older Agent 08 history**, not a checkpoint-50-based integration. The supplied integration artifacts still identify an older base. Agent 01 owns integration onto the genuine checkpoint-50 descendant.
- Scope: packed XM/YM data, named control schemas and directory validation. No HALO program instructions are reconstructed. PM file `[0x278c,0x76c4)` remains external and belongs to Agent 15.
- All new commits use author and committer `Codex <codex@openai.com>`.

## Pinned inputs

The eight original archive parts were joined in order `001,002,003.1,003.2,004,005.1,005.2,006`. The 1,718,401,941-byte compressed archive passed `zstd -t --long=31`; its uncompressed TAR size is 2,992,230,400 bytes. Only the necessary reference and compiler packages were extracted. The HALO reference from `map220v-nabu-firmware` is the same exact revision pinned in this repository:

- Name: `cs35l41-dsp1-spk-prot.wmfw`.
- Size: 34,236 bytes.
- SHA-256: `d1bea27e41001024eb3878276292b5bcc0cda4484ffced0880e12c03ec9bac2c`.
- Original firmware, decoded coefficient inventories and temporary full repacks remain outside Git.
- Rust/cargo 1.99.1 from the supplied archive; offline locked builds with an Agent 16-specific `CARGO_TARGET_DIR`.

## Checkpoint 01 — corrected transport and typed static directory

### Corrected four-word packing

The previous XM/YM inspector split the file into consecutive big-endian triples. That representation round-tripped bytes, but did **not** represent logical DSP words. The official Cirrus converter proves that three big-endian 32-bit host registers contain four 24-bit data words, with register lanes interleaved across those words.

`wmfw_halo_memory` now follows that mapping, rejecting partial 12-byte groups and out-of-range values. `wmfw_loader_image` uses the corrected logical data words and rejects unaligned packed section starts. PM retains its prior five-byte transport-only behavior; this work does not establish PM instruction semantics.

The unchanged official converter independently agrees with the Rust decoder for **all six XM/YM sections: 3,324 logical words / 9,972 transport bytes**, with zero value or descriptor-address disagreements. Those decoded bytes remain externally supplied observations; decoder coverage is not source reconstruction.

### New static directory reconstruction

`wmfw_halo_static_data` generates the complete packed XM directory at file **`[0x68,0xb0)`**, logical XM words **`[0,24)`**, without reading the reference at build time. IDs, revisions and placement fields are explicitly reference-derived typed data. XM allocation lengths are cross-checked against the existing authored control metadata.

| Static entry | ID | Revision | XM allocation | YM allocation |
|---|---:|---:|---:|---:|
| Framework header | `0x400a4` | `0x002100` | `[0x1e0,0x1ee)` | empty |
| CSPL | `0xcd` | `0x1d3500` | `[0x94,0xe1)` | `[0xe,0x7f7)` |
| Event logger | `0xf20b` | `1` | `[0x170,0x1bb)` | empty |

Core identifier is `0x040000`, format revision `0x030000`, vendor ID `2`, and algorithm count `2`. The count excludes the framework header. The last two logical words are the documented `0x00bedead` terminator and an explicitly generated alignment word.

The directory contains **69 bytes of typed fields plus 3 bytes of explicit padding**. The padding's physical file interval is `[0xac,0xaf)`; byte `0xaf` belongs to the terminator. Generated 72-byte SHA-256:

`44510542e96ef4ba1c5a4fea0cb0b516cd81698bc83cb254d30b055f1329dd7f`

Both exact comparison against the pinned file and the unmodified Cirrus `halo_firmware_id_block` parser pass. These are **static image allocations**. No genuine live runtime dump has been supplied, and no actual runtime AHB addresses are asserted.

### Runtime-directory corrections

Linux registers the firmware-header allocation as one region and then adds each algorithm-table entry. The old parser incorrectly required all algorithm allocations to lie inside the header allocation, and its test fixture duplicated the firmware ID as a table row. The corrected parser/resolver:

- Treats header and table allocations as peers, with duplicate-ID, overlap, overflow and capacity checks.
- Resolves framework controls from the header.
- Revalidates manually constructed registries; bus resolution checks full allocations against hardware windows.
- Accepts an exact directory prefix, or that prefix plus a validated big-endian `0x00bedead` terminator. A prefix-only input does not certify the absent terminator. Incorrect or partial trailers fail.
- Uses explicitly synthetic runtime fixtures, separate from the static directory.

### Signed values and calibration schema

`wmfw_halo_fixed_point` checks signed 24-bit limits `[-8388608,8388607]` and requires an explicit fractional-bit count. Rational conversion uses integer arithmetic with either truncation toward zero or nearest, ties-to-even rounding. No universal Q23 format is inferred.

`wmfw_halo_calibration` models the documented successful restore contract: `CAL_R`, `CAL_STATUS=1`, then `CAL_CHECKSUM=CAL_R+1`. Checksum overflow, unknown/unsuccessful status, malformed canonical payloads and checksum mismatch fail. Optional ambient temperature precedes that sequence. The write plan contains only named algorithm-relative offsets, with no device writes. An integration test checks those offsets and lengths against `wmfw_algorithms`.

Encoded resistance remains in encoded units: the public header's ambiguous Ohms conversion is not implemented. Signed ambient limits are representational, not a validated hardware operating range. The library chooses a zero-padded 24-bit canonical register form; rejecting a sign-extended high byte is a normalization policy, not a claim that hardware rejects it.

### Corrected zero-span units

The old sparse/compact repackers' output is unchanged, but their earlier logical-word labels were incorrect. Their authoritative ranges are now **transport byte offsets** within YM:

- `[45,348)` = 303 zero bytes.
- `[669,6114)` = 5,445 zero bytes.

With the corrected converter, the corresponding long logical all-zero runs observed in the complete reference are `[14,116)` and `[222,2039)`. Logical words have scattered file-byte locations; neither byte interval is simply its former word index times three. Historical Agent 08 progress entries must be read with this correction.

### Conservative checkpoint-01 ledger

| Provenance | File bytes |
|---|---:|
| Existing authored WMFW container/control metadata | 3,984 |
| New reference-derived typed static directory fields | 69 |
| New explicit directory alignment padding | 3 |
| Existing reference-observed zero-byte serialization | 5,748 |
| Remaining external DSP payload bytes | 24,432 |
| **Whole file** | **34,236** |

The remaining external bytes include all **20,280 PM bytes** and **4,152 XM/YM bytes**. Independently reconstructed HALO instruction bytes remain **zero**. First missing source interval after adding the directory is file `[0xb8,0x2c8)`.

The optional external repacker generates the directory itself and consumes eight private input files totaling 24,432 bytes. The full output matches the reference, but remains an external-payload repack. Existing legacy modes retain their original 24,807/24,504 external-byte counts and outputs.

### Verification and reproduction

Checkpoint verification covers 223 Rust tests, 13 Python tests, the offline release build, `git diff --check`, exact 72-byte generation, all six data-memory sections against Cirrus's converter, and the eight-input full external repack. No Rust/Python tests were skipped. Corrupt external input and a modified full reference are rejected. Exact tested commit and machine-readable results are supplied alongside the bundle.

```sh
export CARGO_TARGET_DIR=/private/agent16-target
cargo test --workspace --offline --locked
cargo build --workspace --release --offline --locked
python3 -m unittest discover -s tests -p 'test_*.py'
python3 tools/halo/verify-halo-memory-halo.py \
  /private/cs35l41-dsp1-spk-prot.wmfw \
  "$CARGO_TARGET_DIR/release/halo-memory" \
  /private/pinned-cirrus/tools/firmware_converter/wmfw_parser.py
```

The isolated `halo-memory` binary supports `verify`, `emit-static-directory`, `decode-xy` (external diagnostics) and `assemble-external`. Its new modules are registered in a separate integration-only commit; the shared CLI, Cargo manifests, central status and Venus sources are untouched.

## Remaining work and evidence limits

Named control defaults can be represented as reference-derived initialization records. The remaining algorithm-internal tables require further semantic evidence. In particular, the repeated five-word YM records and the 106-word tuning block have no validated public coefficient formula, field order or fixed-point scale. The official Cirrus example tuning image contains a similarly placed block, but its unexplained byte array is not independent source derivation. No such opaque values are embedded in the new generators.

Still needed for a complete Agent 16 reconstruction: source-backed semantics/generators for unresolved XM/YM configuration and coefficients, plus real runtime evidence before claiming active register mappings. Agent 01 must integrate this portable series onto its verified checkpoint-50 descendant. Full-firmware behavior and hardware acceptance remain untested.

## Checkpoint 02 — all named static control initializers

`wmfw_halo_control_defaults` now declares initial values for all **57 named
controls**, covering **264 logical 24-bit words / 792 physical file bytes**.
Every initializer is labeled `ReferenceObservedInitialization`: it models an
observed initial value through a known schema, not the original CSPL algorithm
or a value computed from a newly recovered coefficient formula.

The specification lists every field explicitly and checks its name, algorithm,
memory kind, relative offset, storage length and static allocation. There is
no catch-all zero initializer for unknown fields. The ASCII job name is emitted
from `"Manual"`; named buffers are explicitly sized. Unnamed CSPL offsets
`0x12` and `0x13` remain external. Initial `CAL_STATUS=0` and
`CAL_CHECKSUM=0` are not accepted as successful calibration.

| Named allocation | Logical words | Packed bytes |
|---|---:|---:|
| Framework XM controls | 14 | 42 |
| CSPL XM controls, excluding two unnamed words | 75 | 225 |
| CSPL YM update buffer, internal field meanings unresolved | 100 | 300 |
| Event-logger XM controls | 75 | 225 |
| **Total** | **264** | **792** |

The byte lanes form **11 disjoint file spans**, not one contiguous interval:
`[0x1d8,0x20c)`, `[0x20e,0x210)`, `[0x214,0x2bc)`, `[0x2bd,0x2c0)`,
`[0x360,0x440)`, `[0x443,0x444)`, `[0x4b0,0x4d8)`, `[0x4da,0x4dc)`,
`[0xfb8,0xfba)`, `[0xfbc,0x10e4)`, `[0x10e6,0x10e8)`.

SHA-256 of their generated bytes concatenated in ascending file-offset order:
`d2678789036090ba3901265c62c7126223ace19cb5e1aad968a833407ce1b25d`.
This digest does not describe a contiguous firmware fragment.

### Complete, disjoint byte ledger

The new `wmfw_halo_memory_plan` explicitly partitions every file byte. Of the
792 initializer bytes, **297 were already in the old observed zero ranges**.
They receive the more specific named-initializer category once. Consequently,
the initializers add **495 bytes beyond the previous zero-range coverage**.
Those additional bytes consist of 473 observed zero bytes and 22 nonzero
bytes; this is explicit initialization coverage, not recovered coefficients.

| Provenance at checkpoint 02 | Bytes |
|---|---:|
| Existing authored container/control metadata | 3,984 |
| Reference-derived static directory fields | 69 |
| Explicit directory padding | 3 |
| Reference-observed named control initialization | 792 |
| Remaining reference-observed zero bytes | 5,451 |
| External opaque XM/YM data | 3,657 |
| External PM data | 20,280 |
| **Whole file** | **34,236** |

The source planner accounts for **10,299 generated bytes** across the first
five categories, with every remaining byte left absent. All **23,937 external
bytes** are required to finalize an image. The generated count is a provenance
union, not independently recovered DSP program coverage; instruction coverage
is still zero. First unresolved file offset remains **`0xb8`**, at the start
of the packed region beginning at logical XM word `0x34`. Individual byte lanes
within that region do not follow logical word order.

External inputs are precisely the planner's **17 maximal holes**, each with a
pinned offset, length and SHA-256. Extraction checks the full reference and
every known generated byte before atomically creating the private directory.
Assembly verifies each hole's hash and the whole-file hash. The full output is
an explicitly external repack. No generic image-builder or PM generator is
changed.

### Independent validation and entry points

`tools/halo/verify-halo-memory-controls.py` runs the checkpoint-01 checks, then uses
the unchanged Cirrus converter to parse the original three metadata blocks,
the static directory and every XM/YM word. It independently checks all 57
names/offsets/lengths, all 264 values and every generated physical byte lane.
It independently constructs every byte's provenance, compares the complete
span partition and summary counts, checks overlap accounting and builds the
full file from the 17 private holes. Same-size corruption and missing external
inputs must fail before an output is written. The checkpoint gate covers
232 Rust tests, 13 Python tests, the offline release build and `git diff --check`,
with no ignored or skipped tests. The exact tested commit and complete results
appear in the accompanying checkpoint-02 verification JSON.

Additional isolated commands:

```sh
halo-memory control-defaults
halo-memory control-bytes
halo-memory memory-provenance
halo-memory verify-memory /private/reference.wmfw
halo-memory extract-memory-external /private/reference.wmfw /private/new-inputs
halo-memory assemble-memory-external /private/new-inputs /private/repacked.wmfw
```

The header footer's final field is now correctly documented as a **checksum**
field whose value is zero in the pinned image; it was formerly mislabeled
reserved. No undocumented WMFW checksum calculation is claimed.

### Outstanding scope after checkpoint 02

All named static controls and the static directory are represented. The
**3,657 remaining external XM/YM bytes** include unnamed structures and
algorithm-internal coefficient/configuration tables. A 106-word YM block at
logical `[0x74,0xde)` resembles a public Cirrus tuning image's shorter block,
but no original field schema or generation equations were established.
Repeated five-word and eight-word records remain observations, with no invented
filter identity or Q format. Finishing those bytes requires additional
source/semantic evidence; a live directory capture would separately be needed
to assert actual running-device mappings.

## Primary evidence

1. [Cirrus firmware converter, pinned revision](https://github.com/CirrusLogic/mcu-drivers/blob/a05b2878be911ae9ffaa440840cde17af98ba48e/tools/firmware_converter/wmfw_parser.py): `unpack_memory`, `memory_type_converter`, `halo_firmware_id_block`. SHA-256 `921b84b0c860b5b3cbc5e6596a9bc236e0d9ee40d92ae39de10cebdfcd1264cd`.
2. [Cirrus inverse binary converter](https://github.com/CirrusLogic/mcu-drivers/blob/a05b2878be911ae9ffaa440840cde17af98ba48e/tools/firmware_converter/binary_parser.py): packed group order and relative 12-byte alignment.
3. [Linux Cirrus DSP loader](https://codebrowser.dev/linux/linux/drivers/firmware/cirrus/cs_dsp.c.html): `cs_dsp_halo_setup_algs`, `cs_dsp_read_algs`, register conversion and unpacked data serialization. Registry behavior also checked against Linux v6.12.
4. [Linux WMFW ABI definitions](https://codebrowser.dev/linux/linux/include/linux/firmware/cirrus/wmfw.h.html): header, algorithm structures and control flags.
5. [Cirrus CS35L41 driver, same pinned revision](https://github.com/CirrusLogic/mcu-drivers/blob/a05b2878be911ae9ffaa440840cde17af98ba48e/cs35l41/cs35l41.c): successful calibration status, restore ordering and checksum.
