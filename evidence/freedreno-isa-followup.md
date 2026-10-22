# Nabu exact-rebuild continuation: LLVM 6, firmware provenance, Adreno ISA probes

Date: 2026-10-22. All work described here uses user-provided local firmware and tooling. No original firmware payload or vendor-derived pseudocode is tracked in this GPL-3.0-only Git repository.

## Input provenance (important correction)

The base Alpine gzip/TAR `firmware-xiaomi-nabu-1-r2.apk` contains **nine** firmware files. Its nine APK-extracted payload hashes were cross-checked against `evidence/apk-files.sha256` and against checked-out source copies. Eight correspond exactly to the `map220v-nabu-firmware` source directory. The exception is **`novatek_nt36523_fw.bin`**: that repository has a different binary revision. The `postmarketos-nabu-firmware` source copy has APK-matching SHA-256 **`8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5`**. Using the other Novatek copy invalidates byte-for-byte comparisons even if a reconstruction is perfectly accurate.

To check the correct inputs (outside the repository), run:

```sh
python3 scripts/verify-firmware-provenance.py /path/to/workspace/sources \
    --apk /path/to/firmware-xiaomi-nabu-1-r2.apk --output /tmp/provenance.json
```

The validator rejects hash mismatches and missing inputs. It does not create a new firmware or check licensing rights.

## A640: verified failure of the V4 instruction-decoding hypothesis

Tooling from `nabu-next-toolchain-ubuntu24.04.tar.zst` passes its SHA-256 manifest checks and exposes functional **LLVM 6.0.1** and **Rizin 0.9.1** on Debian 13. LLVM 6 successfully decodes its own synthetic/assembled Hexagon V4 example. `a640_zap.mbn`, however, yields only **270 recognized** out of **492** nominal four-byte words in its **1,968-byte PF_X** segment; **222** show as `<unknown>`. The nominal address is `0x5000`, file offset `0x3000`, and SHA-256 of the extracted segment is `e24a4f483b71ea19715910f01e1abf94280756df731cad05df48fd7714e34198`.

The ELF advertises Qualcomm `EM_HEXAGON` and `e_flags=0x3`, but the *meaning* of its PF_X bytes has **not** been independently established. Apparent partial decoder success does not demonstrate Hexagon V4 firmware control flow. Ghidra, Rizin, and the legacy LLVM decoder have not produced a reliable executable translation. In particular, avoid turning `llvm-objdump` lines into GPL Rust source as though these were verified functions.

A helper exposes PF_X segments as read-only analysis sections, without embedding original bytes into the GPL project:

```sh
python3 scripts/elf-segment-view.py /path/to/a640_zap.mbn /tmp/a640-ANALYSIS-ONLY.elf
```

It produces a **derivative private research file**, not a redistributable or bootable firmware. Unit tests use synthetic ELF bytes.

## Next independent instruction-set hypothesis: Freedreno's GPU ISA tools

`a640_zap.mbn` is an Adreno A640 GPU ZAP image. Mesa includes `qrisc-disasm`/`qrisc-asm` for Adreno command-processor microcode (QRISC/SQE) and `ir3-disasm` for shaders. **This is a hypothesis test, not proof that ZAP PF_X bytes belong to QRISC or IR3.** Mesa names A630/A650/A660 QRISC firmware IDs; it has no explicitly named A640 profile. Test all three A6xx variants and compare their output with negative controls. In particular use `qrisc-disasm -r -g <hex-id>` to avoid firmware-header auto-detection on bare data and to suppress interpreting the raw segment as a jump-table firmware image.

Official Mesa 26.2.4 source, SHA-256 `bce5f7fbebb934373b86c999a064d52fb5065878dc57f287f95346648ec832e9`:

- https://archive.mesa3d.org/mesa-26.2.4.tar.xz
- https://fossies.org/linux/misc/mesa-26.2.4.tar.xz/mesa-26.2.4/src/freedreno/qrisc/disasm.c
- https://fossies.org/linux/misc/mesa-26.2.4.tar.xz/mesa-26.2.4/src/freedreno/qrisc/meson.build

Fetch/build on **Ubuntu 24.04 x86-64** using `scripts/codex-download-freedreno-ubuntu24.04.sh`. This script was **syntax-checked on Debian**, not executed on Ubuntu. It retains Meson/Ninja errors in a size-limited transfer archive. With the returned binaries extracted on Debian, run:

```sh
scripts/analyze-a640-mesa.sh /path/to/a640_zap.mbn \
    /path/to/nabu-freedreno-ubuntu24.04/bin /tmp/a640-gpu-probes
```

The script records alternate raw QRISC readings and an IR3 shader hypothesis, not verified code. The Ubuntu build also tries a **byte-for-byte assembler round-trip on Mesa's public test firmware**; that validates the tooling on independently published test data, but not the OEM ZAP ISA.

## What a real exact reconstruction still requires

1. Determine the ISA and control flow for each executable/code region with independent validation (instruction-stream integrity, calls, references and hardware behavior where feasible).
2. Reconstruct independently authored semantics in suitable languages. Rust can target some Hexagon and ARM ISA families, but compiled output is not automatically identical to the proprietary compiler's output.
3. Match instruction scheduling, branch relaxation, data placement, relocations and all non-code metadata with exact byte comparisons.
4. Authentication requires special treatment: copying an unchanged original signature may preserve validity, but producing a *different* signed payload generally needs valid signing authority. Treat all certificates/signature/authentication blocks separately from source licensing; no signing keys are present.
5. Verify each completed file with SHA-256 and `cmp` against the actual APK version; **never** infer success from a metadata-only prefix match.

Current independently reconstructed result remains **1/9 complete files** (`modemuw.jsn`, 713 bytes). Also exact, but incomplete: **six 4,096-byte ELF metadata prefixes** and **128 WMFW metadata bytes**. None of the eight remaining firmware executables or data images has been reconstructed byte-identically from Rust.

No firmware or signature bytes were added to this GPLv3 repository, and no Xiaomi Nabu hardware tests occurred.
