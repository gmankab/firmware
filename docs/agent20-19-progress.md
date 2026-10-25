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
python3 tools/verify-novatek-dlm.py \
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

The 52 Python skips belong to pre-existing A640/ADSP/CDSP/modem/WLAN checks requiring separately supplied firmware and/or assembler tools. Their exact test names and reasons are included in `agent20-19-checkpoint-01-verification.json` and the handoff logs. No new Agent 19 test is skipped in the recorded full run. A skipped test is not counted as passing.

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
