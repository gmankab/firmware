# Agent 01 — Venus service forward reconstruction

Assigned checkpoint-50 base: `b7ed4b587633b9b475a40f5a3ace5cc90db620ff`.

**Availability caveat:** that checkpoint bundle is present in Project Library metadata, but its raw bytes cannot be materialized in this execution session. Work here is based on the *older* mounted `recompile-firmware.bundle`, commit `69ebf0d1c50859189a9196a21155c388ac914d7b`; it must be applied/cherry-picked onto the designated checkpoint-50 branch before claiming that parentage. This separate branch is a portable compatibility patch, not an integrated checkpoint-50 history.

## Region coverage

- Owner: Venus ARM32 service executable `[0x12e5c,0x18000)`, absolute reference offsets `vma+0x4000`.
- Independent typed Rust fragment begins at `0x12e5c`, file offset `0x16e5c`.
- Generated from ARM32 instruction semantics and explicit operands, not pasted OEM instruction words.
- No unknown bytes are padded or added to the output. Only the generated fragment is emitted.
- Rebuilt fragment is not runnable on its own, nor OEM-authenticated.
- Deterministic validation: `cargo run --offline --locked --bin agent01-venus-forward -- verify /private/venus.mbn` and `cargo test --workspace --offline --locked`.

Generated output is a partial function; the next missing instruction is the reported `END` in `venus_agent01_forward.rs`. The file should remain isolated during integration; the only shared edit is adding the module export to `devices/xiaomi_nabu/mod.rs`.

## Agent 01 checkpoint 02

- Independently re-encoded `[0x12e5c, 0x13800)` from the original Venus ARM32 runtime (`0x9a4` = 2,468 bytes), MBN file `[0x16e5c,0x17800)`.
- Codec mode selector includes a non-instruction 11-target pointer table at `0x13680–0x136ac`; it is modeled as 11 named branch destinations, with repeated targets retained.
- This region ends inside a larger function, so it remains **partial**. Next uncovered instruction at runtime `0x13800`.

## Agent 01 checkpoint 03

- Typed ARM32 coverage `[0x12e5c,0x14000)` = 4,516 bytes, MBN offsets `[0x16e5c,0x18000)`; exact comparison passed.
- Completed a first codec session function ending at runtime `0x13b14`, with six data words at `0x13afc–0x13b14` and one 11-entry branch table at `0x13680–0x136ac`.
- The next session function begins at `0x13b14`. At checkpoint 03 its implemented portion ends at `0x14000`, with the function still incomplete.
- Public checked ARM32 SMULBB encoder introduced with dedicated ISA tests; this common-arm32 change is committed separately from the fragment and should be reviewed separately during checkpoint-50 integration.

## Agent 01 checkpoint 04

- Reconstructed `[0x12e5c,0x14800)` = 6,564 consecutive exact ARM32 bytes; source MBN file interval `[0x16e5c,0x18800)`.
- Two additional structured jump tables: 12 targets at `0x14094–0x140c4` and 11 targets at `0x14550–0x1457c`, modeled explicitly (no misidentified executable opcodes).
- Function begun at `0x13b14` remains incomplete; first uncovered code is `0x14800`.

## Agent 01 checkpoint 05

- Byte-identical forward region `[0x12e5c,0x15ae8)` = 11,404 bytes; MBN offsets `[0x16e5c,0x19ae8)`.
- The larger codec session routine starting at `0x13b14` now includes original return at `0x15a6c`, continuation-tail at `0x15a70–0x15ad4`, and five separately identified data words at `0x15ad4–0x15ae8`.
- Recorded three more structured dispatch tables: 12-target mode table, 11-target frame table and 12-target packet table, none misclassified as executable opcodes.
- A conditional BLNE instruction at `0x14f78` was corrected by preserving its condition field; subsequent full-region byte comparison passed.
- Next uncovered ARM address: `0x15ae8`.

## Agent 01 checkpoint 06

- Independently emitted and byte-verified `[0x12e5c, 0x163e8)` = 13,708 bytes, MBN offsets `[0x16e5c,0x1a3e8)`.
- Rebuilt small helper routines and codec-plane state calculation at `0x15ae8–0x163e8`.
- Explicitly modeled 10-target dispatch table `[0x15f0c,0x15f34)` and literal pools `[0x15df0,0x15e00)`, `[0x15ea4,0x15ea8)`, `[0x163cc,0x163e8)` as data. No vendor code bytes were pasted into Rust.
- Next uncovered runtime instruction is `0x163e8` (start of a new function).

## Agent 01 checkpoint 07

- Generated `[0x12e5c,0x1698c)` = 15,152 consecutive reference-exact bytes, MBN `[0x16e5c,0x1a98c)`.
- Complete codec-channel setup routine `[0x163e8,0x1698c)`, including two four-entry runtime jump tables (`0x16570–0x16580`, `0x16904–0x16914`) and six data words (`0x16974–0x1698c`).
- All reference comparisons passed without copying proprietary executable bytes. Next function begins at `0x1698c`.

## Agent 01 checkpoint 08

- Independently rebuilt `[0x12e5c,0x174e4)` = 18,056 byte-identical bytes, MBN file `[0x16e5c,0x1b4e4)`.
- Includes a complete codec frame-plane routine (`0x1698c–0x174e4`) with four-entry jump table `[0x170ec,0x170fc)` and three data words `[0x174d8,0x174e4)`.
- Function terminates with original POP/PC at `0x174d4`, not an invented return.
- Next ARM32 function entry is `0x174e4`.
