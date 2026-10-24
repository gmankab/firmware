# Agent 06 — Qualcomm modem firmware

## Repository base and boundary

- Assignment baseline: **checkpoint 50, `b7ed4b587633b9b475a40f5a3ace5cc90db620ff`**.
- **Availability caveat:** checkpoint-50 bundle is not mounted in this agent's file environment. The actual available ancestor is `recompile-firmware.bundle`, HEAD `69ebf0d1c50859189a9196a21155c388ac914d7b`. Work lives on `agent/06-modem`, with isolated `modem_*` modules and a standalone `modem-rebuild` binary. Commit(s) must be cherry-picked onto checkpoint 50 by the integration agent. Do not report these as direct descendants of checkpoint 50.
- Reference firmware: `modem.mbn`, SHA-256 `6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782`. It is held **outside Git** in the user-supplied archive.

## Architecture, mapping and reference verification

`modem.mbn` has little-endian ELF32, e_machine `164` (Qualcomm Hexagon DSP6), e_flags `0x66`, and ELF e_entry `0x8e000000`. Its first executable `PT_LOAD` is file `[0x3000,0x5020)`, ELF virtual `[0xc0800000,0xc0802020)`, and physical `[0x8e000000,0x8e002020)`. Its next main R-X segment starts at file `0x7c000` (ELF virtual `0xc0890000`, physical `0x8e090000`), size `0x183cf0`. These address spaces are deliberately never conflated.

Rizin 0.9.1 Hexagon V6 decodes the entry's eight vector slots, privileged `S6` (SSR), `S18` (SYSCFG) register transfers, set-bit operations and cache-kill instructions. Historical LLVM 6.0.1 Hexagon MC (`hexagonv60`) independently assembles the vector jumps and set-bit operations; it does not accept several privileged operations in its syntax. For those, the typed encoder follows the Rizin/Hexagon instruction-field patterns, rather than embedding OEM instruction words.

## Checkpoint 01: entry/reset vector code

- Exact file offsets: `[0x3000,0x3044)` (**68 bytes**).
- Physical addresses: `[0x8e000000,0x8e000044)`; ELF virtual: `[0xc0800000,0xc0800044)`.
- 17 independently generated single-instruction Hexagon packets, no embedded reference opcode data. One vector dispatches to physical `0x8e00111c`; after privileged setup control transfers to physical `0x8e0005c4` (file offset `0x35c4`).
- `cargo run --release --offline --locked -p firmware-tools --bin modem-rebuild -- emit /tmp/entry.bin` emits the fragment.
- `cargo run --release --offline --locked -p firmware-tools --bin modem-rebuild -- verify /private/modem.mbn` reports exact byte agreement; the emitted 68-byte SHA-256 is `80611bd86bb0827c0bf6db324f898deec1d91658d4d5193ccf195ea1f499e7d4`.
- `verify-mbnv6-hashes` on the private original confirms the hash table for 20 segments, **not** a regenerated, separately signed image. No full modem firmware or signer keys have been reconstructed.
- `cargo test --workspace --offline --locked`, `cargo build --workspace --release --offline --locked`, and Python unit tests pass on the actual available baseline (Rust: existing tests plus 5 new focused tests; Python: 13 tests). `git diff --check` passes.

## Remaining work

The following file interval `[0x3044,0x35c4)` appears to contain a memory-map/translation table and is **not yet generated**; it must not be counted as executable or interpreted as linear Hexagon instructions. Additional startup code begins at physical `0x8e0005c4` (file `0x35c4`) and requires typed packet reconstruction. All other executable and data sections remain unimplemented, and OEM authentication cannot be independently reissued.
