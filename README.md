# Xiaomi Nabu firmware reconstruction research (partial)

**Status: 1 of 9 regular files byte-identically reproduced from newly authored Rust code; 8 of 9 NOT reconstructed. This is NOT a firmware replacement.**

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

As an additional practical toolchain limitation, Rust 1.99.1 here lists no
`hexagon-*` or Cirrus HALO compilation target in `rustc --print target-list`.
A C-like Hexagon decompilation cannot simply be fed to standard `rustc` to
produce executable Hexagon firmware; an appropriate Rust backend/runtime and
platform-specific linker/image builder would be needed even if the program
logic had been independently reimplemented.

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
