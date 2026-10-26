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

## Checkpoint 02 — Controller setup, polling, copying and reporting

Added 224 instruction bytes in file `[0x6bc,0x79c)`, completing six further
helpers including the two-byte return stub. Cumulative Agent 17 coverage is
**306 consecutive instruction bytes** in file `[0x66a,0x79c)`, runtime
`[0x46a,0x59c)`. Both profiles have 117 instructions: 36 four-byte and
81 two-byte. There are no literal-pool, metadata, fill, or externally copied
bytes in this generated span.

The source preserves the explicit IFCALL9/IFRET16 operations used to share
polling setup, implements a counted byte-copy loop, and distinguishes
reserved compact branch encodings from actual indirect calls. New guard
tests cover the compact register holes, invalid halfword offsets, backward
IFCALL9 targets, and the JRAL5/BNES38 alias defect found in the inherited
implementation.

Both pinned references and independent GNU assembly compare with **zero
byte mismatches over all 306 bytes**. The verifier checks 19 EX9 uses and
13 distinct table entries (52 external Agent 18 bytes, zero added coverage),
including the special table-context JAL target interpretation. The new
helpers call lower-ILM routines outside the current emitted interval; their
targets are recorded explicitly in the verification JSON.

| Profile | SHA-256 of all 306 generated bytes |
| --- | --- |
| postmarketOS | `066d3373d35e3188cda9e2aa4b4d117c54c1a91953343d4f8f694e3f2deaa6e1` |
| map220v | `b925ba7160eeba829cba6a25c11395453cddbd19d372f05effe5f3d67e8d720f` |

Required gates: **206 Rust tests and 13 Python tests passed, zero skipped**;
offline locked release build and `git diff --check` passed. All 117
instructions independently assemble and decode under unmodified GNU 2.25.1,
including EX9 and IFC forms. Evidence is included under
`verification/agent20-17/checkpoint02-*`.

First uncovered file offset: **`0x79c`**, runtime **`0x59c`**.
Remaining exclusive postmarketOS assignment: **63,588 bytes**. Including
the inherited prefix, adjacent emitted instruction spans total 844 bytes;
the inherited semantic corrections remain outstanding.

## Checkpoint 03 — Descriptor scan and status routine

Added the complete 446-byte routine in file `[0x79c,0x95a)`, runtime
`[0x59c,0x75a)`. It scans descriptor indices 3 through 14 with a 16-byte
stride, accumulates a status mask, and preserves the routine's polling,
reporting calls, and shared IFC tails. Profile-specific workspace addresses,
direct call targets, and EX9 indices are explicit source parameters.

Cumulative coverage is **752 consecutive instruction bytes** in file
`[0x66a,0x95a)`, runtime `[0x46a,0x75a)`: 292 instructions, comprising
84 four-byte and 208 two-byte instructions. Metadata/data, layout fill,
and externally copied bytes in the generated span are each **zero**.

Both pinned references and independent GNU assembly compare with **zero
byte mismatches over all 752 bytes**. The verifier checks 40 EX9 uses and
20 distinct table entries (80 external dependency bytes, zero coverage).
Its control-flow report distinguishes the 12 EX9 call uses from ordinary
PC-relative calls and records 14 edges outside the current emitted span.

| Profile | SHA-256 of all 752 generated bytes |
| --- | --- |
| postmarketOS | `6c681dc74cd79a2c6e4306ae16ded929345dc6c31ec0d8a394aab237eca96294` |
| map220v | `27839e61bdfe398b348b720fad13eed8380a85a0a822d1f1ee2eb32dce4b6e50` |

Required gates: **206 Rust tests and 29 Python tests passed, zero skipped**;
offline locked release build and `git diff --check` passed. The 16 new
verifier regression tests exercise profile identity, descriptor bounds,
contiguity, instruction/data separation, EX9 effects, and IFC/direct branch
target interpretation. All 292 instructions independently assemble and
decode under unmodified GNU 2.25.1. Evidence is included under
`verification/agent20-17/checkpoint03-*`.

First uncovered file offset: **`0x95a`**, runtime **`0x75a`**.
Remaining exclusive postmarketOS assignment: **63,142 bytes**. Including
the inherited prefix, adjacent emitted instruction spans total 1,290 bytes;
the inherited semantic corrections remain outstanding.

## Checkpoint 04 — Complete startup routine with distinct profile layouts

Added the startup routine beginning at file `0x95a`: 448 bytes for
postmarketOS and 450 bytes for map220v. The routine inspects peripheral
status, clears state, installs a descriptor pointer, calls the preceding
diagnostic routine, copies descriptor bytes, swaps byte pairs through an
explicit IFC tail, and dispatches the returned status. Numeric peripheral
addresses retain their exact values without speculative hardware names.

At runtime `0x828`, postmarketOS uses the two-byte `EX9 #444`, whose table
effect is a GP-relative word store. map220v uses a four-byte `SWI.GP` at
that position. Every subsequent map220v instruction location and local
branch target is explicitly shifted by two bytes. The emitter, comparison,
listing metadata, and verifier now use profile-specific endpoints.

| Quantity | PostmarketOS | map220v |
| --- | ---: | ---: |
| Cumulative file interval | `[0x66a,0xb1a)` | `[0x66a,0xb1c)` |
| Cumulative runtime interval | `[0x46a,0x91a)` | `[0x46a,0x91c)` |
| Generated executable bytes | **1,200** | **1,202** |
| Instructions | 455 | 455 |
| Four-byte / two-byte instructions | 145 / 310 | 146 / 309 |
| Metadata/data bytes | 0 | 0 |
| Layout/fill bytes | 0 | 0 |
| External bytes copied into generated span | 0 | 0 |
| Reference byte mismatches | **0** | **0** |
| GNU/Rust byte mismatches | **0** | **0** |
| EX9 uses / distinct table entries | 55 / 29 | 54 / 28 |
| External Agent 18 table dependency bytes | 116 | 112 |

All external table dependency bytes contribute **zero** Agent 17 source
coverage. Each profile has 16 verified EX9 control-transfer uses and
27 control-flow edges outside the emitted interval; targets are recorded
in the verification JSON. The report distinguishes these checked edges
from a complete IFC-state or hardware model.

| Profile | SHA-256 of complete generated span |
| --- | --- |
| postmarketOS | `d46efb9d0f33d90e96f4339352b4c408384548b5da06b45de6d37c2cbf30e2e2` |
| map220v | `b5b86148494d47c1ff40466740eeb8f7e59671652510f2d99e3962976ed8d53b` |

Required gates: **209 Rust tests and 35 Python tests passed, zero skipped**;
offline locked release build and `git diff --check` passed. Unmodified
GNU binutils 2.25.1 independently assembled and decoded all 455 instructions
for each profile. The verified release emitter SHA-256 is
`39712931515f6281adcc9acf7af761e6af162ad9e3f58e8076370bf2b21fcb1a`.
Detailed reports and logs are under `verification/agent20-17/checkpoint04-*`.

New regression cases cover the layout divergence, GP/immediate alignment
and range checks, compact register limits, and multi-register masks. Review
also found a verifier robustness issue: a successful command that wrote no
new output could leave a stale artifact eligible for verification. Every
producer now removes its expected output first and must create a fresh
file; regression tests cover that failure mode and protect input files.
The preceding helper's control-word comment now states the exact OR
operation without assuming the caller's high byte was zero.

First uncovered postmarketOS file offset: **`0xb1a`**, runtime **`0x91a`**.
The map220v comparison first uncovered address is file **`0xb1c`**, runtime
**`0x91c`**. Both are complete routine boundaries. **62,694 bytes** of the
exclusive postmarketOS assignment remain. The full assigned lower ILM has
not been reconstructed. No hardware-execution or flashability claim is made.

## Next work and integration handoff

Continue with the small setter at postmarketOS runtime `0x91a` / map220v
`0x91c`. The postmarketOS routine at `0x922` has no corresponding map220v
block, so later code needs a new explicit profile mapping; a uniform
two-byte shift must not be extrapolated past that divergence.

Agent 01 should integrate this portable contribution into its verified
checkpoint-50-based history and correct the inherited JRAL5 and compact
register defects documented in the accompanying audit. Agent 18 owns the
EX9 table source and its separately counted dependencies. The published
checkpoint-50-based merged history remains an outstanding integration
dependency; applying a portable patch alone does not establish that base.

## Checkpoint 05 — Complete setter before the diverging next routine

Added exactly 8 code bytes at postmarketOS file `[0xb1a,0xb22)` and map220v `[0xb1c,0xb24)` (runtime `[0x91a,0x922)` and `[0x91c,0x924)` respectively). The independent emitter uses `MOVI55 R0,1; SWI.GP R0,[controller_active]; RET5 LP`; the per-profile field addresses are `0x2c584` and `0x2c55c`. Both pinned references compare with zero byte differences.

The cumulative generated spans are now **1,208** and **1,210** bytes; 458 instructions per profile, 146+312 four/two-byte instructions for postmarketOS and 147+311 for map220v. 55/54 EX9 uses and 29/28 distinct upper-ILM dependencies remain unchanged. Metadata, fill and externally copied bytes remain zero.

**Verification qualification:** This environment does not have the GNU 2.25.1 `nds32le-elf-*` tools used in checkpoint 04. Rust source-to-reference comparison and regression tests were performed, but a fresh independent GNU assembler verification cannot be claimed for checkpoint 05. The linker/emitter dependency is documented rather than bypassed with raw OEM bytes.

PostmarketOS first missing file address: `0xb22`, runtime `0x922`; next routine is absent or relocated in map220v. Remaining assigned postmarketOS bytes: **62,686**. This is an isolated contribution to the old Agent 09 history, not checkpoint-50 integration.

## Checkpoint 06 — PostmarketOS-only controller-state check

- Follow-up NDS32 routine `[0xb22,0xb42)` contributes **32 independently encoded executable bytes (13 instructions)** to the required profile. Cumulative postmarketOS coverage `[0x66a,0xb42)` is **1,240 executable bytes / 471 instructions** (149 32-bit, 322 16-bit), no layout/data/OEM copied bytes and zero reference mismatches.
- Separately verified older map220v profile remains at `[0x66a,0xb24)` = 1,210 bytes / 458 instructions. This code path is absent here in the old firmware; no bogus register-shift offset alignment was assumed.
- Three new EX9 upper-ILM dependencies are independently checked from their **semantic effects**, but not counted as reconstructed Agent17 executable code: entry `0xab` = `lwi.gp r0,[0x2eca8]`; entries `0x2e`, `0x141` = `swi.gp r0,[0x2c73c]`, `swi.gp r0,[0x2c740]`. These dependency table bytes belong to Agent18.
- Added checked GP halfword, signed compact comparisons, bitfield extraction and compact register ADD45/SUB45 typed encoders with tests. NDS32 opcode assignments cross-checked against GNU binutils and OpenOCD sources; byte comparison against both profiles passes.
- `cargo test --workspace --offline --locked`, `cargo build --workspace --release --offline --locked`, Python discovery and Git cleanliness pass. **Independent GNU NDS32 assembler is not present in this environment; its reassembly check is pending**, not silently reported as passed.
- First missing postmarketOS file offset `0xb42`; runtime `0x942`.

## Checkpoint 07 — Touch controller event and status helpers

- Extended authoritative postmarketOS ILM reconstruction by **754 executable bytes / 287 typed NDS32 instructions**. Complete span `[file 0x66a,0xe34)` = **1,994 contiguous instruction bytes (758 instructions: 239 32-bit, 519 16-bit)** with zero reference byte mismatches. No OEM opcode byte arrays, data-table bytes, or linker fill emitted.
- New group `[runtime 0x942,0xc34)` contains several separately returning routines: touch-report state counter and interface synchronization, suspend/resume or wakeup flags, event/error callback dispatch, and controller global flag setters. Instruction-level semantic identifications remain provisional pending hardware traces and independent assembler.
- Upgraded protected verifier with checked EX9 expected effects for **63 distinct upper-ILM entries (252 external reference bytes, never counted as reconstructed)**. Total 132 EX9 use sites, 50 of which transfer control; 192 directly validated control-flow edges; 77 targets outside current source span; 26 indirect branch sites. EX9 table belongs to Agent 18.
- Fixed private decode-only script's incorrect `a600/ae00` compact byte load/store classification (requires 9-bit opcode mask). The committed typed source now reflects LBI333/SBI333 rather than erroneously identifying halfword instructions. Added four additional Python synthetic tests for checked assembler syntax, external effects and failure conditions (37 Python tests total).
- Both revisions verified byte-identical in their own profiles: postmarketOS 1,994 bytes; map220v remains independently pinned at 1,210 bytes. The older revision is *not* assumed to share the additional postmarketOS addresses.
- Tests: 212 Rust, 37 Python, offline release build and Git diff check pass. The external GNU 2.25.1 NDS32 assembler/decoder is **unavailable**, so the full verifier intentionally returns partial/exit 2 with `independent_assembly.status=skipped`. Do not claim independent GNU reassembly of these new instructions.
- **Next missing file offset `0xe34`, runtime `0xc34`** (next function entry). Remaining assigned range: `0x10000 - 0xe34 = 61,900` bytes. No source past this address is claimed at this checkpoint.

## Checkpoint 08 — Typed ILM event/report helpers to file 0x1c1a

- Required postmarketOS interval now covers **5,552 consecutive executable bytes** `[file 0x66a,0x1c1a)` (`[runtime 0x46a,0x1a1a)`) with zero reference byte differences. This checkpoint extends the prior `0xe34` endpoint by **3,558 executable bytes** and adds **1,316 typed NDS32 instructions**. The emitter does not load, copy, or include OEM code bytes; original references remain outside Git.
- PostmarketOS cumulative count: **2,074 instructions**, **702 32-bit** and **1,372 16-bit**. No source metadata/data, filler bytes or copied OEM bytes are counted. The older map220v profile is still separately verified on its original `[file 0x66a,0xb24)` interval, **1,210 bytes**, because its later routines have different layout and must not be extrapolated.
- The additional code includes event/status dispatch, compact ring and reporting helpers, sensor calibration and coordinate arithmetic, and EX9 call sites. The reconstruction ends immediately before a currently unsupported NDS32 ALU2/DSP instruction at **file `0x1c1a`**, runtime `0x1a1a`. The last routine is **incomplete**, not a generated return or skipped-data hole. Remaining assigned postmarketOS lower ILM span is `[0x1c1a,0x10000)` = 58,342 bytes.
- New typed ISA constructors include N16 `ADDI10.SP`, `ADDRI36.SP`, `LWI37.SP`, sign-extension, compact arithmetic/MUL; N32 signed branches, SLTI, SRAI, DIVSR and indexed signed halfword loads, plus checked numeric CSR indexes. Invalid widths, registers and reserved encodings are rejected. Source operand fields follow public binutils NDS32 ISA definitions.
- The explicitly checked external EX9 dependencies now number **401 uses / 131 distinct entries (524 bytes)**, separately resident in Agent 18's upper-ILM table. Their original bytes are never emitted as Agent 17 source, and semantic field values are verified against the pinned profile; **626** explicit control-flow edges are recorded (no dynamic IFC-state simulation).
- Full Rust build/tests, Python tests, byte comparison and release build are required for this checkpoint. The independent historical GNU 2.25.1 NDS32 assembler/objdump is **not installed in this execution environment**; the verifier reports `partial` / exit code 2 for `--skip-assembler`. Do not claim a fresh independent GNU assembly success for newly reconstructed instructions. Separate Rust encoders and public opcode-field checks provide additional coverage but are not a substitute for that gate.
- The contribution remains an independent branch based on old Agent 09 `51b1f91f32d32542aa03e114f5a2f4d94b559e2a`, pending integration into a truly checkpoint-50-anchored repository.

## Checkpoint 09 — Sensor DSP arithmetic and event handling to file 0x23d0

- Added **1,974 independently emitted NDS32 executable bytes**, file `[0x1c1a,0x23d0)`, runtime `[0x1a1a,0x21d0)`, with no proprietary instruction-word arrays or implicit padding.
- **Cumulative required postmarketOS source** `[file 0x66a,0x23d0)` = **7,526 contiguous executable bytes**, 2,776 instructions (**987 32-bit; 1,789 16-bit**). Generated-span SHA-256 `0996d6fc8f97f03334c28b2f59a1f940ccf7ad418064d422eee7ad01599251d0`. The original pinned reference compares with zero byte mismatches.
- Implemented typed encoder variants for signed immediate byte/halfword loads, N16 SLLI333/ADDI333, ALU1 register-OR right shift and zero extension, N32 SUBRI/SLLI, and N32 ALU2 MUL/MULR64/MULSR64/MAX/MADDR32. These architectures have public operand-level semantics; per-opcode width and operand constraints are tested. Semantics are not verified on real hardware.
- The additional source includes status processing, touch coordinate arithmetic, frame dispatch and sensor/report handling; the runtime return/call edges are recorded in a separate static verifier. No full IFC-state execution was simulated.
- The source verifier now checks **477 EX9 uses / 151 distinct entries** in the **Agent 18** upper-ILM dependency table (**604 externally owned bytes, zero added Agent17 coverage**). Analyzed **750 direct/control-flow edges** and 66 register-dependent transfer sites. Never copied EX9 table bytes into the source generator.
- Separate map220v reconstruction is unchanged and verified at **1,210 byte-identical bytes** on `[file 0x66a,0xb24)`; incompatible later profile code was not extrapolated.
- **All checks passed:** `cargo test --workspace --offline --locked`, `cargo build --workspace --release --offline --locked`, Python test discovery, full generated-span postmarketOS/map220v comparison, `git diff --check`, Git history/bundle verification. The historical GNU NDS32 2.25.1 assembler is **not available** in this execution environment, so `tools/verify-novatek-ilm-low.py --skip-assembler` retains **partial / exit code 2**, not an independent assembler pass.
- The first unsupported instruction is **file `0x23d0`**, runtime `0x21d0`, NDS32 compact prefix `0xb200` requiring independent ISA-family confirmation before extending further. **Remaining assigned ILM: 56,368 bytes** (`0x10000-0x23d0`). The ending routine at this checkpoint is unfinished.
- This branch is still rooted in the prior Agent 09 base `51b1f91f32d32542aa03e114f5a2f4d94b559e2a`, not Agent 01's integration branch. The cumulative patch must be verified against the actual merged HEAD before claiming checkpoint-50 ancestry.

## Checkpoint 10 — Restore NDS32 LWI45.FE, BR1 and JR5; extend to file 0x2724

- Required postmarketOS lower-ILM reconstruction now spans `[0x66a,0x2724)`, **8,378 consecutive independently emitted executable bytes**, 3,109 NDS32 instructions (1,080 four-byte, 2,029 two-byte). New `[0x23d0,0x2724)` adds **852 bytes / 333 instructions**. Firmware reference byte mismatches: **zero**; metadata, linker fill and externally supplied source bytes in this span: **zero**.
- Correctly encoded `LWI45.FE` (implicit r8 base, signed negative word offset `[-128,-4]`), `BR1` register comparison branches and nonlinking `JR5`. Previous decoder incorrectly categorized `JR5` as a PC-relative `J8`; typed opcode cases and distinct negative/invalid tests now reject that ambiguity. ISA encodings corroborated by published GNU binutils NDS32 instruction definitions.
- 512 cumulative EX9 references use **164 distinct semantic entries** / 656 source-external dependency bytes, all owned by Agent 18. Added explicit checks for 13 newly referenced entries including `CMOVZ`, `SLTI`, `indexed` memory and GP byte/word loads. They are checked against the original table, never copied or counted as Agent 17 code.
- The full explicit reference verification returns `status=partial` / exit **2** only because independent GNU `nds32le-elf-as` is unavailable in this container. No Ghidra/GNU assembly success is claimed. The Rust serializer/reference equality, control-flow and EX9 semantic checks succeed; the map220v separate profile remains `[0x66a,0xb24)` (1,210 bytes), also verified.
- **Next unimplemented file offset `0x2724` (runtime `0x2524`)**, N32 opcode `LHI.BI` requiring typed postindexed load support. Remaining assigned `[0x2724,0x10000)` = 55,516 bytes. This branch is still based on Agent 09, not the integrated checkpoint 50.

## Checkpoint 11 — Correct the jump-table boundary after JR5, file 0x2724–0x275c

- **Crucial classification correction:** the ten bytes following `JR5 $r15` at file
  `[0x2724,0x272e)` are **a ten-way byte-offset jump table**, not five
  instructions. The caller sets `$r15 = runtime 0x2524`, loads a byte selected
  by its bounded index, adds the byte to `$r15` and jumps through that register.
  The ten author-named case destinations are `[0x252e,0x2536,0x253a,0x253e,
  0x2542,0x2554,0x2554,0x2546,0x254a,0x2550]`. The emitter validates
  each even one-byte displacement and derives the data bytes from these targets;
  **no OEM byte array was copied**. Linear decoding at `0x2724` falsely
  identified `LHI.BI`, `SDI.BI` and a bogus data prefetch.
- Following the table, code `[file 0x272e,0x275c)` contributes **46 new
  source-encoded executable bytes / 19 NDS32 instructions**, completing the
  ten-way field update and returning with an architectural `POP25`.
- Required postmarketOS cumulative span `[0x66a,0x275c)` = **8,434 byte-identical
  source-generated bytes**: **8,424 executable instruction bytes** (3,128
  NDS32 instructions: 1,084 N32 and 2,044 N16) **plus ten separately
  classified jump-table data bytes**. No implicit fill or externally copied
  OEM code/data. Source-emitter SHA-256:
  `b1c1f93029d7a73cdaadd3b61909ff764527d44a2f27cf4ec648dcb9ff082c9d`.
- The table targets all align to existing source-authored instruction boundaries,
  checked by the verifier. New EX9 index 421 is checked as GP byte-store
  `SBI.GP R0,[0x2c72e]`, with **513 cumulative EX9 uses / 165 distinct entries**
  (660 external bytes all owned by Agent 18; zero counted as Agent 17 code).
- Added architecture-checked byte/half/word post-indexed memory, doubleword
  memory and `XORI` variants and compact `LWI333.BI`/`SWI333.BI` encoders,
  with negative tests; these are reusable ISA forms, **not** claims that the
  switch-table bytes are executable examples of those instructions.
- `cargo test --workspace --offline --locked`, offline release build,
  40 Python tests, firmware-profile byte comparisons, and source-derived
  data-table verification all pass. Independent historical GNU NDS32 assembler
  remains unavailable; the verifier correctly exits **2 / partial** with
  `independent_assembly.status=skipped`.
- Map220v is independently pinned at `[file 0x66a,0xb24)` (1,210 bytes), not
  extended with postmarketOS-specific bytes.
- **First uncovered file byte: `0x275c`**, runtime `0x255c`. Remaining assigned
  lower ILM = `0x10000-0x275c = 55,460` bytes. The reconstructed control-flow
  and table boundaries are sound, but the full lower ILM and hardware
  verification remain unfinished.

## Checkpoint 12 — Lower ILM controller state and memory helpers through file 0x3000

- Source-generated postmarketOS span `[file 0x66a,0x3000)` = **10,646 contiguous bytes**, comprising **10,636 executable NDS32 bytes (3,990 typed 16/32-bit instructions)** and **10 separately generated branch-dispatch bytes** at file `[0x2724,0x272e)`. No OEM source bytes or implicit filler are included in the Rust emitter.
- Added **2,212 executable bytes** in `[0x275c,0x3000)` from 862 typed operations (**244 32-bit, 618 16-bit**). This interval includes unrolled touch-controller register-copy routines, controller flags, byte-lane state checks, and memory/reporting loops. Functional descriptions remain provisional without hardware traces.
- Firmware reference SHA-256 remains pinned to **postmarketOS** `8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5`; end-to-end Rust emission matches its exact bytes. The older map220v profile remains separately verified on file `[0x66a,0xb24)` = 1,210 bytes; incompatible extra routines were not extrapolated to that revision.
- Cumulative 4,000 source entries (3,990 executable instructions plus one 10-byte data table, so the Rust `program` has **3,991** entries), 1,328 N32 and 2,662 N16 instructions. The emitted interval references **666 EX9 use sites / 188 unique externally owned Agent 18 entries** (including 23 newly referenced and not yet independently semantically classified). These external table bytes are not included as authored Agent 17 code. Existing 165 EX9 entries retain the previous strict semantic checks.
- The full original strict Python EX9 verifier is **not ready** for the new 23 entries and retains partial status. Rust source-vs-pinned-reference bytes, unit tests, release build and Git integrity are the completed gates. Independent GNU NDS32 assembler is still unavailable and therefore not claimed. The decoder that assisted the manual architecture classification is *not* included in the source artifact; no opaque OEM instruction arrays or data snapshots are committed.
- **Next missing file byte `0x3000`**, runtime `0x2e00`; remaining reserved `[0x3000,0x10000)` = 53,248 bytes. The current endpoint is only a chunk boundary, **not** a guaranteed function boundary.
