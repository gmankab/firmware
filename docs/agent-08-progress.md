# Agent 08 — Cirrus CS35L41 WMFW reconstruction

**Branch:** `agent/08-cirrus-wmfw`. The user's prescribed starting point is checkpoint 50 (`b7ed4b5`), but that bundle was **not present in the mounted files** for this agent session. The supplied `recompile-firmware.bundle` instead ends at `69ebf0d1c50859189a9196a21155c388ac914d7b`; this independent WMFW branch starts there. The branch must be cherry-picked onto checkpoint 50 after the checkpoint-50 bundle is provided. No Venus source files are changed.

## Reference

- `cs35l41-dsp1-spk-prot.wmfw` is 34,236 bytes, SHA-256 `d1bea27e41001024eb3878276292b5bcc0cda4484ffced0880e12c03ec9bac2c` (matches `reference/xiaomi-nabu/firmware.sha256`).
- Header `0x0000–0x0028`: WMFW v3, HALO core 4, source version `0x0300`; timestamp, four DSP memory-size words and footer are modeled independently.
- Eleven WMFW sections have 8-byte descriptors; packed memory kinds are `0x10` PM, `0x11` XM, `0x12` YM; `0xf2` is algorithm metadata and `0xff` is informational text.
- HALO PM payload `0x278c–0x76c4` is 20,280 bytes (4,056 packed 5-byte program words); *this is not evidence of an instruction encoding*, and no HALO program has been authored yet.
- Public format reference: Linux `include/linux/firmware/cirrus/wmfw.h` and `drivers/firmware/cirrus/cs_dsp.c`.

## Checkpoint 08-01 — checked layout

New `wmfw_plan` builds the exact 40-byte header and eleven 8-byte region descriptors with a strict `ImagePlan`, rejects overlaps/truncation and checks structural region boundaries and packed word counts. **128 independent bytes of 34,236**; the eleven payloads remain explicitly unimplemented. It cannot finalize a WMFW firmware using implicit zero-fill or reference payload copying. Command-line coverage and strict metadata checking are included.

### Verification

```sh
cargo test --workspace --offline --locked
cargo build --workspace --release --offline --locked
python3 -m unittest discover -s tests -p 'test_*.py'
cargo run --offline --locked -p firmware-tools -- wmfw-coverage
cargo run --offline --locked -p firmware-tools -- verify-wmfw-layout /private/cirrus/cs35l41-dsp1-spk-prot.wmfw
```

**Not flashable / not HALO program reconstruction.** The packed DSP program and proprietary algorithm-control values remain unimplemented. A byte comparison only certifies the independently generated fields, not their runtime semantics or rights to redistribute original code.

## Checkpoint 08-02 — text region

The 48-byte `INFO_TEXT` payload at `0x30–0x60` is now produced from an
independently authored human-readable build timestamp. Cumulative independent
coverage is **176 / 34,236 bytes**, and the ten remaining opaque DSP/algorithm
payloads are deliberately absent. All reference fields compare exactly,
and the partial builder still refuses to finalize a full WMFW.

## Checkpoint 08-03 — HALO framework algorithm controls

Reconstructed the **948-byte** first `ALGORITHM_DATA` payload (`0x76cc–0x7a80`)
from high-level register offsets and twelve named firmware controls (HALO
state/heartbeat, audio block size, firmware build metadata, multirate fields,
LRCLK delay and bypass flags). The WMFW v3 encoder uses checked counted-string
fields with 4-byte padding, typed memory kinds, flags and register lengths;
it never imports the reference bytes to build output. The bounded parser
accepts the generated algorithm, rejects truncation and nonzero alignment
padding. Independent coverage is **1,124 / 34,236 bytes**, with nine region
payloads still missing.

## Checkpoint 08-04 — CSPL and event logger controls

Added 33 CSPL and 12 event-logger controls as independently authored names,
register offsets, packed-memory types, access flags and value lengths. The
WMFW v3 length-prefixed serializer produces exactly the 2,100-byte CSPL
(`0x7a88–0x82bc`) and 760-byte event metadata (`0x82c4–0x85bc`) regions.
The host-diagnostic parser validates the records and rejects malformed data.
**Cumulative independently generated metadata: 3,984 / 34,236 bytes.** Seven
DSP memory payloads are still missing, including all 20,280 program bytes.

## Checkpoint 08-05 — HALO PM/XM/YM packed-memory model

Corrected the memory-mapping model using the Linux Cirrus HALO loader: XM/YM
are **24-bit packed** (three bytes per DSP word, with four DSP words mapped
into three 32-bit host registers), while PM uses **40-bit packed** storage
(five bytes per DSP program word). The reference therefore contains 4,056 PM
words and 2,044 YM words; the initial 72-byte XM region holds 24 DSP words.
Added overflow-checked mapping, reversible transport pack/unpack, word-count
summaries, and bounded-reference checks. These routines **do not decode HALO
instructions** and **do not increase independent reconstruction coverage**:
3,984 / 34,236 bytes remains the correct count.

## Checkpoint 08-06 — exact image packaging with explicitly external DSP code

Added a dependency-free SHA-256 implementation with FIPS vectors and padding
boundaries, plus an exact-variant manifest for all **seven opaque HALO DSP
memory regions** (30,252 bytes). The CLI can extract those proprietary regions
to a **private directory outside Git**, and can rebuild a full WMFW file from
the independent 3,984 metadata bytes plus the separately provided original
DSP payloads. Every required region is checked for file offset, WMFW type,
length and SHA-256; the whole regenerated image is checked against its pinned
reference SHA-256. No unknown data is silently padded or read from the
original WMFW by the builder. All immutable OEM program bytes remain external
source inputs, never embedded or committed.

Example, with `$REF` the verified 34,236-byte WMFW reference and `$PRIVATE`
a directory outside the repository:

```sh
cargo run --offline --locked -p firmware-tools -- \
  extract-wmfw-external-payloads "$REF" "$PRIVATE"
cargo run --offline --locked -p firmware-tools -- \
  assemble-wmfw-with-external-payloads "$PRIVATE" /tmp/rebuilt-external.wmfw
cmp "$REF" /tmp/rebuilt-external.wmfw
sha256sum /tmp/rebuilt-external.wmfw
```

Local end-to-end reference test: output **34,236 bytes, 0 differences**,
SHA-256 `d1bea27e41001024eb3878276292b5bcc0cda4484ffced0880e12c03ec9bac2c`.
A deliberately modified opaque input is rejected before output. This is an
**externally supplied opaque-payload repack, NOT an independently rebuilt
HALO program**; source-derived coverage remains **3,984 / 34,236 bytes**.
The proprietary blob files and generated complete WMFW are excluded from Git.

## Checkpoint 08-07 — sparse packed DSP word address spaces

The independent WMFW v3 header declares exactly 1,452 X-memory words,
2,044 Y-memory words and 4,056 program-memory words. A checked address-map
builder now validates that the five XM regions initialize 1,280 words without
overlap and leave four **uninitialized DSP address ranges** totaling 172
words (`0x18–0x34`, `0xe4–0x140`, `0x350–0x354`, `0x500–0x530`). YM and PM
are fully initialized by their respective one packed-file region. The new
`wmfw-memory-map` CLI prints this map and distinguishes uninitialized DSP
memory from the seven proprietary missing *file* payloads. Tests reject
unbounded and overlapping memory ranges. Independent file-byte coverage
remains **3,984 / 34,236 bytes**; this checkpoint adds architectural
reconstruction and strict memory-map validation, not HALO machine code.

## Checkpoint 08-08 — named DSP coefficient control spans

The 57 independently authored HALO/CSPL/event-logger metadata controls now
have checked **DSP-relative** memory intervals. 56 controls target X memory;
the 400-byte `CSPL_UPDATE_PARAMS_CONFIG` block is 100 Y-memory words,
separate from its XM controls. The framework's highest referenced control
word is `0xe`, CSPL's highest XM word `0x4d`, and event logger's `0x4b`.
This code catches duplicate algorithm/control IDs, overlapping address spans,
non-word-aligned lengths, unknown memory types and unsupported flags. The
`wmfw-control-ranges` CLI reports relative ranges; it deliberately does NOT
assert absolute per-algorithm bases from a guessed DSP program. File-byte
coverage stays **3,984 / 34,236 bytes**, and no HALO opcode reconstruction
is claimed.

## Checkpoint 08-09 — conservative 40-bit program inventory

A new HALO packed PM inspector splits the original PM region into 4,056
five-byte transport words, counts 3,701 distinct values and 29 all-zero
words, and locates repeated four-word records where the final two words
match across multiple consecutive entries. The original image contains a
six-entry candidate repeated template beginning at PM word zero, plus two
shorter three-entry runs. The detector rejects truncated input and runs on
synthetic vectors in unit tests. It is **not an instruction decoder**:
repetition alone cannot identify a HALO opcode, branch or semantic behavior.
No proprietary program words are embedded in the implementation; source-
reconstructed coverage remains **3,984 / 34,236 bytes**. This inventory is
for identifying potential future function/data boundaries, not build output.

## Checkpoint 08-10 — CS35L41 real AHB/register destination mapping

Mapped every packed memory region to hardware register addresses from the
public Linux `include/sound/cs35l41.h` definitions: XM `0x02000000`, YM
`0x02c00000`, program memory `0x03800000`. The `wmfw-cs35l41-bus-map` command
resolves all seven file payloads to exact DSP-word and register ranges, tests
packed 32-bit host alignment and the documented register end bounds, and
rejects overlaps/out-of-range writes. This enables auditing the loader's
transfer schedule without including any OEM DSP payload or claiming HALO
instruction semantics. File-byte reconstruction remains **3,984 / 34,236**;
complete byte-identical files can still only be produced using seven
explicit, SHA-256-pinned external DSP payloads from checkpoint 08-06.

References: `https://codebrowser.dev/linux/linux/include/sound/cs35l41.h.html`,
`https://github.com/torvalds/linux/blob/master/sound/soc/codecs/cs35l41-lib.c`.

## Checkpoint 08-11 — end-to-end audit and checkpoint-50 integration guide

Added `tools/verify-agent08-wmfw.sh`, a single offline audit that runs Rust,
Python and release-build checks, verifies all independently encoded metadata,
reports checked DSP/control/CS35L41 AHB maps, and then **explicitly imports**
the original seven external DSP payloads only into a temporary directory to
confirm the complete WMFW exactly matches the reference SHA-256. Temporary
OEM payloads are removed upon exit. The dedicated integration runbook explains
how to apply only the agent's Codex commits to checkpoint 50 without merging
the unrelated older main-branch ref, and documents the HALO ISA/source blocker.
Independent firmware-byte coverage is unchanged: **3,984 / 34,236**.

## Checkpoint 08-12 — private, all-or-nothing external WMFW payload extraction

Hardened the **external-only** repackaging workflow: all seven reference-region
hashes and the WMFW layout are checked before any output is created. The
extractor now requires a previously nonexistent destination directory, writes
all payloads into an adjacent private staging directory (`0700` on Unix,
`0600` per file), synchronizes each file, and commits the directory by rename.
A failed write/invalid name removes the staging directory and leaves no
partially extracted proprietary payloads. An existing destination or symlink
is rejected, preserving user files and preventing inadvertent overwrite.
Synthetic unit tests cover rollback, overwrite refusal and private modes.
This affects only safe handling of **externally supplied OEM bytes**; source-
independent WMFW coverage remains **3,984 / 34,236 bytes**.

## Checkpoint 08-13 — independently encoded sparse Y-memory zero initialization

The reference's packed Y-memory section (WMFW file `0x0f90–0x2784`, 2,044
24-bit DSP words) includes 1,815 consecutive zero-valued words at DSP offsets
`223..2038`, corresponding to **5,445 bytes** at file offsets
`0x122d..0x2772`. The new `wmfw_sparse_ym.rs` builder emits these bytes as
an explicit bounded zero-initialization range, not by copying them from a
source file. The remaining Y-memory prefix (669 bytes) and suffix (18 bytes)
and all six XM/PM payloads are separately SHA-256-pinned external sources.
The full WMFW matches its reference SHA-256 when supplied with eight files
totaling **24,807 external bytes**; the original seven-whole-region repacker
is preserved as an alternative. Both use private all-or-nothing extraction,
and altered or missing source fragments cause failure.

The zero run was **inferred by inspecting the exact proprietary reference**.
Its re-expression as `vec![0;5445]` is a verified serialization optimization,
**not independent reconstruction of HALO executable source**. Continue to
report source-derived metadata as **3,984 bytes**, reference-verified zero
bytes as **5,445**, and proprietary external payload requirements as
**24,807 bytes**. This separate accounting prevents counting the zero run
as recovered DSP instructions. The 4,056 program-memory words remain wholly
opaque and require an independently validated 40-bit HALO instruction set
before claiming decompilation.

Commands (reference and input directory are private and outside Git):

```sh
cargo run --offline --locked -p firmware-tools -- \
  extract-wmfw-sparse-y-inputs "$REF" "$PRIVATE/new-sparse-inputs"
cargo run --offline --locked -p firmware-tools -- \
  assemble-wmfw-with-sparse-y "$PRIVATE/new-sparse-inputs" "$PRIVATE/rebuilt.wmfw"
cmp "$REF" "$PRIVATE/rebuilt.wmfw"
```

`tools/verify-agent08-wmfw.sh` now checks both the original seven-input and
new sparse eight-input repackers and tests altered sparse source rejection.

## Checkpoint 08-14 — two checked packed-Y zero ranges

In addition to the large `223..2038` zero run, the exact original Y-memory
payload contains 101 all-zero 24-bit words at DSP indices `15..116` (303
serialized bytes). The optional `wmfw_compact_y.rs` builder models both
ranges explicitly. Its WMFW output consists of **3,984** source-derived WMFW
metadata bytes, **5,748 reference-confirmed zero bytes** (no raw bytes
copied), and **24,504 SHA-256-verified external DSP bytes** in nine private
source files. The full 34,236-byte output has the exact original SHA-256.
The original seven-region and single-run eight-file builders remain available.

The two Y zero runs are an input-compaction technique learned by inspecting
the proprietary reference, **not independently reverse-engineered DSP
instructions or proof of original source code**. The program-memory block
still has all 20,280 bytes externally supplied. Source-independent code
coverage remains **zero**, and conservative source-derived WMFW metadata
remains **3,984 bytes**, separate from the reference-confirmed zero runs.

```sh
cargo run --offline --locked -p firmware-tools -- \
  extract-wmfw-compact-y-inputs "$REF" "$PRIVATE/compact-y"
cargo run --offline --locked -p firmware-tools -- \
  assemble-wmfw-with-compact-y "$PRIVATE/compact-y" "$PRIVATE/compact-rebuilt.wmfw"
cmp "$REF" "$PRIVATE/compact-rebuilt.wmfw"
```

`tools/verify-agent08-wmfw.sh` checks all three workflows and deliberately
corrupts an external compact Y part to confirm SHA-256 enforcement.

## Checkpoint 08-15 — checked HALO runtime algorithm directory

Linux's public `wmfw_v3_id_hdr`, `wmfw_halo_id_hdr` and
`wmfw_halo_alg_hdr` structures define a **runtime** algorithm directory:
a 40-byte identifier header followed by six big-endian 32-bit fields per
algorithm. The HALO processor exposes these fields through **unpacked 32-bit
DSP register memory**, which is not equivalent to parsing the WMFW packed
XM payload directly.

The `wmfw_halo_registry` module decodes these runtime structures, rejects
truncated tables, excessive/duplicate algorithm IDs, overflowing memory
ranges and conflicting XM/YM allocations. Given an authentic running-device
registry dump, `resolve_named_controls` combines the 57 independently
reconstructed metadata controls with each algorithm's actual XM/YM base,
validating the bounds of every resolved DSP-word span.

The new `inspect-halo-runtime-registry` CLI accepts such an unpacked register
dump; a synthetic three-algorithm registry is used in both Rust tests and the
automated audit. **No actual device runtime register dump has been supplied**,
so the reconstructed program remains unverified at instruction level and no
real runtime algorithm bases are asserted. Firmware-byte accounting is
unchanged: 3,984 independently authored WMFW metadata bytes, 5,748
reference-verified Y zero bytes, and 24,504 external DSP bytes in the
most compact full-file repackaging mode.

Public ABI references:
- `https://raw.githubusercontent.com/torvalds/linux/master/include/linux/firmware/cirrus/wmfw.h`
- `https://raw.githubusercontent.com/torvalds/linux/master/drivers/firmware/cirrus/cs_dsp.c`

## Checkpoint 08-16 — reproducible sparse HALO loader memory image

The new `wmfw_loader_image` module replays all seven declared packed DSP
payload regions into three independent X/Y/program word-address spaces,
mirroring the *logical placements* from the Linux CS35L41 loader. Its
`Option<u64>` representation distinguishes uninitialized X memory addresses
from explicit zero-valued packed words. Exactly **1,280 / 1,452** X words,
**2,044 / 2,044** Y words and **4,056 / 4,056** program words are loaded;
unknown X-memory words remain unknown, not silently fabricated.

Each packed memory region can be re-encoded from the resulting word image;
all 30,252 proprietary payload bytes are recovered **only because they were
supplied as input**, not independently generated. An optional AHB transfer
manifest divides the logical packed-write span into at-most-128-byte aligned
chunks, checking file, DSP and register ranges. It does **not** claim a precise
SPI/I2C bus transaction schedule or HALO instruction behavior. The new
`simulate-halo-wmfw-load "$REF"` command validates the checked reference
and reports the allocated/unallocated memory coverage. The end-to-end audit
runs it alongside the three existing exact-file repackers.

Synthetic Rust fixtures exercise word reversal, explicit zero preservation,
control of uninitialized ranges, bad region kinds, out-of-range transport
words, overlapping placements, and invalid DMA span limits. The independent
metadata/source count remains **3,984**; reference-confirmed zero bytes
remain **5,748** in the most compact mode, with **24,504 external bytes**.
