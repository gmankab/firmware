# A640 stage 2 — typed QRISC assembler and partial Rust code generation

2026-10-22 · Debian 13 x86-64 · Rust 1.99.1 · Mesa Freedreno 26.2.4

## Decision

Do not keep optimizing the old five-word assembly mismatch: its previous
private research reassembly was already byte-for-byte exact. Prioritize a
newly authored Rust program representation and validate it against Mesa and
original firmware. Keep OEM-derived instruction listings and shader outside
Git and separate from GPLv3 sources.

## New source capabilities

- `src/qrisc_codec.rs`: `ret` vs `iret` correctly distinguished using public
  ISA bit 25; the A640 variant bit 24 remains **uninterpreted** and must not
  be assigned invented semantic meaning. A typed firmware-header constructor
  accepts family ID and low twelve-bit revision/metadata.
- `src/qrisc_program.rs`: two-pass symbolic label resolution, signed branch
  displacements, word-indexed calls, checked operands and shader slot widths.
- `src/bin/qrisc-public-demo.rs` plus `scripts/verify-qrisc-against-mesa.sh`:
  publicly specified synthetic QRISC program cross-assembled with Mesa.
- `src/a640_routines.rs`: two typed instruction sequences constructed from
  named, publicly documented GPU register operations. The design was guided
  by decompilation, so copyright independence is **not established**.

## Measured results

| Check | Result |
|---|---|
| Rust tests | 25 passed |
| Python tests | 4 passed |
| Source provenance | All nine firmware files match the postmarketOS package checksums |
| Rust release build | Pass, offline |
| Rust vs Mesa for synthetic 20-byte program | Identical, no copied OEM bytes |
| A640 entry routine | 16 of 16 bytes exact |
| A640 wait routine | 20 of 20 bytes exact |
| A640 control ISA decode/reencode | 364/364 words, zero unknown; OEM-input roundtrip, not source reconstruction |
| A640 return encoding categories | 4 normal `ret`, 4 `ret` with undocumented bit 24, 0 `iret` |
| Previous OEM-derived QRISC/IR3 private reassembly | All 1,968 bytes exact, SHA-256 as below |
| Independently rewritten entire A640 image | NOT achieved |

The private exact 1,968-byte output has SHA-256:
`e24a4f483b71ea19715910f01e1abf94280756df731cad05df48fd7714e34198`.
It results from the private OEM-derived disassembly containing raw IR3 data,
with Mesa's leading 4-byte output prefix removed and the original firmware-ID
metadata restored. It **must not** be treated as newly authored firmware,
GPL-licensed material, or a bootable replacement.

The 36 matching bytes of newly typed routines include a 4-byte metadata word;
32 bytes represent actual QRISC instructions. Comparing independently
compiled snippets against the original is an engineering identity check, not
proof of clean-room licensing. Their relative positioning does not create a
complete executable program.

## Next tasks

1. Recover IR3 slot 61 without assuming undocumented semantics. Characterize
   the shader's memory safety and preemption behavior using public IR3 docs.
2. Express the surrounding A640 security-state transitions as a Rust-written
   behavioral model, with independently constructed tests and an emulator or
   hardware observation oracle, then compile to QRISC using this assembler.
3. Implement deterministic IR3 code generation from named operations, not raw
   copied shader bytes. Compare both semantics and exact bytes.
4. Reconstruct the rest of the MBN container using typed metadata while
   keeping authentication/signature dependencies explicitly unresolved.
5. Independently reconstruct the other firmware images with separate ISAs and
   validate on a test device only with an appropriate recovery procedure.

No new toolchain download is needed for the current QRISC Rust assembler or
Mesa cross-check. The current project provides **no flashable replacement**.
