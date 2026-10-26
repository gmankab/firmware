# Agent 20-14 — WLAN late Hexagon reconstruction

Latest checkpoint: **18**, with **14,728 contiguous executable bytes**
and **496 independently generated dispatch-table data bytes**, for **15,224**
source-derived bytes in disjoint intervals. Checkpoint 18 adds **1,260**
Hexagon instruction bytes in the counted-record processing function at
`[0x103660,0x103b4c)`. The next function begins at `0x103b4c`. The
Agent 13 leading seam `[0x100000,0x1001c4)` remains unimplemented here.

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

## Checkpoint 04 — dispatch tables derived from source labels

The two dispatch-table dependencies identified in checkpoint 03 are now
reconstructed as **496 data bytes**, emitted by
`tools/build-wlanmdsp-late-tables.py`. The 5,472-byte executable fragment is
unchanged. Two assembly header comments now describe the separate table
builder; their source-file hashes change, but their encoded bytes do not.

| Table | File range | Runtime range | Entries | Bytes |
| --- | --- | --- | ---: | ---: |
| Types `0x0000..0x0012` | `[0x2f6cd0,0x2f6d1c)` | `[0xb02b6cd0,0xb02b6d1c)` | 19 | 76 |
| Tags `0x1002..0x106a` | `[0x2f6d1c,0x2f6ec0)` | `[0xb02b6d1c,0xb02b6ec0)` | 105 | 420 |

The builder assembles fixed snapshots of the two registered source functions,
checks their complete code fingerprints, and reads the resulting ELF32 symbol
tables. Each output word is a little-endian runtime address calculated from
the registered function base plus a named label's section offset. The source
specifies case-type sets and a default label; it contains no copied firmware
address array. Optional reference access occurs only after data generation.

Every required target must be a defined symbol in the correct `.text` section,
strictly inside the function and at a structurally parsed packet start. A
missing explicit case label is an error, never an implicit default. LLVM 18
with `hexagonv66` and LLVM 6 with `hexagonv65` produce identical dependency
code, symbol-derived target mappings and table bytes. These are two LLVM
versions, not two independent ISA implementations.

Both default mappings target **`restore_status`**, not `return_success`.
The short dispatch packet has already set `r0` to zero; the wide dispatch
packet has already set `r3` to zero. Their table loads use the old register
value from the same packet. The short table has 14 explicit cases and five
defaults, for types `1,7,8,10,16`; defaults target `0xb00c0d6c`. The wide table
has 53 explicit cases and 52 defaults targeting `0xb00c125c`. All 124 generated
entries equal the pinned reference. The complete mappings and dependency
fingerprints are recorded in checkpoint-04 evidence.

The new 25 tests cover malformed and ambiguous ELF symbols, missing labels,
undefined and wrong-section targets, out-of-function targets, instructions
inside a packet, dependency drift, reference-free generation, exact reference
equality and a changed reference byte outside the emitted span. The full
regression passes **160 Rust tests and 108 Python tests, with zero skips**;
the release build and `git diff --check` also pass.

### Coverage and gaps

The separate code and table outputs must be placed at their own file offsets.
They must not be concatenated and treated as one contiguous firmware range.
No intervening bytes, alignment fill or original firmware bytes are emitted.

| Classification | File range | Bytes |
| --- | --- | ---: |
| Source-derived executable | `[0x1001c4,0x101724)` | 5,472 |
| Source-derived data | `[0x2f6cd0,0x2f6ec0)` | 496 |
| Total covered union | Two disjoint ranges | 5,968 |
| Uncovered leading seam | `[0x100000,0x1001c4)` | 452 |
| Uncovered between code and tables | `[0x101724,0x2f6cd0)` | 2,053,548 |
| Uncovered after tables | `[0x2f6ec0,0x353d38)` | 380,536 |

The remaining allocation is **2,434,536 bytes**. Forward code reconstruction
can continue at file `0x101724`, runtime `0xb00c1724`. The lower seam still
requires coordination with Agent 13 / the integration owner. Neither the
later RW load nor another agent's allocated range has been modified.

Code SHA-256, unchanged from checkpoint 03:

```text
4242c17caba9322e7ac687357acffa510839599dc5ebf85b856fc414ffa638e7
```

Combined 496-byte table SHA-256:

```text
f33f4dae122e666e9eefcfd539eb2405bc967d4928e96a72b3897907144f7390
```

### Reproduce the separate table artifact

```sh
python3 tools/build-wlanmdsp-late-tables.py /tmp/wlan-late-tables.bin \
  --llvm-mc "$WLAN_LATE_LLVM_MC" \
  --independent-llvm-mc "$WLAN_LATE_SECOND_LLVM_MC" \
  --reference "$WLANMDSP_REFERENCE" \
  --manifest /tmp/wlan-late-tables.json
```

Omit `--reference` for source-only emission; its manifest then records
`reference_checked: false` and null mismatch counts. The main code builder
with `--through-checkpoint 3` emits all 26 routines. Run the same Rust,
release and Python regression commands documented under checkpoint 01.

The checkpoint-04 handoff records the exact tested commit, clean-clone and
portable-patch replay results, artifact hashes and complete coverage union.
This remains a portable contribution on `d6104bb`, pending checkpoint-50
integration. The two reconstructed tables resolve those specific data
dependencies; external callees, other referenced data, hardware behavior,
authentication and whole-firmware completion remain unverified.

## Checkpoint 05 — packet-status writer and status availability helper

Two adjacent, independently authored Hexagon V66 routines have been
reconstructed at file `[0x101724,0x101870)` (332 bytes), mapped to runtime
`[0xb00c1724,0xb00c1870)`:

- `[0x101724,0x101848)`: build a status/error command from the live interface
  descriptor. It checks a status bit, computes an available byte count, packs
  four little-endian payload fields, and submits the command to an external
  diagnostic/transaction helper. The packet at `[0x101840,0x101848)` is a
  **forced constant-extended far jump**; its two original A32-sized Hexagon
  encoded words are generated from `jump ##0x1e3ff0`, not pasted binary data.
  The source is split into two adjacent fragments strictly to preserve this
  far-jump packet's encoding.
- `[0x101848,0x101870)`: check the active status-word flag and calculate a
  count/offset, restoring the frame with `dealloc_return`.

The body has a required `:mem_noshuf` packet to preserve the original
two-word load/store encoding; omitting it would silently compress the packet
and corrupt branch offsets. PC-relative external calls must be adjusted for
the independent object origin. All three fragment SHA-256 hashes, exact
locations, and V66 decode counts are in
`sources/xiaomi-nabu/wlanmdsp/agent14-late-regions.json` and
`docs/evidence/agent20-14/checkpoint-05.json`.

Build commands (the compiler files and reference remain outside Git):

```sh
python3 -S tools/build-wlanmdsp-late.py /tmp/agent20-14-code.bin \
  --llvm-mc "$LLVM18_MC" --cpu hexagonv66 \
  --independent-llvm-mc "$LLVM6_MC" --independent-cpu hexagonv65 \
  --llvm-objdump "$LLVM18_OBJDUMP" \
  --reference /private/wlanmdsp.mbn \
  --through-checkpoint 5 --manifest /tmp/agent20-14-code.json
```

With `--reference` omitted, the generated code remains byte-for-byte
identical, and the manifest correctly indicates that reference comparison
was not performed. The independent toolchains are separate LLVM versions
but share LLVM's Hexagon ISA lineage. Exact encoding agreement does not
prove runtime execution or high-level algorithm identity.

Code coverage `[0x1001c4,0x101870)` = **5,804 bytes**, table coverage
`[0x2f6cd0,0x2f6ec0)` = **496 bytes**, total **6,300**. These disjoint spans
must not be concatenated into a fake contiguous MBN. `[0x100000,0x1001c4)`
remains the unhandled Agent13 seam; `[0x101870,0x2f6cd0)` remains uncovered.
No original firmware/signature/certificate bytes are committed to Git.

## Checkpoint 06 — post-status service and helper routines

Six complete small functions immediately follow the two checkpoint 05
status-packet routines, at file `[0x101870,0x101be4)` and virtual address
`[0xb00c1870,0xb00c1be4)`:

| File range | Length | Source-derived operation group |
| --- | ---: | --- |
| `[0x101870,0x101a3c)` | 460 | Build a live buffer-completion record, apply status/limit checks, invoke a diagnostic hook and tail-transfer control. |
| `[0x101a3c,0x101a64)` | 40 | Forward a session warning through the shared diagnostic helper. |
| `[0x101a64,0x101b20)` | 188 | Probe session status and conditionally enable its active descriptor. |
| `[0x101b20,0x101b78)` | 88 | Update buffer/status counters and issue a size report. |
| `[0x101b78,0x101bd0)` | 88 | Validate descriptor level and restore saved registers. |
| `[0x101bd0,0x101be4)` | 20 | Emit fixed warning arguments and call a terminal diagnostic helper (non-returning inferred only). |

The 460-byte record builder requires two Hexagon packet encoding details:
a `##0x1e3dfc` extended far jump and an explicitly ordered two-word
load/move packet; shuffling that packet changes the original duplex choice.
Both problems are resolved using legal Hexagon assembly source operations,
without embedding original firmware opcode bytes.

The entire 884-byte continuation and preceding 5,804 bytes are independently
reassembled with LLVM 18/V66 and LLVM 6/V65 and exactly match the pinned
WLAN reference. The builders operate without reading that reference unless
verification is explicitly requested. The previously recovered 496-byte
dispatch table is a separate output interval and must remain disjoint from
code. The remaining owned WLAN file budget is 2,433,320 bytes, and
`[0x100000,0x1001c4)` still needs Agent 13's coordination.

Build with `--through-checkpoint 6` instead of `5` in the checkpoint-05
command. See `docs/evidence/agent20-14/checkpoint-06.json` for current
per-region hashes and test provenance. Original private reference firmware
and OEM signature material remain outside Git.

## Checkpoint 07 — metadata query and auxiliary session dispatch

Four adjacent complete Hexagon routines were reconstructed in the pinned
`wlanmdsp.mbn` at file `[0x101be4,0x102014)` (1,072 executable bytes):

| File range | Bytes | Source-derived purpose |
| --- | ---: | --- |
| `[0x101be4,0x101da8)` | 452 | Inspect interface state, emit warnings, and dispatch conditional callbacks. |
| `[0x101da8,0x101e24)` | 124 | Validate command fields and record length limits. |
| `[0x101e24,0x101e64)` | 64 | Adjust byte counters and route the warning path. |
| `[0x101e64,0x102014)` | 432 | Apply session state flags, inspect status, submit auxiliary packet and transfer control. |

The first and fourth functions require Hexagon `:mem_noshuf` to retain the
original memory load/store instruction order within packets, and the third
and fourth functions require explicit `##` jump constant extenders. Without
these architectural details their assembler outputs differ. The legal,
source-authored packet mnemonics were independently assembled by LLVM 18/V66
and LLVM 6/V65; **all 1,072 added bytes match the original reference**.
Reference comparisons are optional: the emitter never reads the OEM MBN
unless `--reference` is supplied explicitly.

Checkpoint 07 contiguous source-derived code: **7,760 bytes** at
`[0x1001c4,0x102014)`; separate dispatch table data **496 bytes** at
`[0x2f6cd0,0x2f6ec0)`. Total non-overlapping authored bytes: **8,256**.
No source bytes are claimed for intervening holes or for the Agent13 seam
`[0x100000,0x1001c4)`. Next adjacent complete function starts at
`0x102014`; the rest of the large WLAN executable segment is unfinished.
Code equality does not prove behavioral identity or hardware execution.

## Checkpoint 08 — interface event handler and status reply

Recovered two adjacent *complete* Hexagon functions with typed V66 packet
assembly: `[0x102014,0x102280)` (620 bytes) processes live interface
events, warning/command callbacks and descriptor state; `[0x102280,0x102368)`
(232 bytes) builds a bounded status response.

The event handler has a mandatory `:mem_noshuf` packet at offset `+0x154`;
a regular memory-shuffle packet emits the same operations in a different
word order and fails byte matching. The status reply requires original
instruction ordering around a `##25992` constant extender to preserve the
Hexagon encoding. Both were rebuilt with legal source instructions, not
firmware opcode copies. LLVM18 V66 and LLVM6 V65 independently yield exact
OEM-reference bytes in both functions.

Contiguous code `[0x1001c4,0x102368)` now totals **8,612 bytes**.
The source-derived 496-byte dispatch tables remain separate at
`[0x2f6cd0,0x2f6ec0)`; total nonoverlapping independently authored
bytes **9,108**. There are no zero-filled hidden gaps.
The following command-parser function starts at file `0x102368`; its
unverified body is not included. The Agent13 leading seam remains.

## Checkpoint 09 — status/command parser and dispatch tail

The complete 852-byte Hexagon V66 function at file `[0x102368,0x1026bc)`
(runtime `[0xb00c2368,0xb00c26bc)`) decodes status/command bytes from
an interface object, checks count and subtype constraints, traverses a
bounded pointer table and conditionally invokes response/diagnostic helpers.
It restores/returns through the existing terminal dispatch. Comments label
message validation and error paths; names for external helpers remain
inferred from call sites, not independently verified C-level APIs.

The original decoder emission alone was insufficient to reproduce the OEM
packet stream. Exact source assembly required (1) preserving `r0 = #2`
before the `##0xffff` operation in one packet, (2) preserving the precise
combine/store packet ordering in the block handling the message's address
pair, and (3) explicitly extending three external far jumps with `##`.
No `.word`, `.incbin` or private firmware payload is used in the authored
assembly source. The selected routine has **114 packets and 213 encoded
32-bit words**, of which **18 are constant extenders and 18 are duplex
words**. It has no hardware-loop packet markers.

Both independent compilers produced the same 852 bytes: LLVM **22.1.8**
assembler targeting Hexagon V66 and LLVM **6.0.1** targeting V65.
The V66 decoder accounted for every encoded word with zero unknowns.
The entire **9,464-byte** cumulative executable prefix matches the pinned
`wlanmdsp.mbn`, SHA-256
`87c2c7bb1fe97eaf007a947832feba0898644b4d6626ac8ef49d8b94dd82604b`.
The separately built 496-byte tables also match. These are instruction-level
encoding equivalences; on-device behavior and high-level source semantics
are not proven.

```
python3 tools/build-wlanmdsp-late.py /tmp/wlan-late-code.bin   --llvm-mc "$WLAN_LATE_LLVM_MC" --cpu hexagonv66   --independent-llvm-mc "$WLAN_LATE_SECOND_LLVM_MC"   --independent-cpu hexagonv65   --llvm-objdump "$WLAN_LATE_LLVM_OBJDUMP"   --reference "$WLANMDSP_REFERENCE" --through-checkpoint 9   --manifest /tmp/wlan-late-cp09.json
```

Rust workspace: **160 passed**. Python discovery: **125 discovered,
120 passed, 5 skipped** (optional environment-dependent cases). Release
build and `git diff --check` pass. The current continuation is the
function at `0x1026bc`; **2,430,544 bytes of the exclusive assignment** remain
uncovered, counting the leading 452-byte seam and excluding the separate
496-byte jump-table range. This is not a complete or signed firmware.

## Checkpoint 10 — session activity update and descriptor normalization

Three adjacent, complete functions were reconstructed after the command
parser. `[0x1026bc,0x102754)` (152 bytes) checks current WLAN control
state, updates active session metadata or counters, and preserves the
original far-jump diagnostic thunk after its return. `[0x102754,0x1027a8)`
(84 bytes) normalizes optional descriptor bytes and forwards a bounded
status message. `[0x1027a8,0x102820)` (120 bytes) selects a status byte
by attribute type and sends the associated request.

Hexagon packet encoding requires **one `:mem_noshuf` packet in the second
function and two in the third**. Without these architectural constraints,
the assembler either shuffles predicate-dependent load/store operations or
uses a duplex word, losing the original packet size and shifting branches.
The first function's terminal far jump uses `jump ##-792140` so the extender
is explicitly retained. LLVM 22/V66 and LLVM 6/V65 independently reproduce
all 356 bytes and the complete 9,820-byte code fragment without an OEM
firmware input. An optional pinned-reference comparison reports zero byte
mismatches. The separate 496-byte table generator was also checked unchanged.

Build using `--through-checkpoint 10`, and see
`docs/evidence/agent20-14/checkpoint-10.json`. Tests run in a private Cargo
target directory: 160 Rust tests, 103 Python test cases (98 executed and
passed, five skipped for optional dependencies), and workspace release build
completed. `git diff --check` passed. These results do not establish high-level
source-level equivalence, on-device execution or signed firmware support.


## Checkpoint 11 — normalized record status parser and packed reply

Two complete V66 Hexagon routines reconstruct `[0x102820,0x102aa8)`,
adding **648 executable instruction bytes**. `[0x102820,0x1029c4)`
(420 bytes) validates record type and width fields, conditions the controller
callback path, and updates the active record after bounded checks.
`[0x1029c4,0x102aa8)` (228 bytes) normalizes 6-byte status fields into
packed reply memory and chooses between normal and failure callbacks.
The two memory packets in the second routine require `:mem_noshuf` and
its two far jumps have explicit constant extenders; a third far jump in
the first routine is also explicitly extended. These are source-authored
Hexagon mnemonics and VLIW packets; there are no proprietary opcode-word
arrays, `.incbin` directives or copied OEM bytes in the assembler sources.

An independent LLVM 22.1.8 Hexagon V66 assembler and LLVM 6.0.1 V65
assembler produce identical fragments. The full 10,468-byte contiguous
code interval matches the pinned `wlanmdsp.mbn` exactly; separately generated
496-byte dispatch tables remain unchanged and byte-identical. Current total
**10,964 independently authored bytes**, with zero mismatches, zero generated
layout fill, and no external reference bytes used in the emitted artifacts.

Full checkpoint tests: **160 Rust** passed; **105 Python test cases**
(83 passed, 22 skipped because optional external validation capabilities
were absent in this process environment). Workspace release build passed.
These checks establish Hexagon instruction *encoding* equivalence, not
source-level semantic identity or device execution. The leading Agent 13
452-byte seam `[0x100000,0x1001c4)` remains unimplemented. The next function
starts at file `0x102aa8`; the remaining exclusive allocation remains large.


## Checkpoint 12 — counted frame preparation

The complete 552-byte routine at `[0x102aa8,0x102cd0)` reads the current
frame and counted descriptor, validates controller replies and count/type
fields, copies selected six-byte hardware address fields and dispatches
request packet completion. Explicit Hexagon packet source is assembled from
mnemonics and local labels (no OEM opcode arrays). The far `jump ##1977576`
needs its extension word and the `r21 = #0x6; r2 = #0x6; r1 = r21`
source order preserves the original two-word packet; changing either
produces a different reference binary.

LLVM 22.1.8 V66 and independent LLVM 6.0.1 V65 produce **identical 552 bytes**,
with zero mismatches against pinned `wlanmdsp.mbn`. The complete contiguous
emitted source is 11,020 executable bytes, plus 496 separately source-generated
jump-table data bytes = **11,516 distinct source-derived bytes**. No layout
padding or externally supplied OEM payload was used in these outputs.

All **160 Rust tests** passed, 107 Python tests discovered (85 passed and
22 skipped for optional external dependencies), and the release build passed.
Leading Agent 13 seam `[0x100000,0x1001c4)` remains unimplemented.
Next unimplemented complete function starts at `0x102cd0`. These are
packet-level encoding equivalences, not verified original C algorithms or
hardware execution.


## Checkpoint 13 — status flag helper and checksum initializer

The **20-byte** routine `[0x102cd0,0x102ce4)` stamps a controller status
flag using the stored two-register response and returns. The **552-byte**
routine `[0x102ce4,0x102f0c)` initializes the output record, serializes a
controller status payload, and contains a bounded checksum routine with an
architectural Hexagon hardware loop. Both function boundaries are independent
of Agent 13 and do not overlap another agent's ELF interval.

Two architectural packet details are critical. The far dispatch uses
`jump ##1976776`; without an explicit Hexagon extender the V66 assembler
silently emits an incorrectly shortened jump. Two memory operations require
`:mem_noshuf` to preserve load/store order. Finally, the original
`[0x102cec,0x102cf8)` packet consists of two ordinary instructions and a
terminal duplex pair. LLVM's automatic shuffler chooses a different duplex
from the same mnemonics, so the source owns a **second, typed assembly
fixture** containing the three named operations as separate legal
mini-packets. The builder checks that its encoding has parse kinds 11/11/00
and changes **only** the first two Hexagon parse markers to continuation
kind 01, producing a single legal original-layout 12-byte packet. No OEM
raw instruction words, `.word` directives, or `.incbin` payloads are present;
the combined packet is independently built under both LLVM 22/V66 and
LLVM 6/V65 and checked against the pinned firmware. Packet framing for the
entire function is validated on the resulting bytes.

Full generated contiguous code `[0x1001c4,0x102f0c)` is **11,592 bytes**;
separate dispatch tables remain **496 bytes** at `[0x2f6cd0,0x2f6ec0)`,
total **12,088 source-derived bytes**, with zero mismatches and zero
copied external reference bytes or hidden layout padding. All 160 Rust
tests passed, 110 Python tests discovered (88 executed successfully, 22
skipped for optional external dependencies), and release build passed.
The next function starts at `0x102f0c`. Code encoding equivalence is not
proof of original high-level semantics or hardware boot.


## Checkpoint 14 — interface status construction

The 516-byte source fragment `[0x102f0c,0x103110)` constructs an interface
status record from packed fields, serializes record lengths and copies
hardware address data, invokes external checksum/copy helpers, and dispatches
normal response vs controller error. The external checksum helper is a
call target; a local checksum hardware-loop body is **not** present in this
function. These semantics are inferred from packet dataflow, not proven
original source API names.

An explicit `jump ##1976456` constant extender is required to reproduce
the original error branch; a non-extended jump would shorten the function
and shift local branch destinations. LLVM 22/V66 and LLVM 6/V65 emit
identical **516 bytes** with zero reference mismatches. The complete source
emits **12,108 executable bytes** contiguously `[0x1001c4,0x103110)` plus
**496 source-generated data-table bytes** in a separate original address
range, total **12,604** source-derived bytes. No proprietary opcode or
OEM payload bytes are copied into compiled fragments.

All 160 Rust tests passed; Python discovered 112 tests with 90 executed
and passed, 22 skipped for optional external capabilities. The release build
passed. The next complete function begins at file `0x103110`.

## Checkpoint 15 — status frame command and duplex mini-packets

The complete 692-byte Hexagon function `[0x103110,0x1033c4)` (runtime
`[0xb00c3110,0xb00c33c4)`) parses a counted response, tests the record
signature and subtype, checks frame boundaries and dispatches an appropriate
status/diagnostic command. It ends in the original explicit extended
`jump ##1975360` to a shared return/error handler. The observed register
and memory dataflow supports these descriptions, not recovered high-level C
function names or on-device behavior.

Two three-word packets at local offsets `0x4c` and `0x14c` rely on the Hexagon
duplex selection. LLVM's default shuffler selects a different legal
three-word grouping, even with identical source instructions. Two short
source-authored assembly fixtures express the intended operations using
three legal mini-packets (ordinary/ordinary/duplex). The builder verifies
packet parse kinds `11/11/00` and transforms only the first two packet-ending
markers into continuation kinds (`01/01/00`); **no raw OEM instruction words,
`.word` overrides or vendor byte arrays** are inserted. The resulting typed
instruction stream is byte-for-byte identical to the pinned reference and
independently agrees under LLVM 22/V66 and LLVM 6/V65.

The verified adjacent executable code total is **12,800 bytes** over
`[0x1001c4,0x1033c4)`. The 496-byte separate source-derived dispatch tables
remain unchanged: total **13,296 independently source-derived bytes**, of
which none are padding or external payload bytes. In-scope leading gap
`[0x100000,0x1001c4)` remains 452 bytes and depends on Agent 13's crossing
function. The next uncovered file offset is `0x1033c4`. Full firmware and
hardware operation are not established.

## Checkpoint 16 — packed controller response and diagnostic handoff

The complete Hexagon routine `[0x1033c4,0x103560)` (412 bytes) builds a
controller response from packed address fields, initializes a tagged output
record, copies data through two external helpers, selects a size/offset, and
hands the result to an external completion routine. This description is based
on typed register/memory dataflow; it does not claim recovered original C or
verified hardware behavior.

The original runtime packet grouping requires three source-level constraints:
1. A packet at function offset `0x78` must state `r2 = #0x10` **before**
   the two address-register operations, preserving its 3-word duplex choice.
2. The load/store packet at offset `0x17c` must use `:mem_noshuf`; without
   this, LLVM changes the memory operation order and emitted bytes.
3. The final external far jump is encoded as `jump ##1975020`, retaining a
   Hexagon extender. Using the section-relative target without its proper
   call-site displacement produces different branch bits.

The generated 412 bytes match pinned WLAN firmware SHA-256
`87c2c7bb1fe97eaf007a947832feba0898644b4d6626ac8ef49d8b94dd82604b`
(note: the authoritative full image digest is the `reference_sha256` field in
`checkpoint-16.json`), and the independent LLVM 6.0.1/V65 assembler emits
exactly the same fragment as Ubuntu LLVM 18.1.3/V66. Complete contiguous
source-built code now covers `[0x1001c4,0x103560)` = **13,212** executable
bytes; the **496** dispatch-table data bytes remain disjoint at
`[0x2f6cd0,0x2f6ec0)`. No intervening code is implied, and the starting
452-byte Agent 13 seam remains unimplemented.

A pre-existing checkpoint-15 regression test assumed its region was the
last registry entry, which becomes false as new checkpoints are appended. It
now locates that *same* named region explicitly before altering its two
packet join offsets; the negative test's intended behavior is unchanged.

The next function starts at file offset `0x103560` (runtime `0xb00c3560`).
Checkpoint 16 passes 160 Rust tests, 143 Python tests, the workspace
release build, byte equality with the pinned reference and two independent
Hexagon assembler runs; source bytes are authored mnemonics with no raw
opcode arrays. Registry and evidence were generated from the successful
reference-checked run, rather than copying the original firmware bytes.

## Checkpoint 17 — auxiliary interface reply and controller I/O reset

The two adjacent functions in `[0x103560,0x103660)` add **256 source-built
executable bytes**, with no extra data words, layout fill or reference bytes
in generated output. `[0x103560,0x1035f8)` (152 bytes) decodes an interface
status record and updates the caller's packed reply fields; it contains a
normal completion branch plus a diagnostic/error path. `[0x1035f8,0x103660)`
(104 bytes) checks two controller status bytes, invokes the platform routine
when set, resets one byte, and returns by restoring its register pair.

For the auxiliary reply, preserving the original VLIW layout requires stating
`r16 = r4` before the predicate comparison in the first packed packet and
using external jump displacement `1974888` in its ordinary terminal packet.
The other source requires explicit extended values and preserves the original
register pair restoration. All 256 new bytes independently match LLVM
18.1.3/V66 and LLVM 6.0.1/V65 output and the pinned `wlanmdsp.mbn`.

Cumulative contiguous source-rebuilt executable: `[0x1001c4,0x103660)` =
**13,468** bytes. Two separate jump tables contribute 496 verified source-
built DATA bytes at `[0x2f6cd0,0x2f6ec0)`, for **13,964 distinct bytes**.
In-scope leading 452-byte crossing-function seam remains unimplemented.
Next new complete routine starts at `0x103660`. The original WLAN image
and validated whole-device behavior remain outside this output.

Validation: **160 Rust tests**, **145 Python tests, zero skipped**, release
build, complete cumulative byte comparison, independent LLVM 6/V65 assembly,
LLVM 18/V66 decoding, Git whitespace and bundle integrity checks passed.
Checkpoint 15's negative fixture test was revised in checkpoint 16 to locate
its named record, rather than assuming future registry additions cannot occur.

## Checkpoint 18 — counted interface record pipeline

The reconstructed complete routine `[0x103660,0x103b4c)` adds **1,260
byte-identical executable bytes**. It inspects an active interface record,
uses a registered callback, checks a status bit and a count field, traverses
counted six-byte tokens in a Hexagon hardware loop, updates status counters,
and serializes diagnostic metadata. The external callback names are inferred
from instructions, not recovered C interfaces; no hardware behavior is claimed.

The source expresses the original V66 packet framing using named Hexagon
mnemonics. Five packet groups require reordered independent assembly
instructions or `:mem_noshuf` for precise duplex/load-store selection. The
hardware loop explicitly has `loop0(0xc,r2)` and its matching `:endloop0`,
expressing a loop target relative to the loop instruction rather than the
object's original section offset. An external `jump ##1973488` includes the
original constant extender. All resulting 1,260 source-built bytes match the
pinned reference, and independent LLVM 6/V65 agrees with LLVM 18/V66.

Cumulative contiguous executable `[0x1001c4,0x103b4c)` is **14,728 bytes**.
The disjoint dispatch tables remain 496 authored data bytes, total
**15,224** distinct source-derived bytes. Layout fill or original firmware
payload is not used in either output. The Agent 13 452-byte leading seam
still requires separate coordination. Next complete function entry is
`0x103b4c`.

Full checks: **160 Rust tests**, **147 Python tests with zero skipped**,
workspace release build, two independent Hexagon assemblers, V66 decoder,
cumulative pinned byte equality, `git diff --check`, and bundle verification.

## Checkpoint 19 — interface command pipeline, word field unpacking and status replies

The complete Hexagon V66 function `[0x103b4c,0x104398)` is source-authored
from named packet mnemonics and local labels, producing **2,124 exact
executable bytes**. Its conditional paths validate record signatures, call
an interface callback, unpack byte fields into 32-bit command words, check
per-interface counters and write packed reply metadata. External helper
identities and names are inferred from dataflow rather than recovered C
function symbols or observed on-device execution.

Using explicit `##` branch extenders preserves two long in-function predicates
and the shared external jump. External call displacements are relative to
packet starts rather than absolute function offsets. Four packets need
carefully chosen legal source instruction ordering to preserve duplex choices
at offsets `0x0d0`, `0x0308`, `0x0510`, and `0x051c`; one mixed memory packet
at `0x05cc` requires `:mem_noshuf`. All other encoded packets are rebuilt
normally using ISA mnemonics; **no raw instruction words, vendor binaries,
or reference-derived executable slices** appear in the generated source.

The primary LLVM 18/V66 builder and independent LLVM 6/V65 assembler emit
identical **2,124** bytes, SHA-256
`9fe0ef263735014e95566985b5043ec1c94e22ede8818f6e074796768711c190`,
which match the pinned `wlanmdsp.mbn` reference with zero differences.
The V66 decoder recognizes all words. Source derived contiguous WLAN code
now covers `[0x1001c4,0x104398)` = **16,852 bytes**. The independently
assembled two dispatch tables remain **496 separate data bytes** in
`[0x2f6cd0,0x2f6ec0)`, for **17,348 unique source-derived bytes total**.
These are not full-image bytes and the leading Agent 13 seam
`[0x100000,0x1001c4)` remains uncovered. First next complete function starts
at `0x104398` (runtime `0xb00c4398`).

Checkpoint 19 passes **160 Rust tests** and **125 Python-discovered tests**
(**103 passed, 22 skipped** for optional external tools), workspace release
build, Git whitespace checks, original-reference binary comparison,
independent LLVM assembler output, and strict packet-structure validation.

## Checkpoint 20 — status serialization and interface attribute helpers

Four adjacent complete functions in `[0x104398,0x1045e8)` add **592 byte-exact
executable bytes**, reconstructed from Hexagon V66 mnemonics and original
packet boundaries rather than raw OEM instruction words:

- `[0x104398,0x104488)` (240 bytes): normalize little-endian command fields,
  construct a status reply and tail-call the existing output helper.
- `[0x104488,0x1044f0)` (104 bytes): briefly clear two record-tag bytes,
  query a status helper, restore the original values, and issue diagnostics
  if the returned 16-bit value differs.
- `[0x1044f0,0x104524)` (52 bytes): bounded skip through variable-length
  tagged input records and tail call for matching type-2 descriptors.
- `[0x104524,0x1045e8)` (196 bytes): validate, inspect and conditionally
  update an interface status field through a shared external handler.

LLVM 18/V66 and LLVM 6/V65 independently assemble all four functions with
zero byte differences against the pinned firmware. At offset `0x88` in the
fourth function, expressing the predicate operation before the two register
writes forces LLVM to retain the required original duplex encoding.
No reference byte slices, linker fill, private certificate material or
embedded opcodes are emitted.

Total independently reconstructed code `[0x1001c4,0x1045e8)` = **17,444
contiguous executable bytes**. Dispatch tables remain **496 noncontiguous
authored data bytes**, total **17,940 source-derived bytes**. The next complete
routine begins at file `0x1045e8` (runtime `0xb00c45e8`); Agent 13's leading
452-byte crossing-function seam remains unimplemented. Firmware signing and
on-target behavior are not claimed.

Validation: **160 Rust tests**, **128 Python-discovered cases** (**106 passed,
22 optional checks skipped**), release build, independent LLVM6/LLVM18
assembly and strict reference comparison passed. The checkpoint-19 test was
updated to select the original checkpoint explicitly, so later registry
growth does not change its negative-boundary assertion.

## Checkpoint 21 — interface record command dispatcher

A complete V66 function `[file 0x1045e8,0x1050d8)` (runtime
`[0xb00c45e8,0xb00c50d8)`) is expressed in named Hexagon mnemonics with
338 explicit packets and 700 encoded words, totaling **2,800 exact executable
bytes**. The routine validates byte fields and signatures, dispatches typed
interface records, traverses a counted record array and emits status replies.
The names reflect static dataflow analysis, not recovered official function
symbols or verified device behavior.

The original instruction stream uses rare Hexagon duplex combinations and
memory-ordering packets. Two read/write packets explicitly use `:mem_noshuf`
at relative offsets `0x0280`, `0x0424`; two conditionally linked external
calls require forced `##` extenders at `0x077c`, `0x0860`. Other packets use
reordered legal source instructions to preserve the original duplex selection
and parse bits. **No raw reference opcode words, unknown-byte padding,
`.word` arrays or copied OEM executable blobs** appear in the source.

LLVM 18/V66 and LLVM 6/V65 independently assemble the original bytes exactly,
SHA-256 `edb482bed18bae21b5da93724800ea18b9e9c7054ed45cbcf838b176287b619a`.
The fragment contains 338 packet endings and 700 fully decoded machine words
with zero unknowns. The cumulative main contiguous independently generated
executable range `[0x1001c4,0x1050d8)` is now **20,244 bytes**. Two original
separately built dispatch tables `[0x2f6cd0,0x2f6ec0)` contribute an additional
**496 authored data bytes**, giving **20,740 source-derived bytes** across
noncontiguous ranges. No linker fill, vendor authorization, secret keys or
reference-supplied executable bytes are included.

The next complete function starts at file `0x1050d8` (runtime `0xb00c50d8`).
The original 452-byte seam `[0x100000,0x1001c4)` crosses Agent 13's prior
boundary and is still not covered. The repository remains a portable Agent 14
branch based on prior WLAN checkpoint `d6104bb`, **not** an integrated
checkpoint-50 descendant.

## Checkpoint 22 — acknowledgement/status helper

Reconstructed complete function `[0x1050d8,0x105140)` = **104 independently
assembled exact executable bytes** (ten Hexagon packets and 26 encoded words).
The helper logs an error or requests a status record, constructs a small
structured byte reply, and tail-returns through the original stack frame.
The function description is inferred from instruction dataflow rather than
validated against hardware or official symbol names.

LLVM 18/V66 and independent LLVM 6/V65 emit identical bytes, SHA-256
`2352709d0573685290f97eeae53d86644b3fbc4f62dd0d75f31219afbd861df9`,
matching the pinned `wlanmdsp.mbn`. Cumulative contiguous executable bytes
`[0x1001c4,0x105140)` = **20,348**. The two separate source-derived
jump tables total **496 bytes**, giving **20,844 source-derived bytes**.
The next function starts at file `0x105140` (runtime `0xb00c5140`), while
Agent 13's 452-byte leading function seam remains unfinished. No OEM
reference byte arrays, headers, signatures, linker padding or hardware
execution evidence are included in generated code.

## Checkpoint 23 — interface-status query and record traversal

A complete Hexagon V66 function `[0x105140,0x1055f8)` is independently
assembled from named packet mnemonics and branch targets, producing **1,208
byte-exact executable bytes**. The routine queries an active record, checks
tagged and counted interface fields, searches a session table, formats reply
records, and reports status through callbacks. These are inferred semantics
from program dataflow, not recovered source-level documentation or evidence
of on-device execution.

LLVM 18/V66 and independent LLVM 6/V65 produce the exact same 1,208 bytes,
SHA-256 `008129dc9a659e40a2d94247b6c45b31f5d87beab97296df2c02873a3d936379`,
identical to the pinned WLAN firmware. There are **147 full Hexagon packets and
302 completely decoded instruction words**. One conditional call requires
an explicit `##` extender; four memory/duplex packets preserve original
instruction order through named source reordering or `:mem_noshuf`. No OEM
opcode arrays or other binary reference slices are present in Git.

Cumulative code `[0x1001c4,0x1055f8)` now totals **21,556 bytes**; distinct
authored dispatch-table data is **496 bytes**, giving **22,052 source-derived
bytes**. The next function starts at file `0x1055f8` (runtime `0xb00c55f8`).
Original 452-byte boundary seam with Agent 13 is still uncovered. The branch
remains based on prior WLAN checkpoint `d6104bb`, pending checkpoint-50-based
integration; the firmware remains unauthenticated and partial.

## Checkpoint 24 — WLAN session and controller-slot helpers

Independently re-assembled six adjacent, nonoverlapping Hexagon V66 helper
routines from named packet mnemonics at firmware file offsets
`[0x1055f8,0x105744)` (runtime `[0xb00c55f8,0xb00c5744)`). They add
**332 source-assembled executable bytes**, **43 complete packets**, and
**83 decoded instruction words**, with **zero unknown decodes**:

- `[0x1055f8,0x105638)` (64): update two status bit flags in a shared
  controller descriptor, with an external call and a return/store packet.
- `[0x105638,0x105668)` (48): request status-cache allocation and publish
  the shared pointer; the call offset is explicitly resolved from source.
- `[0x105668,0x10567c)` (20): session-start diagnostic and tail jump.
- `[0x10567c,0x1056b4)` (56): attach an existing buffer slot or allocate and
  initialize one if the slot was empty.
- `[0x1056b4,0x1056e0)` (44): invalidate and detach a slot, retaining the
  original `:mem_noshuf` packet ordering on a store/load pair.
- `[0x1056e0,0x105744)` (100): clamp a counted channel field and initialize
  up to the available per-channel byte markers.

All six fragments assemble **exactly** with LLVM 18/V66 and LLVM 6/V65;
independent emitted bytes and packet framing match the pinned
`wlanmdsp.mbn` SHA-256
`87c2c7bb1fe97eaf007a947832feba0898644b4d6626ac8ef49d8b94dd82604b`.
The complete code registry now produces **21,888 contiguous executable bytes**
at `[0x1001c4,0x105744)`, plus the existing **496 authored dispatch-table
bytes** in separate noncontiguous regions, for **22,384 source-derived bytes**.
Cumulative code SHA-256:
`8ac6710d76638d76d4efd7b2669c11780eeab815420c52aed6b1bbc8e0964171`.

Offline `cargo test --workspace --offline --locked`: **160 passed**.
`python3 -m unittest discover`: **138 discovered, 116 passed, 22 skipped**
(the skips are prior optional external-tool checks).
Workspace `cargo build --workspace --release --offline --locked`: passed.
The builder's independent decoder accounts for all 5,472 current code words,
with zero unrecognized instructions, and reports zero reference mismatches.

The next complete function starts at file offset `0x105744`, runtime
`0xb00c5744`. The original 452-byte Agent13 boundary seam
`[0x100000,0x1001c4)` remains unimplemented. Current Git history is
based on original Agent07 portable base `d6104bb`, not on checkpoint50.
This is not a full/signed/flashable firmware or evidence of on-device behavior.

## Checkpoint 25 — script operand interpreter

The next complete Hexagon V66 function at file offsets `[0x105744,0x105bac)`
(runtime `[0xb00c5744,0xb00c5bac)`) is independently assembled from
named instructions and control-flow labels as **1,128 executable bytes**.
It interprets variable-size tagged records, uses hardware `loop0` for byte
accumulation, a jump-through-table for operator classes, and status/error
exit paths. The labels are inferred from instruction dataflow, not source
symbols or device-execution evidence.

The first candidate had branch displacements dependent on packet PC plus
LLVM shuffling of two duplex packets. The final assembly retains the correct
packet-relative displacement on external calls and explicitly orders named
operations within the packets at relative offsets `0x3c` and `0x17c`.
**Both LLVM 22/V66 and independent LLVM 6/V65 assemble 1,128 exact bytes**,
SHA-256 `2ec923140346b32f11b007778ca1e13f7399eefac839fb434ef058f6a4e5dc95`.
There are **149 valid Hexagon packets, 282 encoded words, zero unknowns**, and
no OEM opcode arrays, raw instruction directives, original firmware data,
layout fill, or signing material included in the output.

Cumulative code `[0x1001c4,0x105bac)` now totals **23,016 contiguous
executable bytes**. Existing independently reconstructed jump tables total
**496 noncontiguous data bytes**, giving **23,512 source-derived bytes**.
Next complete function begins at file `0x105bac`, runtime `0xb00c5bac`.
The 452-byte Agent13 leading seam `[0x100000,0x1001c4)` remains uncovered;
other independently reconstructed functions later in the owned region are
still outstanding. Portable base `d6104bb`; checkpoint-50 integration pending.

## Checkpoint 26 — record validation and session status dispatch

The next complete function at file `[0x105bac,0x105e70)` (runtime
`[0xb00c5bac,0xb00c5e70)`) uses counted records, descriptor slots,
status callbacks and bounded failure paths. Its **708 executable bytes**
are independently reassembled as 94 valid Hexagon packets (177 encoded words)
from named instructions, including the original conditional far jumps and
external diagnostic callback addresses. LLVM22/V66 and LLVM6/V65 match the
pinned reference byte-for-byte (SHA-256
`7f95ee99aa874871289cce4d5bead6d5c001d310baca8646d364072503fd9581`).
There is no original executable data embedded in the checked-in assembly.

The adjacent executable output `[0x1001c4,0x105e70)` now totals **23,724
bytes**. Separate source-derived dispatch tables total **496 bytes**,
giving **24,220 source-derived bytes** across the two regions. The next
function starts at file `0x105e70` (runtime `0xb00c5e70`); the 452-byte
leading Agent13 seam is still uncovered, and the broader WLAN firmware is
not independently reconstructed or newly signed. Portable branch ancestry
remains on `d6104bb`, pending validated checkpoint-50 integration.

## Checkpoint 27 — session record helpers and status guard

Four complete adjacent Hexagon V66 functions at file offsets `[0x105e70,0x106128)`
(runtime `[0xb00c5e70,0xb00c6128)`) contribute **696 source-assembled
executable bytes**, independently verified with LLVM 22/V66 and LLVM 6/V65:

- `[0x105e70,0x105f48)` — 216-byte record/session initialization and status publication.
- `[0x105f48,0x106044)` — 252-byte session record allocation, field setup and publication.
- `[0x106044,0x1060e0)` — 156-byte detach, record list update and callback dispatch.
- `[0x1060e0,0x106128)` — 72-byte guarded session-state predicate and zero-result return.

The second helper has a duplex instruction packet at function offset `0x68`;
source-level reordering of independent operations preserves its precise
Hexagon parse/class bits without raw reference instruction words. All four
fragments have **zero byte differences** under both assemblers and match
pinned `wlanmdsp.mbn`. No OEM executable bytes or signed content are checked in.

Cumulative executable coverage `[0x1001c4,0x106128)` is **24,420 bytes**.
The two noncontiguous source-authored dispatch tables remain **496 data
bytes**, totaling **24,916 source-derived bytes**. The next function starts
at `0x106128`. The 452-byte Agent13 boundary seam and the remaining owned
WLAN executable region remain unimplemented. On-device behavior is unverified.

## Checkpoint 28 — codec mode resource helpers

Five contiguous, complete V66 routines covering file `[0x106128,0x1063f4)`
(runtime `[0xb00c6128,0xb00c63f4)`) add **716 reconstructed executable bytes**:

- `[0x106128,0x1061ec)` — codec-mode dispatch to device handlers (196 bytes).
- `[0x1061ec,0x1062a8)` — controller resource allocation and state report (188 bytes).
- `[0x1062a8,0x106318)` — mode diagnostics and notify callbacks (112 bytes).
- `[0x106318,0x106360)` — counter update and completion (72 bytes).
- `[0x106360,0x1063f4)` — resource access/creation and attached status (148 bytes).

The resource-access function requires `:mem_noshuf` for a mixed load/store
packet and source reordering of one equivalent duplex packet. LLVM 22/V66
and LLVM 6/V65 both emit byte-identical reference output. No original
instruction words or padding are embedded in source. A minor LLVM 6
signed/unsigned literal warning does not affect the assembled bytes.

Cumulative continuous executable `[0x1001c4,0x1063f4)` = **25,136 bytes**.
Separate authored dispatch-table data remains **496 bytes**, totaling
**25,632 source-derived bytes**. The next function begins at file `0x1063f4`
(runtime `0xb00c63f4`). Agent 13's leading 452-byte seam and the rest of
this large WLAN executable allocation remain unfinished. On-device behavior
and new firmware authorization remain unverified.

## Checkpoint 29 — interface record and session helpers

Seven consecutive complete Hexagon V66 functions now rebuild exactly at
WLAN file `[0x1063f4,0x106694)`, VMA `[0xb00c63f4,0xb00c6694)`.
They add **672 executable bytes**:

- `[0x1063f4,0x10656c)`, 376 bytes: validated interface-record application, error paths.
- `[0x10656c,0x106578)`, 12 bytes: status logging thunk.
- `[0x106578,0x106598)`, 32 bytes: session list search.
- `[0x106598,0x1065a8)`, 16 bytes: descriptor event dispatch.
- `[0x1065a8,0x106638)`, 144 bytes: session attach, status publication.
- `[0x106638,0x106644)`, 12 bytes: session continuation thunk.
- `[0x106644,0x106694)`, 80 bytes: guarded session-record lookup.

The session-attach packet combining a store with a register-pair load uses
explicit `:mem_noshuf` so LLVM preserves the original packet word ordering.
All seven sources assemble byte-identically under LLVM22/V66 and independently
under LLVM6/V65, agree with the pinned reference image and have no raw
OEM instruction words or unexplained padding. The `agent14-late-regions.json`
manifest records every new source and exact SHA-256.

Cumulative contiguous executable `[0x1001c4,0x106694)` is **25,808 bytes**;
the two disjoint source-authored dispatch tables remain 496 data bytes,
total **26,304 source-derived bytes**. The next complete function starts
at file `0x106694`. The 452-byte seam with Agent13 remains unowned by this
branch, and substantially more of Agent14's large WLAN allocation remains
unreconstructed. On-device behavior and new OEM authorization remain unverified.

## Checkpoint 30 — session-mode callbacks, queue updates and packet dispatch

Seven adjacent V66 functions covering file `[0x106694,0x106ab4)`
(runtime `[0xb00c6694,0xb00c6ab4)`) add **1,056 executable bytes**:

- `[0x106694,0x10671c)`: session mode and status-bit handling, 136 bytes.
- `[0x10671c,0x1067d4)`: session bit flag dispatch and notifications, 184 bytes.
- `[0x1067d4,0x1068bc)`: session event/status update, 232 bytes.
- `[0x1068bc,0x10691c)`: event queue initialization, 96 bytes.
- `[0x10691c,0x1069c4)`: callback/buffer init path, 168 bytes.
- `[0x1069c4,0x1069d0)`: missing-session diagnostic thunk, 12 bytes.
- `[0x1069d0,0x106ab4)`: guarded session packet dispatch, 228 bytes.

Two packets initially assembled with a different, still-legal duplex pairing.
Reordering three independent named operands in each packet preserved the
original parse/duplex fields; no reference opcode words were copied into the
source. All seven full functions independently assemble under LLVM22/V66
and LLVM6/V65 and match the pinned MBN with zero byte differences.

Cumulative contiguous executable `[0x1001c4,0x106ab4)` contains
**26,864 bytes**; the existing authored dispatch tables contribute another
496 separate data bytes, giving **27,360 source-derived bytes**. The next
function begins at file `0x106ab4`. The initial 452-byte Agent13 seam and
remaining WLAN code are still unimplemented. No on-device validation or
new OEM authentication has been performed.
