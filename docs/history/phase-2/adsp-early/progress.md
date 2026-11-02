# Agent 20-07 — ADSP first-segment tail and early RWE entry

## Actual Git base and reference (IMPORTANT)

- **Requested integrated base:** a checkpoint-50 descendant as published by the integration agent. This is **not available** in the uploads. The checkpoint-50 SHA `b7ed4b587633b9b475a40f5a3ace5cc90db620ff` cannot be verified here.
- **Actual branch base:** Agent 04's `agent-04-checkpoint-32-adsp.bundle`, HEAD `d7cb8b118c275a0458d29bb65dafb82b80613825`, itself based on older `69ebf0d…` history. Branch `agents20/07-adsp-entry-provisional`. This history must be rebased/cherry-picked after the true integrated base is available; **do not claim checkpoint-50 ancestry**.
- **Pinned original ADSP** file `adsp.mbn` from the correctly reconstructed multipart firmware archive, SHA-256 `4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e`, kept privately outside Git.
- Architecture: Hexagon V66 (`e_machine=164`, `e_flags=0x66`), little-endian, mixed packet parse bits, ELF entry `0x8be00000`; first executable `PT_LOAD` file `[0x3000,0x521c)` maps physical `[0x8be00000,0x8be0221c)`.

## Checkpoint 01: complete RX segment tail `[0x51dc,0x521c)`

Reconstructed **64 independently encoded executable bytes**, Hexagon v66 TLB snapshot routine. This completes the earlier Agent 04 controller region `[0x41e0,0x51dc)` up to the first executable `PT_LOAD` end. It reads a privileged S54 register, copies 128 TLB pairs into controller memory, then cleans a set of cache lines with a Hexagon hardware loop, and returns.

- **Source:** `sources/xiaomi-nabu/adsp/adsp-controller-tlb-snapshot.S` with original packet grouping and symbolic hardware loop labels. LLVM6 assembles 14 of 16 instruction words from mnemonic source (including extenders and loop parse markers). Two unsupported privileged operations use *checked* typed encoders after verifying the exact NOP sentinel location: `R23 = S54` and `R3:2 = tlbr(R0)`.
- A new general checked `tlb_read_pair(destination_even,index_register)` Hexagon encoding models pair constraints, architecture fields and packet termination. It reproduces Agent 04's existing independent `R5:4=tlbr(R3)` as a second test vector.
- A private reference is used only as a comparison oracle and is not embedded in source. The generator rejects reference SHA mismatch, malformed packet/sentinel placement, and unknown operand fields.
- SHA-256 generated 64-byte interval: `b010632bfba034f3a3f61757c130c8bec172c82d14c27cac82ca11800534ce35`. Exact MBN file `[0x51dc,0x521c)`, relative vaddr `[0x21dc,0x221c)`, physical address `[0x8be021dc,0x8be0221c)`.
- Earlier `[0x3044,0x41e0)` executable fragment is **still missing** and cannot be replaced with implicit zeros. The complete ADSP and signing remain unresolved.

## Reproduce

```bash
HEXAGON_LLVM_MC=/path/to/llvm6/bin/llvm-mc
NABU_ADSP_REFERENCE=/private/adsp.mbn
python3 -S tools/adsp/entry.py --llvm-mc "$HEXAGON_LLVM_MC" \
  --reference "$NABU_ADSP_REFERENCE" --output /tmp/adsp-tlb-snapshot-64.bin
HEXAGON_LLVM_MC="$HEXAGON_LLVM_MC" NABU_ADSP_REFERENCE="$NABU_ADSP_REFERENCE" \
  python3 -m unittest discover -s tests -p 'test_*.py'
```

The second `RWE` executable segment begins at ADSP file `0x6000`, with mapped vaddr `0xf0003000`, paddr `0x8be03000`, file size `0x32fe8`. Its first `0x30` bytes include non-instruction vector/linkage data; executable function at file `0x6030`. **Do not count those initial bytes as source-reconstructed code**. Further work will reconstruct executable routines from their actual entries.

## Checkpoint 02: first 92 bytes of second RWE executable segment `[0x6030,0x608c)`

- Original ADSP second `RWE` PT_LOAD: file `[0x6000,0x38fe8)`, `vaddr 0xf0003000`, physical `[0x8be03000,0x8be35fe8)`, file size `0x32fe8`, permissions `RWE`. Original file `[0x6000,0x6030)` contains boot metadata and is **not** modeled as 12 instructions; the first actual function begins at file `0x6030`.
- The 92-byte generated fragment encodes a controller initialization/permission check. Source-level packet grouping includes an extended call to the external platform initializer, saving/restoring `R1:0`, `SYSCFG` permission bit clearing, `SGP0` pointer access, two guarded extended memory reads, and two conditional return paths.
- All **23** V66 words source-derived: LLVM6 independently assembles **20**, then three checked privileged register-transfer encodings replace strictly checked LLVM6 `nop` sentinels (`R1=SYSCFG`, `SYSCFG=R1`, `R0=SGP0`). Rizin independently decodes all 23. An important address-mapping subtlety: external calls/jumps must calculate displacement from the **physical** ADSP address `0x8be03030`, not MBN file address `0x6030`; treating file offsets as PC made an external branch wrong by `0x3000` and was corrected.
- The last instruction jumps into the later ADSP segment's processing function. This range is a *partial function group*; do not conflate it with a contiguous rebuild from `0x6000`, or with newly authenticated image bytes.

Reproduce:

```sh
HEXAGON_LLVM_MC=/path/to/llvm6/bin/llvm-mc
NABU_ADSP_REFERENCE=/private/adsp.mbn
python3 -S tools/adsp/rwe.py --llvm-mc "$HEXAGON_LLVM_MC" \
  --reference "$NABU_ADSP_REFERENCE" --output /tmp/adsp-rwe-startup-92.bin
```

## Checkpoint 03: controller dispatch `[0x608c,0x60d4)` and 164-byte RWE prefix

Added 72 byte-identical bytes (`[0x608c,0x60d4)`) to the 92-byte RWE startup, making an independently source-assembled **164-byte** packet sequence `[0x6030,0x60d4)`, SHA-256 `835f82782f328e8684869c95902e342a79823acf11b99725a69f6e0d8d61bd10`. Physical address range `[0x8be03030,0x8be030d4)`.

This builds two mode-dispatch predicates, four external Hexagon controller calls with **explicit instruction extenders**, and a handler register pair. LLVM6's default scheduler omitted the four extenders; forcing the original packet form via mnemonic `immext` reconstructs the correct V66 bytes while retaining understandable function names and relocation offsets. The cross-segment external call targets are expressed in *physical* PC-relative terms, not ELF file offsets. The source contains **no reference opcodes**.

The overall ADSP source coverage now includes the 68-byte boot prefix, the 4,156-byte closed controller RX span `[0x41e0,0x521c)`, and a separate 164-byte RWE entry fragment; they are **disjoint**. The next missing RWE instruction is at `0x60d4`; the earlier `[0x6000,0x6030)` metadata is not executable code.

## Checkpoint 04: complete initial RWE controller initialization `[0x6030,0x6148)`

The source-generated RWE interval is now **280 contiguous, byte-identical bytes** from ADSP file `[0x6030,0x6148)` (physical `[0x8be03030,0x8be03148)`), SHA-256 `f270f97637d294721dd1d8de4778f1235e02914862f586ba2895664162934603`.

The final 116 bytes `[0x60d4,0x6148)` write the controller's memory-mapped status, clear SYSCFG bit 4, zero the SSR configuration, perform cache-line cleanup, invoke two platform services and enter the startup trap loop. Four privileged SYSCFG/SSR accesses are field-encoded as **checked V66 operations**, not copied vendor instruction words.

The last packet contains two normal 32-bit scalar immediates and an original one-word **Hexagon duplex** `r0=#24; r1=-1`. LLVM6's automatic packet pairing chose a different and byte-incompatible instruction schedule; the source emitter disables auto-pairing and replaces exactly the final two source-assembled scalar words with a *separately source-assembled duplex instruction* (`rwe-duplex.S`). Every source operand and packet position is checked. Both LLVM and Rizin verify the original 29 instruction words of this extension. The diagnostic constants and attached OEM signatures are not included in the regenerated artifact.

The next runnable RWE function starts at file `0x6148`. The initial RWE `[0x6000,0x6030)` remains distinct boot/linkage data; this is **not** 0x118 bytes generated starting from `0x6000`.

## Checkpoint 05: warm boot-pointer checks `[0x6148,0x6194)`

Added **76 code bytes** source-assembled directly from Hexagon LLVM6 mnemonic packets. This code checks three controller global pointers with predicated branches, resets six pairs of storage fields (including original Hexagon duplex zero-writes) and clears the reference flag before entering a hardware wait loop. The three GP-relative reads need explicit instruction extenders; omitting them generates a different 64-byte instruction stream, so packet width is tested exactly. No privileged placeholders or reference opcode slices are needed for this 76-byte interval.

The cumulative second-PT_LOAD instruction fragment is **356 bytes**, `[0x6030,0x6194)` (`physical [0x8be03030,0x8be03194)`). The next routine starts at `0x6194`. The reference-only 0x30-byte data header immediately preceding file `0x6030` remains unimplemented metadata and is **not counted** as executable reconstruction.

## Checkpoint 06: controller global-memory state `[0x6194,0x6220)`

Another **140 exact independently generated bytes** reset and check the controller's shared global status, choose an ADSP logical thread, save its predicate group, and clear multiple cache/status slots. The source preserves three explicit GP-load extenders, nested predicate branches, Hexagon duplex zero-writes and a conditional controller handler call. LLVM6 assembles **33 ordinary packet words**; the two V66-only operations (`R1=HTID`, `R28=P3:0`) are replaced only at checked privileged sentinels, preserving the packet-continuation bit on the predicate group read.

Combined RWE code now covers `[0x6030,0x6220)` = **496 exact bytes** (physical `[0x8be03030,0x8be03220)`), with no OEM code bytes embedded. The next uncovered executable instruction is at file `0x6220`, still in the same controller-initialization chain. Its earlier metadata header `[0x6000,0x6030)` remains separate and unimplemented.

## Checkpoint 07: guarded Hexagon thread/processor state `[0x6220,0x625c)`

Another **60 exact source-derived bytes** decode selected controller-thread bits, read `HTID`, check the 64-bit synchronization slot via `memw_locked`, branch on predicate ownership, then restore packed `P3:0` from an input GPR. The three-instruction-plus-extension packet following the predicate write is preserved exactly. LLVM6 encodes thirteen ordinary instruction words; two v66-only HTID/predicate-group words are strictly checked typed replacement operations (one one-word packet, one packet continuation). The registered external branch destinations use physical address arithmetic and are independently decoded by Rizin.

Total contiguous independently reconstructed executable bytes in the second ADSP `RWE` section: **556 bytes** `[0x6030,0x625c)`. Next uncovered instruction is the `allocframe(#8)` at `0x625c`. Do not count `[0x6000,0x6030)` loader metadata as executable.

## Checkpoint 08: per-thread codec frame state `[0x625c,0x62c8)`

Another **108 source-assembled exact instruction bytes** prepare thread-local
codec frame metadata, validate its packed type and controller predicate, send
the attached metadata to a platform handler and clean the corresponding cache
line. The frame's 27 V66 instruction words are built entirely from human-written
Hexagon assembly in `sources/xiaomi-nabu/adsp/rwe/rwe-frame-init.S`. LLVM6 packetization
matches the original bytes when the signed `mpyi(r28,#0xe0)` immediate remains
**unextended**; using `##0xe0` creates an additional extender and changes
byte identity. No original byte arrays or privileged placeholder patches are
needed in this extension. Rizin V66 decoding provides a second decoder check.

RWE coverage is now **664 bytes**, ADSP file `[0x6030,0x62c8)` and physical
`[0x8be03030,0x8be032c8)`, generated SHA-256
`a35018c28ecf2adc1ba4f09f7f22c19128d8815c68c9738e968c186be74da95a`.
The first 0x30 bytes of the RWE load are still external data metadata, not
reconstructed instructions. Next missing code starts at file `0x62c8`.

## Checkpoint 09: controller calls and per-thread privileged state `[0x62c8,0x638c)`

New **196 executable Hexagon V66 bytes** reconstruct a controller-service call
chain, processing-mode notification, thread identity indexing and special
register state update. The first 112 bytes are directly generated by LLVM6
from packetized source (`sources/xiaomi-nabu/adsp/rwe/rwe-runtime-calls.S`): exact named
physical call targets, explicit instruction extenders and cache-line cleaning.
The final 84 bytes (`sources/xiaomi-nabu/adsp/rwe/rwe-runtime-registers.S`) contain six
V66-only privileged register transfers (`HTID`, `MODECTL`, `SYSCFG` read/write,
`SSR` read/write), generated from bounded typed register field encoders.
The source-assembled register producer for `MODECTL` is checked *before*
replacement and preserves the `.new` dependent memory-store packet. No
OEM instruction words are copied into generated output.

Combined RWE executable reconstruction is now **860 bytes**, file interval
`[0x6030,0x638c)`, physical `[0x8be03030,0x8be0338c)`, SHA-256
`cc010032ab48fe0bc8d75c9846427b243871c2c1d1b2b95e1e38daf8c3cc4c88`.
Byte-for-byte checked against the pinned original ADSP and independently
disassembled with the Rizin Hexagon decoder. The next unbuilt instruction
starts at `0x638c`. This is still only a small fragment of the overall
ADSP RWE `PT_LOAD`; reference-assisted full images remain outside scope.

## Checkpoint 10: runtime diagnostics, recovery and controller wait `[0x638c,0x6474)`

The next **232 independently authored executable bytes** write captured
controller `SSR`, `ELR`, `DIAG` and `SYSCFG` into mapped status slots, clean
cache lines, follow a conditional fallback chain, and poll the global
controller pointers until their event loop is ready. LLVM6 compiles three
bounded mnemonic packet sources; four V66 system-register reads are strictly
checked typed replacements for LLVM6 placeholders, preserving the exact
original packet parse markers. The recovery and polling fragments require **no
raw opcode replacements**. All new words match the pinned ADSP reference and
Rizin cross-decodes the Hexagon packets.

The RWE code now covers **1,092 contiguous independently reconstructed bytes**,
file `[0x6030,0x6474)`, physical `[0x8be03030,0x8be03474)`, SHA-256
`79ad3e2b076a6766817c40c55330295914caefcf3103b6facb968d4a05a6cb2c`.
The trailing one-word no-progress polling loop is genuine firmware control
flow; the next independent routine starts at file `0x6474`. Metadata
`[0x6000,0x6030)` is still not reconstructed, and complete ADSP code and
authentication are not implied.

## Checkpoint 11: thread launch, mode register handoff `[0x6474,0x64d8)`

**100 additional exact executable bytes** reset event memory, clean it,
read V66 `HTID` and `MODECTL`, persist thread-specific controller state and
invoke a controller service. The source has two separate original functions:
`[0x6474,0x64b4)` (64 bytes, thread-control and wait loop) and
`[0x64b4,0x64d8)` (36 bytes, controller-ready update and physical branch
trampoline). LLVM6 generates all ordinary Hexagon packets; only the two
privileged V66 operations are checked typed encodings, preserving the `.new`
producer/consumer packet of the MODECTL write. All 25 packet words match the
original reference; an independent Rizin decode checks controller flow.

RWE source-built code now covers **1,192 consecutive bytes** in file range
`[0x6030,0x64d8)` (physical `[0x8be03030,0x8be034d8)`), SHA-256
`839bee5dbdc28d7f2f92a96808a57e7a64ce2bb7bcdc9bb2792e869820733679`.
The next code begins at `0x64d8`, which is separate from the preceding short
physical-branch trampoline; whole ADSP ELF/auth data remain incomplete.

## Checkpoint 12: controller reset and privileged cache sequence `[0x64d8,0x6568)`

Another **144 exact source-generated Hexagon V66 bytes** initialize the
controller's `SSR`/`SYSCFG` state, issue sequential platform manager calls,
set global mode flags, write and clean the hardware event register and return
to the genuine startup halt loop. LLVM6 assembles 30 original mnemonic
instruction words and six constrained V66 system-register transfers use named
operand-field encoders only at checked placeholder locations. The final `jump`
to itself is real control flow; it is not synthesized alignment or a
stand-in for missing code.

The continuous ADSP second `RWE` executable fragment grows to **1,336 bytes**,
file `[0x6030,0x6568)`, physical `[0x8be03030,0x8be03568)`, SHA-256
`bee69122cab18a7d2e2b1a0c801e2b8044fdf1c1364752be434e503fe0267ab3`.
Next distinct function starts at `0x6568`. Original 0x30-byte metadata
before `0x6030`, remaining RWE code and OEM authentication remain outside
this reconstruction.

## Checkpoint 13: ADSP codec register snapshot and fallback `[0x6568,0x65f4)`

The new **140 executable bytes** use original Hexagon V66 packet boundaries,
physical-address relative calls and non-reordered instruction extenders.
Source `sources/xiaomi-nabu/adsp/rwe/rwe-register-snapshot.S` models the controller
frame, chained external service calls, SGP0 mapped descriptor reads, ELR
snapshot, HTID-based indexing and a guarded error-loop return. LLVM6 source
assembles **31 of the 35 instruction words**. Four remaining privileged
transfers (`r1=SGP0`, `r9=SGP0`, `ELR=r4`, and a packet-continuing `r3=HTID`)
are emitted through checked typed V66 register-field encoders after exact
LLVM6 placeholder validation; there are no copied OEM opcode slices.

The cumulative RWE instruction fragment is now **1,476 exact bytes**, ADSP
file `[0x6030,0x65f4)` (physical `[0x8be03030,0x8be035f4)`), SHA-256
`9c3e5a6a6336700544c6a3fbc572ddc0e488271fc8d23c85f622ff7db0781e75`.
The next instruction at `0x65f4` is a separate one-word return stub; it
has not been included in this checkpoint. The 0x30-byte boot metadata at the
second RWE segment entry remains distinct original data, and the original
OEM signature/certificate block remains externally authenticated reference.

Verification: LLVM6 mnemonic assembly + checked V66 encoder values; independent
Rizin hexagon-v66 decoding of the complete 140-byte interval; direct SHA-pinned
reference byte comparison; workspace offline Rust/Python tests and release build.
Branch is still rooted in Agent04's older `d7cb8b1…` snapshot, **not** the
missing checkpoint-50 integration tree.

## Checkpoint 14: version guard and return stub `[0x65f4,0x661c)`

The additional **40 source-assembled V66 bytes** include a 4-byte standalone
`jumpr lr` stub and a revision-gated controller configuration routine. Nine
ordinary instructions/duplex packets are LLVM6 source-assembled; the tenth is
a checked architectural register read `r0 = REV` (`SystemRegister.REV`, S29),
inserted only after verifying an exact assembler NOP slot. It masks the
revision, conditionally returns, reads a physical MMIO flag and writes the
ready marker when absent. Rizin's independent V66 decoder agrees on all
packet boundaries and operands. The register read has a unit test and the
source emitter compares its full output to the pinned MBN without copying
original code bytes.

RWE fragment: **1,516 contiguous bytes** `[0x6030,0x661c)` (physical
`[0x8be03030,0x8be0361c)`), SHA-256
`68534204cf65712bc055995149dfa10efd690e5f5aae1f918fb441dd72b06537`.
The following controller diagnostic/cache routine starts at `0x661c` and has
not yet been rebuilt. Firmwares beyond the selected fragment and OEM signing
remain outside this checkpoint.

## Checkpoint 15: V66 diagnostic cache-bank scanner `[0x661c,0x6734)`

Added **280 exact source-generated executable bytes** for the ADSP controller
cache/tag scan. A 70-word LLVM6 packetized assembly models controller
boot calls, revision-dependent cache layout, two nested hardware loops,
MMIO result buffers and SYSCFG toggling. **62 ordinary instruction words**
(including authentic two-level endloop NOP packet) compile directly from
mnemonics; **eight privileged reads/writes** are checked register encoders:
`SYSCFG` read/write, `LC0`/`LC1` control reads, and `ictagr`/`icdatar`
cache-bank reads. Every original register/packet position is checked against
an exact LLVM6 NOP placeholder before encoding. A real terminal NOP at
+0xd8 is intentionally preserved and tested separately, not treated as a
missing privileged operation. All 70 resulting words independently decode
with Rizin V66 and match the pinned reference.

RWE continuous code `[0x6030,0x6734)` now has **1,796 bytes**, SHA-256
`9a8d8dabdc7d76572dfd2b2a6671061e79fd74211c3c4c3565faf2d9738ef360`.
Physical mapping is `[0x8be03030,0x8be03734)`. The next independent
routine starts at file `0x6734`; the original 0x30-byte boot metadata, other
ADSP segments and OEM auth are still not independently rebuilt. This branch
remains provisional on the older Agent 04 base rather than checkpoint 50.

## Checkpoint 16: physical memory and cacheline helpers `[0x6734,0x67e8)`

Reconstructed **180 additional executable bytes** across three adjacent
Hexagon v66 functions: a 56-byte hardware-loop word copier with D-cache
cleaning; a 60-byte controller CFGBASE-to-physical-load program; and a 64-byte
wrapper publishing two physical-bank pointers then copying/cleaning them.
LLVM6 assembles 42 of 45 instruction words from packetized mnemonic source,
including original loop packet parsing. Three privileged words are checked
register/physical-load typed encodings (`r0=CFGBASE`, twice
`r0=memw_phys(r2,r0)`) replacing strictly verified source NOPs. The initial
CFGBASE read is *inside* a two-operation packet with an explicitly encoded
packet-continuation bit. No reference opcode words are reused in the emitter.

The cumulative rebuilt RWE code now covers **1,976 consecutive bytes** of
file `[0x6030,0x67e8)` (phys `[0x8be03030,0x8be037e8)`), SHA-256
`0bb7559997f4557ca8305eb05d95a999823b2812f4d6456c1df790a8428ada40`.
The following routine begins at `0x67e8`. Tests include packet-boundary,
physical register-field bounds, direct byte comparison, LLVM6 assembly and an
independent Rizin V66 decoder check. The input MBN file remains outside Git;
there is no newly OEM-signed firmware output.

## Checkpoint 17: four-word physical register scan `[0x67e8,0x6818)`

**48 additional exact Hexagon V66 executable bytes** read a cached controller
configuration, translate the physical register selector, and iterate four
hardware-register words into a mapped cache buffer. LLVM6 assembles 11 words
from explicit mnemonic packets; the single `r3=memw_phys(r0,r2)` instruction
is encoded using checked source/destination/address register fields and
strictly verified NOP placement. No original firmware opcodes are copied.

Total RWE source-built bytes: **2,024 contiguous**, file `[0x6030,0x6818)`
physical `[0x8be03030,0x8be03818)`, SHA-256
`eb2594282616b9c8fe2fd9898c44fa9a5b5e18ab8db9debb856e0d030dce62e3`.
Independent Rizin reads the same packet boundaries; next routine begins at
`0x6818`. This remains a provisional Agent04-based branch awaiting an actual
checkpoint-50 descendant; source-level semantics/hardware trust are unproven.

## Checkpoint 18: codec feature-mode classifier prefix `[0x6818,0x68bc)`

Added **164 fully source-assembled Hexagon V66 instruction bytes** forming the
first four bit-pattern cases of a hardware-codec mode matcher. LLVM6 compiles
all **41 instruction words** from readable predicates, shifts, explicit
`immext` operations and PC-relative branches with the correct physical
addresses; no typed privileged substitutions or opaque word arrays are
needed. Rizin's V66 decoder independently confirms the `.new` predicate
packet groupings. The function is **incomplete**, with branches into code not
yet reconstructed; no synthetic return or implicit zeros have been added.

The continuous source-built RWE region is **2,188 bytes**, ADSP MBN file
`[0x6030,0x68bc)` / physical `[0x8be03030,0x8be038bc)`, SHA-256
`f52c0d96b6bfa2ebe7b63cfbb755e504967e7fb67d80f735845c2a3bce44f37e`.
The next instruction to recover is file `0x68bc`.

## Checkpoint 19: Hexagon controller mode classifier continuation `[0x68bc,0x697c)`

Adds **192 independently source-assembled V66 executable bytes** after the
prefix in checkpoint 18. A manually authored packetized Hexagon source file
`sources/xiaomi-nabu/adsp/rwe/rwe-feature-modes-b.S` checks upper-bit codec prefixes
`0x5c`, `0x50`, `0x61` and the alternate sign-bit marker, using masked
shifts and `.new` predicate-dependent branches. Failure paths tail-jump to
the original controller error endpoint with the two distinct negative status
codes; all branch offsets are expressed as symbolic physical-address targets.
No original instruction bytes are embedded in the emitter or this assembly.
All **48 32-bit instruction words** are generated by LLVM6 and compared
against the SHA-pinned ADSP image. Rizin independently decodes the resulting
packet boundaries. The special three-word status/jump packets must keep
`immext`, error register and jump in one packet; splitting the packet changes
the parse marker and branch displacement.

Continuous ADSP second executable RWE coverage is now **2,380 exact bytes**,
file `[0x6030,0x697c)`, physical `[0x8be03030,0x8be0397c)`, SHA-256
`827f05929eae764a67b2e5a32113d9357121fc7789161e2e4cfe535cd0b880a2`.
The current codec-classifier function continues beyond `0x697c`; that is the
next uncovered instruction, not a claim of completed program logic.

## Checkpoint 20: dependent codec classifications `[0x697c,0x6ab0)`

The next **308 source-generated Hexagon V66 bytes** test the 0x96 upper
prefix and two alternate low-bit sign markers, then the two dependent
four-predicate profiles requiring a zero upper field and an aligned `0x3f`
codec byte. Both code fragments are independently authored in
`tools/rwe-feature-modes-{c,d}.S` and generated by LLVM6
with exact packet parse-marker agreement. Physical branch targets are
expressed using symbolic assembler offsets. The three-word negative-error
packets (`immext` + register assignment + jump) must remain indivisible;
otherwise parse markers/PC-relative branch fields change.

This adds **77 independently assembled 32-bit instruction words** without
borrowed OEM code words, and extends contiguous RWE reconstruction to
**2,688 bytes**, ADSP file `[0x6030,0x6ab0)` and physical
`[0x8be03030,0x8be03ab0)`. Generated SHA-256:
`350430f6928737b8bd71e1d40dee5d3931aca9f93fc1a7134aad01dd01670547`.
The classifier function remains incomplete after the next instruction at
`0x6ab0`; all upstream code and external OEM signatures remain unchanged.

## Checkpoint 21: complete codec-format classifier `[0x6ab0,0x6c10)`

Another **352 byte-identical source-assembled V66 executable bytes** finish
the bitmask classifier (the paired `0x20`/`0x1f` and `0x40`/`0x3f` codec
classes). The branch-prediction annotations `:nt` vs `:t` on `.new`
predicate jumps are retained verbatim; removing the hints changes machine
words. Error `-0x19` and fallback `-1` are source-assembled into two separate
controller return packets, followed by the function's genuine
`deallocframe`/`jumpr lr` epilogue. LLVM6 emits **88 instruction words**, all
byte-equal to the pinned ADSP original and independently decoded by Rizin.

The **complete classifier** is now `[0x6818,0x6c10)` = 1,016 bytes;
cumulative source-generated ADSP RWE interval is **3,040 contiguous bytes**,
file `[0x6030,0x6c10)` and physical `[0x8be03030,0x8be03c10)`.
SHA-256 `78881b6f1f11029d0689ed1dd1e6034bdf0e5681b3776645e774c6c04aaee269`.
The next **separate TLB controller helper** begins at `0x6c10`; other ADSP
executable sections and the original auth container remain unimplemented.

## Checkpoint 22: ADSP controller TLB restore `[0x6c10,0x6c30)`

Added a **32-byte** source-generated Hexagon V66 helper that restores the 128
saved TLB entry pairs from controller SRAM `0xf000cf20`. The LLVM6-authored
mnemonic source in `sources/xiaomi-nabu/adsp/rwe/rwe-tlb-restore.S` retains the original
packet pairing of the initial register assignment and address extender and
the `.new` predicate loop-back. Seven ordinary V66 instruction words are
emitted directly by LLVM6. The privileged `tlbw(R3:2,R0)` occupies a checked
single-word NOP sentinel and is generated with the existing bounded V66 TLB
writer (`tlb_write_pair_r3_r2_index_r0`), not copied from OEM firmware.

**Exact reference match:** 32 of 32 bytes; full contiguous ADSP RWE interval
`[0x6030,0x6c30)` = **3,072 bytes**, physical
`[0x8be03030,0x8be03c30)`, SHA-256
`be799169c7080733cbe019aeaae39de6dcbf8a06ee0ef9a6cb5a5a0a9d870c49`.
LLVM6 and Rizin V66 independently decode the same source-generated packets;
Rizin confirms the backward branch target `0x8be03c1c` and register fields.
The next 60-byte readback helper starts exactly at `0x6c30` and is not yet
included in this checkpoint. Any later zeros in the RWE segment are *not*
counted as executable source.

Reproduce with the pinned private ADSP reference outside Git:

```sh
python3 -S tools/adsp/rwe.py --llvm-mc "$HEXAGON_LLVM_MC" \
  --reference "$NABU_ADSP_REFERENCE" --output /tmp/adsp-rwe-3072.bin
```

## Checkpoint 23: ADSP TLB readback and cache clean `[0x6c30,0x6c6c)`

Added a **60-byte** Hexagon V66 helper that reads 128 TLB entry pairs,
persists them into the `0xf000cf20` controller array, and cleans 32 D-cache
lines. `sources/xiaomi-nabu/adsp/rwe/rwe-tlb-readback.S` contains source-level
instructions and two labeled loops; LLVM6 compiles 14 ordinary 32-bit
packet words. A bounded typed V66 `tlb_read_pair(2,0)` encoder replaces the
checked single-word `nop` sentinel at `+0x18`, after verifying the opcode
slot and end-of-packet fields. Rizin independently decodes both loops,
`R3:2 = tlbr(R0)`, the 8-byte postincrement and `dccleana(R1)`.

Reference SHA-256 and MBN PT_LOAD header checks pass. **The entire
`[0x6030,0x6c6c)` RWE executable block is now 3,132 source-built,
byte-identical bytes**, physical `[0x8be03030,0x8be03c6c)`; SHA-256
`507b5a42a35b95cf68af607520bb2c2a49bdb436bb274ce9dc75ed5eea6426e0`.
The readback routine's 15 instruction words contain one typed privileged
operation. Rust (160 tests), Python (159 tests) and the offline release
build pass with the real Hexagon LLVM6 compiler and pinned ADSP reference.

**Important boundary:** The original second `RWE` PT_LOAD contains a long
file-backed zero region from `0x6c6c` up to its next nonzero byte at
`0xff20`. The zero region is not source-assembled executable instructions
and must not be appended to `rwe.py` output or counted as a
source-reconstructed routine. The next investigation should first classify
the nonzero material at `0xff20` before attempting any further code.

## Checkpoint 24: first code after RWE sparse linker regions `[0x10330,0x1034c)`

The pinned original ADSP RWE `PT_LOAD` is *not* continuously executable after
`0x6c6c`. Its file-backed zero run `[0x6c6c,0xff20)` has **37,556 bytes**;
`[0xff20,0xffb0)` is **144 nonzero bytes of unclassified data** that are
**not counted as source-derived metadata or code**; `[0xffb0,0x10330)` is
another **896 bytes of exact zero fill**. This is a sparse region ledger, not
permission to insert OEM bytes into a compiler output or count fill as code.

Disjoint source-authored Hexagon packet sequence `[0x10330,0x1034c)` adds
**28 exact executable bytes**, physical `[0x8be0d330,0x8be0d34c)`, SHA-256
`8a6aa97426de7b6028e2bcbfc54468c3e387d857694c45412b9838796ae0939a`.
The two 64-bit GP-relative memory loads (`0xf000d688`, `0xf000d690`) and
`jumpr LR` are independently LLVM6-assembled. **The `jumpr LR` at `0x10340`
is a packet-continuation word**; Hexagon's packet includes the next function's
extended memory load (`0x10344–0x10348`). Splitting at the apparent return
produces different bytes. Rizin V66 independently decoded the original and
source-built packet. No privileged opcode replacement was needed.

New emitter and verifier: `tools/adsp/rwe-sparse.py`, independently
reconstructing **only** these 28 bytes; the old contiguous emitter remains
`[0x6030,0x6c6c)` = 3,132 source-built bytes. Their combined executable
coverage is **3,160 bytes in two disjoint spans**, not a new contiguous
`[0x6030,0x1034c)` interval. The next unreconstructed instruction is at
`0x1034c`. Rust 160 tests, Python 162 tests, release build and private ADSP
reference byte comparison pass.

```sh
python3 -S tools/adsp/rwe-sparse.py --llvm-mc "$HEXAGON_LLVM_MC" \
  --reference "$NABU_ADSP_REFERENCE" --output /tmp/adsp-rwe-10330.bin
```

## Checkpoint 25: thread cycle accounting after sparse ADSP layout `[0x1034c,0x103a4)`

Another **88 byte-identical source-generated V66 bytes** track per-thread
control-cycle counters. The routine reads `HTID` and the paired `PCYCLE`
register, computes a delta from 64-bit controller state, accumulates it into
per-thread SRAM and stores the absolute pair to a separate cycle table. It
retains the original `.new` predicate branch, 64-bit operations and packet
parse markers. The four privileged slots are generated with checked typed
V66 operations (`read_system_register`, `read_system_register_pair` and
`packet_continuation`); all other 18 instruction words come from LLVM6
source-assembled mnemonics. The final `PCYCLE` read and `mpyi` form a single
hardware packet. LLVM6 cannot assemble the privileged read, so the compiler
emits a standalone checked NOP and separately assembles `mpyi`; the typed
source-level encoder changes **only the PCYCLE read's packet continuation
field** before joining the two instructions. No OEM opcodes are copied.

The disjoint RWE region `[0x10330,0x103a4)` is now **116 exact executable
bytes**, SHA-256 `f86d4af99e535fc517e115e7fb65ce710440482faf52754e6ed195ef09c35b0d`.
Rizin independently decodes `HTID`, `PCYCLE`, accumulation and branch
patterns, and comparison to the pinned ADSP image is byte-identical. Total
source-generated ADSP RWE executable coverage is **3,248 bytes** across
two disjoint intervals `[0x6030,0x6c6c)` and `[0x10330,0x103a4)`.
The linker fill / unclassified table between the intervals is excluded.
The next uncovered instruction is at file offset `0x103a4`.

Reproduce with:

```sh
python3 -S tools/adsp/rwe-sparse.py --llvm-mc "$HEXAGON_LLVM_MC" \
  --reference "$NABU_ADSP_REFERENCE" --output /tmp/adsp-sparse-116.bin
```

## Checkpoint 26: debugger-state/SYSCFG guard `[0x103a4,0x103d0)`

The next **44 exact Hexagon V66 bytes** read `SGP0` and `ISDBEN`, conditionally
clear `SYSCFG` bit 5, issue `isync` and wait for debugger-enable bit 4 to
clear. `sources/xiaomi-nabu/adsp/rwe/rwe-next-isdb-guard.S` independently assembles
the ordinary instructions, including original `.new` predicate branch and
an 11-word packet sequence. Five unsupported privileged V66 register ops
are inserted solely through checked architectural field encoders after
verifying sentinel position and packet parse bits. The first SGP0 read is a
**continuing** operation in a packet with a controller-memory zero store.

The independently assembled, disjoint ADSP RWE block `[0x10330,0x103d0)`
now contains **160 byte-identical executable bytes**, SHA-256
`1cf2932b6a7633ff782e6b43e40c289e378a875feafceb280503fc53d7e9afda`.
Together with the earlier `[0x6030,0x6c6c)` block, this is **3,292
source-derived executable bytes in two intervals**. Linker fill and the
unclassified 144-byte table between them remain excluded. The next
instruction is at `0x103d0`, inside the same controller-initialization
routine. Both Rust (160), Python (162) and offline release build pass with
strict private-reference byte comparison.

## Checkpoint 27: controller SRAM clone and D-cache cleaning `[0x103d0,0x10430)`

Another **96 original-byte-identical Hexagon V66 bytes** source-assemble the
controller's conditional SRAM clone and cache-cleaning path. The source in
`rwe-next-sram-clone.S` verifies status flag bit 21,
conditionally clones 50 32-bit words from the original controller frame
(with a `.new` store dependency) and cleans ten 32-byte cache-line groups.
LLVM6 emits **all 24 instruction words** without any unsupported/privileged
opcode substitutions. The original `r4=memw(r0++#16)` stands in its own
packet; the following `r4=#1` and `memw(r3++#16)=r4.new` share a packet.
The two loops' Hexagon endloop parse bits are preserved.

The disjoint controller fragment is now `[0x10330,0x10430)` = **256 verified
source-built bytes**, SHA-256
`a48adb597242649fbaeab2ace5a54960db887fc56aa572be01274afe0ff3fbe5`.
The previously reconstructed `[0x6030,0x6c6c)` remains separately
**3,132 bytes**; source-built RWE coverage therefore totals **3,388
executable bytes** across two intervals. The large zero-fill ranges and the
unclassified 144-byte material at `[0xff20,0xffb0)` remain unclaimed.
Rust 160 tests, Python 163 tests and the release build pass. Next
unimplemented code begins at file `0x10430`.

## Checkpoint 28: controller MMIO ready polls `[0x10430,0x10460)`

Another **48 source-assembled V66 bytes** poll two controller status registers:
`0xf061e740` and `0xf0017d38`. They issue the two `memw(R0+off) = clrbit(#2)`
commands then wait for bit 0 in the corresponding MMIO readback to clear.
Both Hexagon `.new`-value retry branches and their backward destinations
are preserved by LLVM6 packetization; all **12 instruction words** are
assembled from mnemonic source with no special-register patches.

The disjoint source-built block `[0x10330,0x10460)` is now **304 bytes**,
SHA-256 `63cf29f5f96eabe7d406273cabc60e9b0132682b03f1fda92dafb0b77ab1180f`.
Together with the separate `[0x6030,0x6c6c)` prefix, that makes **3,436
source-generated executable bytes** (not including zeros or the unknown
144-byte table). The next unimplemented ADSP instruction starts at file
`0x10460`. Rust 160 tests, Python 164 tests, release build and reference
comparison all pass.

## Checkpoint 29: V66 debugger/breakpoint register snapshot `[0x10460,0x104b8)`

Added **88 independently source-assembled, reference-identical bytes** to
serialize seven privilege-bank registers (`ISDBCFG0/1`, `BRKPTPC0/1`,
`BRKPTCFG0/1`, `ISDBGPR`) into a controller register table using `.new`
postincrement memory stores. The source runs a three-iteration D-cache
cleaning loop over the saved register array. For these seven unsupported
V66 instructions, the normal LLVM6 mnemonic source deliberately emits a
checked `R0=add(R0,#0)` `.new` producer so LLVM6 can assemble each paired
store with correct packet parse markers. The source-only generator checks
each exact producer location, then replaces it with a strictly typed
`packet_continuation(read_system_register(0,named_special_register))`.
No proprietary OEM instruction bytes are included.

The disjoint source-built interval `[0x10330,0x104b8)` is now **392 exact
executable bytes**, SHA-256
`f8701109d3ab5c63be6a0308d20f9a46cc42ce9d0b364a8e94242fc589974a77`.
Together with `[0x6030,0x6c6c)`, that is **3,524 source-derived executable
bytes** across two intentionally disconnected RWE code spans. The next
unimplemented instruction is file `0x104b8`. Rust 160 tests, Python 165
tests, the offline release build and strict ADSP revision comparison pass.

## Checkpoint 30: controller phase startup and ISDBEN shutdown `[0x104b8,0x10534)`

Extended the disjoint controller RWE source interval to `[0x10330,0x10534)`
(**516 exact executable bytes**), SHA-256
`bccdc199b0719fca0df25a6248e3d442bb0bf9ad237f384576a4df2d503f7c59`.
The new **124-byte** phase controller constructs mode-4/mode-5 branches,
invokes named external physical controller services and records the active
phase before executing `ISDBEN=R0` and `isync`. LLVM6 assembles 30 ordinary
instruction words; the single privileged system-register write uses a checked
`write_system_register(SystemRegister.ISDBEN,0)` replacement at an independently
source-assembled sentinel. No original instruction bytes were imported.

Together with separate `[0x6030,0x6c6c)` (3,132 bytes), **3,648 bytes** of
ADSP executable code are now source-assembled across two disjoint intervals.
The original zero-fill and nonzero 144-byte unclassified table between them are
excluded. Next missing program instruction starts at `0x10534`.

## Checkpoint 31: physical cycle delta, CFGBASE and `memw_phys` `[0x10534,0x10570)`

Another **60 independently encoded byte-identical Hexagon V66 bytes** update
an ADSP controller's 64-bit per-thread cycle delta, increment an event count,
clean the affected cache line and read a physical controller word. Ordinary
LLVM6-assembled source encodes twelve instructions; three privileged Hexagon
operations use named, checked ISA field encoders at exact sentinel positions:
`R1:0=PCYCLE` as a packet continuation, `R1=CFGBASE` and
`R2=memw_phys(R0,R1)`. The sequence's `r1=asl(r1,#5)` intentionally continues
through a zero immediate extender for `r0=##8`; splitting that packet changes
byte identity.

The disjoint ADSP executable interval `[0x10330,0x10570)` is **576 bytes**,
SHA-256 `a5b06fe80408fcf662e12824297b78558307fd44b4961f021cac4df57d3fa965`.
The earlier `[0x6030,0x6c6c)` remains 3,132 byte-identical bytes, so total
source-authored RWE code is **3,708 bytes**, and intervening reference-only
fill/unknown data are not counted. Next uncovered file instruction `0x10570`.

## Checkpoint 32: physical controller scan, retry and wait `[0x10570,0x105c4)`

Added **84 independently source-built and exactly matching Hexagon V66 bytes**:
three checked `memw_phys(Ra,Rb)` reads, a predicate-controlled retry,
cycle-statistics cache cleanup and an external controller callback followed
by `syncht`, `wait(R0)` and a genuine wait loop. Hexagon LLVM6 assembles
17 ordinary instruction words and four unsupported privileged operations are
checked typed encodings, including the dedicated `wait_thread_from_register(0)`
(0x6440c000). Source explicitly expresses the physical call destination
rather than copying a reference displacement. The trailing `jumpr LR` is
packetized with the next function's two memory reads, just as in firmware.

The disjoint controller interval `[0x10330,0x105c4)` now contains **660
independently generated executable bytes**, SHA-256
`72ecab6796775beb135e29f934b77dab0549c09eda618e2af834a0ced78a614e`.
Adding the separate 3,132-byte `[0x6030,0x6c6c)` prefix yields **3,792 bytes
of ADSP independently source-built RWE code**, not counting external table,
linker fill or OEM authentication. The next missing routine begins file `0x105c4`.


## Checkpoint 33: five-bank physical controller state snapshot `[0x105c4,0x10674)`

Rebuilt **176 additional source-authored Hexagon V66 instruction bytes** as five
pairs of physical-controller register reads and ten GP-relative writes to the
saved state bank (`0xf000d6c0..0xf000d6e4`). LLVM6 assembles all normal
operations, including 10 immediate extenders and 10 stores; ten unsupported
`memw_phys` operations are checked typed encodings replacing exactly ten
single-instruction NOP sentinels in independently assembled packets. Source
preserves the fifth pair's distinct `R5=memw_phys(R5,R1)` operand, rather than
mistakenly repeating the preceding `R4` form. Independent Rizin disassembly
confirms all operands and packet boundaries.

The existing disjoint region `[0x10330,0x10674)` is now **836 exact bytes**,
SHA-256 `46eb1561fbce95c69b637c53d1bec4c7c2b879edbbd7715675d2564015ba1158`.
Added to the other `[0x6030,0x6c6c)` source-derived **3,132-byte** RWE prefix,
this gives **3,968 independently source-authored ADSP RWE instruction bytes**
across two **separate** spans. Byte identity and LLVM6/Rizin checks passed;
160 Rust and 170 Python tests pass, including the newly added typed physical
read regression test. The release build passed.

The file interval `[0x10674,0x106e8)` consists of **116 verified zero-filled
bytes**, *not* executable instructions and not counted in rebuilt code. The next
nonzero packet begins at file `0x106e8` (physical `0x8be0d6e8`); it is separate
from this checkpoint's contiguous output.

## Checkpoint 34: instruction-cache tag/data controller snapshot `[0x106e8,0x1084c)`

Verified and reconstructed a **separate 356-byte executable V66 routine**
from the pinned ADSP image, SHA-256
`e462906278996d3c868e5b1ca0890e694327322ca875e7876edc4f2a2e2cdfe5`.
The 89 source-authored instruction words perform a revision guard,
SYSCFG/ISYNC transitions, nested LC0/LC1 hardware loops, instruction-cache
tag (`ictagr`) and data (`icdatar`) reads, and paired cache/MMIO writes.
Normal operations and packet parse markers are independently assembled by
Hexagon LLVM6; **nine** V66 privileged reads/writes use checked typed register
encoders substituted at validated NOP slots. The three original NOPs of the
`:endloop0 :endloop1` packet remain genuine instructions, not placeholders.
Rizin independently decodes the generated packets, including their exact
branch targets, control-register bank and the two return instructions.

This is an **independent third instruction interval**, not a continuation of
`[0x10330,0x10674)`: the preceding **116 file bytes** `[0x10674,0x106e8)`
are independently checked linker zeros. The **996 bytes** `[0x1084c,0x10c30)`
are also original linker fill. Neither is counted as source-built code. Total
ADSP RWE code now consists of **3,132 + 836 + 356 = 4,324 source-authored
bytes** across three disjoint executable intervals. No original OEM signature,
reference instruction array or unknown table is pasted into the emitter.

Run with `HEXAGON_LLVM_MC=/path/to/llvm6/bin/llvm-mc` and original
`NABU_ADSP_REFERENCE=/private/adsp.mbn`. Full tests: **160 Rust and 173 Python
passed**, release build passed, zero byte differences. Next nonzero material
starts at file `0x10c30`, physical `0x8be0dc30`.

## Checkpoint 35: controller record initialization prefix `[0x10c40,0x10cb4)`

Following the instruction-cache snapshot at `0x1084c`, the original firmware
contains **996 file-backed zero-fill bytes** `[0x1084c,0x10c30)` and a
**16-byte unclassified record** `[0x10c30,0x10c40)`. They are **not** interpreted
as executable source. The next function starts at ADSP file `0x10c40`.

The first **116 byte-identical bytes** of this function initialize the
controller record array, save ABI registers, allocate a stack frame, read
base-address metadata, and initialize a counted group of record entries.
All **29 Hexagon packet words** are assembled solely from independently
written LLVM6 mnemonic source. A guarded branch, three calls into the
preexisting record-creation helper, instruction extenders, and memory field
initializers are preserved exactly. No V66 privileged workaround or
reference-derived instruction words are needed for this prefix. SHA-256:
`bec8b2bb23a1a1ae89a1be0a6e2f8217246d166deb570b3e8b5ca0c9f5cc1386`.

This **partial function** ends at `0x10cb4`; it does not yet contain the
remaining record processing, exception handling or return path. Source-built
ADSP RWE instruction coverage rises from **4,324 to 4,440 bytes across four
separate regions** (3,132 + 836 + 356 + 116), with all intervening fill and
unclassified data deliberately excluded. LLVM6 and strict reference comparison
pass. The final MBN image and OEM signatures remain unimplemented.

## Checkpoint 36: complete controller record allocator `[0x10c40,0x10dc4)`

Extended the 116-byte checkpoint-35 record initializer with **272 further
source-generated, byte-identical executable bytes**, completing the whole
**388-byte routine** at file `[0x10c40,0x10dc4)`. The completed code sets
up the remaining command records, walks a three-record hardware loop,
reads the `PCYCLELO` system counter, applies a bitmask to controller state,
and preserves both the ABI-correct `dealloc_return` and the zero-result
`jumpr lr` variant. The ordinary operations are independently assembled
by LLVM6 and all instructions/packet markers match the pinned reference.

The V66-only `R4=PCYCLELO` (S30) is emitted using the checked register
encoder after verifying an exact LLVM6 NOP sentinel. LLVM6 initially put
`p0=cmp.eq(r4,#0)` into a separate packet, but the original groups it with
the following compact R5/R1:0 scalar-immediate duplex; the source builds
the two instructions independently and changes only P0's checked Hexagon
packet-continuation bits. The original has **LOADS** of R19:18 and R21:20
near the epilogue; reversing them to stores was caught by direct comparison
and corrected. Rizin independently decodes the final P0 producer, `PCYCLELO`,
two-register loads and both returns.

The generated `[0x10c40,0x10dc4)` function has SHA-256
`3396fd7b2a302a0b37ae979defc987f2cdb6ab58ded5a349be4a44dae92ef839`.
The **ADSP source-derived RWE executable total is now 4,712 bytes in four
disjoint source-built regions** (3,132 + 836 + 356 + 388); the 996-byte
zero area and 16-byte unclassified table immediately preceding this routine
remain **excluded**. Next uncovered `0x10dc4` is a standalone return followed
by the next function at `0x10dc8`. No complete/signed ADSP image exists yet.

## Checkpoint 37: thread registration, K0 lock/unlock `[0x10dc4,0x10eb4)`

Following the byte-identical `[0x10c40,0x10dc4)` controller-record allocator,
reconstructed a **240-byte source-generated** Hexagon V66 interval. Its first
word at `0x10dc4` is a genuine standalone `jumpr lr` stub; the next function
starts at `0x10dc8` and ends at its `deallocframe`/jump epilogue. The function
registers controller threads, reads `HTID` and `S57:56`, publishes two words of
thread metadata and conditionally executes `k0lock`/`k0unlock` around an
updated flag. All ordinary instructions and packet parse markers were
independently assembled by Hexagon LLVM6. The four otherwise unsupported V66
instructions are generated from narrowly validated register-field encoders:
`R18=HTID`, `R21:20=S57:56`, `k0lock` and `k0unlock`. LLVM6 sentinel NOP slots
are checked before replacement; OEM opcode literals are not copied into the
source. Independent Rizin decoding confirmed those four operations, branches,
register allocations and the original return.

Source-built `[0x10dc4,0x10eb4)` SHA-256:
`949b2be28406779a9e763ec2ff912e7b089356721645722fc84183d08a05df9e`.
The generated bytes match all 240 bytes of the pinned ADSP reference with zero
differences. The following 12 file bytes `[0x10eb4,0x10ec0)` are independently
checked all-zero file-backed **layout fill**, not executable source. The next
function begins at `0x10ec0`.

Current ADSP RWE independent executable reconstruction: **4,952 bytes** in
four disjoint intervals, `[0x6030,0x6c6c)` (3,132), `[0x10330,0x10674)`
(836), `[0x106e8,0x1084c)` (356), and `[0x10c40,0x10eb4)` (628).
The intervening 144-byte unclassified record at `[0xff20,0xffb0)` and
16-byte record `[0x10c30,0x10c40)` remain explicitly **not source-built**.
This is a partial ADSP reconstruction, not a complete device image and not
independently authenticated or newly signed firmware.

## Checkpoint 38: privileged controller startup and SGP0 getter `[0x10ec0,0x10f64)`

Rebuilt the next **164 source-derived Hexagon V66 instruction bytes**, following
the original **12-byte file-backed zero-fill region** `[0x10eb4,0x10ec0)`.
The new block has a standalone `jumpr lr` stub at `0x10ec0`, a distinct
controller-startup function at `0x10ecc` and a short SGP0 status getter at
`0x10f50`. These functions read `SGP0`, `HTID` and `SYSCFG`, synchronize the
controller’s MMIO state, clear a SYSCFG bit, unlock K0 and the TLB, then
perform original service calls and status extraction. **Seven V66 system
instructions** that LLVM6 lacks are generated from narrowly checked typed
register/command encoders at verified sentinel slots. All other words and
Hexagon packet boundaries are independently assembled from mnemonic source;
a `call`/`r16=r0` pair is deliberately grouped into one packet instead of
having its continuation bits guessed or copied from OEM bytes.

The **164 generated bytes match** the pinned ADSP image exactly (SHA-256
`fea3e03da47c690b8e1feb99691ba8a71626ae6a7d3cafd3c7693797fd2d6073`),
and a separate Rizin decoder confirms seven privileged opcodes, original
control-flow branches and the final `jumpr LR`. Three focused tests validate
privileged register fields, packet continuation, strict reference checksum and
rejected corruption. The new code is **disjoint** from the preceding source
interval due to the twelve zero bytes, and must not be described as an
unbroken executable segment.

Cumulative independent ADSP RWE instruction coverage is now **5,116 bytes in
five disjoint intervals**: `[0x6030,0x6c6c)` 3,132; `[0x10330,0x10674)`
836; `[0x106e8,0x1084c)` 356; `[0x10c40,0x10eb4)` 628; and
`[0x10ec0,0x10f64)` 164. OEM-authenticated complete firmware is still not
source-rebuilt. Next executable code begins at file offset `0x10f64`.

## Checkpoint 39: controller-state guard, validation and recovery `[0x10f64,0x10fe8)`

- Added **132 independently source-assembled executable bytes** beginning immediately after the prior SGP0 getter. File interval `[0x10f64,0x10fe8)`, physical `[0x8be0df64,0x8be0dfe8)`. Generated SHA-256 `237769076f87396e531637f0fa034009954e0783b2737bf8e52041933275b0e0`, compared exactly against the pinned ADSP original.
- Main controller-state routine polls three global status registers, invokes the watchdog disable handler when available, reports invalid state and branches to the original controller recovery path; its early-return packets and external calls retain exact physical PC-relative displacements.
- All **33 instruction words** are source-built with LLVM6. Its assembler's canonical error-packet duplex scheduler moves the `R2=#0` scalar into the wrong two-operation pair. The original layout is rebuilt from **two separate mnemonic-only LLVM6 source files**: a regular `R2=#0` instruction (with a checked packet-continuation transform) and the one-word `R0=#36 ; R17:16=memd(R29+#0)` duplex. A strict test first verifies the expected LLVM-assembled surrogate packet before replacing it; no OEM executable bytes are imported.
- New source: `sources/xiaomi-nabu/adsp/rwe/rwe-next-controller-state.S`, `tools/adsp/rwe-next-controller-state.py`, and two tiny source-assembled scalar/duplex files. Focused tests verify original address ranges, the packet pair's independent encodings and corrupted-output/wrong-reference rejection.
- Contiguous source-built RWE code is now **5,248 bytes total across five disjoint intervals**: `[0x6030,0x6c6c)` 3,132; `[0x10330,0x10674)` 836; `[0x106e8,0x1084c)` 356; `[0x10c40,0x10eb4)` 628; and `[0x10ec0,0x10fe8)` **296**. (The original 12-byte fill `[0x10eb4,0x10ec0)` is not code.)
- Next routine begins at file `0x10fe8`. All earlier disjoint code and size accounting remains intact. This checkpoint remains based on older Agent 04 history, **not checkpoint 50**, and is not a full independent ADSP image or a new OEM signature.

Reproduce:

```sh
HEXAGON_LLVM_MC=/path/to/llvm6/bin/llvm-mc
NABU_ADSP_REFERENCE=/private/adsp.mbn
python3 -S tools/adsp/rwe-next-controller-state.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-state-132.bin
```

## Checkpoint 40: K0-locked controller helpers and SSR/SGP0 getters `[0x10fe8,0x11088)`

- Another **160 source-built executable bytes** match the pinned ADSP exactly, physical `[0x8be0dfe8,0x8be0e088)`; SHA-256 `9e233fd82b5fe05dea0d6b7e24c7b113737c8e03bef1c2384ad46e899a5926b0`.
- Reconstructed the hardware K0-locked word compare/swap loop and two error returns, a separate K0-protected global busy-status setter, a small SSR status getter and SGP0 context forwarding. LLVM6 source assembles 33 ordinary words; **seven** privileged V66 SGP0/SSR/K0 operations are supplied by existing checked architecture encoders, at exact independently assembled `nop` sentinels. The original packet extender and `.new` predicate boundaries are preserved.
- The original **8 zero bytes** `[0x11088,0x11090)` following these functions are linker fill, not independently generated instructions. Next code starts at `0x11090` and requires a new disjoint source fragment.
- ADSP RWE source-derived executable coverage totals **5,408 bytes** in five disjoint regions: `[0x6030,0x6c6c)` (3,132), `[0x10330,0x10674)` (836), `[0x106e8,0x1084c)` (356), `[0x10c40,0x10eb4)` (628), and `[0x10ec0,0x11088)` (456). This does not count the 8-byte linker fill or earlier data/zeros.
- Source: `sources/xiaomi-nabu/adsp/rwe/rwe-next-locked-controller.S`, checked emitter `.py`, regression tests. Not a complete ADSP, not OEM-signed, and not based on the missing checkpoint-50 integrated tree.

```sh
python3 -S tools/adsp/rwe-next-locked-controller.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-locked-controller-160.bin
```

## Checkpoint 41: SSR-gated controller scanner `[0x11090,0x11108)`

- **120 Hexagon V66 source-built executable bytes**, file `[0x11090,0x11108)`, physical `[0x8be0e090,0x8be0e108)`; generated SHA-256 `6b87d5d25140462bcf0e6d70dc42c4f1434d0922274c180572ec168918269a19`. Strict pinned ADSP comparison passes with zero mismatches.
- The routine scans three controller-record slots under SSR gating, reading GP-relative status, selecting qualifying records and returning either a matched record value or zero. LLVM6 assembles ordinary instructions; its `R2=SSR` placeholder is replaced by an existing checked V66 encoder.
- Its 3-word hardware-loop end has a nondefault source arrangement: two scalar instructions and a duplex `r0+=4/r2+=1` with `:endloop0`. The source generator recompiles the four scalar operations separately with LLVM6 `-mno-pairing`, requires correct source ordering/packet markers, and inserts a separately source-assembled duplex. No reference opcode bytes are included in source output.
- The **eight file-backed zero bytes** `[0x11088,0x11090)` are linker fill and excluded from independently authored executable coverage. Total ADSP source-built RWE code is now **5,528 bytes across six disjoint intervals**, including the new 120-byte span.
- This remains a portable branch based on Agent 04 rather than checkpoint-50 integration. Next executable code begins at file `0x11108`.

```sh
python3 -S tools/adsp/rwe-next-ssr-context-scan.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-ssr-context-scan-120.bin
```

## Checkpoint 42: packed controller offsets and plane descriptors `[0x11108,0x11164)`

- Added **92 source-built Hexagon V66 executable bytes** from two separate leaf functions, physically `[0x8be0e108,0x8be0e164)`. SHA-256 `28590b1d3b085252ae014b84f021140b1f475ee2ff594f84398625f6cdc6ae9f`. Every instruction and packet boundary is independently LLVM6 source-assembled; there are no privileged patches.
- The first helper packs a controller memory address into window/cross-page capability fields (`0x11108–0x1113c`), while the second packs plane offset/length bitfields (`0x1113c–0x11164`). Their final `jumpr LR` instructions match.
- **12 file-backed zero-fill bytes** `[0x11164,0x11170)` are verified layout, *not independently authored executable code*. The next nonzero ADSP code starts at `0x11170`.
- Total independently source-rebuilt ADSP RWE code is now **5,620 bytes in seven disjoint intervals**, excluding both the preceding 8-byte gap and the 12-byte trailing gap, and all earlier linker fill/table regions.

```sh
python3 -S tools/adsp/rwe-next-packed-offsets.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-packed-offsets-92.bin
```

## Checkpoint 43: partial EVB/i-cache relocation and controller format guards `[0x11170,0x111dc)`

- **108 executable bytes** at ADSP file `[0x11170,0x111dc)` (physical `[0x8be0e170,0x8be0e1dc)`) source-assemble and match the pinned revision with zero mismatches. SHA-256 `5e2fa9fe85dca6489dcfcd5a36da7520b63fbbbe0dca15663cca8465774ec2fb`.
- The new routine begins with an EVB register read, invalidates an instruction cache entry, syncs instruction fetching, derives packed-window addresses from controller offset fields, tests capacity and branches toward fallback paths beyond the current partial function. LLVM6 assembles 26 ordinary Hexagon instruction words, with the **single EVB transfer** reconstructed through the checked typed V66 register encoder.
- Exactly **12 zero-filled, file-backed bytes** `[0x11164,0x11170)` separate this code from the preceding 92-byte pair; these are linker layout and not executable reconstruction. This function **remains incomplete** from `0x111dc` onward; do not infer a successful return yet.
- Total ADSP RWE source-built code is **5,728 bytes in eight disjoint intervals**, excluding the 12-byte linker fill and earlier zero/data gaps. Required integrated checkpoint-50 Git base remains unavailable; this branch descends from Agent 04's older history.

```sh
python3 -S tools/adsp/rwe-next-icache-relocation-prefix.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-icache-relocation-prefix-108.bin
```

## Checkpoint 44: complete instruction-cache relocation and format-slot selection `[0x11170,0x11394)`

- Fully reconstructed **548 byte-identical Hexagon V66 instruction bytes** from mnemonic assembly and one existing checked EVB register encoding. Physical address `[0x8be0e170,0x8be0e394)`, SHA-256 `988a851691d8ab8c7fa7c63a39d2f60f9bc62400a2ce4865ccd83b3b332dd706`. All four **contiguous source sections** (108-byte prefix, 64-byte early continuation, 184-byte bounded descriptor selector, 192-byte completion) are independently source-assembled and rechecked as one complete function. This supersedes checkpoint 43's partial 108-byte-only claim (no double counting).
- The routine invalidates an instruction-cache entry using `EVB`, computes relocation address windows, checks bound violations, walks up to seven candidate descriptor slots, writes the selected controller entry, cleans/invalidate/synchronizes caches, and returns through its original `dealloc_return`. Its fail-closed path calls original controller diagnostics then enters a real branch-to-self trap.
- Original **92-byte diagnostic text section** `[0x11394,0x113f0)` contains C-string messages, **not Hexagon executable instructions**, and is strictly excluded from rebuilt code; the next routine begins at `0x113f0`. Unlike previously counted linker fill, this data may later be re-authored as named constants, but no OEM text bytes are copied into these code fragments.
- Total independently source-built ADSP RWE executable code reaches **6,168 bytes across eight disjoint intervals**, including now-complete `[0x11170,0x11394)` 548 bytes; no OEM-supplied instruction bytes or unexplained fill are included in generated artifacts.
- Checkpoint history still descends from the older Agent 04 Git lineage rather than the unavailable checkpoint 50 integrated tree.

```sh
python3 -S tools/adsp/rwe-next-icache-relocation.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-icache-relocation-complete-548.bin
```

## Checkpoint 45: controller dispatch stub and MMIO flag helpers `[0x113f0,0x11450)`

- Three **disjoint executable functions**, all independently assembled in LLVM6 and byte-matched against pinned ADSP: stub `[0x113f0,0x113f8)` **8 bytes**, flag-window writer `[0x11400,0x11428)` **40 bytes**, and conditional MMIO flag sync `[0x11430,0x11450)` **32 bytes**. Total **80 source-built instruction bytes**, no V66-only substitutions or opaque code words.
- A **92-byte diagnostic ASCII pool** `[0x11394,0x113f0)` precedes the stub and is unimplemented **data**, not executable code. The 8-byte zero fills `[0x113f8,0x11400)` and `[0x11428,0x11430)` are structural linker fill; no fill bytes are counted as reconstructed instructions.
- Rebuild using `tools/adsp/rwe-next-flag-helpers.py` with `--output-dir`. It emits three separate binary fragments and `source-coverage.json` rather than silently padding holes into one image. All 20 Hexagon words are source-assembled, and exact offsets, hashes and missing parts are validated.
- Cumulative independently reconstructed ADSP RWE executable code increases to **6,248 bytes in eleven disjoint intervals**. The next executable function begins at file `0x11450`; the full ADSP firmware, OEM signature and checkpoint-50-based integration remain incomplete.

```sh
python3 -S tools/adsp/rwe-next-flag-helpers.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output-dir /tmp/adsp-flag-helpers
```

## Checkpoint 46: partial global codec-controller startup `[0x11450,0x114a8)`

- **88 source-built Hexagon V66 executable bytes** at ADSP file `[0x11450,0x114a8)` (physical `[0x8be0e450,0x8be0e4a8)`) match the pinned `adsp.mbn` byte-for-byte, including independently preserved packet extenders and the loader's first controller global-state accesses. Source: `sources/xiaomi-nabu/adsp/rwe/rwe-next-controller-global-startup.S`, verified emitter `.py`, focused tests.
- LLVM6 source-assembles **19** ordinary instruction words. The three V66-only forms `CSwi(R0)`, `ciad(R0)` and `IMASK=R1` have **checked named/operand encoders** replacing exact LLVM6 `nop` sentinels. Rizin confirms all three original encodings, and alternative-register experiments revealed that the `CSwi/ciad` register selectors are **not linear**. Encoders intentionally reject every non-R0 cache command rather than inventing a register mapping; IMASK uses the existing independent named system-register write fields.
- Entire fragment length is 22 words, with no copied reference opcode bytes, zero fill or DSP data counted as source. The partial function continues at `0x114a8`; **do not claim a complete startup function or OEM-signed firmware**.
- New cumulative independently rebuilt ADSP RWE executable code is **6,336 bytes in eleven disjoint intervals**. This is a portable branch rooted in the prior Agent 04 history, not the unavailable checkpoint-50 integrated repository.

```sh
python3 -S tools/adsp/rwe-next-controller-global-startup.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-global-startup-88.bin
```

## Checkpoint 47: controller global MMIO initialization loop `[0x114a8,0x11548)`

- Added **160 source-built, byte-identical executable bytes** from 40 ordinary Hexagon instruction words, file `[0x114a8,0x11548)` (physical `[0x8be0e4a8,0x8be0e548)`). No V66-only instruction replacement is required: LLVM6 assembles all packets, and independent Rizin decoding confirms their packet parse markers.
- Source builds controller MMIO pointer and state layout, publishes multiple GP-relative status registers, initializes a bounded hardware context, and retries the startup callback using `.new` condition branches. Literal extenders are explicitly present in source, preserving the original four-instruction packet boundaries.
- The complete **248-byte continuous controller-global prefix** `[0x11450,0x11548)` (checkpoint 46 + 47) is independently regenerated and compared byte-by-byte with the pinned ADSP reference. The function remains **incomplete** beyond `0x11548`; no imaginary return or missing executable bytes have been emitted.
- Cumulative source-built ADSP RWE executable code is **6,496 bytes across eleven disjoint intervals**; separate OEM diagnostic strings and file-backed zero fill remain excluded. Git lineage still derives from the older Agent 04 branch, awaiting the checkpoint-50 integration tree.

```sh
python3 -S tools/adsp/rwe-next-controller-global-memory.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-global-memory-160.bin
```

## Checkpoint 48: global controller SRAM and MMIO pointer-map publication `[0x11548,0x11624)`

- Added **220 byte-identical ADSP executable bytes** `[0x11548,0x11624)` at physical `[0x8be0e548,0x8be0e624)`. Both fragments (76-byte pointer handoff `[0x11548,0x11594)` and 144-byte memory layout `[0x11594,0x11624)`) are assembled **entirely by LLVM6**, without privileged substitutions or imported executable bytes.
- Reconstructs GP-relative memory-map publication, multiple 256-byte-aligned controller block offsets, a bounded initialization loop with `.new` conditional branch, and 32-/64-bit controller MMIO stores. Packet grouping preserves all explicit instruction extenders and register update semantics.
- The total **468-byte source-built contiguous controller-global prefix** `[0x11450,0x11624)` matches the pinned `adsp.mbn` exactly; no OEM padding or data are counted. The function continues beyond file `0x11624` and remains **partial**.
- Cumulative independently reconstructed ADSP RWE executable total: **6,716 bytes in eleven disjoint intervals**, retaining the older Agent 04 ancestry pending checkpoint-50-based integration.

```sh
python3 -S tools/adsp/rwe-next-controller-global-layout.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-global-layout-220.bin
```

## Checkpoint 49: controller physical address handoff `[0x11624,0x1167c)`

- Source-assembled **88 new byte-identical Hexagon V66 executable bytes** at `[file 0x11624,0x1167c)` and physical `[0x8be0e624,0x8be0e67c)`; every instruction of this 22-word region is emitted by LLVM6 **without V66 patches**.
- The source invokes external initialization/callbacks, saves the MMIO pointer for its 128KiB physical window, branches on the checked GP status field, and restores controller registers before tail-calling platform handoff. The `if(cmp.eq(R2.new,#0))` packet and following combine operations retain the exact original packet parse markers.
- Controller-global reconstruction from checkpoints 46–49 now covers **556 continuous bytes** `[0x11450,0x1167c)`; a conditional execution path continues at `0x1167c`, so the function is **not yet complete**. This is not a signed/flashable ADSP image.
- Cumulative independently source-rebuilt ADSP RWE executable code: **6,804 bytes in eleven disjoint intervals**, explicitly excluding all OEM diagnostic strings and structural fill. Git branch is still portable old-agent-04 ancestry, not checkpoint-50-rooted integration.

```sh
python3 -S tools/adsp/rwe-next-controller-global-handoff.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-global-handoff-88.bin
```

## Checkpoint 50: **complete** global controller startup `[0x11450,0x11698)`

- All **584 executable bytes (146 original Hexagon V66 instruction words)** reconstruct from five consecutive source sections, with no vendor opcode arrays and no copied OEM file spans. LLVM6 assembles 143 ordinary words, and three named/operand-checked V66 operations (`CSwi(R0)`, `ciad(R0)`, `IMASK=R1`) replace strictly validated source NOP placeholders. Independent Rizin decoding agrees on original packet positions.
- Physical mapping `[0x8be0e450,0x8be0e698)`. The complete function stores MMIO pointers, initializes controller per-channel SRAM offsets, runs a conditional hardware setup callback, performs a 128KiB physical-resource publication and returns through its *original* saved-frame/branch handoff at file `0x11694`.
- Exactly **eight following zero bytes** `[0x11698,0x116a0)` are verified linker fill, **not** reconstructed executable source. The next function begins at file `0x116a0` (physical `0x8be0e6a0`), separate from this completed function.
- Cumulative independently source-generated RWE Hexagon code is **6,832 bytes across eleven disjoint executable intervals**, with original firmware authentication and many ADSP later text/data regions still absent. The checked forward work remains a portable older-Agent-04-based branch until proper checkpoint-50 integration.

```sh
python3 -S tools/adsp/rwe-next-controller-global-full.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-global-complete-584.bin
```

## Checkpoint 51: SRAM-zero initialization hardware-loop prefix `[0x116a0,0x116d8)`

- Added **56 independently source-assembled Hexagon V66 executable bytes** `[file 0x116a0,0x116d8)` at physical `[0x8be0e6a0,0x8be0e6d8)`. All fourteen source instruction words are produced directly by LLVM6; no unsupported privileged operations or OEM opcode arrays were needed.
- The code creates a controller stack frame, prepares a non-NULL SRAM destination, and clears 128 eight-byte records via original `loop0`/`:endloop0` packet structure. The original load/store packet includes `:mem_noshuf`, which is explicit in the mnemonic source and required to preserve the byte-identical packet order.
- The **eight file-backed zeros** `[0x11698,0x116a0)` are linker fill and are strictly **excluded** from counted source code. The next conditional path begins at `0x116d8`, so this is a partial function, not a new complete ADSP firmware.
- Total independently generated executable bytes in the ADSP RWE section: **6,888 bytes across twelve disjoint intervals**. The exact reference image was used for comparison only; the branch remains based on older Agent 04 history pending checkpoint-50 integration.

```sh
python3 -S tools/adsp/rwe-next-sram-zero-init.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-sram-zero-initialization-56.bin
```

## Checkpoint 52: SRAM/controller status and initialization `[0x116d8,0x11754)`

- Added **124 independently emitted V66 executable bytes** immediately after checkpoint 51's SRAM zeroing prefix. Physical address range `[0x8be0e6d8,0x8be0e754)`; pinned `adsp.mbn` file `[0x116d8,0x11754)`; resulting SHA-256 `617023865f00ada1b88418e572f470dbae8ee4f4aee9915dafe9ce7cc8bce1ff`.
- The source groups `call` with the subsequent GP store or immediate assignment **within the same Hexagon packet**. Without this grouping LLVM6 places different parse-marker bits in five calls despite having the same operands.
- LLVM6 assembles **29 of 31 instruction words** from mnemonic source. Two V66-only reads (`R2 = SYSCFG`, `R10 = SGP0`) are substituted only into exactly validated LLVM6 `nop` placeholders via existing typed, register-checked encoders. The reference image is never read during output generation; it is used only by `verify` for SHA-256 and exact byte comparison.
- Rizin independently decodes the full generated 124-byte stream; both privileged reads and all branch/call targets match. Four focused tests verify the packet grouping, typed operation locations, exact reference comparison and corruption rejection.
- Total independently reconstructed ADSP RWE executable code now **7,012 bytes across 12 disjoint intervals** (previous 6,888 + 124). `[0x116a0,0x11754)` is one **180-byte contiguous** partial SRAM/controller routine, while the preceding eight zero bytes `[0x11698,0x116a0)` remain linker fill and are excluded from code counts.
- The routine continues from file `0x11754`; its later branches are not complete. Entire ADSP and OEM signature reconstruction remain outstanding; this is an older Agent 04-based branch, not the unavailable checkpoint-50-integrated branch.

Reproduce:

```sh
python3 -S tools/adsp/rwe-next-sram-controller-status.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-sram-controller-status-124.bin
```

## Checkpoint 53: branch-controlled SRAM state switch `[0x11754,0x11798)`

- Added **68 exact independently source-assembled ADSP bytes** (17 Hexagon instruction words) at file `[0x11754,0x11798)` and physical `[0x8be0e754,0x8be0e798)`, SHA-256 `ae8fa220a511dc7cad586d93141ce8349665d270f1de0d7a6e3bf821c2ba7bbc`.
- The switch handles states 0–3, deciding whether to publish register values to mapped controller state slots. LLVM6 constructs every instruction and packet parse marker, including `P0.new` conditional stores and a two-operation duplex; **zero checked privileged patches** were needed.
- The source-generated 68 bytes match the pinned `adsp.mbn` and Rizin decodes the conditional branches to physical `0x8be0e798`. Focused tests reject corrupted output and wrong-reference revisions.
- The full SRAM/controller function `[0x116a0,0x11798)` now covers **248 contiguous executable bytes**. Cumulative ADSP RWE code is **7,080 independently generated bytes across 12 disjoint intervals** (6,888 + 124 + 68); no layout fill is included. The function continues at `0x11798`.
- This branch still descends from prior Agent 04 work, **not** the missing checkpoint-50-integrated base; it does not represent independently signed or flashable firmware.

```sh
python3 -S tools/adsp/rwe-next-sram-state-cases.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-sram-state-cases-68.bin
```

## Checkpoint 54: controller SRAM publication `[0x11798,0x1180c)`

- Added **116 byte-identical source-assembled executable bytes** (`29 Hexagon V66 instruction words`), file `[0x11798,0x1180c)`, physical `[0x8be0e798,0x8be0e80c)`, SHA-256 `462e536ec818f510421ef4b23df1e95ae35ad90e44583a60f6a01b57bb6d50d1`.
- The controller calls platform initialization routines, loads/stores GP-relative MMIO pointers, publishes an event state and writes source-defined command IDs 26/1. Two external `call` packets must include their subsequent `immext` and GP write/argument update; treating them as separate packets changes the parse bits even if their scalar operands match.
- All 29 words are independently LLVM6-assembled with **no privileged placeholder substitutions**, and verified byte-for-byte against the pinned ADSP reference. Focused tests reject changed bytes and a wrong firmware revision.
- The contiguous `[0x116a0,0x1180c)` SRAM/controller routine currently contains **364 independently rebuilt bytes**; cumulative ADSP RWE executable source is **7,196 bytes over 12 disjoint intervals**. The routine continues at file `0x1180c`; original ADSP, signing and other segments are not complete.

```sh
python3 -S tools/adsp/rwe-next-controller-publish.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-publish-116.bin
```

## Checkpoint 55: complete ADSP SRAM/controller state routine `[0x116a0,0x11894)`

- Added **136 independently source-built bytes** `[file 0x1180c,0x11894)`, physical `[0x8be0e80c,0x8be0e894)`, SHA-256 `b95077dde6b16537d44a0b6ca54346b7f5d14a6e439372c1955d800ed2c500e6`. LLVM6 assembles 30 of 34 words; four validated V66 field encodings replace source `nop` placeholders: `R0=SYSCFG`, `SYSCFG=R0`, and two `R2=SGP0` reads. The original ABI-sensitive packet grouping is preserved, including paired call/load instructions, `memw_locked`, isync, and a `jump + deallocframe` tail call.
- The **entire 500-byte controller SRAM function** `[0x116a0,0x11894)` is independently assembled from five contiguous source files (56+124+68+116+136), and the concatenation matches the pinned ADSP image byte for byte. Rizin decodes the newly generated source/privileged operations independently. A focused unit test rejects source tampering, incorrect reference revisions and unchecked privileged substitution.
- Exactly **four trailing zero bytes** `[0x11894,0x11898)` are original linker fill and are **not** counted as independently reconstructed code. The next function begins at `0x11898`.
- Total source-generated RWE Hexagon executable is now **7,332 bytes in 12 disjoint intervals**. ADSP other segments and OEM-signed full-image construction remain unfinished. Branch ancestry is older Agent 04 history, **not** checkpoint 50.

```sh
python3 -S tools/adsp/rwe-next-sram-state-finalize.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-sram-finalize-136.bin
```

## Checkpoint 56: controller SRAM publication leaf `[0x11898,0x118b8)`

- Added **32 independently reconstructed Hexagon V66 bytes** at file `[0x11898,0x118b8)`, physical `[0x8be0e898,0x8be0e8b8)`, SHA-256 `8cc7ecdef39bc91ffba85fbbe50bd28ffceb1cf862cc5def8b2bc79870e05804`. Seven instruction words are LLVM6 source-assembled; `R10=SGP0` uses a strict typed encoder applied at a checked `nop` placeholder.
- The helper obtains the current controller state via platform calls, publishes the GP-sourced value and returns through original `dealloc_return`. The following **eight zero bytes** `[0x118b8,0x118c0)` are linker fill, not independently rebuilt executable instructions. The **four zero bytes** `[0x11894,0x11898)` before this helper are also linker fill.
- Cumulative source-built RWE executable is **7,364 bytes across 13 disjoint intervals**, excluding all alignment zeros and diagnostic strings. New code begins at `0x118c0` after linker fill; the entire ADSP and OEM authentication remain unfinished.

```sh
python3 -S tools/adsp/rwe-next-sram-state-publish-leaf.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-sram-publish-leaf-32.bin
```

## Checkpoint 57: global controller record initialization prefix `[0x118c0,0x11928)`

- Rebuilt **104 V66 instruction bytes** at file `[0x118c0,0x11928)`, physical `[0x8be0e8c0,0x8be0e928)`, SHA-256 `c1700121caea7264587b92c57eb2326e3590963775691d06561b423da9a0bd4f`. The original routine continues past the current interval; do not claim it is complete.
- LLVM6 assembles **22 ordinary words** (including GP-relative extended loads, conditional `.new` branches/stores, 64-bit record fields and packed allocframe). Four V66 register-bank reads (`R2=SGP0`, `R1=REV`, `R2=GP`, `R4=PCYCLELO`) are emitted using individually checked typed field encoders at LLVM6 `nop` sentinels. Rizin independently decodes the output.
- The preceding `[0x118b8,0x118c0)` eight-byte linker fill is excluded from source-built code. Total independently generated ADSP RWE code: **7,468 bytes in 14 disjoint intervals**. Next missing code begins at file `0x11928`, still in this record-initialization routine.

```sh
python3 -S tools/adsp/rwe-next-global-record-prefix.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-global-record-prefix-104.bin
```

## Checkpoint 58: complete global-controller record initializer `[0x118c0,0x119c0)`

- Added **152 exact V66 executable bytes** `[file 0x11928,0x119c0)`, physical `[0x8be0e928,0x8be0e9c0)`, SHA-256 `028a722724122f81dcc2556614e53502cc3d1ac9759fe6e354085aae0fadcbf4`. LLVM6 assembles 37 of 38 instruction words; one independently checked `k0unlock` V66 instruction is encoded at its validated LLVM6 `nop` position.
- Completed the **256-byte global record function** `[0x118c0,0x119c0)` (104-byte prefix + 152-byte completion). Source preserves memory-field initialization, 64-bit record transfers, GP-relative descriptor loads, physical controller callbacks, saved-frame return and original error-report/trap path. The two GP-relative loads use distinct offsets `0xf00250cc` and `0xf00250d0` as confirmed by packet-context decoding.
- The following bytes beginning at **`0x119c0` contain embedded diagnostic text**, not executable Hexagon instructions; they are excluded from source-code coverage. A long linker-fill area follows the text. The next candidate nonzero routine begins at `0x11ae0` and requires separate packet/section classification.
- Cumulative source-generated ADSP RWE executable is now **7,620 bytes across 14 disjoint code intervals**. Reference-only authentication and other ADSP executable/data regions are still missing, and branch ancestry remains older Agent 04 rather than an actual checkpoint-50-integrated branch.

```sh
python3 -S tools/adsp/rwe-next-global-record-complete.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-global-record-complete-152.bin
```

## Checkpoint 59: source-built controller-table initializer `[0x11ae0,0x11ba0)`

- Independently authored **192 exact Hexagon V66 instruction bytes** beginning at file offset `0x11ae0` (physical `0x8be0eae0`) and ending at file `0x11ba0` (physical `0x8be0eba0`), SHA-256 `adb6fe0fe056064ec3686af3cb9c962bd9d25725abab6b9d6d705d072fe8c74d`.
- The code reads shared controller memory, clears aligned record words and tail bytes with **two hardware loops**, writes four controller table arrays, then jumps to the platform finalizer. LLVM6 encodes all 48 scalar/duplex instruction words from mnemonic source. The single changed packet-end field at file `0x11b40` is an explicitly checked architecture **parse-field-only adjustment**: LLVM6 split the register-immediate packet just before its predicate-dependent duplex; no non-parse opcode bits or reference bytes are inserted.
- Diagnostic string text `[0x119c0,0x11a11)` and the intervening file-backed zero fill up to `0x11ae0` are excluded from source-built executable coverage. This initializer is the first executable island after that data/fill section, so it is **disjoint** from the 256-byte global record initializer at `[0x118c0,0x119c0)`.
- Cumulative independently generated ADSP RWE executable source: **7,812 bytes across fifteen disjoint intervals** (7,620+192). The following function starts at `0x11ba0`; the rest of the RWE segment and OEM authentication remain unreconstructed.

Reproduce with the pinned original ADSP reference (only required for verification):

```sh
HEXAGON_LLVM_MC=/path/to/llvm6/bin/llvm-mc
NABU_ADSP_REFERENCE=/private/adsp.mbn
python3 -S tools/adsp/rwe-next-controller-table-init.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-table-init-192.bin
```

## Checkpoint 60: complete atomic controller-slot registration `[0x11ba0,0x11c88)`

- Added **232 independently reconstructed, byte-identical Hexagon V66 executable bytes** at file `[0x11ba0,0x11c88)` and physical `[0x8be0eba0,0x8be0ec88)`. SHA-256 `65fcb5b4999823fea41702074e4cce311a83bb1171b00f21a993aeb578320db5`.
- The complete function implements an atomic lock-free slot publication and bitmap acquisition, with `cl0`/`ct0` bit classification, several `memw_locked` retry loops, a GP-relative index bitmap update, platform notifications and the original `dealloc_return`. LLVM6 independently assembles **57 of 58** instruction words; `R2=SGP0` is encoded only at its checked `nop` placeholder using the V66 special-register operand enum. Rizin independently decodes the V66-only register operation.
- The two new functions `[0x11ae0,0x11c88)` now form **424 consecutive exact executable bytes** and have a focused concatenation test. Cumulative RWE executable reconstruction is **8,044 source-built bytes across 15 disjoint intervals** (previous 7,812 + 232); the diagnostic strings and linker fill preceding `0x11ae0` are still excluded.
- Next missing independent function begins at file `0x11c88` (physical `0x8be0ec88`). No source-built complete ADSP file or new OEM signature is claimed.

```sh
python3 -S tools/adsp/rwe-next-controller-slot-register.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-slot-register-232.bin
```

## Checkpoint 61: complete ADSP atomic slot-release function `[0x11c88,0x11d68)`

- Added **224 byte-identical executable bytes** from an independently authored Hexagon V66 source, file `[0x11c88,0x11d68)` (physical `[0x8be0ec88,0x8be0ed68)`). SHA-256 `b5ab9d2d93173ef61049859c90a7108f7b82f4af2d3fb3a6bdab55307cbb0f4a`.
- The complete routine handles freeing occupied slots, multiple retrying `memw_locked` atomic updates, bitmap release and global callback notification. Its original conditional return paths and the `0xf04148a8` GP table index are explicitly retained. LLVM6 assembles **55/56 words**; the one validated SGP0 read is encoded by the checked V66 register helper. Rizin independently decodes all ordinary instructions and the privileged read.
- The preceding adjacent slot-register function `[0x11ba0,0x11c88)` and this release function form **456 consecutive byte-identical executable bytes**; including the controller-table initializer they form a 648-byte continuous code island `[0x11ae0,0x11d68)`. This island remains disjoint from earlier source-built fragments due to OEM diagnostic text and linker fill.
- Cumulative independently rebuilt ADSP RWE executable is **8,268 bytes across fifteen disjoint intervals**. The following distinct Hexagon helper begins at `0x11d68`. Whole ADSP and OEM authorization remain unfinished.

```sh
python3 -S tools/adsp/rwe-next-controller-slot-release.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-slot-release-224.bin
```

## Checkpoint 62: four complete ADSP controller-slot helper functions `[0x11d68,0x11e6c)`

- Added **260 byte-identical source-assembled Hexagon V66 bytes** (65 machine words), file `[0x11d68,0x11e6c)`, physical `[0x8be0ed68,0x8be0ee6c)`, SHA-256 `47853283eede3581b73c8f6ec55314e319b0a32710ef64752499cf8a485f6b18`. All 65 words are generated natively by LLVM6 from mnemonic packet source; **no** privately sourced opcode values or privileged placeholder substitutions are required.
- Four separate functions: controller slot query (40 bytes), mark slot inactive (104), mark slot active (108), and unsupported-index sentinel (8). Packet grouping preserves a load and `dealloc_return` within the same original packet, as well as paired GP-based status updates and calls.
- Source-built registration, release and four helpers now combine to **716 contiguous executable bytes** `[0x11ba0,0x11e6c)`; with the preceding table initializer, the controller region is **908 contiguous bytes** `[0x11ae0,0x11e6c)`, separately validated against the original reference.
- Total independently assembled ADSP RWE executable is **8,528 bytes across fifteen disjoint intervals** (prior 8,268 + 260). Next function begins at `0x11e6c`. Controller strings, linker fill and unrelated executable sections remain incomplete; the result is not an OEM-signed firmware.

```sh
python3 -S tools/adsp/rwe-next-controller-slot-helpers.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-slot-helpers-260.bin
```

## Checkpoint 63: controller bitmap status query `[0x11e6c,0x11ed0)`

- Added **100 exact source-assembled Hexagon bytes** in 25 instruction words at file `[0x11e6c,0x11ed0)` (physical `[0x8be0ee6c,0x8be0eed0)`), SHA-256 `4da131cc506f5dc9a07150b761cc71737f443eae459aa61a8baf841d376af9c8`.
- Completely implemented the slot bitmap query, separating regular and preallocated controller-map bitfields and preserving the original conditional returns. All instructions and parse markers come natively from LLVM6. One initial physical address was corrected from `0x8be0ef6c` to `0x8be0ee6c`; the fix restores the exact external callback displacement. A zero-value assignment and `dealloc_return` must share their original packet.
- Together with the preceding lookup/activation/deactivation/sentinel block, `[0x11d68,0x11ed0)` is **360 contiguous source-built bytes**. The larger controller island from `[0x11ae0,0x11ed0)` is **1,008 exact executable bytes**, with intervening vendor diagnostic text/linker fill excluded from source coverage.
- Cumulative original executable code independently emitted in ADSP RWE: **8,628 bytes in fifteen disjoint regions**. Next function begins at file `0x11ed0`. Authenticated complete ADSP firmware not produced.

```sh
python3 -S tools/adsp/rwe-next-controller-slot-status.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-slot-status-100.bin
```

## Checkpoint 64: controller slot bitmap publication `[0x11ed0,0x11f18)`

- Added **72 exact source-assembled Hexagon bytes**, file `[0x11ed0,0x11f18)` (physical `[0x8be0eed0,0x8be0ef18)`), SHA-256 `2749c2e454c9f7e655ed6a5426bb9692f41fb83d6458a9646515c68bf08142be`.
- The leaf calls the controller-state getter, derives the five-bit word/bit index of the selected channel, publishes the bit to the GP-backed table and calls the original controller notification helper. All 18 original instruction words come directly from LLVM6, including the `if (p0.new)` conditional read in the same packet as its predicate producer, with no privileged substitutions.
- With the preceding lookup and status helpers the ADSP slot-control executable island `[0x11ae0,0x11f18)` now totals **1,080 contiguous byte-identical code bytes**. Original diagnostic strings, linker fill, unrelated regions and all OEM authentication are not counted.
- Cumulative independently reconstructed ADSP RWE executable source: **8,700 bytes across fifteen disjoint intervals**; next missing code begins at `0x11f18`.

```sh
python3 -S tools/adsp/rwe-next-controller-slot-bitmap-publish.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-slot-bitmap-publish-72.bin
```

## Checkpoint 65: controller slot-class bitmap mask `[0x11f18,0x11f58)`

- Independently generated **64 exact executable bytes** (16 Hexagon V66 instruction words) at ADSP file `[0x11f18,0x11f58)`, physical `[0x8be0ef18,0x8be0ef58)`. SHA-256 `d679d216aa29ad36b1e521467f180cccfa339241896941580fa7680753c26cfc`.
- Calls the controller getter, isolates the selected slot bit index, looks up the corresponding packed bitmap via a signed class mask, then returns either zero or `0x10000` depending on `p0.new`. All opcodes and packet parse markers are independently assembled from LLVM6 mnemonic source. No proprietary opcode-copy or privileged substitution is used.
- The contiguous slot-control executable island `[0x11ae0,0x11f58)` is now **1,144 exact source-assembled bytes**; it remains disjoint from earlier program regions because diagnostic strings and linker fill are excluded.
- Total independently generated ADSP RWE executable is now **8,764 bytes across fifteen disjoint intervals**. Next function starts at file `0x11f58`.

```sh
python3 -S tools/adsp/rwe-next-controller-slot-bitmap-mask.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-slot-bitmap-mask-64.bin
```

## Checkpoint 66: atomic controller slot-class bitmap toggle `[0x11f58,0x11fd8)`

- Added **128 exact Hexagon V66 executable bytes** (32 LLVM6-assembled words) at file `[0x11f58,0x11fd8)`, physical `[0x8be0ef58,0x8be0efd8)`. SHA-256 `56e7f46e65261a29e279c6cd467c29c04f2f0860832eaefd91ec42d8e74a21ea`.
- Emits checked controller context lookup, class/bit extraction, bitmap read/modify/write, and associated global controller MMIO table publication. Correctly preserves all packet boundaries and `p0.new` dependent control flow, ending in the original `dealloc_return`; all opcodes are LLVM6-assembled from Hexagon source, **zero privileged substitutions**.
- The source-built contiguous controller/slot island `[0x11ae0,0x11fd8)` now contains **1,272 bytes**, separate from intervening unrelated OEM text, diagnostic strings, or alignment fill.
- Cumulative ADSP RWE executable source: **8,892 bytes across fifteen disjoint intervals**. Next complete function starts at file `0x11fd8`.

```sh
python3 -S tools/adsp/rwe-next-controller-slot-bitmap-toggle.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-slot-class-bitmap-toggle-128.bin
```

## Checkpoint 67: atomic slot reservation and publication `[0x11fd8,0x12098)`

- Added **192 exact source-reconstructed Hexagon bytes** (48 instruction words) at ADSP file `[0x11fd8,0x12098)`, physical `[0x8be0efd8,0x8be0f098)`. SHA-256 `6e1c383765039699f41ba6e1efecfd8dc49b41496f43bfe311031d2f0ecdba87`.
- The controller slot-reservation path uses an atomic `memw_locked` compare/store retry loop, packs metadata in a per-slot structure, sets the active-class bitmap and emits runtime notification and optional callback. LLVM6 generates **47 ordinary words**. The one V66-only `R2=SGP0` is reconstructed with a checked register-field encoder. The assembler closes the `r3:2=combine(#1,#0)` packet one instruction early: the exact two Hexagon parse bits are adjusted with a checked `packet_continuation` transformation, not an OEM opcode literal.
- Continuous controller executable island from `[0x11ae0,0x12098)` is now **1,464 bytes**; other disjoint islands retain their original untouched boundaries. The overall independently reconstructed RWE executable totals **9,084 bytes across fifteen disjoint intervals**.
- **Next function:** file `0x12098`. Partial code, unrelated read-only strings, linker fill and signed OEM fields are not claimed as rebuilt.

```sh
python3 -S tools/adsp/rwe-next-controller-slot-reserve.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-slot-reserve-192.bin
```

## Checkpoint 68: controller slot deactivation `[0x12098,0x12114)`

- **124 independently source-assembled Hexagon V66 bytes** (31 instruction words), file `[0x12098,0x12114)`, physical `[0x8be0f098,0x8be0f114)`. SHA-256 `2801241a15300e18c9ee413670249e519c74270203032dbf65e3e75fdd6b9ac8`.
- Disables the selected slot, clears per-controller state words, publishes a class-flag indication and invokes the optional controller callback. LLVM6 assembles all 31 exact words with original packet boundaries and extender offsets, **no privileged opcodes or parse-bit patches**. The following `[0x12114,0x12118)` contains four original zero linker-fill bytes, explicitly not counted as code.
- Previous `[0x11ae0,0x12098)` and this routine form a **1,588-byte code island** `[0x11ae0,0x12114)`; the source-built RWE executable total is **9,208 bytes in fifteen disjoint islands**. Next executable function starts at file `0x12118`.

```sh
python3 -S tools/adsp/rwe-next-controller-slot-deactivate.py \
 --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
 --output /tmp/adsp-controller-slot-deactivate-124.bin
```

## Checkpoint 69: four controller slot helper leaves `[0x12118,0x121bc)`

- Rebuilt **164 byte-identical executable bytes** (41 Hexagon V66 words), file `[0x12118,0x121bc)`, physical `[0x8be0f118,0x8be0f1bc)`, SHA-256 `e227be877be8b4ea68a6b1b66c217042f77af8297001d565ff59cbc3f20d32c7`.
- Four complete functions, each independently source-assembled: range-check slot table `[0x12118,0x12140)` (40 bytes), publish class flag17 `[0x12140,0x12168)` (40 bytes), publish selected slot bitmap `[0x12168,0x121a0)` (56 bytes), and publish class flag16 `[0x121a0,0x121bc)` (28 bytes).
- LLVM6 assembles 40 instruction words directly. One `R3=SGP0` uses checked V66 register fields. `:mem_noshuf` retains controller-memory store/load order in the second helper. The four-byte linker fill `[0x12114,0x12118)` is not counted as executable code. No OEM opcode-array substitutions are used.
- **9,372 independently reconstructed RWE executable bytes across 16 disjoint intervals**, with the newest separate island `[0x12118,0x121bc)`. Next nonzero program material follows `[0x121bc,0x121c0)` zero fill.

```sh
python3 -S tools/adsp/rwe-next-controller-slot-leaves.py \
 --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
 --output /tmp/adsp-slot-leaves-164.bin
```

## Checkpoint 70: controller-slot scanner prefix `[0x121c0,0x12220)`

- Source-assembled **96 byte-identical Hexagon V66 executable bytes** (24 instruction words), file `[0x121c0,0x12220)`, physical `[0x8be0f1c0,0x8be0f220)`, SHA-256 `24cfc910d10268d647a141debe5a5eba275eb56967636ed7b90e687aa5162e6f`.
- Begins a controller slot scanner with `allocframe(#56)`, global controller-table address loads, and `loop0` over allocated slot entries. The register move and `memd` store at file `0x121cc–0x121d8` must share the original packet. LLVM6 assembles all 24 instruction operands, but its packetizer emits separate packets for `p0=cmp.eq` and `r0=#1`; **only two checked architectural packet-continuation parse fields** are changed at `0x121cc` and `0x121d0` to preserve the original packet grouping. No opcode/body bits are inserted from the reference.
- The file-backed four-byte linker fill `[0x121bc,0x121c0)` is **excluded** from executable source counts. Previous executable island ends at `0x121bc`. New partial function ends mid-execution at `0x12220`; the same function continues immediately thereafter.
- Cumulative independently reconstructed RWE code is **9,468 bytes across 17 disjoint intervals** (prior 9,372 plus new 96). Full ADSP firmware and OEM authentication remain incomplete.
- LLVM6/reference verification and deliberate revision/byte-mutation tests passed. 160 Rust tests and 293 Python tests pass when the tools and ADSP reference are configured; release build succeeds.

```sh
python3 -S tools/adsp/rwe-next-controller-slot-scan-prefix.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-slot-scan-prefix-96.bin
```

## Checkpoint 71: source-rebuilt controller slot scan and atomic update `[0x12220,0x12310)`

- **240 exact Hexagon V66 executable bytes**, file `[0x12220,0x12310)`, physical `[0x8be0f220,0x8be0f310)`, SHA-256 `e68fe74776d9b7346dcb039af7eeb528f1078eef619ebc2a0d3b5b802d623d5b`.
- Three *independently authored* assembly source sections encode descriptor setup (52 bytes), lock-free slot acquisition (64 bytes), and packed halfword tag verification (124 bytes). All **60 instruction words are assembled directly by LLVM6**, preserving `memw_locked` retries, `.new` register dependencies, instruction extenders and conditional calls. No OEM opcode-word substitutions are needed in this interval.
- The preceding scanner prefix `[0x121c0,0x12220)` and these 240 bytes form **336 adjacent exact code bytes** `[0x121c0,0x12310)`, with direct reference comparison in focused tests. The function's bitmap walk and return remain unimplemented after `0x12310`.
- Cumulative independent ADSP RWE code **9,708 bytes across 17 disjoint intervals**. The earlier four-byte linker fill `[0x121bc,0x121c0)` is excluded.
- Source, mutation and revision tests passed; 160 Rust tests and 296 Python tests pass when the ADSP reference and LLVM6 are configured. Release build succeeds.

```sh
python3 -S tools/adsp/rwe-next-controller-slot-scan-operations.py \
 --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
 --output /tmp/adsp-slot-scan-operations-240.bin
```

## Checkpoint 72: complete controller-slot scanner `[0x121c0,0x1242c)`

- Added the final **284 exact executable bytes** `[0x12310,0x1242c)`, physical `[0x8be0f310,0x8be0f42c)`: bitmap walk/locked state changes (144 bytes), class flags and bit handling (80 bytes), loop epilogue and original `dealloc_return` (60 bytes).
- The entire **620-byte** scanner `[0x121c0,0x1242c)` independently rebuilds from **155 Hexagon V66 instruction words**, sha256 `322bae494c2e3d6048ea096aea64ff4be6081896439413eb25f88d0fb2bd6fe6`. The final 71 instruction words and packet delimiters come entirely from LLVM6 source. The only parse-field adjustments in the complete function are the previously validated two packet-continuation bits at `0x121cc` and `0x121d0`.
- All five constituent source sections are verified both separately and as a single contiguous function, with deliberate byte-corruption and wrong-revision rejection. Four-byte alignment before the function remains excluded from code totals.
- **Cumulative source-built ADSP RWE code: 9,992 bytes across 17 disjoint executable intervals.** Next visible material after the function starts at file offset `0x1242c`; inspect it before classifying as code or fill.
- Configured LLVM6 and pinned-reference tests: **160 Rust** and **299 Python**, all pass; release build passes. Still not a complete ADSP image or independently signed OEM firmware.

```sh
python3 -S tools/adsp/rwe-next-controller-slot-scan-complete.py \
 --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
 --output /tmp/adsp-slot-scan-complete-620.bin
```

## Checkpoint 73: controller-slot remove/search `[0x12430,0x1259c)`

- Reconstructed **364 independently assembled, byte-identical executable bytes** (91 Hexagon V66 instruction words), file `[0x12430,0x1259c)`, physical `[0x8be0f430,0x8be0f59c)`, SHA-256 `5cac635ed34b47baf9c9b637f20cf500e584d88b425272e199d032809b48d3fb`.
- The new function searches active controller bitmap tables and releases a selected slot through atomic `memw_locked` retry loops, conditional callback dispatch, counter adjustment and the original `dealloc_return`. Its three original-source-style assembly parts (104/124/136 bytes) are assembled **entirely by LLVM6**; no privileged instructions, packet-marker patches or OEM opcode-copy are necessary.
- The immediately preceding complete scanner `[0x121c0,0x1242c)` and the release function are separated by the original four-byte linker fill `[0x1242c,0x12430)`, which is **not** included in executable bytes. Verified that both generated functions concatenate around exactly those four zero bytes in the reference.
- Cumulative source-built ADSP RWE code **10,356 bytes across 18 disjoint intervals**. Next nonzero material begins at `0x125a0`, after another four-byte alignment gap. Full ADSP and OEM authentication remain incomplete.
- 160 Rust tests, **302 Python tests**, reference byte equality, release build and Git integrity pass (configured LLVM6 toolchain and correct firmware reference).

```sh
python3 -S tools/adsp/rwe-next-controller-slot-remove-complete.py \
 --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
 --output /tmp/adsp-slot-remove-364.bin
```

## Checkpoint 74: global controller-query and hardware table search `[0x125a0,0x126a4)`

- **260 original-exact, independently source-assembled Hexagon V66 executable bytes** (65 words), file `[0x125a0,0x126a4)` and physical `[0x8be0f5a0,0x8be0f6a4)`, SHA-256 `e30fe3f56f951d6ff1040061cebd6b71bd473c07a71f260bd3d5098b04bf29ee`.
- The function queries a controller record via checked `R2=SGP0`, issues two conditional runtime callbacks, inspects header/mode fields, iterates a platform-record table with `loop0`, and returns through the original `dealloc_return`. LLVM6 assembles **64 of 65 instruction words**; the one privileged `SGP0` read at `0x125ac` uses the existing validated V66 register encoder. The one NOP at `0x12654` is an **intentional hardware-loop delimiter**, not an unresolved privileged placeholder.
- The preceding complete slot-removal routine `[0x12430,0x1259c)` and this code are separated by an actual four-byte linker-fill region `[0x1259c,0x125a0)`. Both outputs are separately and concatenation-byte checked; no padding is counted as program source.
- Cumulative independently generated ADSP RWE source **10,616 executable bytes across 19 disjoint intervals**. Next nonzero firmware material starts at file `0x126a8`, after a four-byte zero gap. Full ADSP image, remaining executable sections and OEM authentication are not reconstructed.
- 160 Rust tests and **305 Python tests** pass using the pinned original and LLVM6. A release build also succeeds. No original opcode words are imported from ADSP firmware.

```sh
python3 -S tools/adsp/rwe-next-controller-global-query-complete.py \
 --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
 --output /tmp/adsp-global-controller-query-260.bin
```

## Checkpoint 75: partial controller codec format selector `[0x126a8,0x126dc)`

- Independently assembled **52 exact executable bytes**, file `[0x126a8,0x126dc)`, physical `[0x8be0f6a8,0x8be0f6dc)`, SHA-256 `2d3b8898a01a9c2307bbc99919af189358dd3599a50354cf20051b35be1c9c2f`.
- The entry reads codec mode data through a GP-backed controller pointer and dispatches an indexed handler through the **external runtime table at `0xf000f9bc`**, not an inline embedded opcode table. All 13 instruction words are directly assembled from LLVM6 source; `:mem_noshuf` is necessary for the original memory-operand ordering. No privileged substitutions or parse-marker patches are used.
- The preceding complete query `[0x125a0,0x126a4)` and this entry are separated by four original zero linker-fill bytes `[0x126a4,0x126a8)`, excluded from code counts. The downstream switch/case bodies **are not yet reconstructed**, so this is a partial function.
- Cumulative source-built ADSP RWE code: **10,668 bytes in 20 disjoint intervals**. Next uncovered code starts at `0x126dc`. Full ADSP image, unrelated executable sections and OEM authentication remain incomplete.
- 160 Rust tests and **308 Python tests** pass with LLVM6 and the pinned reference, along with the release build.

```sh
python3 -S tools/adsp/rwe-next-controller-format-selector-prefix.py \
 --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
 --output /tmp/adsp-format-selector-prefix-52.bin
```

## Checkpoint 76: initial format-specific controller cases `[0x126dc,0x12764)`

- Added **136 source-assembled Hexagon V66 executable bytes** (34 words), file `[0x126dc,0x12764)`, physical `[0x8be0f6dc,0x8be0f764)`, SHA-256 `7d66ba2456d4b1f738cda688400df3bdc39c41d74324455d4e3f8950da66411c`.
- This is the first five cases of the codec format selector: platform callback invocation, source-authored format-string addresses, predicate-guarded mode selection, and a nested indirect jump via the **external** `0xf000f9f4` handler table. All 34 words are assembled directly by LLVM6, without privileged substitutions or packet-marker correction.
- Adjacent to the 52-byte selector entry, so the selector now has **188 exact source-built bytes** `[0x126a8,0x12764)` but the function is **not yet complete**. Next missing code at `0x12764`.
- Cumulative independently source-built ADSP RWE executable code: **10,804 bytes across 20 disjoint intervals**. Original OEM authorization and unimplemented executable/data sections remain outside the generator.
- With pinned source inputs configured, **160 Rust** and **311 Python** tests pass and the release build succeeds.

```sh
python3 -S tools/adsp/rwe-next-controller-format-selector-cases-a.py \
 --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
 --output /tmp/adsp-format-cases-a-136.bin
```

## Checkpoint 77: controller codec format table validation and return `[0x12764,0x127c8)`

- Added **100 independently source-assembled, byte-identical Hexagon V66 executable bytes** (25 instruction words), original ADSP file `[0x12764,0x127c8)`, mapped RWE physical `[0x8be0f764,0x8be0f7c8)`, SHA-256 `02ee978612ea82dcc295b49f80db350d7fb44fbd5ec35c7e5933772a68c10e1d`.
- The continuation handles two codec-specific platform dispatches and a bounded `R2=and(R3,#7)` index into the `0xe0`-byte controller record table, checks its magic identifier `0xbeefbeef`, logs and copies a field into the stack frame, and restores the caller's registers through the original `dealloc_return`.
- All 25 words assemble **directly with LLVM6**: no original OEM opcodes, privileged substitutes, raw `.word` instructions, or packet-bit patching. Keeping `r2=and(r3,#7)` in the same packet as its extender and the immediate comparison value, and spelling `r2=add(r16,mpyi(#0xe0,r2))` with the immediate first, are required to match the reference's V66 packets. Both forms are verified by regression tests.
- Adjacent to the previously reconstructed selector entry and five case paths: contiguous partial function now `[0x126a8,0x127c8)` = **288 exact executable bytes**. More external-table destinations after `0x127c8` remain unimplemented; this function is still partial, despite the completed return on one path.
- Cumulative **10,904 source-assembled executable bytes** over 20 disjoint ADSP RWE regions. The firmware ELF layout, separate OEM signing chain, data and zero-fill regions are not counted as reconstructed code.
- Full offline validation: **160 Rust tests and 314 Python tests pass**, full release build, independent Rizin V66 decoding, wrong-revision and corrupted-code rejection. Original ADSP reference checksum is pinned and verified.

```sh
python3 -S tools/adsp/rwe-next-controller-format-selector-cases-b.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-format-cases-b-100.bin
```

## Checkpoint 78: conditional codec mode paths and shared controller return `[0x127c8,0x1289c)`

- Reconstructed **212 independently LLVM6-source-assembled, byte-identical Hexagon V66 executable bytes** (53 instruction words), ADSP file `[0x127c8,0x1289c)` and physical `[0x8be0f7c8,0x8be0f89c)`, SHA-256 `d552ae58f3ec66748b16d57343ac75364b3d851e21c5bf637661567d97c79343`.
- Distinct codec-format branches dispatch through an external platform formatting callback; `p0.new` comparisons to `0x62`, `0x61`, `0x60`, and `0xbf` are preserved, including branch hints. The shared suffix calculates a `0xe0`-byte controller-record offset, publishes four fields to stack slots and returns via the original `dealloc_return`.
- All 53 instructions **assemble directly with LLVM6** using Hexagon packet mnemonics: no privileged substitutions, bitfield patches, or original OEM machine-word arrays. Rizin V66 independently decodes all 53 source-built words. Combine this section with the prior `[0x126a8,0x127c8)` source to verify **500 contiguous exact bytes** through this return. The function has further original case bodies at `0x1289c` and later; not yet complete.
- Cumulative independently source-rebuilt ADSP RWE executable code is **11,116 bytes in 20 disjoint intervals**. External handler tables, diagnostic strings, linker fill and original authentication are not part of these code counts.
- Full validation: **160 Rust tests, 317 Python tests**, release build, exact pinned-reference comparisons, wrong-revision and deliberate-byte-corruption test, and independent Rizin disassembly pass.

```sh
python3 -S tools/adsp/rwe-next-controller-format-selector-cases-c.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-format-cases-c-212.bin
```

## Checkpoint 79: 18 codec-format report case stubs `[0x1289c,0x129bc)`

- Independently source-assembled **288 byte-identical ADSP Hexagon V66 executable bytes** (72 instruction words), original file `[0x1289c,0x129bc)`, mapped physical `[0x8be0f89c,0x8be0f9bc)`, SHA-256 `19d909acd9d466c8b628f889cc8bf4720e11d5e1f69afc8d7d86660750ff5e84`.
- Eighteen source-authored diagnostic cases each call the same platform callback (`0x8be11438` mapped physical), put a distinct named diagnostic-string address into R0 via its instruction extender and branch to the shared callback (`0x8be0f860`). Every source-authored four-word sequence matches exactly with LLVM6. Messages themselves are **external static data**, not emitted or called independent executable source.
- Entire controller format-selector function code `[0x126a8,0x129bc)` is now **788 contiguous independently assembled bytes**, with the original return and all format-specific stubs. Tests rebuild its five adjacent parts and compare them as one blob.
- The following `[0x129bc,0x12a50)` is a **non-instruction 37-entry pointer table** indexed by the earlier `r2=memw(r2<<#2+##0xf000f9f4)` instruction; do NOT disassemble those 148 bytes as executable V66 source. The subsequent bytes contain error strings and metadata. Original firmware authentication is not independently generated.
- Cumulative independently reconstructed ADSP RWE code **11,404 bytes over 20 disjoint intervals**. The adjacent pointer table contributes **zero** executable source bytes.
- Verified reference SHA-256, zero mismatches, 160 Rust tests, **320 Python tests**, release build, independent Rizin decoding, and corrupted-byte/revision rejection.

```sh
python3 -S tools/adsp/rwe-next-controller-format-selector-reports.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-format-reports-288.bin
```

## Checkpoint 80: source-derived 37-entry codec handler pointer table `[0x129bc,0x12a50)`

- Independently regenerated **148 byte-identical non-executable DATA bytes** for the 37-entry codec-format virtual dispatch table, ADSP file `[0x129bc,0x12a50)`, virtual handlers in `[0xf000f6dc,0xf000f9ac]`, SHA-256 `ed0b75feae45d904044b73c64dc7bd3e1ba350e93a35cb073c909065b838958f`.
- Each entry is generated from the source-authored case file offset and the ELF executable `PT_LOAD` virtual mapping: `virtual_address = 0xf0003000 + (case_file_offset - 0x6000)`. Validated all 37 aligned targets lie in the rebuilt selector code and preserve repeated defaults. Unsupported/synthetic addresses are rejected; wrong revision and intentional corruption tests pass. No original OEM data slices are used for generation.
- This data follows the **788-byte completely source-assembled executable selector** `[0x126a8,0x129bc)` directly. Combined code + structured pointer table `[0x126a8,0x12a50)` now accounts for 936 reference-exact bytes, **788 executable + 148 data**. The adjacent diagnostic strings from `0x12a50` onward are separate original non-executable data, not source reconstructed here.
- **Executable coverage unchanged: 11,404 Hexagon instruction bytes over 20 disjoint intervals.** 148 additional bytes are explicitly classified as authored handler-pointer metadata and must never be added to the executable count.
- Full offline validation: 160 Rust tests, **323 Python tests**, release build and reference comparison pass. Existing LLVM6/Rizin checks for preceding code remain valid.

```sh
python3 -S tools/adsp/rwe-next-controller-format-handler-table.py \
  --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-format-table-148.bin
```

## Checkpoint 81: controller record lookup and allocator `[0x12e40,0x12f50)`

- Reconstructed **272 independently generated, byte-identical Hexagon V66 executable bytes** (68 words) at original ADSP file `[0x12e40,0x12f50)`, mapped physical `[0x8be0fe40,0x8be0ff50)`, SHA-256 `637c4b1da9569ef108bd2b0b000378155007e26adb2bde1877416552919dc711`.
- This is **a separate, disjoint function** after the codec handler-pointer table, the reference's original diagnostic strings `[0x12a50,0x12e35)` and zero alignment `[0x12e35,0x12e40)`. The diagnostic string bytes and fill are **not** counted as reconstructed program code.
- The function follows controller records via a GP-relative linked list, matches a pair of metadata fields, allocates and initializes a new record if needed, publishes the index and invokes platform callbacks. It returns through the original `dealloc_return` at `0x12f4c`.
- LLVM6 independently assembles 67 instruction words. The one V66-only `R2=SGP0` read at `0x12e50` is produced by a checked named-register encoder after verifying a single expected LLVM6 NOP sentinel; no OEM machine words are copied into generated bytes. Instruction reordering of a source packet of zero register initializers was avoided by placing `r7=#0` first in the source (no machine-code parse-bit patching required). Rizin V66 decodes all 68 words from the source-built image.
- **Source-built ADSP RWE executable: 11,676 bytes across 21 disjoint intervals**. Plus 148 bytes of independently generated codec handler *data* table from checkpoint 80; metadata is not in executable totals. The remaining executable and OEM-authenticated ADSP image are incomplete.
- Exact pinned-reference comparison, 160 Rust tests, **326 Python tests**, release build and deliberate wrong-revision/corruption rejection pass.

```sh
python3 -S tools/adsp/rwe-next-controller-record-lookup.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-record-lookup-272.bin
```

## Checkpoint 82: indexed controller record selection and normal return `[0x12f50,0x13084)`

- Added **308 byte-identical Hexagon V66 executable bytes** (77 instruction words), ADSP file `[0x12f50,0x13084)`, mapped physical `[0x8be0ff50,0x8be10084)`, SHA-256 `7f93ac885a941e8686adb6cba39a95d0a7c98deedbff48fc59d30ea0cf04d301`.
- The routine checks an index against the controller's GP-backed table, maps 0x20-byte records, checks the metadata class/marker, updates the per-record active state and invokes the platform callback. An inner record walk handles bitmap/status flags and the function normally returns via `dealloc_return` at file `0x13080`.
- LLVM6 directly assembles 76 of 77 original instruction words. One privileged `R2=SGP0` read at `0x12f68` uses the existing typed V66 encoder after validating a NOP sentinel. The unusual four-register packet initializes R7/R5/R20/R0 in exactly the original order from mnemonics (no reference opcode copying or parse-marker patches).
- The function has **an additional error/recovery continuation after `0x13084`** not included in this checkpoint; do not call the full function complete yet. Adjacent to the prior 272-byte record-list initializer, the independently rebuilt run `[0x12e40,0x13084)` is **580 consecutive executable bytes**.
- **Total source-authored executable ADSP RWE code: 11,984 bytes across 21 disjoint intervals**. The 148 bytes of selector pointer-table DATA generated in checkpoint 80 remain separate. OEM certificates, calibration and remaining executable content are not rebuilt or signed.
- Validation: pinned-reference byte comparison, **160 Rust tests and 329 Python tests**, release build, deterministic source assembler, SGP0 typed encoding tests, and deliberate corrupted-byte/revision tests all pass.

```sh
python3 -S tools/adsp/rwe-next-controller-record-select.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-record-select-308.bin
```

## Checkpoint 83: controller record-recovery/error continuation `[0x13084,0x130c4)`

- Reconstructed **64 byte-identical Hexagon V66 executable bytes** (16 words), original file `[0x13084,0x130c4)`, mapped physical `[0x8be10084,0x8be100c4)`, SHA-256 `cdba03a997337aaa3178e4bd36368cb1954bb07d83f766abd4f2705a335ccbae`.
- The error branch clears the selected record's active flag, updates the queue pointer, executes `k0lock`, initializes the error record fields and invokes a controller cleanup service. The 12 bytes `[0x130c4,0x130d0)` immediately afterwards are **file-backed zero fill**, not reconstructed instructions; the next unrelated function begins at `0x130d0`.
- LLVM6 directly assembles **15** of the 16 executable words; the one privileged V66 `k0lock` at `0x1308c` is produced through `KernelZeroLock.LOCK` after checking the original source-assembled placeholder. No original OEM opcode bytes or signed images are copied into the emitter.
- Combining adjacent source-built fragments `[0x12e40,0x12f50)`, `[0x12f50,0x13084)`, and `[0x13084,0x130c4)` produces **644 consecutive byte-identical code bytes**. The normal return and error/recovery tail are both represented; the trailing zero fill remains separate and uncounted.
- **Total independently generated ADSP executable code: 12,048 bytes across 21 disjoint intervals**, plus the independent 148-byte controller handler-table *data* from checkpoint 80.
- Pinned reference comparison, **160 Rust tests and 332 Python tests**, full release build, K0 lock named-encoder validation, wrong-revision and deliberate corruption tests pass. No hardware behavior/MBNv6 signature acceptance is claimed.

```sh
python3 -S tools/adsp/rwe-next-controller-record-recovery.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-record-recovery-64.bin
```

## Checkpoint 84: second indexed controller record query, ordinary return `[0x130d0,0x131e0)`

- Reconstructed **272 byte-identical Hexagon V66 executable bytes** (68 instruction words) at ADSP file `[0x130d0,0x131e0)`, mapped physical `[0x8be100d0,0x8be101e0)`, SHA-256 `856ca4658b230473719605bb2c59749d1958e0d3b3727a3135dc1c68bd7c2486`.
- The function walks a GP-backed controller-record index table, validates record types/flags and calls platform logging and update functions. It handles multiple error branches with the correct negative branch targets, bit positions, `.new` predicate dependencies, record-type fallbacks and the original ordinary `dealloc_return` at file `0x131dc`.
- It begins after **12 reference zero-filled alignment bytes** `[0x130c4,0x130d0)` that are verified but **not counted** as executable source. The additional linked-list/error continuation begins at `0x131e0` and will be reconstructed separately; this is a normal-return path, not yet a whole-function claim.
- LLVM6 source-assembles **67 instruction words**; the one `R2=SGP0` at file `0x130e0` uses the existing checked V66 encoder after a validated NOP placeholder. No machine-code word imports or packet-bit patches were needed. Predicate branch targets, including back branches to `0x1312c`, are explicitly named in the source.
- Cumulative **12,320 independently reconstructed Hexagon executable bytes across 22 disjoint regions**. The previously authored 148-byte handler pointer table is **data**, not code. Many further ADSP segments and authenticated OEM inputs remain unreconstructed.
- Full verification passed: **160 Rust tests, 335 Python tests**, release build, exact reference byte comparison and deterministic corruption/wrong-revision rejection.

```sh
python3 -S tools/adsp/rwe-next-controller-record-query.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-record-query-272.bin
```

## Checkpoint 85: indexed controller-record query linked-list continuation `[0x131e0,0x13250)`

- Independently reassembled **112 exact Hexagon V66 executable bytes** (28 instruction words), ADSP file `[0x131e0,0x13250)`, physical `[0x8be101e0,0x8be10250)`, SHA-256 `94d39659f8a662a12ff24a66a2d517f6bed97e48db0f2f94f780222a29a5ebdd`.
- The recovery path searches a linked list ordered by the controller record's packed tag, inserts the descriptor at the correct position or at the head, applies a privileged K0 lock, notifies the controller and writes its status fields. Both conditional insertion branches use exactly the original packet-side memory stores.
- LLVM6 independently assembles 27 ordinary instruction words. One V66-only `k0lock` at `0x13214` uses the named `KernelZeroLock.LOCK` encoder after validating the source NOP sentinel. No raw OEM machine code or signed image bytes are embedded in the source.
- Combined with checkpoint 84, the indexed record-query normal return and recovery tail `[0x130d0,0x13250)` form **384 consecutive byte-identical executable bytes**. Their three earlier alignment words `[0x130c4,0x130d0)` remain classified as non-executable file-backed zeros; they are not silently added to program coverage. The next independent function begins exactly at file offset `0x13250`.
- **Total ADSP RWE independently generated executable code: 12,432 bytes across 22 disjoint intervals.** The 148-byte structured codec pointer table remains separately accounted metadata. Much larger code, data and OEM-authenticated ELF sections still need rebuilding.
- Validation: **160 Rust tests, 338 Python tests**, full release build, reference byte-for-byte comparison and corrupted-code/wrong-revision rejection passed.

```sh
python3 -S tools/adsp/rwe-next-controller-record-query-tail.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-record-query-tail-112.bin
```

## Checkpoint 86: controller record reset and linked-descriptor cleanup `[0x13250,0x13390)`

- Authored **320 byte-identical Hexagon V66 executable bytes** at ADSP file `[0x13250,0x13390)`, physical `[0x8be10250,0x8be10390)`, SHA-256 `ebeea99e92e22593b4713dc8ca981ddd693ce438cdb6c579264bc4b8d7517369`.
- The routine validates a controller record ID, clears active record pointers, walks a linked list with field updates, sends deactivation/cleanup callbacks, and returns through the original `dealloc_return`. It continues directly after the prior indexed query and recovery routines; the source-built `[0x130d0,0x13390)` **704-byte interval** was checked as one artifact.
- LLVM6 assembles 78 instruction words from explicit Hexagon packetized source, including the original `.new` predicates and branch targets. Two privileged V66-only words use named checked encoder operations (`R2=SGP0` at `0x1325c` and `k0lock` at `0x132f4`) after verifying independent assembler NOP sentinels; no OEM instruction words are imported.
- **Total ADSP RWE source-derived executable: 12,752 bytes across 22 disjoint intervals**, plus 148 independently emitted **data**, not code, bytes for the codec pointer table. Large ADSP ELF segments and OEM-authenticated file contents remain unreconstructed.
- Pinned SHA-256 reference comparison: zero byte mismatches; full offline Rust tests, Python tests and release build passed. Deliberately corrupted output and wrong reference revision are rejected.

```sh
python3 -S tools/adsp/rwe-next-controller-record-reset.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-record-reset-320.bin
```

## Checkpoint 87: controller record snapshot and conditional lookup `[0x13390,0x13478)`

- Independently rebuilt **232 byte-identical ADSP Hexagon V66 executable bytes** (58 instruction words), file offsets `[0x13390,0x13478)`, physical `[0x8be10390,0x8be10478)`, SHA-256 `88052358a9af55e97fd8e43d19e9dd770ffb4a42f35589b327ca738a9342c0e2`.
- The routine checks a controller record's identifying pair, walks the linked list, handles four record-class variants and buffer-status fallbacks, and returns the selected identifier or an empty pair through the original `dealloc_return`.
- LLVM6 independently assembles **57 instruction words**. The privileged V66 `R2=SGP0` at file `0x1339c` uses an existing checked register encoder after a verified source NOP placeholder. The `R17:16=combine(#0,#0)` at `0x133dc` requires a **single validated packet continuation marker** (`packet_continuation`) after LLVM6 assembly; its operand bits remain unchanged. No reference instruction bytes are imported.
- Source code preserves the exact packet parse boundary around the following `immext`, GP-relative loads, and conditional `.new` branches. Rizin independently decodes the resulting 58 Hexagon V66 words. The whole `[0x13250,0x13478)` controller-record reset + snapshot span is now byte-identical, and the function ends at `0x13478`.
- Cumulative ADSP RWE **source-derived executable bytes: 12,984** across **22 disjoint** intervals, plus the separate 148-byte source-generated controller handler *data table* from checkpoint 80; linker fill, diagnostic strings and original OEM authentication are excluded.
- Exact pinned `adsp.mbn` SHA-256 `4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e`. **160 Rust tests and 344 Python tests** pass with LLVM6 and reference configured. Release build, independently decoded Rizin output, corruption/wrong-revision rejection and Git bundle verification pass. This is executable instruction-byte reconstruction, not independent source-level behavioral proof or a complete/flashable ADSP image.

```sh
python3 -S tools/adsp/rwe-next-controller-record-snapshot.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-record-snapshot-232.bin
```

## Checkpoint 88: controller record-detach initial lookup `[0x13480,0x13514)`

- Added **148 byte-identical executable bytes** (37 Hexagon V66 words) at ADSP file `[0x13480,0x13514)`, physical `[0x8be10480,0x8be10514)`, SHA-256 `ef228e884d45f0f18550e17e72e51e9a528a5af477cee7689019306a397115e8`.
- Starts after eight bytes of reference zero linker fill `[0x13478,0x13480)`, which remain **excluded** from independently authored executable coverage. The routine is **partial**: controller-record field processing and its normal/recovery return continue after `0x13514`.
- Explicit Hexagon V66 packets implement the record index check, lookup of GP-backed controller globals, linked-list traversal, and three error branches. LLVM6 assembles 36 of 37 words; the one V66-only `R2=SGP0` at `0x1348c` uses a checked named operand encoder after verifying a NOP sentinel. No source-derived instructions are copied as raw OEM words.
- Total source-built ADSP RWE executable: **13,132 bytes across 23 disjoint intervals**, plus 148 bytes of source-generated codec handler *data* from checkpoint 80. File linker fill and diagnostic text remain outside code totals.
- The pinned ADSP image SHA is `4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e`. Byte comparison, revision/corruption safety tests, independent LLVM6 assembly, and regression suites pass. This is a partial function and not independently OEM-authenticated firmware.

```sh
python3 -S tools/adsp/rwe-next-controller-record-detach-prefix.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-record-detach-prefix-148.bin
```

## Checkpoint 89: complete controller detach/list cleanup `[0x13514,0x13610)`

- Reconstructed **252 byte-identical Hexagon V66 executable bytes**, 63 instruction words, physical `[0x8be10514,0x8be10610)`, SHA-256 `a7ed38a832b193b393c7459f9d0c9b318506c93cb9cdc82aa6db442dac426b77`.
- The continuation checks the record class, unlinks it from its GP-backed list, clears its descriptor metadata, locks with K0 and walks a controller-linked list. It updates hardware-status words or invokes the deactivation callback as required, then returns through the original `dealloc_return` at file `0x1360c`.
- LLVM6 emits 62 ordinary instruction words. One privileged `k0lock` at `0x13590` is inserted via the strictly checked `KernelZeroLock.LOCK` encoding after validating the original source NOP placeholder. One source-assembled jump at `0x13538` needs the Hexagon packet-continuation parse marker to match the original interleaving of list unlinking; operand bits are unchanged. No OEM machine instruction is copied into generated source.
- Combined with checkpoint 88's independently emitted 148-byte prefix, the **complete 400-byte function `[0x13480,0x13610)` is byte-identical**. Its preceding 8-byte file-backed zero-fill remains excluded from code accounting. The next distinct routine begins at `0x13610`.
- **Total independently reconstructed ADSP RWE executable code: 13,384 bytes across 23 disjoint intervals**, plus 148 bytes separately authored handler-pointer data. The rest of the ADSP executable and OEM-authenticated image remain incomplete.
- Pinned-reference exact comparison, independent LLVM6 assembly/Rizin decoding, deliberate wrong-revision/corrupt output rejection, full Rust and Python regression suites and release build pass.

```sh
python3 -S tools/adsp/rwe-next-controller-record-detach-tail.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-record-detach-tail-252.bin
```

## Checkpoint 90: complete controller record purge `[0x13610,0x13744)`

- Added **308 source-assembled, byte-identical Hexagon V66 executable bytes** (77 instruction words), file `[0x13610,0x13744)`, physical `[0x8be10610,0x8be10744)`, SHA-256 `427aee6e0d17d12c367003be00ae6350464b5b29cfec1429a67fb09d07033fa7`.
- The routine iterates the controller's GP-linked record list, checks class ownership, zeroes selected record fields, acquires/releases K0 around dependent descriptor deactivation, and notifies downstream controller callbacks. Its original `dealloc_return` occurs at `0x13740`.
- LLVM6 directly assembles **73 ordinary words**. Four privileged V66 operations are generated through checked encoders at verified source NOP placeholders: `R2=SGP0` at `0x13624`, `k0unlock` at `0x13650`, and `k0lock` at `0x136a0` and `0x13704`. The source explicitly includes the intentional Hexagon NOP pair at `[0x136c8,0x136d0)` with distinct continuation/termination parse bits; these are real executed packet slots, not fill or unknown source. Branches into `check_next` preserve the original target `0x1365c`.
- Together with checkpoints 88/89, the controller-record detach and purge `[0x13480,0x13744)` form **708 byte-identical executable bytes**. The **eight bytes before** `[0x13478,0x13480)` and the **twelve bytes after** `[0x13744,0x13750)` are verified file-backed linker fill, **excluded** from code coverage.
- Cumulative ADSP RWE independently reconstructed executable coverage: **13,692 bytes across 23 disjoint intervals**, plus 148 source-generated handler-pointer table *data* bytes. The full ADSP ELF/MBNv6 and OEM authentication are still incomplete.
- Pinned-reference byte comparison, LLVM6/Rizin independent decoding, deliberate corrupt-output and wrong-revision rejection, full Rust/Python suites and release build pass.

```sh
python3 -S tools/adsp/rwe-next-controller-record-purge.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-record-purge-308.bin
```

## Checkpoint 91: linked controller-record remove `[0x13750,0x13828)`

- Rebuilt **216 byte-identical executable ADSP V66 bytes** (54 instructions), file `[0x13750,0x13828)`, physical `[0x8be10750,0x8be10828)`, SHA-256 `35e4f47f86d5ce46d2fa1f76fc41196640598f2d42c3bfbaaf480c5cf8958bdb`.
- The routine traverses the global controller record list and per-record sublists; it unlinks an owned entry, updates status fields, marks the hardware slot for cleanup and invokes the controller notification callback. The original `dealloc_return` is preserved at `0x13824`.
- LLVM6 independently assembles all **54 words**. The external `call` at `0x13760` shares its original packet with following address-extender/memory operations; the checked `packet_continuation` changes only the parse marker. A labeled retry branch was corrected to the original `walk_owners` test, with no reference-code word imported.
- The **twelve preceding bytes `[0x13744,0x13750)` are file-backed zero fill** and are excluded from source-derived code. This starts a new disjoint interval, while record-detach + record-purge `[0x13480,0x13744)` remains separately covered.
- **Total source-assembled ADSP RWE executable code: 13,908 bytes across 24 disjoint intervals**, plus 148 source-generated handler-pointer DATA bytes. Full ADSP executable, OEM signatures and on-device semantics are not yet complete.
- Pinned-image comparison, LLVM6/Rizin independent decoding, corruption/wrong-revision rejection, Rust/Python regression suites and release build pass.

```sh
python3 -S tools/adsp/rwe-next-controller-linked-record-remove.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-linked-record-remove-216.bin
```

## Checkpoint 92: controller record-purge wrapper `[0x13828,0x13844)`

- Authored **28 byte-identical Hexagon V66 executable bytes** (seven words), file `[0x13828,0x13844)`, physical `[0x8be10828,0x8be10844)`, SHA-256 `79390e13a1a4f0e9e16d845c84b24805b5d5441bec6e7fb80940564652763eda`.
- The wrapper saves an empty frame, reads controller `SGP0`, traverses the shared pointer fields, extracts a seven-bit record class and calls the previously reconstructed record purge function at file `0x13610`. The original `dealloc_return` is in the final duplex packet.
- LLVM6 assembles six words directly. The V66 `R2=SGP0` uses the checked source placeholder. Placing `extractu(...)` and `call controller_record_purge` **in the same source packet** recovers the exact branch-relative displacement and parse markers; no opcode-bit patch is needed.
- This extends the adjacent source-built interval `[0x13750,0x13844)` to **244 exact executable bytes**. The next twelve original bytes `[0x13844,0x13850)` are zero-filled linker alignment, **not independently reconstructed code**.
- **Total source-built ADSP RWE executable: 13,936 bytes across 24 disjoint intervals**, plus 148 source-generated handler-pointer data bytes. The remaining executable and authenticated image are still incomplete.
- Pinned-reference revision verification, mutation detection, LLVM6/Rizin cross-check, 160 Rust tests, Python tests, release build and Git bundle integrity pass.

```sh
python3 -S tools/adsp/rwe-next-controller-purge-wrapper.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-purge-wrapper-28.bin
```

## Checkpoint 93: controller runtime-cycle and thread-start wrapper `[0x13850,0x138ec)`

- Rebuilt **156 byte-identical Hexagon V66 executable bytes** (39 instruction words), ADSP file `[0x13850,0x138ec)`, physical `[0x8be10850,0x8be108ec)`, SHA-256 `c63e8c8a0bbbfa890462553634551a8a5a193f60c59171fb02dd936bf9ce7f90`.
- The wrapper synchronizes the controller cycle counter, writes timing registers, cleans an instruction/data-cache line, starts the selected thread, performs K0-protected callbacks and records post-startup cycle deltas. It branches to the original controller finalizer with a restored frame; following four bytes are file-backed zero-fill, **not code**.
- LLVM6 source-assembles **34 ordinary instructions**, including `dccleaninva(r0)` and the combined memory operations under `:mem_noshuf`. Five V66-only instructions are individually encoded from checked operands: paired `PCYCLE` reads into `R1:0` and `R3:2`, `start(R0)`, `k0lock`, and `k0unlock`. Two instruction words at offsets `0x1386c` and `0x13878` use only validated `packet_continuation` parse changes; none of their operand bits is altered or replaced with OEM machine code.
- This function begins after **twelve bytes `[0x13844,0x13850)` of zero linker fill**. **Cumulative independently source-built ADSP RWE code: 14,092 bytes across 25 disjoint intervals**, plus separately source-authored 148-byte format-handler data. No OEM-authenticated image signing or hardware behavior is claimed.
- Pinned ADSP SHA-256 verified, byte comparison exact, independent LLVM6/Rizin decode, deliberately corrupted output/wrong revision checks, Rust/Python test suites, release build and Git bundle verification pass.

```sh
python3 -S tools/adsp/rwe-next-controller-runtime-clock.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-runtime-clock-156.bin
```

## Checkpoint 94: controller PMU and SYSCFG restoration `[0x138f0,0x139c8)`

- Rebuilt **216 byte-identical Hexagon V66 executable bytes** (54 instruction words), file `[0x138f0,0x139c8)`, physical `[0x8be108f0,0x8be109c8)`, SHA-256 `84387939f67494b86139a563b7d501212253f476d5a4ac507289fe247d49bbde`.
- The routine uses two LL/SC (`memw_locked`) sequences to preserve controller PMU/SYSCFG state around a register-bank restore, clears and sets SYSCFG bit 9 with `isync`, loads the saved performance counters and returns via `jumpr lr` on either path.
- LLVM6 source-assembles **38 ordinary packet words**, including atomic memory operations and branch retry paths. A checked V66 register encoder supplies **16 original system-register transfers**: read/write SYSCFG, writes to PMUCNT0..3, S44..47, PMUCFG, S55, PMUEVTCFG and S54. Every source NOP placeholder is validated before substitution; no reference opcode is imported. New named `SystemRegister` enum entries are PMUCNT0..3, S44, S45, S46 and S55.
- The four bytes `[0x138ec,0x138f0)` preceding this function are original zero linker fill, excluded from executable coverage. The function's two return instructions end at `0x139c8`, followed by a separate cycle-counter helper.
- **Total reconstructed ADSP RWE executable bytes: 14,308 across 26 disjoint intervals**, plus 148 source-authored codec-handler pointer data bytes. Most of ADSP and OEM signature/cert content remain unreconstructed.
- Pinned ADSP revision and byte equality, LLVM6 assembler and independent Rizin decoder, corrupted-reference/output rejection, complete Rust and Python tests, release build and Git bundle verification pass.

```sh
python3 -S tools/adsp/rwe-next-controller-performance-restore.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-pmu-restore-216.bin
```

## Checkpoint 95: controller cycle-counter helper `[0x139c8,0x139dc)`

- Added **20 byte-identical executable bytes** (five V66 words), original ADSP file `[0x139c8,0x139dc)`, physical `[0x8be109c8,0x8be109dc)`, SHA-256 `f67f9c6f8fc83d81141176f92e4254305f03ee9b70f7d971b26a0ecc82c02a7d`.
- The leaf reads the paired `PCYCLE` counter into `R1:0`, loads the GP-backed 64-bit saved accumulator, adds them, and tail-jumps into the earlier source-built controller cycle routine.
- LLVM6 source-assembles four words. The V66 `R1:0=PCYCLE` read is constructed from the strictly checked paired-system-register operands (`R1:0`, S31:30) after validating its single NOP placeholder. No copied OEM word or packet-marker patch is needed.
- Contiguous with checkpoint 94, the PMU restore plus clock leaf `[0x138f0,0x139dc)` now contains 236 exact executable bytes. The **four bytes following `[0x139dc,0x139e0)` are reference linker fill**, not code. Next routine begins at `0x139e0`.
- **Total independently reconstructed ADSP RWE executable code: 14,328 bytes across 26 disjoint intervals**, plus 148 authored pointer-table data bytes. Large ADSP executable regions, semantics and OEM authentication remain incomplete.
- Pinned-reference comparison, independent Hexagon LLVM6/Rizin decoding, wrong-revision/corrupt-output tests, full Rust and Python tests, release build and Git bundle verification pass.

```sh
python3 -S tools/adsp/rwe-next-controller-cycle-helper.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-cycle-helper-20.bin
```

## Checkpoint 96: controller-mode dispatch prefix `[0x139e0,0x13a2c)`

- Added **76 byte-identical source-rebuilt Hexagon V66 code bytes** (19 instruction words), original ADSP file `[0x139e0,0x13a2c)`, physical `[0x8be109e0,0x8be10a2c)`, SHA-256 `0109d17552424ed2d70c6c53150c4fa6b82aa91cf69c54ec40e8a41f58bd5d58`.
- First controller-mode routine validates the input mode and shared controller pointer, has the original early return, and branches into the later clock and atomic-lock paths (still unreconstructed at this checkpoint).
- LLVM6 source-assembles 18 ordinary packet words and groups the `R18=#0`/`P0` test in the original common packet, preserving the branch target. The privileged V66 `R2=SGP0` instruction is independently generated with the checked system-register encoder at a validated LLVM NOP placeholder.
- Three source-assembled instruction words require **only checked packet parse-field continuation** (at `0x139e0`, `0x139f8`, `0x13a04`); no operand or branch target bits are patched. Source contains neither OEM binary words nor `.incbin`.
- The four bytes `[0x139dc,0x139e0)` before this new function are linker fill, not new executable code. **Total independently source-built ADSP RWE code is now 14,404 bytes across 27 disjoint intervals**, plus 148 separately reconstructed pointer-table data bytes.
- Pinned-reference SHA-256 check, exact generated-vs-reference bytes, wrong-revision and corruption rejection, Rust/Python regression tests and offline release build pass. Further paths start at `0x13a2c`.

```sh
python3 -S tools/adsp/rwe-next-controller-mode-prefix.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-controller-mode-prefix-76.bin
```

## Checkpoint 97: controller-mode cycle snapshot and startup trap `[0x13a2c,0x13a7c)`

- Added **80 byte-identical Hexagon V66 executable bytes** (20 instruction words) at ADSP file `[0x13a2c,0x13a7c)`, physical `[0x8be10a2c,0x8be10a7c)`, source SHA-256 checked against pinned reference.
- A checked paired `R1:0=PCYCLE` V66 encoder replaces one validated LLVM NOP. LLVM6 source generates the remaining 19 words with **original packet grouping**, including the `R1:0` timestamp stores and linked failure calls; no operand or packet-marker patches were needed.
- Error trap and controller-snapshot paths branch to the original source-authored physical targets; function is still partial after `0x13a7c`.
- With checkpoint 96, `[0x139e0,0x13a7c)` now includes **156 contiguous, source-built executable bytes**. Total ADSP RWE executable code: **14,484 bytes across 27 disjoint regions**, plus 148 separately authored codec-handler pointer data bytes; OEM signing and remaining ADSP executable are not reconstructed.
- Wrong reference revision and changed output are rejected. Pinned SHA-256 and full exact-byte verification, offline Rust and Python tests, release build and Git bundle verification pass.

```sh
python3 -S tools/adsp/rwe-next-controller-mode-clock.py \
  --llvm-mc "$HEXAGON_LLVM_MC" --reference "$NABU_ADSP_REFERENCE" \
  --output /tmp/adsp-mode-clock-80.bin
```
