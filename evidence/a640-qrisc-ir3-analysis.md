# A640 ZAP: QRISC and IR3 instruction recovery (2026-10-22)

**Status:** Validated decoder-level identification and private roundtrip, **not**
an independently authored GPLv3 firmware rebuild. All source code in this
repository is GPL-3.0-only; OEM-derived disassemblies are deliberately kept
outside the Git history.

## Supplied tools

The user-supplied `nabu-freedreno-ubuntu24.04.tar.zst` was successfully unpacked
on Debian 13. Zstandard verified, 189 archive members were checked for safe
paths, and `MANIFEST.sha256` passed. Mesa 26.2.4 `qrisc-disasm`, `qrisc-asm`,
`ir3-disasm` run on the Debian host with the archive's `RNN_PATH` register
metadata and optional `LD_LIBRARY_PATH` runtime. No ClientError occurred.

## A640 firmware identification

- Input SHA-256:
  `bb1b61bae977aab5e5a68bd88848f4479c8d156911d96c7de73f7195b674db06`.
- ELF32 file length: 14,256 bytes; `e_machine=164` (Hexagon wrapper),
  `e_flags=3`; executable segment at file offset `0x3000`, virtual address
  and entry `0x5000`, length `0x7b0` (1,968 bytes = 492 words).
- SHA-256 of executable segment:
  `e24a4f483b71ea19715910f01e1abf94280756df731cad05df48fd7714e34198`.
- The first payload word supplies QRISC firmware ID `0x6ff`, **not** one of the
  exact IDs recognized by the unmodified Mesa utility. Forcing Mesa's A630,
  A650 or A660 QRISC family decoder produces the same complete control-code
  listing. This is strong evidence that the wrapper's Hexagon flag must not be
  used to decode the executable payload as Hexagon V4.

| Segment-relative word range | Bytes | Inferred ISA or role | Coverage |
|---|---:|---|---|
| `0..272` | 1,088 | QRISC control program incl. one FW-ID metadata word | all control instructions decode |
| `272..400` | 512 | Embedded 64-slot IR3 GPU shader | 63/64 slots decode with `-g FD640` |
| `400..492` | 368 | QRISC control routines and returns | all control instructions decode |

Combined QRISC regions contain 364 words: 363 recognized instructions plus
one firmware-ID metadata word. IR3 slot 61 is not understood by Mesa 26.2.4;
slot 62 decodes as `end` and slot 63 as `nop`. The other 63 shader slots have
no decoder warnings. Other GPU IDs tested did not resolve slot 61, so it may
be an extension or non-instruction data. Do **not** invent its semantics.

The apparent IR3 start and end can be found without embedding known OEM bytes:
a generic QRISC unconditional jump at control word 269 skips two A6xx NOP
words followed by 128 shader words, landing at control word 400. Public
QRISC/IR3 decoders independently validate this segmentation hypothesis.

## Reconstruction experiments

Mesa `qrisc-asm` writes a 4-byte leading zero word, so its `.fw` output must
be compared after removing that prefix. The unmodified assembler does not
recognize firmware ID `0x6ff`; the experiment used A630 `0x6ee` as an
**assembly-only proxy**. With the proxy and raw shader payload, 1,963 of the
1,968 output bytes matched: five different words, specifically one firmware
ID and four otherwise identical `ret` instructions whose ignored/undocumented
bit 24 differs. The ordinary Mesa textual disassembler cannot distinguish
those `ret` encodings.

In a **private proof only**, preserving those four return-word variants as
raw literals and restoring the original first firmware-ID word produced a
byte-for-byte matching 1,968-byte executable segment. That experiment uses
OEM-derived opcodes and embedded shader bytes. It is a reassembly compatibility
test, **NOT** an independently authored GPLv3 program or complete firmware.
It was not included in this Git bundle.

The new GPLv3 Rust `src/qrisc_codec.rs` implements independent, typed QRISC
word-field parsing and encoding using public instruction-format information.
On the original control region all **364/364** words decode and reencode
exactly. This is a *field-level roundtrip with original instructions as
input*, not source reconstruction. The Rust CLI `extract-a640-isa` validates
ELF boundaries and finds the IR3 candidate without executable-code tables.

## Reproducing locally

```bash
cargo test --offline --locked
cargo run --offline --locked -- extract-a640-isa /path/to/a640_zap.mbn /tmp/a640-regions
cargo run --offline --locked -- verify-a640-isa-fields /path/to/a640_zap.mbn
scripts/analyze-a640-mesa.sh /path/to/a640_zap.mbn \
  /path/to/nabu-freedreno-ubuntu24.04/bin /tmp/a640-mesa
```

The Mesa script automatically sets `RNN_PATH` and any packaged runtime path,
and uses the correct `FD640` identifier. Its output directory contains
OEM-derived binary sections and textual disassembly; **do not add it to the
GPLv3 repository**.

## Outstanding work and important constraints

1. Recover the precise behavior of IR3 slot 61 and compare GPU register side
   effects and timing, including QRISC branch delay slots and preemption.
2. Independently specify and implement the control algorithm and shader in
   Rust, with tests based on public device contracts and an original-free
   generated code path. A generic opcode parser and OEM assembly transcript
   do **not** satisfy this requirement.
3. Implement a deterministic Rust QRISC and IR3 code generator (not the Rust
   LLVM Hexagon backend: A640's payload is *not* Hexagon), matching instruction
   and data layout where possible.
4. Recover or independently generate the full MBN container metadata and
   authentication material. The firmware includes signing-related content;
   without OEM signing keys, regenerating identical cryptographic signatures
   for an independently altered payload is not generally possible. Reusing
   original signed bytes would make the result depend on vendor-owned content.
5. Repeat ISA identification and independent reconstruction for the other
   eight package files, with original byte-identity and device testing as
   separate gates.

**Whole-file byte-identical GPLv3 Rust status: remains 1/9 (`modemuw.jsn`).**
The six exact 4 KiB ELF metadata prefixes and 128-byte Cirrus metadata
reconstruction remain unchanged; A640 now has a private exact code-segment
*reassembly*, not clean Rust source of its behavior.
