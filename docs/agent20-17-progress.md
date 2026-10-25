# Agent 20-17 — Novatek lower ILM

## Assignment and provenance

- Branch: `agents20/17-novatek-ilm-low`.
- Actual parent: `51b1f91f32d32542aa03e114f5a2f4d94b559e2a`, prior Agent 09 checkpoint 18.
- Exclusive postmarketOS file interval: `[0x66a,0x10000)` (63,894 bytes).
- Runtime mapping: file offset minus `0x200`, therefore `[0x46a,0xfe00)`.
- This is a **separately rooted portable contribution**, not a checkpoint-50-based integrated repository. The original checkpoint-50 bundle was recovered in this session and its advertised HEAD is `b7ed4b587633b9b475a40f5a3ace5cc90db620ff`. Agent 01's latest inspected staging bundle, HEAD `1284d0e97330465881dd65230e8e564fdc8e9cb5`, is still an older-base preview. Integration remains Agent 01's responsibility.
- No central module registry, Cargo manifest, generic builder, earlier Agent 09 implementation, or another agent's address range is modified.

The user-supplied downloads archive was joined in the prescribed order and passed `zstd -t` (2,992,230,400 uncompressed archive bytes). Only the two reference touchscreen images were extracted for this work. Both are kept outside Git.

| Reference profile | Full image bytes | SHA-256 |
| --- | ---: | --- |
| Required postmarketOS | 196,608 | `8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5` |
| Separate map220v comparison | 196,608 | `72364eca0a91855d59e043371aca774354e5ad0885df5024edcec8f976543ad2` |

## Checkpoint 01 — Complete controller reset tail

Reconstructs file `[0x66a,0x6bc)`, runtime `[0x46a,0x4bc)`: **82 instruction bytes**, consisting of 25 instructions (16 four-byte and 9 two-byte). The last POP25 completes the reset routine begun in the inherited prefix at file `0x5de`.

The source names absolute SRAM targets and system registers. It clears the last state byte, initializes interrupt masks, updates IVB and PSW, installs `INT_MASK2=0x29c1007c` and `INT_MASK3=0x633`, sets PSW bit zero, and restores the saved frame. The low-ILM generator does not consume a reference image or any reference byte array.

Five EX9 instructions select three table entries: INT_MASK2 write, INT_MASK3 write, and `ORI r0,r0,1`. The verifier checks their expected operations and operands against the separately supplied table. **Those 12 table bytes are external semantic dependencies in Agent 18's range; they add zero bytes to Agent 17's source coverage.**

| Quantity | PostmarketOS | map220v |
| --- | ---: | ---: |
| Generated executable bytes | 82 | 82 |
| Metadata/data bytes in the new span | 0 | 0 |
| Layout/fill bytes in the new span | 0 | 0 |
| Copied external bytes in the generated span | 0 | 0 |
| Reference mismatches | 0 | 0 |
| GNU-assembled instructions | 25 | 25 |
| GNU/Rust mismatches | 0 | 0 |

Generated span SHA-256:

- PostmarketOS: `cd132d10e1822130a63124dfada899e8d3336589993a3958fc10a9c91b56ab13`.
- map220v: `381dba6533ed6367c33d7e65e7cf3a1ed6f2a9cf999da2f0d5169818cd77de48`.

Validation: **203 Rust tests passed, 13 Python tests passed, zero skipped**, offline locked release build passed, `git diff --check` passed. Full-span GNU binutils 2.25.1 assembly/link/objcopy comparison passed for both profiles with relaxation disabled; no `.byte`, `.word`, or other data fallback was used. pypcode 4.0.1 additionally decoded the first 25 instructions. Test totals describe this portable branch, not the integration owner's repository.

The first uncovered file offset is **`0x6bc`**, runtime **`0x4bc`**. **63,812 assigned bytes remain** at this checkpoint. The inherited 538-byte prefix plus this contribution gives 620 adjacent instruction bytes `[0x450,0x6bc)`, but the inherited prefix has the semantic defects documented in `agent20-17-inherited-encoder-audit.md`; this checkpoint does not silently certify or correct them.

## Reproduction

Use Rust 1.90.0 or another compiler supporting the inherited workspace's integer APIs. Build with an isolated Cargo target directory:

```sh
CARGO_TARGET_DIR=/tmp/agent17-target cargo test --workspace --offline --locked
CARGO_TARGET_DIR=/tmp/agent17-target cargo build --workspace --release --offline --locked
python3 -m unittest discover -s tests -p 'test_*.py'
git diff --check
```

The standalone `novatek-ilm-low` binary supports `emit PROFILE OUTPUT`, `verify PROFILE REFERENCE`, and `listing PROFILE`. The lightweight Rust comparison intentionally reports the required whole-image hash; use the Python gate below to enforce it.

```sh
python3 tools/verify-novatek-ilm-low.py \
  --profile postmarketos --reference /private/postmarketos/novatek_nt36523_fw.bin \
  --emitter /tmp/agent17-target/release/novatek-ilm-low \
  --as /tools/nds32le-elf-as --ld /tools/nds32le-elf-ld \
  --objcopy /tools/nds32le-elf-objcopy --objdump /tools/nds32le-elf-objdump \
  --output-directory /tmp/agent17-verification
```

Repeat with `--profile map220v` and its exact reference. Use **unmodified GNU binutils 2.25.1**, which retains EX9/IFCALL/IFRET support. The newer 2.43.1 assembler lacks those entries; pypcode 4.0.1 also lacks complete IFC coverage. The verifier fails rather than substituting raw bytes for unsupported instructions. Explicit `--skip-assembler` produces a partial report and exit status 2; it cannot satisfy a checkpoint gate.

The bundle includes the typed source, standalone CLI, verifier, inherited-encoder audit, and per-profile verification evidence under `verification/agent20-17/`. OEM references and local tool binaries are deliberately external. The generated fragment is not a complete firmware image; behavioral validation and hardware execution remain unperformed.

## Next work

Continue the controller setup, polling, byte-copy, and reporting helpers through file `0x79c`, then the descriptor/status routine through `0x95a`. Preserve IFC state semantics and verify each referenced EX9 instruction independently. Agent 01 should correct the inherited JRAL5 and compact-register defects; Agent 18 owns the EX9 table's independent source reconstruction.
