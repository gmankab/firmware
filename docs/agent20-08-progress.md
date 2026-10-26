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

**Checkpoint 04:** The second late RX segment's **168-byte startup routine** `[file 0x364000,0x3640a8)` was rebuilt entirely in LLVM6 Hexagon assembly from typed instructions and symbolic register/PC-relative operands, SHA-256 `1c8e4b6d1b7c128032440a941956f3d45ae2968b9b17b36b2acccf5f67d5db03`. It initializes frame limit, GP, controller metadata and invokes the zero-fill helper. Its `call init_platform` target is expressed symbolically as `adsp_late_rx_startup+0xa8`, pointing to the independently rebuilt helper at `0x3640a8`. The two packet-boundary bit differences found during assembly exploration were resolved by meaningful packet grouping; final executable bytes all match the pinned reference exactly.

The startup + zero-fill helper are **248 consecutive byte-identical executable bytes** `[file 0x364000,0x3640f8)`, composite SHA-256 `8ae46f2b16461df880aff54fcd6bfba13587266045ae619997286b8393524d72`. Next file offset `0x3640f8` holds two apparent arithmetic/address words and is followed by padded zeros; further segment byte classifications are needed before extending executable coverage. **Cumulative source-generated executable count = 452 bytes** (84 + 120 disjoint by 12 zero-layout bytes in segment 1, plus 168 + 80 contiguous in segment 2). Prior Agent 04's early ADSP contributions do not overlap these ranges.

**Checkpoint 05:** Three more complete, disjoint source-assembled functions within the middle executable `PT_LOAD`:

- 36-byte register-save/remote-handler handoff `[file 0x390d8,0x390fc)`, SHA-256 `f5289532abf9a3fcc90a50e1c77615cd07a13dae6e70049739d8a1af46c8387c`. An explicit Hexagon `immext` preserves the long PC-relative external jump packet.
- 20-byte register-ready reset `[file 0x39100,0x39114)`, SHA-256 `36e0a7e92392cef26762304bbc9755cb32aa8a9c1086ec5d84992030a041e992`. Source expresses its register-producer and `.new` consumer in the **same packet**, rather than hardcoding the original word.
- 128-byte callback-registry scan `[file 0x39120,0x391a0)`, SHA-256 `47c5dc248bef3234e0de96fd62ffbd04102e5a6f2ec346df1a6180b265e91775`. Source explicitly reconstructs register save/restores, a bounded 200-entry table, the Hexagon `loop0(...):endloop0` packet, and predicate-controlled external handler calls. LLVM6 and Rizin decode agree on the hardware loop.

The middle RX `[file 0x39000,0x391a0)` has **388 independently assembled executable bytes** in five fragments; it contains 28 bytes of interspersed alignment/layout (`0x39054–0x39060` is twelve zeros; `0x390fc–0x39100` is a branch-alignment NOP; `0x39114–0x39120` is three NOPs). These 28 alignment bytes are *not* counted as independently decompiled executable code. Together with the second RX segment's contiguous 248-byte startup, this brings **total source-generated executable coverage to 636 bytes**, in seven disjoint verified functions. The remainder of both enormous executable segments is unfinished.

**Checkpoint 06:** The next two registry lifecycle functions now match the reference exactly: callback deregistration `[file 0x391a0,0x391e0)` (**64 bytes**, SHA-256 `df9c133d787c1b33d1247520b1aa2dcb6d59296610f0ba1aab9b48bea9d7aa98`) and registration/initialization `[file 0x391e0,0x39230)` (**80 bytes**, SHA-256 `e57c396c397d4e9b9fa49302b8d12226bf8032d7ad73f253d7ed36e82246224e`). Both use real Hexagon source packets, conditionals and `.new` data dependence, not imported original machine words. The initializer's backward conditional call requires an explicit original `immext(#0xffffff00)` even though LLVM6 can emit a shorter relocation; this 4-byte packet was authored directly from the ISA representation and cross-checked against the original.

The first late RX executable text now has source-generated functions through original file offset `0x39230`, with three small explicitly **uncounted** structural gaps `[0x39054,0x39060)`, `[0x390fc,0x39100)`, `[0x39114,0x39120)`. Agent 20-08 total **780 executable bytes** in **nine separately verifiable fragments**, of which **248 contiguous bytes** are the second large RX segment's startup functions. Next candidate function begins at file `0x39230`.

**Checkpoint 07:** Two further, separately source-assembled original functions were rebuilt exactly:

- Registry allocator/retry (`file 0x39230–0x392c8`): **152 source-generated Hexagon executable bytes**, SHA-256 `0e8f500eb05bb0f2e7e47bdd399938a00e6146adbab5c3881543ea3ee2bfe14c`. Implements bounded callback scanning, multiplication/counting, platform allocation and a separately verified backward registration call. Original Hexagon packet layouts were preserved, including the signed branch extender and return frame.
- Registry health probe (`file 0x392c8–0x39318`): **80 source-generated executable bytes**, SHA-256 `3c2b3c17db9e33f7f22bf0b8b1d3e8728809d2734ecb5b4b6454a57dbe6a869f`. Checks registry state and queries the platform before exiting through an externally resolved branch.

**Total is now 1,012 independent executable bytes across 11 individually verified functions** in the two late ADSP RX segments, including the continuous 248-byte entry in the second RX segment. The new `tools/emit-adsp-late.py --segment all --output /tmp/adsp-late-fragments --reference /private/adsp.mbn` emits a JSON manifest with exact file/vaddr/paddr mappings, SHA-256 of each source-built fragment, and precisely zero OEM code/implicit padding inserted. It never assembles a fake full image or combines the 28 bytes of interstitial alignment from the first region with the source-generated executable count.

**Checkpoint 08:** Two more true Hexagon helpers in the middle RX load are independently reconstructed: a **16-byte indexed callback descriptor copy** `[file 0x39490,0x394a0)` and a **20-byte LL/SC atomic increment** `[file 0x394a0,0x394b4)`. The latter uses `memw_locked`, `memw_locked(...,p0)` and a retry branch, all reassembled by LLVM6 with the original packet parse markers. They form a **36-byte contiguous source-generated helper cluster**, separate from prior firmware intervals. No source-generated code claims are made for the intervening `[0x39318,0x39490)` span.

**Cumulative Agent 20-08 contribution: 1,048 byte-identical, independently assembled executable bytes in 13 fragments**, with a full pinned-reference verification and machine-readable inventory ledger. Most of the 0x1b97e8-byte middle RX and 0x94b790-byte large late RX remain *unreconstructed*; they cannot be labeled source-derived merely because a tool can copy/repack the original image.


**Checkpoint 09:** Rebuilt the 136-byte QuRT platform-buffer handoff function at
ADSP file `[0x394b8,0x39540)` as Hexagon assembly, SHA-256
`a342e82ea1f2a1b11b39c7358d9b44cf68b4002301dfb29c8a712764c3cafe1c`.
The routine performs three original `trap0` transitions, preserves/restores
callee registers, captures the active GP-relative status word and calls the
platform allocation routine using a symbolic target. Explicit
`immext(#0xf01f1000)` is necessary for the original full-width GP address;
assembling a mathematically equivalent shorter memory operand would drop an
instruction and change the packet encoding. Source packet ordering (`r0`,
`r19`, `memw`) was chosen to retain the original signed Hexagon V66 packet
parse bits. LLVM6 assembler and the independent Rizin disassembler reproduce
all 136 instruction bytes exactly. This function has no reconstructed opaque
raw opcode values; external platform or OEM authentication is not generated.
The preceding `[0x394b4,0x394b8)` consists of an isolated `jumpr LR` whose
function context has not been established, so it is not counted.

**Total independently assembled executable coverage is now 1,184 bytes in 14
function fragments**. The next instruction after this handoff is at `0x39540`.


**Checkpoint 10:** Two further independent Hexagon helper functions were
reconstructed at `[file 0x39540,0x39568)` (**40 bytes**) and `[0x39568,0x39588)`
(**32 bytes**). The first publishes a GP-relative controller state word using
`mpyi`, a conditional backward retry and a four-operation return packet; the
second performs a conditional platform query while preserving the caller's
input pointer and freeing a bounded stack frame. Both were authored as
semantic Hexagon instructions, including explicit full-width `immext` and
symbolic external call offsets, and byte-verified against the exact firmware
with LLVM6 and independent Rizin disassembly.

Together with the previous handoff function, these form a **208-byte
contiguous source-generated region `[0x394b8,0x39588)`**; the preceding
single `jumpr LR` at `[0x394b4,0x394b8)` is deliberately not counted because
its function context is unknown. **Cumulative independently authored ADSP
code: 1,256 bytes in 16 tested functions.** Almost all of the two very large
RX executable `PT_LOAD`s remains unimplemented; matched bytes here do not
establish a complete source-equivalent or authenticated ADSP image.


**Checkpoint 11:** Independently reassembled a 48-byte bounded byte-prefix
comparison at file `[0x39590,0x395c0)` (SHA-256
`7665e8ac2959a11688d253c68fa8bb5096213c161d25a6b907b21e866fc1d91b`)
and a 60-byte short/overlap-aware byte-copy helper at `[0x395c0,0x395fc)`
(SHA-256 `09e58d0402e5b93a33da381494996e0bff8d9e8bb46fe87b36fa31b9c3792bda`).
The latter uses the source-authored Hexagon hardware `loop0` / `endloop0`
instruction packet to move bounded bytes and makes a symbolic call to the
platform overlap fallback only when needed. Both functions reproduce original
Hexagon V66 packet bytes via LLVM6 shared-ISA assembly, with independent Rizin
decoding. The two original NOP alignment words `[0x39588,0x39590)` are
**excluded** from the independently authored executable count.

**Total: 1,364 independently source-assembled executable bytes in 18 verified
functions.** The next code begins at `0x39600` after a separate one-word
alignment NOP `[0x395fc,0x39600)`; no neighboring OEM code is copied.

**Checkpoint 12 — independent small-function cluster beyond the buffer manager:**
Five further complete Hexagon routines have been source-assembled from explicit
controller operations, using no `.word`, `.incbin`, copied OEM payload or
implicit alignment: a **96-byte** eight-nibble status-text formatter
`[0x3a86c,0x3a8cc)`; a **40-byte** codec-context activator
`[0x3ab00,0x3ab28)`; a **4-byte** constant-return helper
`[0x3ab28,0x3ab2c)`; a **64-byte** platform-limit checked counter updater
`[0x3ab2c,0x3ab6c)`; and a **72-byte** callback-before/after probe
`[0x3ab6c,0x3abb4)`.

The four `0x3ab00–0x3abb4` functions form **180 contiguous source-generated
executable bytes** (no intervening padding). The 96-byte status formatter is
disjoint. All LLVM6-assembled bytes match the original pinned ADSP image;
Rizin independently recognizes its nibble extraction, indirect callback and
memory-modification operations. Some helper names reflect observed
instruction-level behavior, *not* a proof of fully recovered high-level
source. This checkpoint adds **276 executable bytes**, bringing Agent 20-08 to
**1,640 independently assembled executable bytes in 23 fragments**. The main
buffer-management procedure `[0x39600,0x398e8)` and intervals preceding these
helpers remain **unreconstructed**, even though Hexagon disassemblers can
list their original instructions. Next high-priority candidates are the
independent helper(s) around file `0x3abb4` and the controller manager at
`0x39600`. No executable bytes from the second RX region's large data/zero
areas have been counted.


**Checkpoint 13 — controller context selection, reuse and release:**
Rebuilt the **128-byte** session-configuration routine at
`[file 0x3aa80,0x3ab00)` with packet-accurate three-register scheduling and a bounded existing-resource
scan. The register assignments must be ordered `(r2, r1, r16)` to preserve
Hexagon's two-operation encoded word rather than its alternative duplex.
Also rebuilt the **136-byte** session-release routine at
`[file 0x3a18c,0x3a214)` with optional notification, platform teardown
and a conditional call. The latter has a `:mem_noshuf` packet containing
both a saved register-pair and a memory read: without this explicit modifier,
LLVM6 chooses a one-word duplex encoding and exact byte identity fails.
The source-assembled outputs for **both routines** now agree with all original
reference instruction bytes. Source file names and modifiers are asserted by
focused regression tests.

It joins checkpoint 12's four adjacent helpers into a **308-byte continuous
independently assembled executable interval `[0x3aa80,0x3abb4)`**. Checkpoint
13 adds 264 executable bytes, for **1,904 total independent executable bytes
across 25 functions**. The neighboring `0x3abb4–0x3abc0` is three four-byte
NOP alignment words; it is **not** counted as decompiled function code. Most
other ADSP program bytes, notably the larger controller buffer manager at
`0x39600`, remain unimplemented; no OEM-authenticated complete firmware is
produced.

**Checkpoint 14 — context creation and initialization:** Rebuilt the complete
252-byte controller-context allocator `[file 0x3a090,0x3a18c)` in source-authored
Hexagon V66-compatible assembly. It initializes ownership/limit fields,
registers and publishes callbacks, updates controller pointers and enters
platform diagnostic paths on failure. The first extended-immediate packet
needed the semantically equivalent ordering
`{ immext(#0x10000); r2 = ##65536; p0 = cmp.eq(r17,#0); r16 = r0 }`
to reproduce the original two-word packet encoding. Symbolic `.set` targets
preserve original PC-relative calls; no OEM opcode bytes were imported.
The 252 generated bytes have SHA-256
`aed2317c061cfa68ab63d95d258f99325c83edb5fa66e97870d27538b6f9a601`.
They join checkpoint 13's adjacent 136-byte session release at
`[file 0x3a18c,0x3a214)` for **388 consecutive source-assembled executable
bytes `[0x3a090,0x3a214)`**. The subsequent 12-byte NOP alignment
`[0x3a214,0x3a220)` remains excluded. Cumulative verified source-assembled
ADSP executable coverage is **2,156 bytes in 26 disjoint function fragments**.
The vast majority of the large ADSP executable segments remains unknown.
All 160 Rust and 141 Python tests, the release build, pinned-reference
byte-compare and source-only assembler checks pass. Provisional old-base
branch, not checkpoint 50 integrated.

**Checkpoint 15 — contextual descriptor traversal and a controller-global accessor:**
Source-assembled an 8-byte control-table pointer helper at `[file 0x3a7e0,0x3a7e8)`
(SHA-256 `d0857940dd31103d52be479feb42a84ecbe355de9155b4e1c183677ab17fa53d`)
and a 124-byte codec-buffer context traversal routine at
`[file 0x3a7f0,0x3a86c)` (SHA-256
`ccf7d38f4190bd77aea761db977518dd7170de7f0fbcd0a56a30cffc64ca8684`).
The latter scans controller descriptors, creates missing buffer handles,
links callback state and logs mismatches. Exact packet reproduction required
placing an `immext` with both a register load and memory-store packet, grouping
an `r20` update with its `.new` dependent store, and grouping a platform
call with its subsequent argument computation. All semantic instructions
assemble independently with LLVM6, then match the private pinned `adsp.mbn`
byte-for-byte; original OEM opcode bytes are never included in generator
source. The 124-byte routine connects directly to the previously rebuilt
96-byte nibble-status formatter, making **220 contiguous source-authored code
bytes `[file 0x3a7f0,0x3a8cc)`**. The 8-byte alignment gap at
`[0x3a7e8,0x3a7f0)` consists of NOPs and is not counted.
Total Agent 20-08: **2,288 independently source-assembled executable bytes
across 28 disjoint fragments**; verified zero mismatches, 160 Rust tests,
143 Python tests and release build. Checkpoint-50-based integration is not yet
established; this remains the provisional Agent04-descended branch.

**Checkpoint 16 — controller-handoff branch continuation:** Source-reassembled
40 exact Hexagon bytes `[file 0x3a7b8,0x3a7e0)` as a fragment of a controller
notification path. It invokes runtime preparatory routines, forwards a
platform event and branches to an earlier shared cleanup epilogue; **this is
not a standalone complete function**. LLVM6 initially emitted an alternate
packet parse flag for the 3-operation call/`immext`/argument packet and for
the branch/deallocframe pair; restoring the original *semantic packet*
grouping reproduces every byte without copying OEM instruction words.
SHA-256 `f72a2df2163b895a5a7fd77f9b5ac009cb7a265e1f05ff420073e88ebe0d29f9`.
It abuts the earlier 8-byte pointer helper at `0x3a7e0`; the two NOPs
`[0x3a7e8,0x3a7f0)` remain excluded from independently authored executable
counts. **2,328 total source-assembled executable bytes in 29 fragments.**
All 160 Rust and 145 Python tests, offline release build, pinned-reference
verification and Git diff checks pass. Agent04-descended provisional branch.

**Checkpoint 17 — controller callback shutdown:** A 68-byte source-assembled
controller cleanup routine `[file 0x3afa0,0x3afe4)` invokes optional
before/after callbacks around platform object cleanup and has two intentional
conditional-return paths. The packet assembler initially selected a duplex
for a merged callback argument/call; splitting the preceding memory load into
its original packet restored the exact 17-word layout. More importantly,
the branch at file `0x3afb8` must skip to the platform-call operation at
`0x3afc4`, not the preceding optional memory read at `0x3afc0`. The resolved
semantics and all bytes independently match the pinned ADSP reference;
SHA-256 `bf3b1b5096280746287d320a30ba84f658edcc4fdea416d42046e2aceae1199a`.
Total independently source-assembled ADSP **2,396 executable bytes in
30 fragment entries**, excluding all interstitial NOPs and copied firmware.
160 Rust and 147 Python tests, release build, exact reference check pass;
this remains a provisional, not checkpoint-50-rooted, branch.

**Checkpoint 18 — later parameter helpers and halfword order:** Independently
source-assembled five small original Hexagon helpers using meaningful operations:
`guarded-multiply` `[0x3bbe4,0x3bbf8)` (20 bytes), `platform-calculate`
`[0x3bbf8,0x3bc08)` (16 bytes), `parameter-forward` `[0x3bc08,0x3bc1c)`
(20 bytes), `parameter-combine` `[0x3be70,0x3be80)` (16 bytes), and
`halfword-order` `[0x3be80,0x3be9c)` (28 bytes). The first three join into
**56 contiguous source-assembled bytes**; the last two form **44 contiguous
bytes**. The handoff to the parameter forwarding platform call needed the
source memory-read packet separated from the following call/pair-combine
packet for the correct Hexagon packet order. The bounded halfword order
function uses `cl0` and a predicated upper cap, no copied original words.
All five byte-match their original pinned reference intervals exactly, with
separate SHA-256s in the manifest. This adds **100 executable bytes**, so
Agent 20-08 now has **2,496 independently assembled source bytes across 35
fragment entries**. Zero guessed code, linker fill, or OEM payload counted.
160 Rust tests, 149 Python tests, offline release build and exact original
verification all pass. The intended checkpoint-50-based integrated Git tree
remains unavailable; this is the isolated Agent04-descended branch.

**Checkpoint 19 — bounded codec bitfield mask:** Added an independently
reconstructed 88-byte Hexagon V66-compatible instruction routine
`[file 0x3ba68,0x3bac0)` that selects a controller mask from a platform
lookup table after computing the trailing-zero count (`ct0`) of a 22-bit
extracted field. An exact `r4 &= and(...)`/`immext`/table-load source packet
recreates the original parse-marker bits, with a separate zero-input exit.
SHA-256 `ee842302127e5e22ce797bd744e6f65115531946096d4a7ae987a89d44572a34`.
No OEM opcode words are embedded. The new source matches all 88 reference
bytes, producing **2,584 total independent executable bytes across
36 verified fragments**. The next huge controller functions and the majority
of both ADSP executable `PT_LOAD`s remain unimplemented. Isolated branch
still needs integration with the actual checkpoint-50-based repository.
All 160 Rust tests, 151 Python tests, pinned MBN verification and release
build pass.

**Checkpoint 20 — typed descriptor comparison and locked event register:**
Reconstructed two adjacent Hexagon V66 routines at `[file 0x3bfb8,0x3c010)`:
a 20-byte descriptor low-20-bit tag comparison and a 68-byte guarded
64-bit event-bit registration. The latter calls platform preparation,
constructs a shifted 64-bit bit mask, updates a channel-indexed runtime
slot, and performs an atomic compare/store through
`memd_locked(r2)` / `memd_locked(r2,p0)` with a predicate retry loop.
Both operations are semantically authored (no `.word`/`.incbin`/copied
vendor instructions) and exactly reassembled by LLVM6 against the pinned
ADSP reference. Combined **88 source-assembled executable bytes** form a
continuous `[0x3bfb8,0x3c010)` span, with distinct manifest SHA-256s.
Cumulative Agent 20-08 is now **2,672 independently generated exact executable
bytes in 38 disjoint fragment entries**; the rest of the ADSP executable
program remains vastly larger and largely unrecovered. 160 Rust and 153
Python tests, release build and direct reference checks passed.

**Checkpoint 21 — codec format constraint and vector routing:** Reconstructed
`[file 0x3bdd4,0x3be70)` as **156 byte-identical source-assembled Hexagon
executable bytes**. The routine validates codec mode flags, assembles a
signed fixed-point conversion code, performs a two-word `vaslw` vector shift,
and routes between platform callbacks depending on format acceptance. The
three independent source operations in a two-word V66 packet require the
ordering `{ r2 = #7; r5 = r16; r4 = #3 }` for exact duplex encoding; LLVM6
otherwise chooses a different valid packet arrangement. Original source
SHA-256 `80a4e3800a5f851e51271d456ef637b44317fcc331901fba008759cfa541ed01`.
The 156 bytes join the previously reconstructed 16-byte parameter combiner
and 28-byte `cl0` halfword-order helper to form **200 contiguous
source-generated bytes `[file 0x3bdd4,0x3be9c)`**, with no copied vendor
instructions or padding in the output. **Cumulative independent executable
coverage is 2,828 bytes across 39 verified fragment entries.** The two large
ADSP executable segments remain mostly unreconstructed. All 160 Rust and
155 Python tests, release build, reference check and Git diff validation pass.
The branch remains provisional until integration onto the original
checkpoint-50-descended repository.

**Checkpoint 22 — packet type, six-slot stack marshalling and shutdown tail:**
Three adjacent source-assembled Hexagon V66-compatible routines at
`[file 0x3d048,0x3d0c8)` reproduce **128 executable bytes exactly**.
The 20-byte packet-type check at `[0x3d048,0x3d05c)` checks class 19 and
an associated platform descriptor. The 60-byte stack marshaller at
`[0x3d05c,0x3d098)` copies six argument slots into a 24-byte outgoing
frame before forwarding platform work. The 48-byte shutdown adapter at
`[0x3d098,0x3d0c8)` obtains a shared controller handle, invokes its
cleanup and tail-jumps into an existing epilogue. Hexagon packet bounds,
new-register dependencies and PC-relative calls are expressed as source
instruction groups and symbolic `.set` labels, not original firmware
opcode arrays. The original ELF32 ADSP reference has pinned SHA-256
`4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e`.
Each fragment's independently assembled output matches that reference,
with individual SHA-256s recorded in the emitted coverage manifest.
**Cumulative independent executable coverage: 2,956 bytes across 42
verified fragments** (2,828 preexisting + 128 new). The rest of ADSP
executable is still largely unknown; no structural padding or OEM
payload is counted as rebuilt source.

**Checkpoint 23 — five controller parameter accessors:** Reconstructed five
independent 28-byte Hexagon routines, each of which supplies a different
parameter selector (`28`, `52`, `56`, `48`, `112`) to the same external
platform function, saves/restores the original callee-saved frame, and
returns the queried field. Their original file intervals are contiguous:
`[0x3ee58,0x3ee74)`, `[0x3ee74,0x3ee90)`, `[0x3ee90,0x3eeac)`,
`[0x3eeac,0x3eec8)`, `[0x3eec8,0x3eee4)`. LLVM6 emits the exact original
five 28-byte functions from five independent semantic Hexagon source files.
The combined **140 byte-identical executable bytes** have SHA-256
`24a23bc0d2103ca689d9420c8402b5d6d0ce7a608302895a64cbc04a1faae3ab`.
The total independently source-generated contribution is **3,096 bytes in
47 verified fragments** (previously 2,956 bytes in 42 fragments). No NOPs,
padding, copied private OEM code, or external signed material are counted.
The preceding `[0x3d0c8,0x3d0d0)` contains two alignment NOPs; the nearby
buffer trimmer and the large adjacent routine remain unreconstructed. The
firmware is not independently rebuilt as a complete executable or signed
image. Full reference revision SHA-256 remains pinned to
`4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e`.

**Checkpoint 24 — three adjacent callback and controller-state helpers:**
Rebuilt the original 40-byte callback-vector table walker
`[0x3ec1c,0x3ec44)`, 32-byte state-flush routine
`[0x3ec44,0x3ec64)`, and 16-byte controller tail-jump adapter
`[0x3ec64,0x3ec74)`. These form **88 contiguous executable bytes**
(three source-authored packetized Hexagon functions, not padding). The
walker iterates a null-terminated callback vector and invokes callbacks
indirectly; the flush saves/restores `r16:17`, calls a platform state
query, and writes a controller byte. Exact source reassembly required
`{call; immext; r0=...}` and `:mem_noshuf` for the original packet
ordering. The 16-byte tail forwards a handle to a PC-relative platform
function. Every byte matches the pinned ADSP reference; no proprietary
opcode values are copied into source. **Cumulative verified executable
coverage: 3,184 bytes across 50 disjoint inventory entries**, up from
3,096 bytes and 47 entries. The two NOPs at `[0x3d0c8,0x3d0d0)` and
three NOPs at `[0x3ec74,0x3ec80)` remain excluded from authored code.

**Checkpoint 25 — controller descriptor callback and initialization path:**
Reconstructed two additional Hexagon controller routines exactly against
original ADSP firmware: a 40-byte descriptor callback/tag dispatcher at
`[file 0x3e0a4,0x3e0cc)` and a 128-byte runtime initialization/retry/error
path at `[0x3e0d0,0x3e150)`. They are distinct, separated by the one
alignment NOP at `[0x3e0cc,0x3e0d0)`, which is **not** counted as
source-reconstructed executable code. Their original relative call sites
are represented through named `.set` symbols, not OEM opcodes, and the
controller initializer preserves Hexagon producer/consumer packets,
including the two-instruction return preparation. Both independently
compiled outputs match the pinned reference byte-for-byte; SHA-256 digests
are asserted in regression tests. **Cumulative independent executable:
3,352 bytes across 52 verified fragments.** This is not a complete
source-derived or newly authenticated ADSP image. LLVM6 is used for
reassembly; an independent Rizin decode is not run in this environment
because the packaged Rizin build lacks compatible `libzip.so.4` and
`libmspack.so.0` runtime dependencies. Three external-tool tests remain
skipped rather than claimed as passed.

**Checkpoint 26 — codec controller descriptor refresh:** Rebuilt the
80-byte Hexagon routine at `[file 0x3e480,0x3e4d0)` from source-authored
ISA operations and explicit packet grouping. The routine checks a
controller-descriptor tag, conditionally refreshes cached state, and
calls platform state-management helpers before tail-jumping to the
previously verified shutdown function at original file `0x3afa0`.
The latter cross-fragment destination is represented as a relative `.set`
symbol `adsp_late_controller_descriptor_refresh-0x34e0`, not an OEM
instruction literal. The independently assembled 80 bytes match the
reference exactly, SHA-256
`9bd8cf35a95e3e0979b840e7b31a65c2ab38b8e0b79f284ffe3d54189081ac7d`.
**Cumulative verified source-generated executable coverage: 3,432 bytes
across 53 disjoint fragments**. The large nearby controller functions,
platform gaps, non-executable data and OEM authentication remain
unreconstructed and outside the coverage totals.


**Checkpoint 27 — checked controller-mode argument relay:** Added a source-authored
76-byte Hexagon V66 function at original ADSP file `[0x3efa8,0x3eff4)`
that rejects out-of-range controller mode arguments, forwards six stack
arguments and dispatches an external platform handler. Its 19 instruction
words are exact under independent LLVM6 reassembly; SHA-256
`dad239580745f2b380bdf94ba096884c5ff4ef5f95bb7c0c4b662fef1759b932`.
The four-byte NOPs at `[0x3ef94,0x3efa8)` are **not** counted as code.
This raises independently generated executable coverage to **3,508 bytes
in 54 distinct fragments**, with zero external OEM bytes in output.
The larger controller routines remain unreconstructed.


**Checkpoint 28 — Hexagon controller field routing:** Added ten checked
controller-field shims in file intervals spanning `[0x3ec80,0x3ed1c)`.
Five field base addresses are supplied as named `##` immediates; getter shims
pass the address in `r0` to the shared field lookup, while forwarding shims
form `r1:0` using the caller-provided value. **140 source-assembled executable
bytes** are independently verified against the pinned reference using LLVM6;
the four interstitial 4-byte linker NOPs are intentionally excluded.
Cumulative Agent 08 source-generated coverage is **3,648 executable bytes in
64 disjoint code fragments**. No OEM executable payload is included verbatim.


**Checkpoint 29 — controller descriptor setup/registry allocator:** Rebuilt
Hexagon V66 source for the **136-byte** routine at `[0x3edd0,0x3ee58)`. It
initializes a descriptor, sets up packet metadata, selects a bounded number
of allocations, and invokes a platform queue publisher until the allocated
count is exhausted. Original packet order was preserved for the group
`{ r2 = #0; r16 = r0; memd(r29+#0) = r19:18 }`; LLVM6 then emitted the
exact original 136 bytes. SHA-256:
`237791bfa2488cb43ceb0739be371e84906679119a3ec2beebac6b010d8c8db4`.
The routine immediately joins the five verified getter functions in the
existing `[0x3ee58,0x3eee4)` interval, forming **276 contiguous source-built
bytes**. Cumulative independent executable coverage: **3,784 bytes across
65 distinct fragments**. No NOPs, signed payload or borrowed instruction
words counted as reconstructed code.


**Checkpoint 30 — five controller-field initialization calls:** Independently
source-assembled the **172-byte** controller-field initializer at ADSP ELF file
`[0x3ed1c,0x3edc8)`. The function initializes five controller fields from
16-bit hardware records, calls the previously reconstructed descriptor setup
function and makes a final deallocation/tail-jump packet. Hexagon V65
shuffling was steered using source instruction order to recover the original
producer-consumer and duplex packet layout; no `.word` or OEM code bytes
were embedded. Exact SHA-256:
`a59c40d44e11ef97f978883d3c01f7dc8eec74bb70ab329cdf61c570f66384f8`.
The two NOPs at file `[0x3edc8,0x3edd0)` are excluded. Cumulative independent
ADSP executable coverage is **3,956 bytes in 66 verified fragments**.


**Checkpoint 31 — multi-branch controller release checker:** Reconstructed
**216 independently authored Hexagon executable bytes** at ADSP file
`[0x3e4d0,0x3e5a8)`. The routine validates a registered command tag,
selects three resource-release callbacks and performs a signed 64-bit
capacity check before restoring its caller frame. Exact source packet
ordering (two different duplex arrangements) was required: the tagged
constant initialization must be ordered after the `memd` capture, and the
64-bit sum belongs to the following packet. No OEM opcode directives are
embedded. Direct reference SHA-256:
`f72ca3a18c2ad53cefb74dda821ea1e2397bdd6bb4d99b4237ea336581b186a5`.
Cumulative independent executable coverage is now **4,172 bytes in 67
verified fragments**, zero byte mismatches; the following controller
writer starts at `0x3e5a8` and remains incomplete.


**Checkpoint 32 — release-safe controller status reset:** The 72-byte
Hexagon source routine at `[file 0x3e750,0x3e798)` handles a null controller,
publishes registry state, calls platform diagnostics and returns through the
saved-register frame. Source assembly uses symbolic call targets and named
packet groups, never extracted opcode words. Digest
`3568d1d71fbbb7ab4e72697c61a8d8aa0a629e1f7951ee27f5eac59117ff82b9`.
The two alignment NOPs at `[0x3e798,0x3e7a0)` are excluded from code totals.
Total independently recompiled code: **4,244 bytes in 68 verified functions
or fragments**, with all other large firmware areas still unfinished.


**Checkpoint 33 — contiguous controller allocation and command validation:**
Hexagon source reconstructs the **176-byte allocator** `[0x3e5a8,0x3e658)`
(SHA-256 `f320970f977b0506e07c083d6a216dcf0bd8148bd52515d07042d5b07ca07071`)
and the **248-byte 64-bit input-size validator** `[0x3e658,0x3e750)`
(SHA-256 `015c37d846d9243af290ca2957ec9fb3c22e217c252d0ffb8b7b4c2b45958f36`).
Both preserve packed predicate/new-value branch behavior and the original
control-flow destinations; no OEM opcode literals are copied. The previously
reconstructed release checker (`0x3e4d0–0x3e5a8`) and controller reset
(`0x3e750–0x3e798`) connect without gaps, giving **712 independently authored
contiguous executable bytes**. Cumulative independently authored coverage is
**4,668 executable bytes in 70 fragments**. The immediately following two
4-byte NOPs (`[0x3e798,0x3e7a0)`) are still excluded.


**Checkpoint 34 — bounded indexed controller state:** Reconstructed the
**108-byte** Hexagon V66 source routine `[file 0x3e9e0,0x3ea4c)` controlling
allocation of a small array of registered controller indices. The routine
clamps the requested count, bounds allocation, initializes the indexed
controller table and invokes the failure logger when allocation fails.
Original Hexagon packet and GP-indexed register forms independently assemble
with SHA-256 `ac1162b9f4d84447ea7d2904f93dafacba7d04d4b12403719517d4948b90dcbd`.
The four-byte NOP `[0x3ea4c,0x3ea50)` is excluded from executable-source
bytes. Independently generated cumulative source coverage: **4,776 bytes
across 71 fragments**, byte-for-byte against pinned `adsp.mbn`.


**Checkpoint 35 — platform controller initialization:** The 208-byte
Hexagon executable routine `[file 0x3e910,0x3e9e0)` independently assembles
to SHA-256 `9d46cd1a328569f07d25656c985c194ee3c08bc1be68e2458ebabdbf05de4097`.
It initializes the platform controller globals, registers callbacks, probes
controller status, and performs a tested tail-call handoff. Original packet
boundaries matter: the controller registration call is a separate packet,
while the following register update and MMIO commit share a later packet.
The next 108-byte indexed-state initializer from checkpoint 34 joins
immediately at `0x3e9e0`, forming **316 contiguous executable bytes**.
Cumulative independently source-assembled coverage: **4,984 bytes in 72
fragments**, zero mismatches. OEM-signature and unimplemented executable
ranges are explicitly excluded.


**Checkpoint 36 — bounded opcode switch and platform report trampoline:**
Created Hexagon V66 original-source implementations for the **12-byte**
default-mode dispatch at `0x3e7a0–0x3e7ac` (SHA-256
`3b8d467a5c86b695872c84a8f78d002718be8958adfd1bf3eb71fd7bd7374d3a`),
the **328-byte** controller opcode switch at `0x3e7ac–0x3e8f4`
(SHA-256 `afbd86953d17f7700c67966de024551851a1851f2b44eae67a1644da839ceae7`),
and the **28-byte** diagnostic callback adapter at `0x3e8f4–0x3e910`
(SHA-256 `55c74b7996d6309fea659f55c0143b45a56fa47bf138bf685d0f3b8727614875`).
The 328-byte function dispatches hardware opcode values through a bounded
87-entry table in the platform firmware; the table itself is not copied or
counted as source generated. All source packet boundaries and tail calls
match exactly, including the original `:mem_noshuf` restrictions as applicable.
Together with checkpoint 35, the range `[0x3e7a0,0x3ea4c)` contains
**684 contiguous independently assembled executable bytes**. Two preceding
4-byte alignment NOPs `[0x3e798,0x3e7a0)` are explicitly **excluded**.
Cumulative independent ADSP executable coverage: **5,352 bytes across
75 Hexagon fragments**, zero mismatches, not an entire ADSP or signed image.
