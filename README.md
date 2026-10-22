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

REA produces *signature candidates* and possible embedded payloads. The 234 reported matches do **not** mean 234 extracted functions or recoverable sources; many are incidental short zlib and GPG-looking byte sequences. `a640_zap.mbn` is an ELF32 Qualcomm Hexagon executable. REA/Ghidra reported only one entry function, with an invalid-instruction warning, before terminating the recovered control flow with `halt_baddata()`. No valid executable reconstruction followed. `venus.mbn` is a stripped ELF32 ARM executable; its REA/Ghidra overview attempt exceeded the session's time limit.

Some binaries include Xiaomi-related certificate information. A byte-identical proprietary signed image cannot be asserted from recovered pseudocode. The original source, a precise linker/assembler/compiler environment and possibly authorization for firmware signing are necessary to develop and test an equivalent replacement; modified signed binaries may need valid new signatures. The files have no normal ELF section tables, making reconstruction more difficult.

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

For native decompilation, separately set `GHIDRA_INSTALL_DIR` to the extracted Ghidra directory and use REA's `analyze ... --provider ghidra` and `decompile ... --provider ghidra`. A Ghidra analysis of a large Hexagon/ARM firmware may take much longer and still cannot reconstitute OEM firmware source. Inspect results in `evidence/assessment.json` and `evidence/rea/*.json`; `evidence/apk-files.sha256` contains each file's expected SHA-256.

## Licensing and Fedora / Pocketblue

- **GPL-3.0-only** covers the newly authored Rust sources and project scripts, not the vendor firmware. No OEM firmware binaries are included in the Git history or bundle.
- The input postmarketOS package declares **`license = proprietary`**. Reversing or regenerating its byte content would not, by itself, confer GPLv3 rights over the underlying vendor code.
- Fedora has a [separate firmware policy](https://fedoraproject.org/wiki/Licensing:SoftwareTypes) permitting some binary-only firmware *under acceptable firmware licenses*. That is a licensing/permission review, not something a GPL label or binary-to-Rust conversion can replace.
- The nine-file APK declares dependencies on `linux-firmware-ath10k`, `linux-firmware-qcom` and `linux-firmware-qca` which are **not** the same as the nine payload files. Do not treat this repo as a complete device firmware tree.

## What would be required for the requested full rebuild?

Authoritative source and permission for the proprietary components (or a clean-room, independently implemented replacement), matching Qualcomm/other-vendor compilers and linker scripts, image/layout details, required cryptographic signing authority for modified signed images, hardware test equipment or test devices, and a complete cryptographically verified reference build. None is established by the current reverse-engineering results. Independent reverse engineering may produce an interoperable GPL implementation eventually, but it is not demonstrated here.

## Provenance

- postmarketOS package: `firmware-xiaomi-nabu-1-r2.apk` dated 2026-10-13, `license = proprietary` according to its `.PKGINFO`.
- Firmware Git repository: `https://gitlab.postmarketos.org/panpanpanpan/nabu-firmware`, source commit `181e8e4510b28ef13d2adde3017fa87eb3eda490` (separate from packaged binary set).
- REA 6.7.0 (`morluto/rea`), Binwalk 3.1.0, Ghidra 12.1.4, Rust 1.99.1.
- Evidence is derived from proprietary input and is NOT GPL-relicensed OEM source. Read `evidence/rea/NOTICE`.

Results documented October 22, 2026. No devices have been flashed or tested, and no complete executable replacement has been built.
