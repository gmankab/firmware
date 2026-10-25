# Agent 20-07 — ADSP first-segment tail and early RWE entry

## Actual Git base and reference (IMPORTANT)

- **Requested integrated base:** a checkpoint-50 descendant as published by the integration agent. This is **not available** in the uploads. The checkpoint-50 SHA `b7ed4b587633b9b475a40f5a3ace5cc90db620ff` cannot be verified here.
- **Actual branch base:** Agent 04's `agent-04-checkpoint-32-adsp.bundle`, HEAD `d7cb8b118c275a0458d29bb65dafb82b80613825`, itself based on older `69ebf0d…` history. Branch `agents20/07-adsp-entry-provisional`. This history must be rebased/cherry-picked after the true integrated base is available; **do not claim checkpoint-50 ancestry**.
- **Pinned original ADSP** file `adsp.mbn` from the correctly reconstructed multipart firmware archive, SHA-256 `4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e`, kept privately outside Git.
- Architecture: Hexagon V66 (`e_machine=164`, `e_flags=0x66`), little-endian, mixed packet parse bits, ELF entry `0x8be00000`; first executable `PT_LOAD` file `[0x3000,0x521c)` maps physical `[0x8be00000,0x8be0221c)`.

## Checkpoint 01: complete RX segment tail `[0x51dc,0x521c)`

Reconstructed **64 independently encoded executable bytes**, Hexagon v66 TLB snapshot routine. This completes the earlier Agent 04 controller region `[0x41e0,0x51dc)` up to the first executable `PT_LOAD` end. It reads a privileged S54 register, copies 128 TLB pairs into controller memory, then cleans a set of cache lines with a Hexagon hardware loop, and returns.

- **Source:** `tools/adsp-controller-tlb-snapshot.S` with original packet grouping and symbolic hardware loop labels. LLVM6 assembles 14 of 16 instruction words from mnemonic source (including extenders and loop parse markers). Two unsupported privileged operations use *checked* typed encoders after verifying the exact NOP sentinel location: `R23 = S54` and `R3:2 = tlbr(R0)`.
- A new general checked `tlb_read_pair(destination_even,index_register)` Hexagon encoding models pair constraints, architecture fields and packet termination. It reproduces Agent 04's existing independent `R5:4=tlbr(R3)` as a second test vector.
- A private reference is used only as a comparison oracle and is not embedded in source. The generator rejects reference SHA mismatch, malformed packet/sentinel placement, and unknown operand fields.
- SHA-256 generated 64-byte interval: `b010632bfba034f3a3f61757c130c8bec172c82d14c27cac82ca11800534ce35`. Exact MBN file `[0x51dc,0x521c)`, relative vaddr `[0x21dc,0x221c)`, physical address `[0x8be021dc,0x8be0221c)`.
- Earlier `[0x3044,0x41e0)` executable fragment is **still missing** and cannot be replaced with implicit zeros. The complete ADSP and signing remain unresolved.

## Reproduce

```bash
HEXAGON_LLVM_MC=/path/to/llvm6/bin/llvm-mc
NABU_ADSP_REFERENCE=/private/adsp.mbn
python3 -S tools/adsp-agent20-entry.py --llvm-mc "$HEXAGON_LLVM_MC" \
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
python3 -S tools/adsp-agent20-rwe.py --llvm-mc "$HEXAGON_LLVM_MC" \
  --reference "$NABU_ADSP_REFERENCE" --output /tmp/adsp-rwe-startup-92.bin
```

## Checkpoint 03: controller dispatch `[0x608c,0x60d4)` and 164-byte RWE prefix

Added 72 byte-identical bytes (`[0x608c,0x60d4)`) to the 92-byte RWE startup, making an independently source-assembled **164-byte** packet sequence `[0x6030,0x60d4)`, SHA-256 `835f82782f328e8684869c95902e342a79823acf11b99725a69f6e0d8d61bd10`. Physical address range `[0x8be03030,0x8be030d4)`.

This builds two mode-dispatch predicates, four external Hexagon controller calls with **explicit instruction extenders**, and a handler register pair. LLVM6's default scheduler omitted the four extenders; forcing the original packet form via mnemonic `immext` reconstructs the correct V66 bytes while retaining understandable function names and relocation offsets. The cross-segment external call targets are expressed in *physical* PC-relative terms, not ELF file offsets. The source contains **no reference opcodes**.

The overall ADSP source coverage now includes the 68-byte boot prefix, the 4,156-byte closed controller RX span `[0x41e0,0x521c)`, and a separate 164-byte RWE entry fragment; they are **disjoint**. The next missing RWE instruction is at `0x60d4`; the earlier `[0x6000,0x6030)` metadata is not executable code.

## Checkpoint 04: complete initial RWE controller initialization `[0x6030,0x6148)`

The source-generated RWE interval is now **280 contiguous, byte-identical bytes** from ADSP file `[0x6030,0x6148)` (physical `[0x8be03030,0x8be03148)`), SHA-256 `f270f97637d294721dd1d8de4778f1235e02914862f586ba2895664162934603`.

The final 116 bytes `[0x60d4,0x6148)` write the controller's memory-mapped status, clear SYSCFG bit 4, zero the SSR configuration, perform cache-line cleanup, invoke two platform services and enter the startup trap loop. Four privileged SYSCFG/SSR accesses are field-encoded as **checked V66 operations**, not copied vendor instruction words.

The last packet contains two normal 32-bit scalar immediates and an original one-word **Hexagon duplex** `r0=#24; r1=-1`. LLVM6's automatic packet pairing chose a different and byte-incompatible instruction schedule; the source emitter disables auto-pairing and replaces exactly the final two source-assembled scalar words with a *separately source-assembled duplex instruction* (`adsp-agent20-rwe-duplex.S`). Every source operand and packet position is checked. Both LLVM and Rizin verify the original 29 instruction words of this extension. The diagnostic constants and attached OEM signatures are not included in the regenerated artifact.

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
Hexagon assembly in `tools/adsp-agent20-rwe-frame-init.S`. LLVM6 packetization
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
from packetized source (`tools/adsp-agent20-rwe-runtime-calls.S`): exact named
physical call targets, explicit instruction extenders and cache-line cleaning.
The final 84 bytes (`tools/adsp-agent20-rwe-runtime-registers.S`) contain six
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
Source `tools/adsp-agent20-rwe-register-snapshot.S` models the controller
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
`tools/adsp-agent20-rwe-feature-modes-b.S` checks upper-bit codec prefixes
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
`tools/adsp-agent20-rwe-feature-modes-{c,d}.S` and generated by LLVM6
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
