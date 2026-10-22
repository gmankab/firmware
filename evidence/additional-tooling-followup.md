# Additional tooling and independent compilation check — 2026-10-22

**Licensing and completeness:** Everything introduced in the Rust repository is
newly authored GPL-3.0-only tooling; **vendor code is not relicensed**.
The original reconstruction status is unchanged: `modemuw.jsn` is the only
byte-identical rebuild of 9 files. This experiment does **not** recreate an
A640 ZAP, DSP, modem, Wi-Fi, video, Cirrus, or Novatek firmware binary.

## Archive and firmware verification

- The eight original download parts combined into a 1,718,401,941-byte `.tar.zst`;
  the four additional tooling parts combined into a 605,801,942-byte `.tar.zst`.
- Both archive streams and `context_decompile.tar.xz` passed compression checks.
- Extracted 37,348, 19,595, and 35 archive members respectively.
- GNU tar readback found no changed extracted *contents*; ownership/mode
  metadata differs because extraction deliberately omitted original ownership
  and permissions. All file/link paths resolved within extraction roots.
- SHA-256 of all nine files extracted from firmware-xiaomi-nabu-1-r2.apk
  matches `evidence/apk-files.sha256`. The package identifies itself as
  `license = proprietary`.
- All files covered by additional tooling's `SHA256SUMS-downloaded` pass.

## Hexagon Rust is possible in this toolchain

Contrary to the earlier README, the supplied Rust 1.99.1 binary **does list**
`hexagon-unknown-none-elf`, `hexagon-unknown-linux-musl`, and
`hexagon-unknown-qurt`. The host Rust install contains LLVM Hexagon codegen.
For the bare-metal target, we successfully compiled a newly authored `no_std`
Rust static library containing `nabu_hexagon_probe`, with LLVM targeting
Hexagon v60 by default. The output's archive members are ELF32 machine type
`QUALCOMM DSP6 Processor`. SHA-256 of the original lab probe archive:
`6c045654e8dc10e967e6c2ae1448c036f9d9254f312c7f9d7158e34adb0d6715`
(the committed probe changes its crate name, so the output is not expected to
have this same archive hash).

Reproduction requirements: Rust 1.99.1 installed with the matching `rust-src`
component from the extra tool archive, and the vendored dependencies found
within `rustc-1.99.1-src.tar.xz` at `rustc-1.99.1-src/library/vendor`.

```bash
export PATH=/path/to/installed/rust-1.99.1/bin:$PATH
export RUST_STD_VENDOR=/path/to/rustc-1.99.1-src/library/vendor
scripts/build-hexagon-probe.sh
```

The script is self-contained once those vendor-supplied toolchain inputs are
installed: `cargo -Z build-std=core --offline` resolves the offline dependencies,
compiles `core` and `compiler_builtins`, and emits a Qualcomm Hexagon ELF object
archive. `RUSTC_BOOTSTRAP=1` is needed for stable Cargo's `-Z build-std` in this
*experimental test*, not for the ordinary host build.

**Scope:** This proves the toolchain can compile *new code*, not that Rust
can regenerate the original Hexagon program, reproduce its memory layout,
use the same compiler optimizations, or recreate its authentication tags.
Likewise it says nothing about HALO/Novatek instruction sets or Nabu boot.

## New decompiler experiment

- Original Ghidra 12.1.4 built-in Hexagon (`Hexagon:LE:32:default`) stops with
  `halt_baddata()` for `a640_zap.mbn`; previous C output is in a separate
  proprietary-derived research archive.
- Loaded the supplied external **Ghidra 11.4.3** Hexagon_U extension into a
  local **Ghidra 12.1.4** research copy using an *experimental extension
  metadata version override*. Forced `-processor QDSP6:LE:32:default` on the
  A640 ELF. Headless analysis exited successfully, but **failed** to decode
  instructions at 0x5000 (log: `Unable to resolve constructor`), returning
  `halt_baddata()` immediately. This is no reconstruction or validation of
  the extension's runtime compatibility.
- The supplied Ubuntu x86-64 Hexagon LLVM CLI binaries are not presently usable
  on this Debian 13 runtime because `libc++.so.1`, `libc++abi.so.1`, and
  `libunwind.so.1` are missing. The shipped libraries under its `target/hexagon`
  sysroot cannot satisfy the **host x86-64** runtime dependencies.
- The supplied Rizin v0.9.1 source has an in-tree Hexagon decoder, but its
  offline Meson build requires additional dependency subprojects. Even after
  using the Ubuntu `libcapstone-dev` static archive and its pkg-config file,
  configuration failed at unavailable PCRE2/other subproject resolution.
  An Ubuntu-built Meson launcher works on this Debian host, but no
  independently installed Rizin executable or additional decompiled functions
  resulted.

## Rebuild constraints still unsolved

- A real working replacement requires recovering program behavior, original
  data, relocation and layout rules, ISA specifics, and hardware interfaces.
- C-like Ghidra pseudocode is incomplete, not Rust source and is derived from
  proprietary binary instructions; labeling such code GPL-3.0-only would not
  remove the underlying copyright/license issues.
- Qualcomm authentication segment(s) remain in all six `.mbn` images. A
  modified binary's signature status cannot be assumed; no keys/signing or
  hardware flash tests were done. If a byte sequence truly matches the original
  exactly, any embedded signature bytes also match; whether the device accepts
  *modified* images is a separate question.
