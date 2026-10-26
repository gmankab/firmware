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

## Checkpoint 22: ADSP controller TLB restore `[0x6c10,0x6c30)`

Added a **32-byte** source-generated Hexagon V66 helper that restores the 128
saved TLB entry pairs from controller SRAM `0xf000cf20`. The LLVM6-authored
mnemonic source in `tools/adsp-agent20-rwe-tlb-restore.S` retains the original
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
python3 -S tools/adsp-agent20-rwe.py --llvm-mc "$HEXAGON_LLVM_MC" \
  --reference "$NABU_ADSP_REFERENCE" --output /tmp/adsp-rwe-3072.bin
```

## Checkpoint 23: ADSP TLB readback and cache clean `[0x6c30,0x6c6c)`

Added a **60-byte** Hexagon V66 helper that reads 128 TLB entry pairs,
persists them into the `0xf000cf20` controller array, and cleans 32 D-cache
lines. `tools/adsp-agent20-rwe-tlb-readback.S` contains source-level
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
and must not be appended to `adsp-agent20-rwe.py` output or counted as a
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

New emitter and verifier: `tools/adsp-agent20-rwe-sparse.py`, independently
reconstructing **only** these 28 bytes; the old contiguous emitter remains
`[0x6030,0x6c6c)` = 3,132 source-built bytes. Their combined executable
coverage is **3,160 bytes in two disjoint spans**, not a new contiguous
`[0x6030,0x1034c)` interval. The next unreconstructed instruction is at
`0x1034c`. Rust 160 tests, Python 162 tests, release build and private ADSP
reference byte comparison pass.

```sh
python3 -S tools/adsp-agent20-rwe-sparse.py --llvm-mc "$HEXAGON_LLVM_MC" \
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
python3 -S tools/adsp-agent20-rwe-sparse.py --llvm-mc "$HEXAGON_LLVM_MC" \
  --reference "$NABU_ADSP_REFERENCE" --output /tmp/adsp-sparse-116.bin
```

## Checkpoint 26: debugger-state/SYSCFG guard `[0x103a4,0x103d0)`

The next **44 exact Hexagon V66 bytes** read `SGP0` and `ISDBEN`, conditionally
clear `SYSCFG` bit 5, issue `isync` and wait for debugger-enable bit 4 to
clear. `tools/adsp-agent20-rwe-next-isdb-guard.S` independently assembles
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
`adsp-agent20-rwe-next-sram-clone.S` verifies status flag bit 21,
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
