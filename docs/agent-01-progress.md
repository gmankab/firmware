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
