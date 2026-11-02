# Agent 20-19 — Novatek DLM and envelope

## Base and ownership

- Branch: `agents20/19-novatek-dlm`.
- Actual parent: `941c805f8510c88344133687a77722ace386504e`, the corrected, full-Agent-01 **older-base integration preview**.
- Required checkpoint-50 commit `b7ed4b587633b9b475a40f5a3ace5cc90db620ff` is absent from that bundle. No newer integration bundle was available when this work began. These commits are a **portable contribution**, not a certified checkpoint-50 integration.
- Exclusive implementation range: DLM file `[0x22000,0x2e190)`, SRAM `[0x20000,0x2c190)`, plus Novatek metadata/envelope/CRC validation. ILM implementation belongs to Agents 17 and 18.
- Git author and committer: `Codex <codex@openai.com>`. This is attribution, not a cryptographic signature.

## Exact reference

The uploaded multipart archive was streamed in order `001,002,003.1,003.2,004,005.1,005.2,006`. The complete concatenated Zstandard stream passed `zstd -t`. Only the two Novatek reference revisions were extracted; original payload files remain outside Git and all checkpoint deliverables.

| Revision | Bytes | SHA-256 |
| --- | ---: | --- |
| Required postmarketOS Nabu | 196,608 | `8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5` |
| Older map220v comparison only | 196,608 | `72364eca0a91855d59e043371aca774354e5ad0885df5024edcec8f976543ad2` |

The required hash also appears in `reference/xiaomi-nabu/firmware.sha256`. The new comparison command rejects all other full-image hashes before parsing or source-credit claims.

## Checkpoint 01 — Source-generated index schedules and Gaussian tables

Added **9,252 independently structured DLM data bytes**, with **zero new executable bytes**. Four index-schedule blocks contribute 6,336 bytes and 18 Gaussian half-kernels contribute 2,916 bytes. The generators have no reference-image input and contain no copied opaque coefficient/index arrays. Data semantics and hardware-program behavior are separate claims: the exact mathematical structures are established; the complete controller algorithms are not decompiled.

| File interval, end exclusive | SRAM interval | Bytes | Source recipe |
| --- | --- | ---: | --- |
| `[0x22488,0x22710)` | `[0x20488,0x20710)` | 648 | Four Gaussian half-kernels |
| `[0x24904,0x24fc4)` | `[0x22904,0x22fc4)` | 1,728 | 24 packed index schedules |
| `[0x25d4c,0x262ec)` | `[0x23d4c,0x242ec)` | 1,440 | 20 narrow index schedules |
| `[0x28904,0x28fc4)` | `[0x26904,0x26fc4)` | 1,728 | 24 wide schedules, translated live ring indices |
| `[0x29d4c,0x2a2ec)` | `[0x27d4c,0x282ec)` | 1,440 | 20 narrow schedules, translated live ring indices |
| `[0x2d7f4,0x2df8c)` | `[0x2b7f4,0x2bf8c)` | 1,944 | Twelve Gaussian half-kernels |
| `[0x2df98,0x2e0dc)` | `[0x2bf98,0x2c0dc)` | 324 | Gaussian half-kernels, sigma 40 and 80 |

### Packed schedule structure

A 72-byte record stores 50 unsigned 11-bit indices. Bytes 0–49 store the low eight bits of each index, bytes 50–51 are alignment zeros, and bytes 52–71 contain five little-endian 32-bit words. Each word packs ten three-bit upper fields into bits 0–29; bits 30–31 are zero. The inactive-slot sentinel is `0x7ff`.

The source recipes use descending sequences, alternating paired rows, transposed 6-by-8 selections, circular walks with moduli `48*32-1` and `48*30-1`, half-row translations of 16 or 15, and explicit inactive-slot masks. They do not read encoded tables to generate replacements. The 32-bit and 30-bit layouts, record boundaries, translation rules, and inactive masks are checked independently by Python.

Read-only NDS32 decoding of the pinned ILM provides consumer evidence: runtime `[0xc7ea,0xc87a)` (file `[0xc9ea,0xca7a)`) selects records with a `0x48` stride, reads 50 low bytes, starts high-word reads at offset `0x34`, extracts ten three-bit fields per word, and reconstructs 16-bit indices by adding the upper field shifted by eight. Runtime `0xc9ce` (file `0xcbce`) compares the decoded index to `0x7ff` and skips inactive entries. The inspection used GNU NDS32 objdump 2.30 with the reference's externally supplied EX9 table at file `[0x17dc0,0x185c0)`; no ILM or EX9 payload was added to this contribution.

Configuration descriptors also link the tables to that consumer. The schedule address is `(descriptor_word_at_0x50 & 0x7fff)*4 + 0x20000`. Observed descriptor offsets `0x22a64`, `0x26a64`, `0x25230`, and `0x29230` select the four table SRAM bases above. These configuration scalar values remain external evidence; they do not receive new source-byte credit.

### Gaussian structure

Each kernel contains 81 little-endian unsigned 16-bit samples, ordered `x=80..0`:

`round(400000 / (sigma * sqrt(2*pi)) * exp(-x*x / (2*sigma*sigma)))`.

Sigma is specified as an integer number of tenths. The early kernels use `284,260,272,284`. The twelve later kernels use `256,256,256,260,256,268,252,256,252,252,252,252`, followed by two separated kernels using `400,800`.

All 1,458 coefficients match the reference. An independent 80-digit Decimal calculation, with pi computed by Machin's formula, agrees with the Rust emitter. The smallest distance to a half-integer rounding boundary is approximately `0.0015830089415576845` at sigma 40, x 72. Rust's `f64::exp` accuracy can vary by platform, so the reference/hash comparison remains an integration gate for a new toolchain. The table identities are established; their complete runtime purpose and the original OEM table-generation source are not established.

Read-only consumer evidence additionally links runtime address `0x2b7f4` to initialization at runtime `0x192a` (file `0x1b2a`) and later halfword lookups at offsets separated by 162 bytes. This supports the data classification without claiming additional executable reconstruction.

### Strict envelope verification

The new `novatek_dlm_envelope::verify_envelope(image)` derives all parser state from the same supplied image. It checks the exact postmarketOS geometry and SRAM mapping, externally measured non-CRC metadata scalars, candidate-vector bounds, the existing 538-byte authored boot prefix, every reserved envelope region, the `NVT` marker, the 2,304-byte DLM-prefix mirror, and all 12 stored CRC-32/MPEG-2 fields.

This closes two concrete gaps in a caller's verification path: the inherited CRC API accepts separately supplied layout state, and publicly constructed candidate-vector structures can bypass their parser. A regression demonstrates a descriptor mutation with a repaired bank footer passing a stale-layout CRC check; the new same-image wrapper rejects it. Other regressions reject invalid vectors with otherwise valid CRCs, altered geometry/SRAM bases, unexpected metadata scalars, truncated input, and modified boot/padding/mirror bytes.

The inherited low-level APIs are still available to integration owners. Route final Novatek envelope validation through the strict wrapper, or apply equivalent same-image invariants when integrating it into another command.

### Provenance ledger after checkpoint 01

| Physical-image category | Bytes |
| --- | ---: |
| Source-generated envelope/layout and `NVT` marker | 35,136 |
| Previously authored ILM executable prefix | 538 |
| New source-generated DLM data | 9,252 |
| External metadata/vector scalars | 1,024 |
| Unknown ILM remainder | 108,054 |
| Unknown DLM remainder | 40,300 |
| DLM-derived mirror, counted once | 2,304 |
| **Total** | **196,608** |

Unique independently generated bytes total **44,926**. Unknown primary ILM/DLM bytes total **148,354**. Of the 2,304 mirror bytes, **648** derive from the newly authored early Gaussian kernels and **1,656** still derive from unknown DLM bytes. Mirror duplication does not increase unique source coverage. The first unknown DLM file offset remains **`0x22000`**, SRAM **`0x20000`**.

The ledger rejects empty, reversed, overlapping, unexplained, or out-of-range claims and freezes inherited ILM executable credit at 538 bytes. Claim metadata cannot itself establish authorship; only the independent generators and exact comparison justify the declared data intervals.

## Reproduction and integration

The standalone auto-discovered binary adds no shared module, manifest, or central CLI edits. From the repository, with a private pinned reference:

```sh
export CARGO_TARGET_DIR=/tmp/agent20-19-cargo-target
cargo test --workspace --offline --locked
cargo build --workspace --release --offline --locked
python3 -m unittest discover -s tests -p 'test_*.py'
git diff --check

"$CARGO_TARGET_DIR/release/novatek-dlm" emit /tmp/agent20-19-emitted
"$CARGO_TARGET_DIR/release/novatek-dlm" verify-reference /private/novatek_nt36523_fw.bin
python3 tools/novatek/verify-novatek-dlm.py \
  --reference /private/novatek_nt36523_fw.bin \
  --emitted-dir /tmp/agent20-19-emitted
```

`novatek-dlm ledger` prints complete byte accounting without reading a reference; its report explicitly says `reference_verified: false` and has no mismatch claim. `verify-reference` sets verification evidence only after pinned-hash, envelope, checksum, and all-fragment equality checks succeed. `check-envelope` tests consistency of a full supplied image without implying independent payload authorship. `emit` produces sparse fragments and a manifest, never a full image with guessed hole contents.

Validation records and exact artifact hashes accompany each checkpoint bundle. The full workspace gate uses Rust 1.90.0 with a separate target directory. Low-impact debug-output reductions were used to fit the shared workspace; the release build still uses the repository's normal release profile.

### Executed checkpoint validation

| Check | Measured result |
| --- | --- |
| Full offline locked Rust workspace tests | 398 test executions passed; zero failed/ignored |
| Newly added Rust regressions | 27 passed; the new standalone bin also reruns 32 inherited Novatek tests |
| Full offline locked release build | Passed with Rust 1.90.0 |
| Python workspace suite | 158 test methods: 107 passed, 51 skipped; one additional class setup skipped, yielding 52 reported skips |
| New Python regressions, with exact reference and release binary | All 15 passed, including real Rust emitter comparison |
| Rust release emitter against pinned reference | 22 fragments, 9,252 bytes, zero mismatches |
| Independent Python / 80-digit Decimal calculation | Same 22 fragments and 9,252 bytes; zero mismatches |
| Stored checksums | All 12 CRC fields independently verified |
| DLM prefix mirror | All 2,304 bytes verified |
| Diff whitespace check | Passed |

The 52 Python skips belong to pre-existing A640/ADSP/CDSP/modem/WLAN checks requiring separately supplied firmware and/or assembler tools. Their exact test names and reasons are included in `checkpoint-01-verification.json` and the handoff logs. No new Agent 19 test is skipped in the recorded full run. A skipped test is not counted as passing.

SHA-256 of all newly emitted fragments concatenated in increasing file-offset order: `5c9bc1b5833d9417e494da1db5f8113723f1c2561fcc73e072448da19628cd2f`. This 9,252-byte concatenation is a verification digest input, not a contiguous firmware image.

## Remaining work and boundaries

Forty thousand three hundred DLM bytes still need independent structured reconstruction. This includes the leading controller/calibration configuration, much of the repeated descriptor data, and unresolved tables. Apparent zero-filled records and unknown fixed command bits receive no source credit merely because a compact byte recipe could reproduce them. The 1,024 metadata/vector bytes remain externally measured scalars despite deterministic serialization and computable CRCs.

The precise remaining DLM holes are:

| File interval | Bytes |
| --- | ---: |
| `[0x22000,0x22488)` | 1,160 |
| `[0x22710,0x24904)` | 8,692 |
| `[0x24fc4,0x25d4c)` | 3,464 |
| `[0x262ec,0x28904)` | 9,752 |
| `[0x28fc4,0x29d4c)` | 3,464 |
| `[0x2a2ec,0x2d7f4)` | 13,576 |
| `[0x2df8c,0x2df98)` | 12 |
| `[0x2e0dc,0x2e190)` | 180 |

The 108,054-byte ILM remainder belongs to Agents 17 and 18. Integration Agent 01 must replay this contribution onto a real verified descendant of checkpoint 50, reconcile the ledger with later independently verified ILM contributions, and rerun the full build and byte comparisons. Full firmware behavior, hardware acceptance, and a complete independently authored image remain unfinished. No output here is represented as flashable or newly authenticated firmware.

## Checkpoint 02 — Deterministic Gaussian tables and idle mailboxes

This checkpoint advances independently structured DLM data coverage from
9,252 to **9,260 bytes** (`23` fragments). The previous source-generated index
schedules (6,336 bytes) and Gaussian kernels (2,916 bytes) retain byte-for-byte
reference identity; an additional **eight bytes** are generated from the
controller's dual idle-mailbox initializer at `[0x2e130,0x2e138)` (SRAM
`[0x2c130,0x2c138)`). Those slots each contain the controller idle token
`0x12345678`, modeled as a repeated little-endian `u32`, not a reference-byte
copy. No executable bytes are credited to Agent 19.

The Gaussian emitter now uses **Q48 fixed-point integer arithmetic**:
`√(2π)` is represented by the nearest Q48 integer `705553135224033`;
`exp(-x²/(2σ²))` is reconstructed with the positive Taylor series for
`exp(z)` and integer reciprocal. Values with `z >= 16` are safely rounded
to zero because the peak is bounded by `u16`. This eliminates target `libm`
`exp` variations from emitted bytes. Its 18 tables match both the independent
80-digit Python Decimal calculation and every byte of the pinned reference.
The historical floating-point diagnostic remains available as a cross-check
but is not used to emit firmware data.

| Region | Independently generated | Unresolved | Other accounting |
|---|---:|---:|---|
| DLM `[0x22000,0x2e190)` | **9,260 bytes** | **40,292 bytes** | No source credit for visually empty runs |
| Prior Agent 09 ILM | 538 bytes | 108,054 bytes | Owned by other agents |
| Envelope and layout | 35,136 bytes | — | Fixed layout/marker, not executable code |
| External header and vectors | — | — | 1,024 externally supplied scalar bytes |
| Derived DLM mirror | — | — | 2,304 dependent bytes, counted **once** |

The unique independently generated image coverage is **44,934 bytes**.
The first unresolved DLM byte remains `0x22000`. The two idle slots are in
the tail gap `[0x2e0dc,0x2e190)`, now split into `[0x2e0dc,0x2e130)` and
`[0x2e138,0x2e190)`.

**Verification:** pinned postmarketOS SHA-256
`8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5`,
23 fragments / 9,260 data bytes / **zero source mismatches**, all 12 stored
CRC fields and the 2,304-byte dependent mirror verified. The independent
Python verifier checks the emitted Rust files against its own Decimal model
and strict provenance rows. Concatenated fragment digest (by ascending offset):
`aac0ac6e0c341b0e61e3df6fe4acd39b41680b2e06f0452c5ce23df071e67a2a`.

**Integration:** This is a portable change from older-base preview
`941c805f8510c88344133687a77722ace386504e`, not a verified descendant of
checkpoint 50 `b7ed4b587633b9b475a40f5a3ace5cc90db620ff`. No other agent's
ILM source or generic signing/image builder is modified. The branch and
portable patches must be replayed and tested by Integration Agent 01 on the
real integrated base.

## Checkpoint 03 — Consumer-proven packed schedule links

An additional **68 independently generated DLM bytes** are reconstructed as
34 two-byte schedule-index fields, one in each of 17 descriptor records per
bank. These fields belong at record displacement `+0x50` in the two descriptor
banks beginning at file `0x22a64` and `0x26a64`, with stride `0x8c`. The
observed NDS32 decoder at runtime `0xc7ea–0xc87a` loads the descriptor word,
masks `0x7fff`, multiplies by four and adds DLM SRAM base `0x20000`. The
sources independently reconstruct the low halfword as
`(schedule_file_offset - 0x22000)/4`, where the wide schedule bases are file
`0x24904` and `0x28904` respectively. The two computed indices are `0x0a41`
and `0x1a41`, matching all 34 slots of the original firmware.

The **upper 16 bits** of the enclosing 32-bit records are deliberately NOT
claimed as authored source, despite their regular appearance. Their meaning
is not demonstrated by this consumer and they remain in the unknown DLM
ledger. No entire vendor records or calibration tables are copied into source
modules; each claimed fragment is exactly **two** bytes long. The semantic
model rejects misaligned, out-of-DLM or out-of-field index values. Independent
Python construction verifies exact disjoint extents and comparison against
the SHA-256 pinned postmarketOS firmware.

| Checkpoint 03 accounting | Bytes |
|---|---:|
| Independently authored DLM data | **9,328** |
| Unresolved DLM | **40,224** |
| Prior authored ILM | 538 |
| Unique source-generated image bytes including envelope | **45,002** |
| External metadata/vector scalars | 1,024 |
| Unresolved ILM (separate agents) | 108,054 |
| Dependent mirror, NOT unique source | 2,304 |
| Total image | 196,608 |

The first DLM hole remains `0x22000`. Checkpoint 03 contributes zero executable
instructions. Building a full Nabu Novatek image still requires actual ILM,
independently sourced calibration data, and a separate device-validation gate.
This remains an older-base **portable contribution**, not a descendant of
checkpoint 50. The owner of shared integration must replay these commits.

## Checkpoint 04 — Validated reference-assisted descriptor-bank relocation

The DLM contains two bank pairs with structurally identical 0x8c-byte records
except for a relocation of **nine 16-bit packed word indices** at relative
offsets `+0x48` through `+0x58`. The earlier and later record groups are:

| Earlier file interval | Later file interval | Records | Dependent bytes |
|---|---|---:|---:|
| `[0x22a64,0x233b0)` | `[0x26a64,0x273b0)` | 17 | 2,380 |
| `[0x25230,0x25460)` | `[0x29230,0x29460)` | 4 | 560 |
| **Total** | | **21** | **2,940** |

A deterministic Rust transform adds `0x1000` to each of the nine **15-bit**
word indices (because the bank moves +`0x4000` bytes and addresses are in
32-bit units). A separate high-bit flag (`0x8000`) is preserved, and the tenth
halfword remains shared between both banks. The transform rejects truncated
records or index relocation past 15 bits. It reproduces all 2,940 later-bank
bytes exactly when supplied the corresponding 2,940 original earlier-bank
bytes. The independent Python checker implements the same shift logic from
its own code, tests corrupted records, verifies against the pinned revision,
and checks that the output count cannot be falsely attributed to new source.

**Critical provenance distinction:** the 2,940 transformed bytes are
**reference-assisted**, because their unknown values still come from the
externally supplied first bank. They add **zero independently source-generated
DLM bytes**, **zero newly reconstructed executable bytes**, and **zero** to the
44/45-kilobyte unique-source numerator. The checkpoint-03 source totals remain
**9,328 DLM data bytes**, **40,224 DLM bytes unresolved**, and **45,002
unique source-generated bytes** including the independently authored envelope.

This portable, older-base branch still requires Integration Agent 01's real
checkpoint-50-rooted merge and full firmware-wide verification. Original OEM
payload bytes, certificates and keys are not present in Git or checkpoint
artifacts, and none of these results establish full device behavior.

## Checkpoint 05 — Narrow-table schedule links with byte-level flag separation

The NDS32 table consumer described in checkpoint 01 resolves the schedule
runtime pointer from a 32-bit descriptor word at relative record offset
`+0x50`, masking it with `0x7fff`, multiplying by four and adding DLM SRAM
base `0x20000`. Four 0x8c-byte records per bank beginning at file
`0x25230` and `0x29230` point to the narrow packed schedules at `0x25d4c`
and `0x29d4c` respectively. The derived word indices are `0x0f53` and
`0x1f53`.

The first two descriptor records per bank store the **unflagged** low 16 bits
of the pointer. The last two records in each bank store the index with bit 15
set. No consumer currently proves the meaning or original source of that
selector flag, so **only the lowest byte** of those four flagged records is
claimed as independently authored. The original second bytes are `0x8f`
and `0x9f`, not identical to the semantic index's `0x0f` and `0x1f`;
copying those unknown bytes or crediting them as derived source would be
incorrect. The upper 16 bits of each enclosing word remain external too.

| New owned file intervals | Known bytes | Unknown bits/bytes retained |
|---|---:|---|
| `[0x25280,0x25282)`, `[0x2530c,0x2530e)` | 4 | upper halfwords |
| `[0x25398,0x25399)`, `[0x25424,0x25425)` | 2 | selector flag and upper halfwords |
| `[0x29280,0x29282)`, `[0x2930c,0x2930e)` | 4 | upper halfwords |
| `[0x29398,0x29399)`, `[0x29424,0x29425)` | 2 | selector flag and upper halfwords |
| **Total** | **12 independently generated data bytes** | all unresolved bytes remain unknown |

The Rust generator and independent Python construction both produce the
same eight non-overlapping fragment extents from schedule addresses, without
reading any original descriptor bytes. The pinned reference matches all 12
new bytes exactly. The first DLM unknown byte is still `0x22000`.

| Updated provenance category | Bytes |
|---|---:|
| Agent 19 source-authored DLM data | **9,340** |
| Unknown DLM | **40,212** |
| Unique independently generated image bytes, including prior envelope/ILM | **45,014** |
| External metadata and vectors | 1,024 |
| Unreconstructed ILM (Agents 17/18) | 108,054 |
| Derived DLM mirror (dependent, not new source bytes) | 2,304 |
| Reference-dependent bank translation (0 new source bytes) | 2,940 verified |
| Source-authored executable bytes added by Agent 19 | **0** |

Both 12 CRC fields and dependent mirror remain checked. The independent
verification does not upgrade the unproven flag or any copied descriptor
payload. This remains a portable older-base contribution from parent
`941c805f8510c88344133687a77722ace386504e`, not a checkpoint-50
integrated or flashable firmware.

## Checkpoint 06 — 40-bit control recurrence and adjacent external record structure (UN-CREDITED)

Examining file `[0x2d3b0,0x2d450)` reveals an exact recurrence of **32
little-endian 40-bit words**: for each `i=0..7`, select `16+i` then `31-i`;
for each selector, take phase `0` and `1`. Serialize the low five bytes of
`0x8000064800 | (selector << 24) | (phase << 23)`. The 160-byte recurrence
can be emitted without copying an OEM payload; Rust and an independently
implemented Python encoder generate identical bytes. The fixed operation
field `0x8000064800` is still an **observation**, not a proven command
instruction. Neither an independent NDS32 consumer nor a documented Novatek
DSP/controller command specification has yet established what it means.

The adjacent file `[0x2d450,0x2d4be)` comprises **22** five-byte records,
all with a `0x20,0x00` prefix and a third-byte class in `{0x02,0x19,0x32}`.
Their two-byte payloads vary and are **only parsed from externally supplied
reference bytes**; they are NOT mathematically derived or embedded in Git.
The observed 110 bytes are checked for length, allowed classes, and fixed
prefix. Corrupt lengths, packet classes and prefixes are rejected.

Both spans are an **unproven candidate** useful for future controller-driver
work. They are excluded from `novatek_dlm::generated_fragments()` and
`SourceDlmData`, preserving these checkpoint-05 accounting totals exactly:

| Category | Bytes |
|---|---:|
| Independent DLM data | **9,340** |
| Unresolved DLM | **40,212** |
| Candidate recurrence re-encoded but uncredited | 160 |
| External transfer-record structure classified but uncredited | 110 |
| New source-executable bytes | **0** |

`novatek-dlm inspect-control-candidates <pinned-reference>` checks both
observations only after SHA-256 pinning the **postmarketOS** revision. The
same structures are evaluated by a separate Python model and Rust tests;
this command explicitly prints **zero additional source credit**. The
remaining DLM tables contain irregular tuning/calibration samples for which
neither authoring algorithms nor hardware calibration measurements were
provided. A reference-derived match alone must not be counted as source.

## Checkpoint 07 — Reset-compatible DLM state fields corroborated by Agent 18 ILM

Read-only inspection of Agent 18's **separately reconstructed NDS32 upper ILM**
revealed four reset operations whose typed source semantics establish both
reset values and GP-relative SRAM destinations. The pinned postmarketOS
firmware DLM contains *the same values at those startup state addresses*;
we can generate their initial values without copying an OEM payload.

| ILM typed reset source | GP store | SRAM / DLM file offset | Bytes | Generated value |
| --- | --- | --- | ---: | --- |
| `0x155e2: movi55 r0,-1` | `0x155e4: swi.gp r0,-6244` | `0x2c17c` / `0x2e17c` | 4 | `0xffffffff` |
| `0x14e16: movi55 r0,-1` | `0x14e18: swi.gp r0,-6240` | `0x2c180` / `0x2e180` | 4 | `0xffffffff` |
| `0x1570a: movi55 r1,1` | `0x15738: swi.gp r1,-6232` | `0x2c188` / `0x2e188` | 4 | `0x00000001` |
| `0x16dee: movi55 r0,-1` | `0x16df8: sbi.gp r0,-6228` | `0x2c18c` / `0x2e18c` | 1 | `0xff` |

Runtime addresses map through the independently established postmarketOS
NDS32 GP base `0x2d9e0` and DLM SRAM-to-file delta `0x2000`. For the monitor
flag, EX9.IT slot **436** is independently generated as `sbi.gp r0,-1691`;
it does not overwrite `r1=1` between `0x1570a` and `0x15738`. The optional
read-only checker `tools/novatek/verify-upper-ilm-evidence.py` requires both
Agent 18's source files, verifies each reset instruction pair and EX9 slot,
rejects modified register values/destinations, and reports the 13 file bytes.
It does *not* require or copy Agent 18's executable bytes into this branch.

The adjacent u32 at `[0x2e184,0x2e188)` also starts at `0xffffffff`, but the
observed consumer reads/increments this counter and does not independently
establish its original initial value. That word is **not credited**. Nor are
unexplained bytes `[0x2e18d,0x2e190)`, any calibration samples, or opaque
controller command words. This is correspondence of static initial values to
explicit runtime reset semantics; it is not proof that an OEM high-level
source tree has been recovered or that the complete firmware executes on
hardware.

The independent Rust/Python source generators now produce **9,353 DLM data
bytes**, a 13-byte increase over checkpoint 06, from **69 nonoverlapping
fragments**. The first unknown DLM file byte remains `0x22000`.

| Checkpoint 07 ledger category | Bytes |
| --- | ---: |
| Independently generated Agent 19 DLM data | **9,353** |
| Unresolved DLM | **40,199** |
| Unique independently generated image coverage (including inherited prefix/layout) | **45,027** |
| Pre-existing generated NDS32 ILM code credited by this agent | **0 new bytes** |
| Unknown original ILM, excluding inherited 538-byte prefix | 108,054 |
| External vector and metadata scalars | 1,024 |
| DLM dependent mirror (not counted twice) | 2,304 |
| Reference-assisted bank relocation (0 new source bytes) | 2,940 |
| Uncredited 40-bit control recurrence | 160 |
| Uncredited external 5-byte records | 110 |

**Executed validation:** SHA-256-pinned postmarketOS original verified;
all 69 rebuilt fragments byte-identical (0 mismatches), the 12 CRC fields
and 2,304-byte mirror pass, independent Python/Decimal checks pass, Rust
workspace **413 test executions passed**, Python **169 discovered, 117 passed,
52 skipped** (none of the new 3 cross-agent evidence tests skipped with source
paths supplied), and the offline locked release build passes with Rust 1.99.1.

Reproduce the cross-agent evidence gate with read-only source paths:

```sh
python3 tools/novatek/verify-upper-ilm-evidence.py \
  --upper-ilm-rs /path/to/agent18/novatek_upper_ilm.rs \
  --upper-ex9-rs /path/to/agent18/novatek_upper_ex9.rs
```

This branch remains rooted in the **older preview** at `941c805…`, not the
required checkpoint-50 integration; the contribution must be replayed to the
current verified merged base before declaring an integrated artifact.

## Checkpoint 08 — independently reproduced 110-byte control-transfer recurrence, uncredited

The 22 externally provided five-byte transfer records at file
`[0x2d450,0x2d4be)` can be **re-encoded exactly** from a compact observed
recurrence. Each record has bytes `20 00`, followed by a class byte in
`{0x02,0x19,0x32}` and a little-endian 16-bit field. The previously
source-unaccounted scalar fields follow three observed bases:

- Class `0x32`: two six-record groups from `0x80c8` and `0x9068`, with
  0x190-byte steps and index order `[4,0,1,2,3,4]`.
- Class `0x19`: a single `0x9004` record, then five payloads beginning at
  `0x9068` with index order `[0,2,1,3,5]` at the same `0x190` spacing.
- Class `0x02`: a two-record pair based on `0x4bf4` and `0x4c04`, repeated
  twice with a 16-byte difference.

The Rust `transfer_word_candidate()` and separately implemented Python
`transfer_word_candidate()` both produce exactly 110 bytes. The pinned
reference matches, and corruption of an externally supplied payload now
fails the **explicit pinned recurrence** check. The generic five-byte record
*parser* remains appropriately permissive: it recognizes the prefix and
class but does not invent a payload meaning.

The recurring stride `0x190` also occurs in Agent 18's independent NDS32
instruction source (e.g. `0x11226: addi r0,r14,0x190`), but it is **not
established** that these particular payload fields refer to the same
controller allocation. Neither the `0x20 00` header nor the fixed opcode
bits of the adjacent mirrored 160-byte words have independent semantic
specifications, so **ALL 270 candidate bytes remain UN-CREDITED**.
Neither generator is included in the source-owned `novatek-dlm fragments()`
output; the provenance manifest stays unmodified.

Validation: 414 Rust test executions, 170 Python tests discovered (118 passed,
52 skipped), offline locked release build, full pinned Novatek SHA-256,
12 CRC fields, full 2,304-byte dependent mirror, all 69 independently generated
fragment comparisons, and both uncredited candidate byte comparisons passed.

| Byte-accounting field | Unchanged checkpoint 08 value |
| --- | ---: |
| Source-generated Agent19 DLM data | **9,353** |
| Unknown DLM | **40,199** |
| Unique source-generated image bytes | **45,027** |
| Candidate control recurrence, uncredited | 160 |
| Candidate transfer records, uncredited | 110 |
| New source executable bytes | 0 |

This portable branch remains based on the older preview rather than original
checkpoint 50. Exact byte correspondence is not sufficient to claim recovered
controller logic, calibration semantics, new OEM signing, or hardware behavior.

## Checkpoint 09 — DLM calibration-profile alias, 428 dependent bytes (UN-CREDITED)

A new, separately checked dependency identifies the earlier DLM calibration
profile at file `[0x22224,0x223d0)` as byte-identical to the later profile
`[0x2d648,0x2d7f4)` (**428 bytes** in each location). SRAM ranges are
`[0x20224,0x203d0)` and `[0x2b648,0x2b7f4)`. This is distinct from the
previously established 2,940-byte descriptor *relocation* transform and
270 candidate controller-byte recurrences.

The new `novatek_dlm_calibration_alias` module takes a caller-supplied
428-byte primary profile; its dependent `duplicate_from_external_primary`
result can be used in a future full-file source-built assembler **only when
that primary profile is independently authored**. The module explicitly
classifies the pair as reference-dependent and does not promote either
source or copy to `SourceDlmData`. A named, read-only inspector recognizes
two observed little-endian 16-bit profile headers in the primary:

| Relative offset | Mapped span (observed) | Rows | Parameter | Enabled |
| --- | ---: | ---: | ---: | ---: |
| +`0x04` | 1,600 | 48 | 32 | 1 |
| +`0xd6` | 2,600 | 48 | 70 | 1 |

These values **do not independently establish** a hardware profile format
or the algorithm that produced its irregular calibration samples. Repeating
them or reading them from an original image is not high-level source recovery.
The full reference hash gate and the new alias verifier reject any differing
or truncated mirror. A distinct independent Python implementation checks
bounds, both header interpretations, 428-byte dependence, and mismatch
location. The ordinary `verify-reference` check also includes this strict
relationship.

```sh
novatek-dlm verify-calibration-alias /private/novatek_nt36523_fw.bin
python3 tools/novatek/verify-novatek-calibration-alias.py /private/novatek_nt36523_fw.bin
```

**No increase in unique source reconstruction:** 9,353 independently generated
DLM bytes, 40,199 unknown DLM bytes, zero Agent19 executable bytes, and
45,027 independently generated bytes in the inherited whole-image ledger.
The 428 newly classified dependent bytes and earlier 2,940 + 270
reference-dependent/candidate bytes are disjoint accounting observations,
not extra source-derived coverage. The first still-unreconstructed DLM byte
is at file `0x22000`; the profile samples remain an identified hard blocker
without original calibration algorithms or measurements.

Validation: Rust workspace 417 tests passed, zero failed; Python discovery
174 cases with 53 skipped (121 executed/passed) when cross-agent upper ILM
sources were supplied; offline locked release build passed; 69 generated DLM
fragments matched the pinned original with zero differences; 12 CRC fields
and the 2,304-byte DLM-prefix mirror passed; exact 428-byte repeated profile
passed the Rust and independently written Python checkers. The portable
integration base remains old preview `941c805…`, not verified checkpoint 50.

## Checkpoint 10 — source-verified DLM GP consumer map; no extra byte credit

Using Agent 18 checkpoint 02's independently typed `novatek_upper_ilm.rs` and
`novatek_upper_ex9.rs`, a new **read-only** scanner maps every explicitly
encoded GP load/store (including EX9.IT expansions) into Agent 19's checked
DLM source/unknown ledger. On the pinned sources the full map has **79 access
sites**, 37 distinct addresses, 45 reads, 34 writes, 10 expanded EX9 sites,
and width counts 35 byte / 23 halfword / 21 word. Exactly 62 sites touch
unresolved DLM bytes; 17 touch existing source-authored DLM reset fields.
A strict checkpoint02 input gate rejects shortened/mutated source, a
reclassified ledger, and invalid memory widths/locations. The independent
Python tests cover malformed EX9 inputs, GP bounds, profile hashes, and
source-credit invariance.

Two static bytes match literals written at runtime (`0x2244a = 4` and
`0x2e128 = 1`), but their ILM sources are event handlers, not unconditional
boot default setters. **Both bytes stay uncredited**. The new consumer audit
is evidence for future analysis, not new executable or static data source.

Checkpoint 10 coverage is therefore unchanged at **9,353 independently
generated DLM bytes**, **40,199 unknown DLM bytes**, **zero new executable
bytes**, and **45,027 unique source-generated image bytes**. Full pinned
reference verification, all twelve CRC fields and dependent DLM mirror,
release emitter, Rust/Python test gates and the independent consumer map
must all be passed before merging. The branch remains portable on the
older preview root, not the absent original checkpoint50 base.

## Checkpoint 11 — four exact dependent DLM bank spans, zero source-credit delta

Four distinct, nonoverlapping lower-to-upper DLM aliases are generated by
copying externally supplied earlier-bank bytes to addresses `+0x4000` higher.
The four spans have lengths **406, 228, 182 and 616 bytes**, totaling **1,432
dependent bytes**, with exact comparison to the pinned postmarketOS image.
The earlier bytes remain unknown calibration/controller data; no source
algorithm is inferred, and `novatek-dlm::fragments()` is unchanged.

`novatek_dlm_bank_aliases` and independent Python verifier validate geometry,
source-input lengths, exact mismatch offsets, overlapping claim rejection,
and that **both primary and dependent spans remain `unknown_dlm`**.
The ordinary `novatek-dlm verify-reference` now validates this dependency.
The existing original 428-byte calibration duplicate and 2,940-byte bank
relocation are disjoint from the four new aliases; they are separately
reference-dependent and not included in the 1,432-byte figure.

**Source accounting remains unchanged:** 9,353 independently generated DLM
bytes, 40,199 unknown DLM bytes, 45,027 independently generated unique image
bytes (including inherited layout and NDS32 prefix), zero new executable
bytes. The first unknown DLM file offset is still `0x22000`. This remains a
portable contribution on the older review-preview Git history, **not** the
unavailable checkpoint-50 base. Re-run tests on the correct integration base
before promoting the bundle into an official merged checkpoint.


## Checkpoint 12 — three additional uncredited bank-copy dependencies

Comparison of the pinned upper/lower DLM bank identified three more exact
unknown-only copy spans at file offsets `[0x245a6,0x24904)`,
`[0x24fc4,0x25220)`, and `[0x25c5e,0x25d4c)`, with destinations exactly
`+0x4000` higher. Their lengths 862, 604 and 238 bytes add **1,704 dependent
bytes**. Together with checkpoint 11's four original aliases, the validated
copy plan covers **seven spans totaling 3,136 dependent bytes**. Each is
entirely `unknown_dlm` in the canonical ledger, and all three deliberately
avoid the known generated index schedules, Gaussian tables and previously
validated 2,940-byte bank-index relocation.

This remains **reference-assisted dependency information**, not source data.
Both Rust and independently coded Python models compare each destination with
an externally supplied primary, reject corrupted/truncated copies and
invalidate any artificial ownership claim. Independent authoring stays
**9,353 DLM data bytes** and **40,199 unknown DLM bytes**, with **zero new
executable bytes**. The checkpoint was built on old-preview ancestry; its
portable mbox should be applied by Agent 01 to a proven integrated base.

## Checkpoint 13 — indexed descriptor geometry, 42 pointers, and 12-byte dependent signature

Independent Rust and Python verifiers now check the first 15-bit index at
relative `+0x48` within **42** table records across two groups and two
banks. For each group the index, multiplied by four and added to file
`0x22000`, reaches the end of the 140-byte record array. The unproved high
selector bit is masked and never credited. In addition, the 12-byte profile
signature at `[0x22710,0x2271c)` matches `[0x2df8c,0x2df98)`. These
are reference-assisted observations—not independently recovered controller
values—and all 96 descriptor pointer bytes plus both signature copies remain
`unknown_dlm`. The prior seven copy aliases, 2,940-byte relocated descriptor
banks and 428-byte calibration alias keep their existing separate claims;
none are promoted to source. The current 9,353 generated DLM bytes and
40,199 unresolved DLM bytes are unchanged. The pinned reference, strict
metadata/envelope, all 12 CRC fields, mirror, Rust/Python tests and release
build are rechecked at each checkpoint. See
`docs/history/phase-2/novatek-dlm/dlm-descriptor-geometry.md` for formulas and method limits.

## Checkpoint 14 — typed absolute DLM address cross-references, zero source delta

Agent 18's independent upper-ILM and EX9 sources contain 34 explicit
DLM SRAM address constants: 28 upper-ILM MOVI instructions and six EX9
entries. Converting SRAM pointers with the `+0x2000` DLM file bias and
classifying them against Agent 19's ledger yields 27 references to
17 distinct `unknown_dlm` addresses and seven references into existing
source-authored DLM structures. This is separate from the 79 GP accesses:
an immediate pointer **does not prove an actual read/write or initializer**.

A standalone Python report and tests verify source-address mapping, EX9
slot positions, duplicate/truncated input rejection, provenance integrity,
and the pinned Agent18 checkpoint02 snapshot. The Agent18 code itself stays
outside Agent19's Git bundle. **No source bytes are credited**: 9,353
independently generated DLM bytes remain, 40,199 are unknown, and zero
new executable bytes are added. See
`docs/history/phase-2/novatek-dlm/dlm-absolute-literals.md`.

## Checkpoint 15 — Fail-closed sparse DLM reconstruction with reference-dependent seeds

- Added a true conditional DLM linker that uses existing independently authored
  9,353 data bytes and all previous alias/relocation rules. It accepts 37
  explicit disjoint external seed intervals totaling 33,723 bytes, and derives
  6,476 other unknown bytes by genuine pointer/index relocation and checked
  bank/profile copying. It produces **49,552 exact DLM bytes** only when all
  external seed bytes are supplied, and never counts dependent bytes as source.
- Descriptor transform spans cover 2,940 physical bytes; 40 schedule-link bytes
  were already authored and are now mandatory source/derived equality checks.
- New `novatek-dlm sparse-plan`, `assemble-sparse`, and `verify-sparse-rebuild`
  modes; no original firmware is embedded and missing external data is not
  silently zero-filled. The output is DLM-only, not a flashable image.
- Independent Python implementation validates the complete 37-range plan and
  bytes against the pinned reference and rejects adversarial/malformed inputs.
  Synthetic test proves 189 pointer halfwords are actually relocated, rather
  than copied or left zero.
- **Independently authored DLM bytes remain 9,353; unresolved original DLM
  data remain 40,199 (6,476 strictly dependent + 33,723 externally needed).**
  Firmware source-level implementation remains incomplete; avoid representing
  this exact conditional DLM output as a reference-free build.


## Checkpoint 16 — Reference-dependent 96-byte record suffix copies

- The previous spare DLM assembler is extended by 23 exact-copy relationships
  over 96-byte controller records, copying only suffix `[+9,+96)` of each.
  Two additional early records fall inside preexisting bank aliases and
  are excluded. Byte 0–8 of all remaining records still require external
  values: no controller opcode or configuration bits are guessed.
- This re-derives **2,001 additional dependent bytes** from externally supplied
  earlier records and decreases the minimal unknown seed requirement from
  33,723 to **31,722 bytes**, across **60 explicit seed ranges**.
- Category accounting is now **9,353 independent DLM source bytes**,
  **8,477 uniquely derived dependent DLM bytes**, **31,722 external unknown
  DLM bytes** = exactly **49,552**. The original unresolved/source-unknown
  DLM count remains **40,199**, since every dependent copy also has unknown
  source provenance. **New independently generated source credit is zero**.
- Rust and a separately written Python model produce byte-exact conditional
  DLM output against SHA-256 pinned postmarketOS firmware and reject changed
  copies, malformed seed counts and overlapping aliases; no vendor payload
  is packaged in Git or deliverables.


## Checkpoint 17 — Dependent heads/tails of thirty 40-byte DLM records

- Added a structurally checked inter-bank rule for 30 consecutive 40-byte
  descriptor records at first bank file `0x233b0`. In each record, bytes
  `+0..+23` and `+31..+39` are exact copies in the upper bank at `+0x4000`;
  **seven middle bytes remain unknown and externally supplied**, because
  they include pointers/flags whose firmware semantics remain unproven.
- This generates 60 disjoint new dependent spans totaling **990 bytes**,
  without overlap with existing aliases or authored DLM fragments.
- Current *conditional* DLM reconstruction (49,552 bytes) uses **9,353
  source-generated DLM bytes**, **9,467 uniquely dependent copied/relocated
  bytes** and **30,732 external seed bytes** in **90 ranges**. All output bytes
  match the pinned original, and independently written Python and Rust
  reconstruction paths agree.
- **New source credit remains zero**. The larger unresolved DLM source ledger
  remains at **40,199 bytes**. Proving a repeated bank-copy relationship does
  not establish independent authorship of its unknown primary values.

## Agent 20-19 checkpoint 18 — mutable 40-byte record relocations

- Reconstructed all **210 dependent** mutable bytes in the upper bank of 30
  40-byte record pairs (`+24..+30`), from externally supplied primary-bank
  values; prior checkpoint 17 covered the other 33 bytes of each record.
- Two 16-bit indices per record increment by `0x10` for records 1–8, by
  `0x64` for records 9 and 11, and by `0xc8` for the remaining records
  except record 20 (identity). The seventh byte acquires bit `0x04` only
  for records 12–19. Two unchanged bytes in the middle are copied as opaque.
- These are observed **bank dependencies**, not source-derived initializer
  values or reverse-engineered controller semantics. Zero new source credit.
- Conditional DLM rebuilder: independently authored 9,353, uniquely derived
  **9,677**, external unknown **30,522 bytes** in **60 ranges**. Sum is
  exactly 49,552. The source-authored ledger remains unchanged at 9,353;
  unresolved independent source still totals 40,199 bytes.
- The Rust/independent Python models reject bad span IDs, lengths, missing
  seeds and overflowing relocations; exact private reference comparison and
  CRC/mirror verification are required on integration. All original firmware
  stays outside Git.
- This remains based on the older-base preview `941c805…`, not true
  checkpoint 50. Bring the portable mbox to the integration owner for
  replay and rerun the complete CI gates there.

## Agent 20-19 checkpoint 19 — narrow 4-entry and 18-record bank relocations

- Rebuilt **764 more dependent DLM bytes**, using two independent source-bank
  spans and two upper-bank destinations; **zero independently authored bytes**
  added. Four bounded descriptor pointers shift their low 15-bit word indices
  by `0x1000` between banks. In an adjacent 748-byte window, two u16 indices
  shift by `0x10` in 16 of 18 narrow controller records; special records
  8 and 9 remain identical. Other fields are copied from externally supplied
  lower-bank data, never hardcoded from OEM payloads.
- New dependent destinations `[0x29220,0x29230)` and `[0x29460,0x2974c)`
  are disjoint from preexisting mirrors/aliases/relocations and source data.
  Their file offsets and SRAM mapping (file minus `0x2000`) are individually
  checked. Both original lower-bank intervals remain **unknown source**, so
  this does **not** count as source-level reconstruction.
- Sparse DLM plan: **9,353 source-generated + 10,441 dependent + 29,758
  external = 49,552** bytes, with **58 external ranges** and 40 additional
  source/dependent cross-checks. Source-unknown DLM stays **40,199 bytes**.
- The conditional reassembly, strict postmarketOS hash check, CRC-32/MPEG-2,
  DLM mirror, independent Python model, and external-seed/malformed-input
  regressions must continue passing before promotion to integration.
- See `docs/history/phase-2/novatek-dlm/dlm-narrow-bank-relocation.md` for evidence, complete
  transformations, failure cases, and reproducible commands. Branch still
  derives from old integration preview, not checkpoint-50.

## Agent 20-19 checkpoint 20 — three chained, uncredited DLM bank-copy windows

- A read-only comparison of **both pinned Novatek revisions** confirms
  identical zero-valued windows `[0x2271c,0x2286c)`, `[0x22875,0x22a00)`
  and `[0x262ec,0x26a00)` at destination file `+0x4000`.
- Added **2,543 reference-dependent bytes** to the sparse DLM linker; the
  primary spans still require external source and **zero new independent
  bytes** are credited. The linker copies unknown data, never silently fills
  regions with zeros. Synthetic nonzero-source tests confirm dependency
  propagation through the two-stage three-bank chain and reject altered or
  absent input.
- Conditional exact DLM rebuild now uses **9,353 authored**, **12,984
  dependent**, and **27,215 external seed** bytes in **60 seed ranges**.
  The underlying 40,199 source-unrecovered DLM bytes remain unchanged.
- Full Rust/Python/release and original reference checks remain required.
  See `docs/history/phase-2/novatek-dlm/dlm-zero-bank-aliases.md` for the exact bounds,
  cross-version checks, provenance limitations and reproducer.
- Agent 19 remains a portable branch from the older integration preview,
  not a descendant of the missing checkpoint-50 Git commit.

## Agent 20-19 checkpoint 21 — pinned external seed SHA-256 manifest

- Added a strict **60-range digest manifest** for all 27,215 externally supplied bytes needed by the conditional DLM linker. This is revision-specific to the pinned postmarketOS image (`8b8455…b70b5`) and records **only SHA-256, boundaries and provenance; no OEM payload**.
- New `novatek-dlm seed-manifest` and `verify-seed-manifest <pinned-image>` commands; `assemble-sparse` and `verify-sparse-rebuild` validate the manifest and all seed digests *before* generating output. A synthetic-unpinned library API remains for unit tests but the user CLI fails closed.
- Separate Python `hashlib` implementation verifies the plan and supplied image/seed-directory, and tests reject altered profiles, changed SHA, moved/reordered/missing ranges and content mutations. An actual mutated private seed was rejected before creating an output file.
- Independently source-authored DLM **remains 9,353 bytes**, reference-dependent **12,984 bytes**, external **27,215 bytes**, 60 seed ranges. These hashes are integrity constraints and do **not** recover firmware source or prove OEM authenticity.
- See `docs/history/phase-2/novatek-dlm/dlm-pinned-seeds.md`; original firmware/seed directory and rebuilt conditional DLM remain private and are never committed.

### Checkpoint 21 executed gates

The workspace test run completed with **449 Rust tests passed**, **215 Python test cases discovered (58 skipped, 157 executed/passed)**; the full offline workspace release build passed under Rust 1.99.1. The release-built CLI accepted all 60 exact external seeds and emitted a 49,552-byte conditional DLM matching the original image (DLM SHA-256 `c09ed40d0f123b7680a12097fb329ff35515e5936abd7a3b4526b042fc689591`). An altered private seed failed before an output file was created. The CLI rejected the older map220v firmware revision before seed comparison. No private image, unknown seed blob or rebuilt conditional full DLM is shipped in this checkpoint.

## Agent 20-19 checkpoint 22 — staging-bank boundary copies, zero new source credit

- Added two additional lower-to-upper DLM staging dependencies: 24 bytes `[0x23860,0x23878)` → `[0x27860,0x27878)` and 98 bytes `[0x23a13,0x23a75)` → `[0x27a13,0x27a75)`. The earlier 406-byte codec-bank alias ends at `0x23a12`, so the one-byte mismatch at that boundary remains external; a four-byte preceding gap is also left external.
- Both pinned postmarketOS and older map220v Novatek firmware revisions satisfy the 122-byte relations. Rust and independent Python synthetic tests propagate nonzero primary values and reject tampering/truncation. The original primary data remain unknown; **zero new independently authored bytes**.
- Total DLM plan is now **9,353 independently authored source bytes + 13,106 source-dependent bytes + 27,093 external required bytes = 49,552**; **61 exact external seed ranges**. The strict SHA-256 manifest is updated and rejects the previous 60-range layout or wrong firmware revision.
- The source-unknown ledger still reports **40,199 DLM bytes**. This checkpoint refines reference-conditional reconstruction, not compiler-producible source or OEM firmware authentication. See `docs/history/phase-2/novatek-dlm/dlm-staging-extensions.md`.

### Checkpoint 22 executed gates

The full offline workspace test suite passes **451 Rust tests** and **219 Python discovered cases (58 skipped, 161 executed/passed)**. The workspace release build succeeds. Release-built `novatek-dlm verify-sparse-rebuild` and `verify-seed-manifest` pass on the pinned postmarketOS reference. The release `assemble-sparse` command accepts all 61 checked private inputs and produces the same SHA-256 `c09ed40d0f123b7680a12097fb329ff35515e5936abd7a3b4526b042fc689591` as checkpoint 21's full 49,552-byte conditional DLM output. Rust/Python synthetic tests and the independent cross-revision script verify both new staging dependencies without encoding reference payload as source.
