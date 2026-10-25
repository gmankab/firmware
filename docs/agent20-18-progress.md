# Agent 20-18 — Novatek upper ILM

## Scope and exact provenance

- Branch: `agents20/18-novatek-ilm-high`.
- Actual parent: Agent 09 checkpoint 18, `51b1f91f32d32542aa03e114f5a2f4d94b559e2a`.
- Status: **separately rooted portable contribution**. It is not a verified
  descendant of checkpoint 50 `b7ed4b587633b9b475a40f5a3ace5cc90db620ff`.
  Agent 01 owns integration into that history. No older `main` is merged here.
- Reservation: firmware file `[0x10000,0x1ac80)`, 44,160 bytes. ILM maps file
  offset `0x200` to runtime address zero, so the reservation maps to runtime
  `[0xfe00,0x1aa80)`.
- Reference: 196,608-byte postmarketOS `novatek_nt36523_fw.bin`, SHA-256
  `8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5`.
  This turn recovered the exact same bytes from the distributed package
  `https://mirror.math.princeton.edu/pub/postmarketos/v25.06/aarch64/firmware-xiaomi-nabu-1-r0.apk`,
  member `lib/firmware/novatek/novatek_nt36523_fw.bin`, and checked the full
  pinned digest. The multipart archive was not downloaded or revalidated in
  this turn. The firmware and APK remain outside Git and deliverable bundles.

## Boundary handoff to Agent 01

File `0x10000` is the second half of a four-byte MOVI at `[0xfffe,0x10002)`.
That instruction belongs to a routine starting at file `0xfffc` and ending at
`0x10016`. The first wholly owned function starts at file `0x10016`, runtime
`0xfe16`. This branch leaves `[0x10000,0x10016)` (22 bytes) uncovered rather
than treating a partial instruction as an independently authored instruction.
Agent 17 owns the lower interval; reconcile the cross-boundary routine through
Agent 01 without duplicating ownership. Reading lower bytes to establish this
boundary does not add them to this branch's emitted ranges.

## Checkpoint 01 — Channel-code mapping and two-byte store helper

Reconstructed **155 typed NDS32 operations / 416 executable bytes**:

| Kind | File interval | Runtime interval | Bytes |
| --- | --- | --- | ---: |
| Channel-code selection and bounded workspace mapping | `[0x10016,0x101ac)` | `[0xfe16,0xffac)` | 406 |
| Leaf helper storing two caller-provided bytes | `[0x101ac,0x101b6)` | `[0xffac,0xffb6)` | 10 |

The first routine selects an input mode, initializes a 32-byte workspace at
SRAM `0x2dc58`, updates descriptor bytes, maps bounded input codes, and calls
lower-ILM helpers. These names describe the observed operations; the exact
controller meaning is not established. EX9.IT indices are represented as
instruction-table references. They are not called software interrupts and
are not counted as reconstructed table entries at this checkpoint.

New code is isolated in `novatek_upper_isa.rs`, `novatek_upper_ilm.rs` and the
auto-discovered `novatek-upper-ilm` binary. No shared `mod.rs`, `lib.rs`,
`main.rs`, Cargo manifests, old Novatek encoder, layout, DLM, or lower ILM is
modified. The encoder checks width, compact-register mappings, signed and
scaled immediates, register subsets, and branches measured from instruction
start. Source emission does not read the reference firmware.

### Measured verification

- Typed Rust output versus the full-hash-pinned reference interval: **0 mismatches**.
- Independent GNU NDS32 assembly versus the same interval: **0 mismatches**.
- Span SHA-256: `2dc59e3cd4cff852e54bf53392bc6fe4853ded0a49c9b530d430b3ea37d2a3b4`.
- `cargo test --workspace --offline --locked`: **209 passed, 0 failed, 0 ignored**.
- `cargo build --workspace --release --offline --locked`: passed.
- `python3 -m unittest discover -s tests -p 'test_*.py'`: **16 passed, 0 skipped**.
- `git diff --check`: passed.
- Toolchain: Rust/Cargo 1.99.1; GNU assembler/linker/objcopy 2.30.
- CARGO_TARGET_DIR is isolated for this branch.

GNU 2.30 source archive SHA-256:
`6e46b8aeae2f727a36f0bd9505e405768a72218f1796f0d09757d45209871ae6`.
The assembler uses `.flag verbatim` and `-mno-all-relax`, and the linker uses
`--no-relax`. GNU 2.42 was unsuitable for this image's EX9/IFC instructions;
it can mislabel EX9 as BREAK16. The independent cross-check uses unmodified
2.30, whose EX9 support was separately tested.

### Reproduction

```sh
export CARGO_TARGET_DIR=/your/isolated/agent20-18-target
cargo test --workspace --offline --locked
cargo build --workspace --release --offline --locked
python3 -m unittest discover -s tests -p 'test_*.py'
python3 tools/devices/xiaomi-nabu/verify-agent20-18.py \
  /private/novatek_nt36523_fw.bin \
  --emitter "$CARGO_TARGET_DIR/release/novatek-upper-ilm" \
  --as /path/to/nds32le-elf-as --ld /path/to/nds32le-elf-ld \
  --objcopy /path/to/nds32le-elf-objcopy --report /tmp/agent20-18.json
git diff --check
```

The verification script rejects a wrong reference revision, truncation,
emitter mismatch, assembler mismatch, and a changed expected span digest.
GNU source assembly is included for reproducibility. The byte agreement is
evidence of instruction-level reconstruction, not proof of high-level
decompilation or observed hardware behavior.

### Provenance accounting and remaining work

| Category | New checkpoint bytes |
| --- | ---: |
| Typed executable instructions | 416 |
| Independently reconstructed data/metadata | 0 |
| Layout fill | 0 |
| External reference bytes embedded in source | 0 |
| Reservation still uncovered | 43,744 |

First uncovered offset: `0x10000` (the 22-byte seam). Forward continuation:
`0x101b6`. The range contains ordinary instructions, embedded switch tables,
the EX9 table at `[0x17dc0,0x185c0)`, and static data after `0x185c0`; a linear
disassemble/reassemble of all 44,160 bytes would misclassify data as code.
The existing Agent 09 538-byte lower-ILM contribution is outside the
reservation and is not included in this checkpoint's new executable count.

No hardware execution, complete-image provenance, firmware signing, or
flashability is established.
