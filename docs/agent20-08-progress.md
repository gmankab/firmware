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
