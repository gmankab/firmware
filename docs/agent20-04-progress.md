# Agent 20-04 — Venus controller and middle-code reconstruction

**Intended integration base:** `checkpoint-50-venus-33872.bundle`, commit `b7ed4b587633b9b475a40f5a3ace5cc90db620ff`.

**Available actual parent:** `a3ffc58da64d8f96a65b9a48f4bed24592ba0ac1` from the uploaded Agent 03 late-Venus bundle. Checkpoint 50 itself is not mounted and is **not** this branch's ancestor. This work is therefore a **provisional portable extension**, not a checkpoint-50-rooted integrated release.

**Owned reservation:** runtime `[0x40000,0x68000)` in the first executable `venus.mbn` `PT_LOAD`, mapping to file `[0x44000,0x6c000)`. The executable `PT_LOAD` starts at file offset `0x4000`, vaddr zero, filesz `0x1029f4`; do not apply the ELF physical address as an additive offset.

## Checkpoint 01 — signed dimension normalization and short return

- The reservation begins **in the middle** of a previous large function: runtime `[0x40000,0x40a2c)` is **not** covered by this implementation. It is not declared as rebuilt, not filled with implicit zeros and not copied from the original into the generator.
- Source-derived executable span **runtime `[0x40a2c,0x40c14)`**, file `[0x44a2c,0x44c14)`, exactly **488 bytes = 122 A32 instructions**. No literal-data words in this initial interval.
- Functions: the genuine saved-register-return dimension/classification routine `[0x40a2c,0x40c0c)`, followed by the two-instruction `return 0` stub `[0x40c0c,0x40c14)`.
- The first routine normalizes signed 16-bit input dimensions, calls helpers at `0x81838`, `0x53e8c`, `0x53edc`, and `0x53f98`, handles conditional clipping, and returns a two-bit small-dimension status mask. That is an interpretation of decoded control flow, **not** independently recovered high-level source semantics.
- `cargo run -p firmware-tools --example agent20_04_venus_middle -- verify /private/venus.mbn` compares every emitted byte against the separately extracted, pinned `venus.mbn` reference (SHA-256 `9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085`). **Zero differences**.
- Independently assembled **all 122 ARM mnemonic instructions** with Clang/LLVM, resolving all relative PC+8 branches through linker symbols, and compared its 488-byte result to the Rust emitter and original. All agree exactly. This strengthens instruction encoding verification but does not prove high-level behavioral equivalence or new OEM signing.
- The next verified function entry is `0x40c14`. Do not extend into Agent 05 ownership (runtime at or above `0x68000`).

## Build and verification

```sh
cargo test --workspace --offline --locked
cargo build --workspace --release --offline --locked
python3 -m unittest discover -s tests -p 'test_*.py'
cargo run --offline --locked -p firmware-tools --example agent20_04_venus_middle -- verify /private/venus.mbn
cargo run --offline --locked -p firmware-tools --example agent20_04_venus_middle -- emit /tmp/venus-middle.bin
git diff --check
```

**Integration changes:** the only shared file touched is the single `mod.rs` module-registration line; the new source and CLI example have unique Agent 20-04 filenames. There are no original OEM binaries, signed images, executable opcode arrays or authentication materials in the Git commits. All tests require an agent-isolated `CARGO_TARGET_DIR` to prevent stale `arm32` artifacts from other agent branches.
