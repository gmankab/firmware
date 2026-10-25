# Agent 20-08 — Qualcomm ADSP later executable regions

## Provenance and branch limitations

- Intended integration base: a checkpoint-50-descendant merged from the previous ten agents. **Not present among the mounted inputs.** The current changes are isolated on `agents20/08-adsp-late`, based on Agent 04's bundled `d7cb8b118c275a0458d29bb65dafb82b80613825` (itself rooted in an older common repository). This bundle is therefore **provisional** and is a portable source-only patch for the eventual integrated base; do not claim it is checkpoint-50-rooted.
- Exact pinned reference is private `adsp.mbn`, SHA-256 `4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e`. Toolchain: LLVM6 Hexagon assembler `-mcpu=hexagonv65` **only** for instructions shared with detected Hexagon V66; reference ELF `e_flags=0x66`.
- The two late executable PT_LOAD mappings are verified by `readelf -l` and by `tools/emit-adsp-late.py`:

| Reference file interval (exclusive end) | ELF vaddr | PhysAddr | p_filesz | p_memsz |
|---|---|---|---|---|
| `[0x39000,0x1f27e8)` | `0xf0036000` | `0x8be36000` | `0x1b97e8` | `0x1ba000` |
| `[0x364000,0xcaf790)` | `0xb0000000` | `0x8c500000` | `0x94b790` | `0x94c000` |

**Checkpoint 01:** An 84-byte, independently reconstructed Hexagon callback-drain function `[0x39000,0x39054)` in the first later RX load. Original stack frame, callback iteration and early zero-controller path are explicit source assembly; the 3-register duplex packet order was checked until LLVM6 output matched the pinned reference exactly. The subsequent `[0x39054,0x39060)` contains **12 zero file bytes**, classified separately as alignment/layout, not executable source. The routine's physical mapping is `[0x8be36000,0x8be36054)`; original ELF vaddr mapping is `[0xf0036000,0xf0036054)`.

The emitted 84-byte program SHA-256 is `cc93ab6de47bc8514f946f427a059bc279975b4e99b9cfdc0be450f9d559a233`. It is not a full executable segment, source-level-equivalent codec implementation, or separately OEM authenticated firmware. The second large `R E` region near `0x364000` also includes large stretches of non-opcode data and zeros and must be classified, not blindly decoded.

## Reproduction

```sh
export HEXAGON_LLVM_MC=/private/llvm6/bin/llvm-mc
export NABU_ADSP_REFERENCE=/private/Xiaomi/nabu/adsp.mbn
python3 tools/emit-adsp-late.py --llvm-mc "$HEXAGON_LLVM_MC" \
  --segment queue-drain --output /tmp/adsp-late-queue.bin \
  --reference "$NABU_ADSP_REFERENCE"
python3 -m unittest discover -s tests -p 'test_*.py'
cargo test --workspace --offline --locked
cargo build --workspace --release --offline --locked
```

## Next work

1. Complete the source reconstruction of the next disjoint function(s) from `0x39060` onward, preserving ELF-relative branch deltas, 32/64-bit memory op packet rules and literal tables.
2. Rebuild a genuinely executable function at `0x364000` if the ISA is independently corroborated; do not count structural zero ranges as code.
3. Map actual call targets/relocations in the two later `PT_LOAD` segments; expand to additional bounded function fragments, with a first-hole manifest and byte-accurate tests.
4. Integrate only after the real checkpoint-50-based merged bundle exists.

**Checkpoint 02:** A second *disjoint*, independently source-assembled function is now byte-verified: an **80-byte** zero-fill routine `[file 0x3640a8,0x3640f8)`, linker vaddr `[0xb00000a8,0xb00000f8)`, physical `[0x8c5000a8,0x8c5000f8)`. It clears unaligned front/tail bytes and uses a packetized eight-byte `memd(r0++#8)` loop in the aligned interior. All 20 instruction words were assembled from actual operations; no `.long` instructions were imported. SHA-256 `d8c1f7e013d349af5011c3a1a99db6e19689d99bb2fb7c459ed346625f550f1c`.

**Cumulative source-generated executable count after checkpoint 02: 164 bytes, in two disjoint functions.** The intervening gaps, code/data arrays and tens of megabytes of remaining RX text are not reconstructed and must not be added to this total. In the second RX segment, file `[0x364100,0x364600)` contains predominantly zeros/data and is **not** 1,280 source-generated instruction bytes.

**Checkpoint 03:** Rebuilt a second full 120-byte later-main RX helper `[file 0x39060,0x390d8)`, ELF vaddr `[0xf0036060,0xf00360d8)`, physical `[0x8be36060,0x8be360d8)`. This routine programs a controller MMIO word, dispatches seven statically relocated initialization/callback functions with an explicitly saved `r1:0` argument pair, publishes state via `r18`, and exits to a stop-loop. LLVM6's Hexagon v65-compatible assembler reconstructs the original packets with **zero byte mismatches**, SHA-256 `2b6f8caaaded06ece47aadd8e97090ca9269ac2f7f7e16242e21094f128aadbf`. Interprocedural `.set` targets express the actual instruction-level PC-relative calls and do not insert missing firmware bytes into the binary. The 12-byte structural zero gap `[0x39054,0x39060)` is explicitly omitted from the generated **executable** count.

**Cumulative Agent 20-08 source-generated executable bytes after checkpoint 03: 284** (84 + 120 + 80, across three disjoint functions in two RX segments). No byte-equivalence claims are made for any other ADSP region.
