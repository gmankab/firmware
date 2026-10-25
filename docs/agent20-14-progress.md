# Agent 20-14 — WLAN late Hexagon reconstruction

## Assignment and provenance

Branch: `agents20/14-wlan-late`.

Actual starting commit: `d6104bb257ad60aa52f36eb91f338ef3e08586c5`, from the
self-contained `agent-07-checkpoint-15-wlanmdsp.bundle`. The input bundle passed
`git bundle verify`. This is a **portable contribution on the prior WLAN branch**,
not an integrated checkpoint-50 descendant. The inherited history is preserved.
Only the integration owner should apply this contribution to the merged tree.

The original checkpoint-50 bundle was retrievable in this session and advertises
the expected `b7ed4b587633b9b475a40f5a3ace5cc90db620ff`. It is not a merged
20-agent base. The latest Agent 20-01 artifacts found during preparation were
explicitly named `oldbase-staging` / `oldbase-clean`; their existence is not
treated as successful checkpoint-50 integration.

The exclusive allocation is WLAN MBN file `[0x100000,0x353d38)`, corresponding
to runtime `[0xb00c0000,0xb0313d38)`. Its size is 2,440,504 bytes. These are
reserved bytes within an executable ELF load, not a claim that every byte is
an instruction. The subsequent RW load at file `0x380000` is out of scope.

Reference identity:

```text
File: wlanmdsp.mbn
Size: 4,331,136 bytes
SHA-256: 87c2c7bb1fe97eaf007a947832feba0898644b4d6626ac8ef49d8b94dd82604b
ELF: little-endian ELF32, EM_HEXAGON (164), e_flags 0x66
Executable PT_LOAD: file [0x40000,0x353d38), virtual [0xb0000000,0xb0313d38)
```

The private input was extracted from the supplied multipart downloads archive,
joined in the specified order `001,002,003.1,003.2,004,005.1,005.2,006`.
`zstd -t` passed for the joined archive and the supplied next-toolchain archive.
The full WLAN hash matches `reference/xiaomi-nabu/firmware.sha256`. Original
firmware, reference disassemblies, host toolchains and build products are kept
outside Git. Emission does not require or read the original firmware; an
optional reference is used only for verification after source assembly.

## Lower-boundary handoff

File `0x100000` is inside the function beginning at file `0xfff98`, runtime
`0xb00bff98`. Its entry packet calls the external register-save helper at
`0xb02a5754` and allocates a `0x270`-byte frame. The packet at `0xb00bffec`
contains a branch to `0xb00c0000`.

The entire crossing function is `[0xfff98,0x1001c4)`: 104 bytes before Agent
14's allocation and 452 bytes inside it. The in-scope tail
`[0x100000,0x1001c4)` remains **uncovered**. Its frame/register preconditions
belong to the earlier entry. Agent 13 / the integration owner must coordinate
this seam; no bytes before `0x100000` are added by this branch. The first new
complete function starts at file `0x1001c4`.

## Checkpoint 01 — four complete functions, 868 executable bytes

Generated interval: file `[0x1001c4,0x100528)`, runtime
`[0xb00c01c4,0xb00c0528)`. All intervals in this document are half open.

| Source stem | File range | Bytes | Recovered operations |
| --- | --- | ---: | --- |
| `agent14-late-state-apply.S` | `[0x1001c4,0x100240)` | 124 | Match record/context tokens, decode a status word, update state and a flag. |
| `agent14-late-descriptor-submit.S` | `[0x100240,0x100410)` | 464 | Acquire a descriptor, update packed fields and lengths, submit it, count success, handle failures. |
| `agent14-late-record-gate.S` | `[0x100410,0x1004e8)` | 216 | Apply parsed-record flags, the inverted membership predicate, optional six-byte comparison and a subsequent helper gate. |
| `agent14-late-rate-code-set.S` | `[0x1004e8,0x100528)` | 64 | Check counted byte values against the four-value bitmap using a hardware loop. |

These are explicit Hexagon instruction mnemonics and VLIW packets. They are
not recovered OEM C/Rust source. Higher-level external helper names remain
unproven. There are no embedded raw opcode words, literal pools, metadata,
layout fill or externally copied payload bytes in the output.

The four routines contain 106 packets and 217 encoded 32-bit words, including
17 constant extenders and 18 duplex words. The rate predicate has one loop0
end marker. These counts distinguish encoded words from architectural
instructions; a duplex word contains two operations.

```text
Generated executable bytes: 868
Generated metadata/data bytes: 0
Generated layout-fill bytes: 0
External reference bytes in generated output: 0
Reference mismatched bytes: 0
Generated span SHA-256:
44955f577a3d87ba4fae4c880bd28aac9fae76a9d9cd21307c96a9eace8fa47c
```

### Semantics reviewed

The first handler compares record `+0x14` with context `+0x280`. When they
match, status words `0x01` or `0x80` set context `+0x08` to 2; `0x02` or
`0x100` clear bit 0 of context `+0x284` and set state to 1. Other words leave
those fields unchanged. The external diagnostic calls and its return value
are not modeled as a recovered high-level API.

The counted-byte predicate takes the count from `input[0]`, masks each
subsequent byte with `0x7f`, and returns 1 exactly when all counted codes are
in `{2,4,11,22}`. Empty input count returns 1. Its bitmap is
`(1<<2)|(1<<4)|(1<<11)|(1<<22) = 0x400814`. The function has no input-buffer
length check. The caller at `0xb00c0410` **rejects a nonzero predicate result**;
describing this caller as accepting all four-value lists would invert its logic.

Packet reads normally see old register values. For example, the predicate's
entry sets `r0=1` while moving the incoming pointer from old `r0` to `r2`.
Likewise, the descriptor routine reads an old table pointer while zeroing the
same register, preserves a length byte while shifting the register, and uses
`.new` explicitly for other stores. These dependencies are annotated in source.
Three packets in that routine needed source ordering / `:mem_noshuf` to retain
the original duplex choices and memory order. No binary patching was used.

The descriptor failure helper at `0xb021cfc4` appears to be a terminal
assertion call. Its non-returning behavior is an inference from the recovered
control flow and the next function prologue, not proof from its implementation.

### Verification and tools

Primary emission: supplied Ubuntu LLVM 18.1.3 `llvm-mc`, targeting
`hexagonv66`. Independent assembly: supplied LLVM 6.0.1 `llvm-mc`, targeting
`hexagonv65`; both match all four routines exactly. The latter supports these
particular instructions; this does not establish V65 compatibility for the
whole WLAN image. LLVM 22 is not required for this verified group. The firmware
reference was rechecked after both independent source builds.

The structural parser in `tools/wlan_late_packets.py` checks packet termination,
word bounds, hardware-loop marker placement, duplex classification and extender
structure. It supplements V66 instruction decoding and full byte comparison.
It cannot infer function boundaries, prove arbitrary bytes are instructions,
or validate the entire ISA's slot/dataflow semantics.

Primary format references used to review the parser:

- <https://llvm.org/doxygen/HexagonDisassembler_8cpp_source.html>
- <https://llvm.org/doxygen/HexagonMCCodeEmitter_8cpp_source.html>
- <https://llvm.org/doxygen/HexagonBaseInfo_8h_source.html>

The module also cites the later Qualcomm V73 scalar manual only as corroboration
of common packet framing. V73 opcode definitions are not used as a V66 whitelist.

Toolchain setup initially exhausted temporary disk space because the downloaded
archive and extracted installers were duplicated. The reproducible duplicates
were removed; the complete required Rust checks were then rerun successfully.
Only the successful final runs count as checkpoint evidence.

Final checkpoint-01 regression: **160 Rust tests and 82 Python tests passed,
with 0 skipped tests**. The workspace release build and `git diff --check`
passed. The pinned-reference build, independent assembly and complete V66
decode all passed. Evidence is in `docs/evidence/agent20-14/checkpoint-01.json`.

### Reproduction

Set `WLAN_LATE_LLVM_MC` to a Hexagon-capable `llvm-mc` with V66 support,
`WLAN_LATE_SECOND_LLVM_MC` to the independent assembler, and
`WLAN_LATE_LLVM_OBJDUMP` to the V66 decoder. Runtime libraries may need to be
provided through `LD_LIBRARY_PATH`. The scripts do not download dependencies.

```sh
python3 tools/build-wlanmdsp-late.py /tmp/wlan-late.bin \
  --llvm-mc "$WLAN_LATE_LLVM_MC" \
  --independent-llvm-mc "$WLAN_LATE_SECOND_LLVM_MC" \
  --independent-cpu hexagonv65 \
  --llvm-objdump "$WLAN_LATE_LLVM_OBJDUMP" \
  --reference "$WLANMDSP_REFERENCE" \
  --through-checkpoint 1 --manifest /tmp/wlan-late.json

CARGO_TARGET_DIR=/tmp/agent20-14-target cargo test --workspace --offline --locked
CARGO_TARGET_DIR=/tmp/agent20-14-target cargo build --workspace --release --offline --locked
python3 -m unittest discover -s tests -p 'test_*.py' -v
git diff --check
```

For the inherited early-WLAN tests, also set `WLAN_LLVM_MC` to LLVM 6 and
`WLAN_LLVM_MC_V66` to the V66 assembler. The output manifest records whether
reference, independent assembly and decoder checks actually ran. The checkpoint
handoff report records the exact tested Git HEAD and final test counts.

### Remaining at checkpoint 01

- Leading seam: file `[0x100000,0x1001c4)`, 452 bytes.
- Next complete function: file `0x100528`, runtime `0xb00c0528`.
- Other unimplemented allocated bytes: `[0x100528,0x353d38)`.
- Checkpoint-50 integration, external callees, non-executable regions, whole-image
  behavior, authentication and hardware execution remain unverified.

The emitted file is a code fragment. It is not a complete or flashable firmware
image. Existing Agent 07 bytes are inherited and are not added to the new Agent
14 coverage count. Shared CLI, central module registration, generic image
construction and other agents' address ranges are unchanged.
