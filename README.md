# Xiaomi Nabu firmware reconstruction research (partial)

## Latest: post-toolchain continuation (2026-10-22)

The separately provided LLVM 6.0.1 and Rizin 0.9.1 tools run on Debian 13. LLVM 6 recognizes only **270 of 492** A640 PF_X four-byte words as nominal Hexagon V4 instructions. **A640 is not yet validly decompiled**; misidentifying the microcode ISA would corrupt any attempted Rust rewrite. We added analysis-only ELF section views, input-provenance verification (including a corrected Novatek version), and a QRISC/IR3 hypothesis test for the Adreno A640. See [`evidence/freedreno-isa-followup.md`](evidence/freedreno-isa-followup.md).

The Codex toolchain downloader [`scripts/codex-download-freedreno-ubuntu24.04.sh`](scripts/codex-download-freedreno-ubuntu24.04.sh) targets **Ubuntu 24.04** and pins Mesa 26.2.4 to its published SHA-256. Its returned `qrisc-disasm`, `qrisc-asm` and `ir3-disasm` can be probed against an original A640 file **on Debian** using [`scripts/analyze-a640-mesa.sh`](scripts/analyze-a640-mesa.sh). The download/build script has been syntax-checked but **has not yet been run on Ubuntu**. Both an A6xx decoding match and a vendor-payload-free reimplementation remain unproven.

**Full exact-build status is unchanged: 1 of 9 files.** The six matched 4 KiB ELF pages and 128 matched WMFW bytes are metadata only, not independently rebuilt firmware.

**Status: 1 of 9 regular files byte-identically reproduced from newly authored Rust code; 8 of 9 NOT reconstructed. This is NOT a firmware replacement. The Hexagon compiler follow-up is described below.**

This repository contains GPL-3.0-only Rust **tooling** and a structured rebuild of the small `modemuw.jsn` text configuration, plus REA / Ghidra research evidence about the firmware in the postmarketOS `firmware-xiaomi-nabu` **1-r2** Alpine package. It deliberately contains **no original proprietary firmware binaries**, and no claim that proprietary firmware has become GPLv3. See the [complete GPLv3 text](LICENSE).

## Results

| APK file | Length | REA Binwalk matches | Rebuilt bit-for-bit? |
| --- | ---: | ---: | --- |
| `cirrus/cs35l41-dsp1-spk-prot.wmfw` | 34,236 | 0 | No |
| `novatek/novatek_nt36523_fw.bin` | 196,608 | 0 | No |
| `qcom/sm8150/Xiaomi/nabu/a640_zap.mbn` | 14,256 | 0 | No |
| `qcom/sm8150/Xiaomi/nabu/adsp.mbn` | 14,430,684 | 80 | No |
| `qcom/sm8150/Xiaomi/nabu/cdsp.mbn` | 9,455,252 | 72 | No |
| `qcom/sm8150/Xiaomi/nabu/modem.mbn` | 5,101,612 | 31 | No |
| `qcom/sm8150/Xiaomi/nabu/modemuw.jsn` | 713 | 0 | **Yes** |
| `qcom/sm8150/Xiaomi/nabu/venus.mbn` | 1,159,200 | 1 | No |
| `qcom/sm8150/Xiaomi/nabu/wlanmdsp.mbn` | 4,331,136 | 50 | No |

REA produces *signature candidates* and possible embedded payloads. The 234 reported matches do **not** mean 234 extracted functions or recoverable sources; many are incidental short zlib and GPG-looking byte sequences. `a640_zap.mbn` is an ELF32 Qualcomm Hexagon executable. REA/Ghidra reported only one entry function, with an invalid-instruction warning, before terminating the recovered control flow with `halt_baddata()`. No valid executable reconstruction followed. `venus.mbn` is a stripped ELF32 ARM executable: a subsequent direct Ghidra headless run decompiled 1,588 of 1,593 identified functions. Earlier REA/Ghidra overview sessions timed out.

All six Qualcomm `.mbn` images contain MDT authentication/hash segments with Xiaomi-related X.509 certificate-chain information. These are not signing keys; their presence does not prove whether this particular device checks or can bypass signatures. A byte-identical proprietary signed image cannot be asserted from recovered pseudocode. The original source, a precise linker/assembler/compiler environment and possibly authorization for firmware signing are necessary to develop and test an equivalent replacement; modified signed binaries may need valid new signatures. The files have no normal ELF section tables, making reconstruction more difficult.

**Follow-up correction:** the supplied Rust 1.99.1 toolchain DOES list
`hexagon-unknown-none-elf`, `hexagon-unknown-qurt`, and
`hexagon-unknown-linux-musl`. We built a separate GPLv3 `no_std`
Hexagon static library offline by rebuilding `core` using the supplied
Rust source and vendored dependencies. See
`evidence/additional-tooling-followup.md`. This proves that Hexagon
Rust machine-code generation is possible here; it does **not** imply
the original Hexagon binaries can be regenerated from Ghidra C text,
or that HALO/Novatek, signing, layout and firmware-runtime requirements
are addressed.

## Tested partial build

Requires Rust 1.99.1 or another sufficiently recent Rust toolchain; no Cargo dependencies and no network access needed.

```bash
cargo test --locked --offline
cargo build --release --locked --offline
mkdir -p out
./target/release/nabu-firmware-research rebuild-modemuw out/modemuw.jsn
```

`out/` must exist before invoking `rebuild-modemuw` (or pass any existing directory in the destination path). Alternatively, use:

```bash
mkdir -p out
./target/release/nabu-firmware-research rebuild-modemuw out/modemuw.jsn
./target/release/nabu-firmware-research verify-modemuw \
  /path/to/extracted/usr/lib/firmware/qcom/sm8150/Xiaomi/nabu/modemuw.jsn
scripts/verify.sh /path/to/extracted/usr/lib/firmware
```

The 713-byte output is byte-for-byte equal to the APK version, SHA-256:

`e75d94b61df3e86c5b3f7a976d93f726cb85f2b897a8ea80a0924b9bb9c1a3bf`

The configuration is authored as typed Rust structures with deterministic formatting; this does **not** constitute decompilation or replacement of executable firmware. The Rust ELF32 parser can inspect segment layouts:

```bash
./target/release/nabu-firmware-research inspect-elf32 /path/to/a640_zap.mbn
```

## Inspect all formats (new)

The GPLv3 Rust executable can enumerate the ELF32/Qualcomm MDT program
headers and the candidate authentication segments, independently decode the
Cirrus WMFW v3 HALO container and its 11 region descriptors, and inspect the
Novatek 0x200-byte header using a **tentative** NT36xxx-family partition
format. No DSP instruction sets, linker scripts, original firmware bytes, or
cryptographic keys are embedded in the Rust source.

For `adsp.mbn`, where the image's physical entry point differs from its
virtual memory mappings, an additional **analysis-only bootstrap ELF view**
can be generated. The helper is written entirely in Rust, does not alter the
input and emits an untrusted temporary derivative that **must never be
flashed or redistributed as GPLv3 firmware**:

```bash
./target/release/nabu-firmware-research make-ghidra-bootstrap \
  /path/to/adsp.mbn /tmp/adsp-bootstrap-ANALYSIS-ONLY.mbn
```

A Ghidra analysis of this view found 29 candidate functions and emitted
pseudocode for 25. The complete original ADSP file contains far more code;
the view exposes only the first executable bootstrap segment so Ghidra's
address translation can be investigated without scanning the entire image.


```bash
cargo test --offline --locked
cargo build --release --offline --locked
scripts/inspect-all.sh /path/to/extracted/usr/lib/firmware /tmp/nabu-layouts
```

Novatek partition labels (ILM/DLM) follow a *related* Novatek driver; the
NT36523 interpretation has not been verified and the architecture is unknown.
The Cirrus image is HALO DSP format, not ARM or Hexagon and cannot be fed to
the Ghidra Hexagon decompiler as if it were Qualcomm machine code.

## Re-run function-level decompilation

The repository includes a dedicated Ghidra headless `DecompileNabu.java`
script. It exports per-function **C-like pseudocode** for the six Qualcomm
images. Output is not executable Rust, and recovery counts are not proof of
correctness. Generated OEM-derived pseudocode is intentionally **outside the
GPLv3 Git repository**, in a separate research-evidence archive.

```bash
cargo build --release --offline --locked
scripts/decompile-all.sh /path/to/extracted/usr/lib/firmware \
  /path/to/ghidra_12.1.4_PUBLIC /tmp/nabu-ghidra-results
```

To retry huge images after the first pass, use
`scripts/continue-decompilation.sh ORIGINAL_FIRMWARE_ROOT GHIDRA_DIR OUTPUT_DIR`.
Resource budgets are controlled by `ANALYSIS_SEC`, `DECOMP_SEC` and
`GHIDRA_HEADLESS_MAXMEM`. Large images need more than a single 95-second
function pass. For reproducibility, keep the original firmware package
unchanged and compare files to `evidence/apk-files.sha256`.

## 2026-10-22 additional-tooling follow-up

The supplied Rust sources and vendored dependencies enabled a successful
**offline bare-metal Hexagon Rust compilation test**, independent of any
proprietary firmware. `scripts/build-hexagon-probe.sh` reproduces that result.
A second Ghidra Hexagon decoder did not resolve the A640 entry-point error.
The remaining original firmware outputs are **not** byte-identically rebuilt.
Read `evidence/additional-tooling-followup.md` for details.

## Signing and Nabu boot

Linux `request_firmware()` can return unsigned bytes when no higher-layer
signature enforcement is configured. In the Qualcomm PAS paths, however,
TrustZone/secure firmware authenticates the coprocessor images before boot.
The Adreno A640 ZAP loader also has a PAS authorization path. Skipping
signing is safe for **static research**, not a demonstration that the
resulting executable firmware boots on stock Nabu hardware. Linux itself may
boot with nonessential coprocessors offline, but modem/audio/Wi-Fi/video/GPU
subsystems can fail, and a working display is not guaranteed. The device was
not flashed or boot-tested. Unlocking the Android bootloader does not imply
that independent subsystem firmware signature enforcement is disabled.

## Re-run REA observations

The input bundle included `rea-agents` 6.7.0, Ghidra 12.1.4, Binwalk 3.1.0 and offline dependencies for Ubuntu 24.04. Those binaries/tools are not vendored in this Git bundle; obtain or unpack the separate original download bundle. After making the tools available:

```bash
export REA_BINWALK_COMMAND=/absolute/path/to/binwalk
export REA_NODE_COMMAND=/absolute/path/to/node
scripts/run-rea-scans.sh \
  /path/to/extracted/usr/lib/firmware \
  /path/to/rea-agents/scripts/rea.mjs \
  /path/to/new-results
```

REA can also invoke Ghidra when `GHIDRA_INSTALL_DIR` is configured. For exhaustive offline exports the included direct Ghidra script avoids session-level limits. A Ghidra analysis of a large Hexagon/ARM firmware may take much longer and still cannot reconstitute OEM firmware source. Inspect results in `evidence/assessment.json` and `evidence/rea/*.json`; `evidence/apk-files.sha256` contains each file's expected SHA-256.

## Licensing and Fedora / Pocketblue

- **GPL-3.0-only** covers the newly authored Rust sources and project scripts, not the vendor firmware. No OEM firmware binaries are included in the Git history or bundle.
- The input postmarketOS package declares **`license = proprietary`**. Reversing or regenerating its byte content would not, by itself, confer GPLv3 rights over the underlying vendor code.
- Fedora has a [separate firmware policy](https://fedoraproject.org/wiki/Licensing:SoftwareTypes) permitting some binary-only firmware *under acceptable firmware licenses*. That is a licensing/permission review, not something a GPL label or binary-to-Rust conversion can replace.
- The nine-file APK declares dependencies on `linux-firmware-ath10k`, `linux-firmware-qcom` and `linux-firmware-qca` which are **not** the same as the nine payload files. Do not treat this repo as a complete device firmware tree.

## Native decompilation results

See `evidence/ghidra-summary.md` for run-by-run counts, failure modes,
architectures and findings. Even thousands of recovered functions cannot
recover the original compiler/linker configuration or synthesize a byte-for-byte
signed and executable Rust replacement. Signing is deliberately **not
attempted** here.

## What would be required for the requested full rebuild?

Authoritative source and permission for the proprietary components (or a clean-room, independently implemented replacement), matching Qualcomm/other-vendor compilers and linker scripts, image/layout details, required cryptographic signing authority for modified signed images, hardware test equipment or test devices, and a complete cryptographically verified reference build. None is established by the current reverse-engineering results. Independent reverse engineering may produce an interoperable GPL implementation eventually, but it is not demonstrated here.

## Provenance

- postmarketOS package: `firmware-xiaomi-nabu-1-r2.apk` dated 2026-10-13, `license = proprietary` according to its `.PKGINFO`.
- Firmware Git repository: `https://gitlab.postmarketos.org/panpanpanpan/nabu-firmware`, source commit `181e8e4510b28ef13d2adde3017fa87eb3eda490` (separate from packaged binary set).
- REA 6.7.0 (`morluto/rea`), Binwalk 3.1.0, Ghidra 12.1.4, Rust 1.99.1.
- Evidence is derived from proprietary input and is NOT GPL-relicensed OEM source. Read `evidence/rea/NOTICE`.

Results documented October 22, 2026. No devices have been flashed or tested, and no complete executable replacement has been built.

## 2026-10-22 partial progress toward exact reconstruction (not firmware)

The repository now independently reproduces the **first 4,096 bytes** of each
of the six ELF32 files from declarative Rust ELF/program-header records
(`src/elf32_prefix.rs`, `src/elf_specs.rs`). The complete executable firmware
is **not** reconstructed. No OEM instruction, hash or signature bytes are
embedded in the GPL-licensed Rust. The original `modemuw.jsn` exact build
remains valid.

```bash
cargo test --locked --offline
cargo build --release --locked --offline
./target/release/nabu-firmware-research rebuild-elf-prefix a640_zap.mbn /tmp/a640-HEADER-ONLY.bin
./target/release/nabu-firmware-research verify-elf-prefix a640_zap.mbn /path/to/original/a640_zap.mbn
scripts/elf-byte-budget.py /path/to/original/qcom/sm8150/Xiaomi/nabu evidence/elf-byte-budget.json
```

`/tmp/a640-HEADER-ONLY.bin` is an **incomplete 4 KiB metadata page**, not
firmware and must never be flashed. For the executable-disassembly prerequisites,
see `evidence/next-reconstruction-plan.md` and the Ubuntu-only download/build
script `scripts/codex-download-next-toolchain-ubuntu24.04.sh`.

The public Linux WMFW format also permits deterministic regeneration of
**128 bytes of Cirrus metadata** (40-byte header + eleven 8-byte descriptors);
`verify-wmfw-structure` compares these bytes to the local reference file.
This does not reproduce any HALO DSP code. All 13 unit tests pass on the
extracted Rust 1.99.1 toolchain.

```bash
./target/release/nabu-firmware-research verify-wmfw-structure /path/to/original/cirrus/cs35l41-dsp1-spk-prot.wmfw
```
