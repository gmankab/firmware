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

## Checkpoint 02 — record handling, validation and request lifecycle

Six complete functions add **1,244 executable bytes**, file
`[0x100528,0x100a04)`, runtime `[0xb00c0528,0xb00c0a04)`. Cumulative Agent
14 coverage is now **2,112 consecutive bytes**, file `[0x1001c4,0x100a04)`.

| Source stem | File range | Bytes | Recovered operations |
| --- | --- | ---: | --- |
| `agent14-late-caller-frame-search.S` | `[0x100528,0x10059c)` | 116 | Use caller-frame data to search context-associated candidate patterns. |
| `agent14-late-counted-record-collect.S` | `[0x10059c,0x100698)` | 252 | Walk length-prefixed records, collect at most fifteen masked codes, retain a type-zero record, set a conditional flag. |
| `agent14-late-signature-dispatch.S` | `[0x100698,0x1007c4)` | 300 | Route two type-`0xdd` signatures to distinct child parsers and handle truncated records. |
| `agent14-late-configuration-check.S` | `[0x1007c4,0x100840)` | 124 | Validate scalar ranges and alignment without changing the input. |
| `agent14-late-request-submit.S` | `[0x100840,0x10099c)` | 348 | Construct a stack request, submit it, update flags/counters and invoke optional timer-like helpers. |
| `agent14-late-request-teardown.S` | `[0x10099c,0x100a04)` | 104 | Perform guarded teardown, clear the active flag, reset state and propagate the request result. |

The new span contains 157 packets and 311 encoded words, including 16 extenders
and 24 duplex words. It adds no literal pool, metadata/data, layout fill or
copied reference bytes. New-span SHA-256:

```text
73360393f71fcafebcc45c015c40af23a90e7518ac7636237fde9e505e105334
```

Cumulative 2,112-byte span SHA-256:

```text
ed80dc6c8682df0636d2c5d65f629dc4b7578c88b2f1816955bccc69aaeb1527
```

### Additional semantic findings

The scalar validator accepts an index below the global limit, two ordered
nonzero 16-bit values, another value with no upper-half bits, a size in
`{8,16,24,32,40,48,56,64,72,80}`, and a count in `[1,512]`. It returns zero
when those checks pass and -1 otherwise. No null-pointer check is present.

The signature dispatcher recognizes bytes `00:50:f2:04` and `50:6f:9a:09`
after type `0xdd`, and passes the remaining payload to separate child functions.
It initializes each child-result slot to -1 and returns their sum. No protocol
API name is inferred from those constants alone.

The caller-frame helper has an unusual input contract: after its frame setup,
`r29+0x20` is the caller's stack pointer. It tests the full word at caller
`+0x18c` and passes only that word's low eight bits as a candidate count, with
candidate bytes at caller `+0x164`. A **zero full word returns 1 immediately**.
A nonzero word whose low byte is zero still performs searches with zero
candidates, which return zero. The later helper at `0xb00c0e74`, inspected to
resolve this dependency, is a candidate-pattern search rather than a simple
equality comparison. The pointless-looking final loop only increments
registers after the second miss; it does not repeat the external search.

Request submission sets the active flag even when its external helper reports
nonzero. Teardown clears the flag even when its helper reports nonzero. Both
behaviors are preserved, not replaced with presumed success-only updates.
The setup routine retains the full unsigned 64-bit product of its interval
field and 1000 for the timer-like call; external API identities remain unproven.

Three packets in request setup needed legitimate source-order changes to retain
the original duplex grouping. LLVM 18/V66 and LLVM 6/V65 emit all six functions
byte-identically. The combined build rechecks the earlier four functions too.

The latest-registry regression builds every registered function and verifies
that checkpoint 01 remains an exact prefix. Final checkpoint evidence records
**160 Rust tests and 83 Python tests passing, 0 skipped**, a passing release
build, successful complete V66 decoding, zero reference/independent-assembler
mismatches and `git diff --check`. See
`docs/evidence/agent20-14/checkpoint-02.json`.

Forward continuation: file `0x100a04`, runtime `0xb00c0a04`. The 452-byte lower
seam is still uncovered. Build this snapshot with `--through-checkpoint 2`;
the original `--through-checkpoint 1` output remains reproducible.

An independent clean clone of the checkpoint-02 bundle rebuilt all 2,112 bytes
with no reference path or reference environment variable. The resulting hash,
both assembler outputs and decoder coverage match. Its focused test run passed
39 tests and intentionally skipped the two reference-only tests; this is
separate from the fully supplied 83-test checkpoint gate, which skipped none.
All ten registered sources and their builder/parser/test dependencies were
present in the bundle. Original firmware and host tools are external inputs.

The evidence keys distinguish `starting_base` from `commit_parent`. A prior
checkpoint-02 metadata key called the starting base `actual_parent`; that
ambiguous label is corrected here. The actual Git parent chain was always
`d6104bb` → `762ff9f` → `fb07423`.


## Checkpoint 03 — attribute dispatch, pattern search and session resources

Sixteen complete functions and thunks add **3,360 executable bytes** at file
`[0x100a04,0x101724)`, runtime `[0xb00c0a04,0xb00c1724)`. The cumulative code
run is **5,472 bytes** at file `[0x1001c4,0x101724)`, covering 26 routines.
No data or alignment bytes are added by this checkpoint.

| Source stem | File range | Bytes |
| --- | --- | ---: |
| `attribute-record-apply` | `[0x100a04,0x100ac8)` | 196 |
| `three-byte-record-parse` | `[0x100ac8,0x100b5c)` | 148 |
| `short-type-dispatch` | `[0x100b5c,0x100d78)` | 540 |
| `big-endian-attribute-parse` | `[0x100d78,0x100e14)` | 156 |
| `eight-byte-pattern-match` | `[0x100e14,0x100e74)` | 96 |
| `pattern-pointer-search` | `[0x100e74,0x100eb0)` | 60 |
| `wide-type-dispatch` | `[0x100eb0,0x1012a0)` | 1008 |
| `shared-resource-create` | `[0x1012a0,0x1012f4)` | 84 |
| `failure-forward` | `[0x1012f4,0x101304)` | 16 |
| `session-resource-setup` | `[0x101304,0x101394)` | 144 |
| `session-state-callback` | `[0x101394,0x101524)` | 400 |
| `session-resource-switch` | `[0x101524,0x1015d4)` | 176 |
| `session-resource-release` | `[0x1015d4,0x101680)` | 172 |
| `resource-disable-thunk` | `[0x101680,0x101688)` | 8 |
| `session-mode-reset-thunk` | `[0x101688,0x101694)` | 12 |
| `timing-count-adjust` | `[0x101694,0x101724)` | 144 |

Each source stem above is prefixed `agent14-late-` and suffixed `.S` under
`sources/xiaomi-nabu/wlanmdsp/`. The new group contains 412 packets and 840
encoded words, including 47 extenders and 57 duplex words, with one loop0
end marker. All 840 words decode as V66 instructions with zero unknowns.

The three record parsers recover four-byte big-endian type/length headers,
three-byte records whose length uses only the second byte, bounded optional
field copying, and output flag handling. Signed nonpositive total lengths
follow the original empty-success paths. No extra bounds checks are inserted
into the generated instruction stream.

The short dispatcher handles indices 0 through 18; the wide dispatcher handles
normalized tags `0x1002` through `0x106a`. At this checkpoint their table data
at runtime `0xb02b6cd0` and `0xb02b6d1c` remain separately identified external
inputs. Their bytes are not embedded in or counted as code. Every local case
handler and the original default/error paths are reconstructed.

The eight-byte pattern predicate first invokes an external comparison-like
helper. If it does not report equality, the local fallback accepts equal
first two bytes when the remaining six bytes in the second argument are zero.
The pointer-array search returns 1 on its first accepted candidate and 0 for
no match or an empty array. This resolves the local caller-frame dependency
from checkpoint 02 without asserting the external helper's full contract.

The session routines recover lazy allocation, callback setup, state 2/3
transitions, resource slot switching, release and two small forwarding thunks.
The final timing routine preserves an old-value threshold comparison before
adding the count-scaled interval and input offset. Its ordered stores are kept
intact even when the count pointer and output-time pointer alias.

Seven scheduling differences were resolved using legal packet source ordering
and two `:mem_noshuf` directives. All 3,360 new bytes and the entire previous
2,112-byte prefix match both the pinned reference and the two LLVM versions.
No raw opcode insertion or post-assembly byte repair is used.

New-span SHA-256:

```text
cd7a034676ef728179336b34869dc713d49f509c9ac001648677c8d79d553be1
```

Final regression and source fingerprints are recorded in
`docs/evidence/agent20-14/checkpoint-03.json`. All 160 Rust and 83 Python tests
pass with zero skips; the release build and whitespace check pass. The forward
continuation is file `0x101724`, runtime `0xb00c1724`, at a new function entry.
The lower 452-byte seam remains unimplemented. The wider allocation, external
callees, behavior on a running target and authentication remain incomplete.

Cumulative checkpoint-03 code SHA-256:

```text
4242c17caba9322e7ac687357acffa510839599dc5ebf85b856fc414ffa638e7
```

Four diagnostic-ending entry boundaries in this group rely on the explicit
non-return inference for the external diagnostic target; no hardware test of
that contract has been performed.
