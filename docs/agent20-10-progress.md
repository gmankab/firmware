# Agent 20-10 — CDSP later executable segments

## Provenance and prerequisite

- Assigned plan: CDSP later `PT_LOAD` regions, rather than generic image finalization.
- **Intended integration base:** checkpoint-50 SHA `b7ed4b587633b9b475a40f5a3ace5cc90db620ff`, followed by Agent 20-01's verified merged result.
- **Available Git base:** Agent 05 CDSP checkpoint-17 commit `a02618fce8dec9c5e0dcb88c47d0fc6044624b9d`, which descends from older `69ebf0d…`. No checkpoint-50 Git bundle exists among the mounted uploads. Therefore this is a **preparatory portable branch**, not a checkpoint-50 descendant. On handoff, cherry-pick the agent-specific commits onto the correct integrated base; do not blindly merge old `main`.
- Git branch: `agents20/10-cdsp-late-prep`; author and committer: `Codex <codex@openai.com>`.
- CDSP source SHA-256 `febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea`. Original reference stays outside Git.

## Segment analysis

CDSP is little-endian ELF32 Hexagon V66, `e_machine=164`, `e_flags=0x66`. **ELF VMA and physical address are different** for CDSP, so verification always uses file offsets and decoder uses physical addresses.

- CDSP later `R E` segment **file `[0x3b000,0x5475c)`**, VMA `0xfe0ad000`, physical `0x96cad000`, `p_filesz=0x1975c`. Its 104,284 file bytes contain only 50 unique 32-bit values, dominated by 25,707 zero words and 308 `0x7f7f7f7f` words. An embedded `face_process` string occurs at file `0x3b034`. **This is structural metadata and fill, not a contiguous executable routine**, despite the ELF `R E` flag. Rizin can assign misleading mnemonics to small metadata words. Do not count copied or zero metadata as reconstructed code.
- Next distinct `R E` segment **file `[0x55000,0x554b0)`**, VMA `[0xc00c7000,0xc00c74b0)`, physical `[0x96cc7000,0x96cc74b0)` (1,200 file bytes). This is genuine hardware-controller Hexagon code with structured MMIO command sequences.
- Later `RWE` segments: `[0x56000,0x2f1de4)`, `[0x2fe000,0x8f64d0)` and `[0x8f7000,0x904694)`; each has separate mapping. The later two are reserved for future function-aligned reconstructions. The `[0x2fe000,0x2fe000)` read-only `PT_LOAD` has zero file-backed bytes and `p_memsz=0x79000` — do not treat BSS as code.

## Checkpoint 01 — hardware MMIO-enable prologue

- **56 independently generated executable bytes** at `cdsp.mbn` file `[0x55000,0x55038)`; ELF virtual `[0xc00c7000,0xc00c7038)`; physical `[0x96cc7000,0x96cc7038)`.
- Source-level operation: write immediate `7` to MMIO register addresses `0x083a0040`, `0x083a0044`, and `0x083a0048`, issue Hexagon memory `barrier`, then jump to shared controller routine at physical `0x96cc7070` (outside the current fragment). The 14 four-byte words are eight Hexagon **packets**: six two-word extended-immediate assignment/store packets plus the standalone barrier and jump packets (8 packets total).
- `cdsp_late_mmio::encode_extended_word_store_immediate` provides a checked architecture-field constructor with base register, aligned offset, and 32-bit value. Tests reject misaligned/unsupported offsets. Existing checked `cdsp_boot::encode_extended_assignment` and `ScalarPacket::JumpRelative` construct the other packets; the barrier is the independently LLVM-assembled architecture opcode, not an OEM byte array.
- Verify with `cargo run --offline --locked -p firmware-tools --bin cdsp-late-rebuild -- verify /private/cdsp.mbn` and `tools/verify-agent20-10-cdsp-late.py --llvm-mc /private/llvm-mc --rz-asm /private/rz-asm --generated /private/mmio-56.bin --reference /private/cdsp.mbn`, using the staged Ubuntu 24 Rizin `LD_LIBRARY_PATH` when needed.
- The full 1,200-byte short executable segment is **not reconstructed**. Next uncovered code offset is **`0x55038`**; the remaining 1,144 file bytes are not counted as source-generated. The entire CDSP image, authentication and hardware execution remain unverified.

## Merge guidance

Agent 20-10 owns only new `cdsp_late_*` sources, its verification tool, focused CLI and this progress document. Registering the CDSP late module in the shared `devices/xiaomi_nabu/mod.rs` is a single, separate integration change that Agent 20-01 should reconcile, not overwrite. Do not modify Agent 05's CDSP boot/controller code or any other firmware images.

## Checkpoint 02 — shared MMIO counter wait, contiguous 112 bytes

- Independently emitted `[0x55000,0x55070)` = **112 byte-identical Hexagon executable bytes**; ELF VMA `[0xc00c7000,0xc00c7070)`, physical `[0x96cc7000,0x96cc7070)`.
- Added a 56-byte counter-read helper `[0x55038,0x55070)`: preserve R1/R2/R3 in R8/R9/R10, read the monotonic hardware counter at `0x083a2000`, spin until an unsigned counter delta exceeds R2, restore arguments and jump through R13.
- Checked typed encoder `encode_word_load_immediate` handles only verified small aligned offsets, and `encode_wait_if_unsigned_delta_small` encodes the compound `P0=cmp.gtu(R3,R2); if(!P0.new) jump:t` with a narrow negative offset. The latter branches from physical `0x96cc705c` to `0x96cc7054`, the **second** counter load, not the initial snapshot at `0x96cc7050`.
- An independently authored LLVM 6 fixture emits all 28 words and 17 packet groups exactly; Rizin V66 independently decodes the 28 words, their parse markers and operands, and direct comparison to the exact OEM reference passes.
- Reproduce: `cargo run -p firmware-tools --bin cdsp-late-rebuild -- emit-joined /tmp/cdsp-late-112.bin`, `... -- verify-joined /private/cdsp.mbn`, and `python3 tools/verify-agent20-10-cdsp-late.py --joined --llvm-mc /path/to/llvm6/bin/llvm-mc --rz-asm /path/to/rz-asm --generated /tmp/cdsp-late-112.bin --reference /private/cdsp.mbn`.
- Next unimplemented code is at file `0x55070`. Nothing from the metadata-heavy ELF segment `[0x3b000,0x5475c)` is counted as independent executable source. Checkpoint-50 base remains unavailable, so the branch is **provisional**.

## Checkpoint 03 — MMIO controller mode dispatch, contiguous 168 bytes

- `[0x55000,0x550a8)` = **168 exact, independently generated executable bytes**; runtime ELF VMA `[0xc00c7000,0xc00c70a8)`, physical `[0x96cc7000,0x96cc70a8)`.
- Added hardware configuration detector `[0x55070,0x550a8)` (56 bytes): read controller register at `0x01fc8000`, compare its `0x100` and `0x200` masks, dispatch respectively to physical `0x96cc70a8` or `0x96cc7158`, otherwise branch to `0x96cc7208`.
- Typed Hexagon `encode_extended_and` and `encode_extended_new_p0_eq_branch` reject invalid displacement width and preserve both immediate-extender fields and predicate parse markers.
- **LLVM 6 scheduler caveat:** LLVM6 reorders two three-word predicate `.new` packets, placing the conditional branch first. The independent validation script performs only a documented packet-order/marker normalization **after** comparing the instruction operands and branch displacement; the resulting source-built emitted words are decoded by Rizin V66 in their exact OEM order and independently compared with the reference. Do not claim the unmodified LLVM6 object emits the exact original packet order.
- Passes reference byte comparison with zero differences. Original reference stays private and outside Git. The next missing code begins file offset `0x550a8`, physical `0x96cc70a8`.

## Checkpoint 04 — complete three-way controller MMIO programming, 692 bytes

- Source-built CDSP controller executable bytes `[0x55000,0x552b4)` = **692 exact bytes**; VMA `[0xc00c7000,0xc00c72b4)`, physical `[0x96cc7000,0x96cc72b4)`.
- Added **524 bytes** `[0x550a8,0x552b4)`: three mutually exclusive control-register programming branches (176, 176, 172 bytes), addressing MMIO block `0x08300200` with named register-value tuples. Feature `0x100` sets mode register R14 to `0x31` and hardware offset `0x28` to `2`; feature `0x200` and default use R14=`0x39`, offset `0x28`=`0`. The branches end with a memory barrier, then two jumps or fall-through to shared configuration epilogue at `0x552b4`.
- `build_controller_mode_program(ControllerMode)` assembles checked Hexagon extended assignments and immediate stores from *semantic* register/value pairs. All three mode paths preserve separate branch destinations, packet parse markers and instruction boundaries; no vendor opcode array is included.
- Independently verified the **entire new 524-byte interval** by LLVM6 assembly generated from a separate instruction fixture; all 131 instruction words also decode with Rizin V66 and compare byte-identically to the pinned CDSP reference. This is stronger than merely disassemble/reassemble OEM bytes.
- The initial `R E` segment `[0x3b000,0x5475c)` remains mostly non-code metadata/fill and is *not* counted as reconstructed executable. Within the short `R E` segment `[0x55000,0x554b0)`, **508 bytes remain** unimplemented, starting at `0x552b4`.

## Checkpoint 05 — shared commit and mode-flag clear, 812 bytes

- Independently generated controller executable file `[0x55000,0x5532c)` = **812 matching CDSP bytes**; physical `[0x96cc7000,0x96cc732c)`, VMA `[0xc00c7000,0xc00c732c)`.
- Added **120 bytes** `[0x552b4,0x5532c)`: commit R14/R15 to MMIO offsets `0x204` and `0x208`, then clear feature masks `1` and `7` at offsets `0x200` and `0x238`. The two read-modify-write sequences encode `R5=sub(#-1,R5)` as a short Hexagon instruction, with its parse marker preserved alongside the following word load.
- The first extended R0 assignment is the continuation of a **three-word packet** with `R1=R14`, so its immediate-consumer parse marker must be `ContinuationA`, not a standalone final packet. Independent LLVM6 assembly of the same semantic grouped packet produces **all 120 bytes exactly**, and Rizin V66 decodes all 30 words. The check independently confirms the first consumer packet marker; only treating the instructions as independent singletons would be incorrect.
- Typed constructors separately model `memw(Rbase+#0)=Rsrc`, `AND/OR` register fields, high/low halfword literals, and short `sub(#-1,Rsrc)`. These are backed by LLVM6 and direct reference comparison, not imported OEM words.
- Next missing segment bytes `[0x5532c,0x554b0)` = **388 bytes**, inside the same 1,200-byte late executable `PT_LOAD`.

## Checkpoint 06 — controller feature enable and PIC wait handoff, 924 bytes

- Complete strictly contiguous CDSP executable file `[0x55000,0x5539c)`, **924 independently generated Hexagon instruction bytes**, ELF VMA `[0xc00c7000,0xc00c739c)`, physical `[0x96cc7000,0x96cc739c)`.
- Added the 112-byte feature-enable and spin-loop handoff `[0x5532c,0x5539c)`: invalidates controller cache, changes the MMIO bit field at `0x08300200`, sets bit at `0x08300238`, reloads the controller base and jumps to the shared wait routine at `0x96cc7038` after `R13=add(PC,#8)`.
- Corrected the precise checked PIC-link field from `0x6a49_c00d` to `0x6a49_c40d` (bit 10) based on independently authored LLVM6 Hexagon assembly **and** Rizin V66 decoding. Tests check this field, the branch displacement and exact generated count.
- `tools/verify-agent20-10-cdsp-handoff.py` independently assembles all 112 bytes with LLVM6 and decodes 28 words with Rizin V66; direct reference comparison reports zero differing bytes.
- The remaining short `R E` segment is file `[0x5539c,0x554b0)` = 276 bytes. The previous `R E` region `[0x3b000,0x5475c)` is non-executable metadata despite flag `R E`; not counted.
- The checkpoint remains a **provisional preparation branch** based on Agent 05 `a02618f…`, not the absent checkpoint-50 integrated base. `git -c commit.gpgsign=false` uses author and committer identity `Codex <codex@openai.com>`; there is no new OEM signature.

## Checkpoint 07 — feature-spin and two cache-invalidate bit sets, 1,032 bytes

- Independently encoded CDSP controller text `[0x55000,0x55408)` = **1,032 bytes** matching the exact original image, VMA `[0xc00c7000,0xc00c7408)` and physical `[0x96cc7000,0x96cc7408)`.
- New **108-byte** interval `[0x5539c,0x55408)`: read controller bitmask from R4, test `R2 & R1`, conditional P0 `.new` backwards branch to physical `0x96cc738c`, then set feature mask `1` after `dcinva(R7)` in MMIO `0x0830020c` and `0x08300200`. Each 48-byte MMIO sequence is constructed from hardware addresses and named operations, with strict address/mask input validation.
- Packet-marker bit grouping is critical: `sub(-1,R5)` and the following `memw` load share a two-word packet; separate AND, OR and MMIO store instructions each terminate their own packets. This was verified against **independent LLVM6 assembly of all 108 bytes** and checked by Rizin V66 decoding all 27 four-byte instruction words.
- The next missing bytes are `[0x55408,0x554b0)` (168 bytes remaining in this executable `PT_LOAD`).
- Test and release build status: all offline workspace Rust and Python tests and release build pass. This remains a provisional Agent-05-descended branch, not checkpoint-50 Git ancestry, and not an authenticated/flashable CDSP image.

## Checkpoint 08 — entire short CDSP controller `R E` segment, 1,200 bytes

- Fully reconstructed **all 1,200 independently encoded Hexagon instruction bytes** in `cdsp.mbn` file `[0x55000,0x554b0)`. VMA `[0xc00c7000,0xc00c74b0)`, physical `[0x96cc7000,0x96cc74b0)`. This exact length matches the `PT_LOAD` executable section's `p_filesz`, not merely a function fragment.
- Added the final **168 bytes** `[0x55408,0x554b0)`: update controller register `0x0830002c`, mask out bits `0x71f`, combine DSP state from R16 with `0x200`, update controller bit at `0x08300028`, poll P0.new until ready, write `0` to the three mailbox/IRQ registers at `0x083a0040`, `+4`, `+8`, barrier and `jumpr LR`.
- Strict checked helpers model `R3=asl(R16,#0)` and `P0=cmp.eq(R3,R2); if(P0.new)jump:t -8`. The `ASL` operation and `R2=##0x200` share one packet. Independently authored LLVM6 source `tools/agent20-10-cdsp-controller-tail.s` reproduces **all 42 four-byte instruction words exactly**, including the parse marker; Rizin V66 decodes all 42 and confirms the branch target and MMIO instruction semantics.
- Entire 1,200-byte executable segment compares exactly to the original CDSP reference, with **zero mismatches**. This **does not** imply completion of the other, much larger CDSP `PT_LOAD` segments, bootability or new OEM signature creation.
- Next independent code to reconstruct is in `RWE` file segment starting at **`0x56000`**, VMA `0xc00c8000`, physical `0x96cc8000`. The `[0x554b0,0x56000)` gap between ELF loads is not executable reconstruction; it must remain structural layout.
- Git base caveat unchanged: still a portable branch from Agent 05 `a02618f…`; checkpoint50's integrated SHA was unavailable in the supplied materials.

## Checkpoint 09 — first CDSP `RWE` startup executable function, 84 disjoint bytes

- Distinct `RWE` segment begins at MBN file `0x56000`, VMA `0xc00c8000`, physical `0x96cc8000`. Reconstructed first genuine runtime callback function **84 bytes** at `[0x56000,0x56054)` with **zero byte mismatches**; this interval is explicitly **disjoint** from the completed `[0x55000,0x554b0)` controller segment.
- `cdsp_late_runtime` independently encodes Hexagon frame allocation, save R27, high/low 16-bit immediate halves, conditional startup hook in R28, `R2=#0` followed by the one-word `R3=#0;R1=#0` duplex, runtime table traversal through R27, callback invocation, negative loop jump, restore/deallocframe and `jumpr LR`. Values and checkable packet layout are typed; the original opcode bytes are not imported into this source.
- Program semantic assembly `tools/agent20-10-cdsp-rwe-entry.s` independently **reassembles all 84 bytes exactly via LLVM6**, including the unusual duplex order (LLVM selects the original layout when semantically equivalent zero assignments are ordered as R3,R2,R1). Rizin V66 confirms 21 valid Hexagon words and the original branch target.
- Following 12 bytes `[0x56054,0x56060)` are zero-filled source file layout **not instruction bytes**, so they are not counted as independently reconstructed firmware. The next procedure appears to start at `0x56060`, with cross-segment calling conventions and init-data relocations still to reconstruct.
- Combined unique verified later CDSP executable source at the two separate sites: **1,200 + 84 = 1,284 bytes**. Do not call this contiguous across the segment gap; the image has multiple disjoint `PT_LOAD` mappings. The old Agent 05 core/source fragment is separately verified and not included in this count.
- Full offline workspace Rust and Python test suites and release build pass. This remains a provisional branch from old Agent05 (`a02618f…`), not checkpoint-50 Git lineage; no full CDSP firmware or new OEM signature has been generated.

## Checkpoint 10 — CDSP RWE PIC state and GP reset, another 64 disjoint bytes

- **64 independent source-generated bytes** at file `[0x560d4,0x56114)`; VMA `[0xc00c80d4,0xc00c8114)`, physical `[0x96cc80d4,0x96cc8114)`. The output matches all original firmware bytes with **zero differences**.
- The code begins with the return stub at `0x560d4`; the following 40 bytes save LR and SP to controller-global locations through extended PC-relative adds, set R0 to -1 and branch to physical `0x96dc3ec0`; the next five-word packet clears the GP-relative state slot `0xc028c014` and returns.
- Original 40-byte PIC body plus `jumpr LR` (44 bytes) independently LLVM6-assemble to the exact firmware bytes using `tools/agent20-10-cdsp-rwe-pic.s`. The GP store is a **forced-extended five-word V66 packet** in the original, whereas LLVM6 independently produces a **shorter 16-byte semantically equivalent sequence**. `tools/verify-agent20-10-cdsp-rwe-pic-gp.py` explicitly checks the LLVM6 equivalence, Rizin V66 decoding of all 16 original instruction words, and a strict full 64-byte pinned reference equality; it does *not* falsely claim that the LLVM6 GP packet has identical length or opcode order.
- Checked constructors independently reconstruct PC-relative fields, explicit negative-one literal, extended jump and GP `.new` packet with bounded original operands. No raw OEM bytes or original signature/certificates are copied into Git.
- **Disjoint coverage count for Agent 20-10:** completed controller code 1,200 + RWE callback entry 84 + runtime PIC/GP 64 = **1,348 source-built executable bytes**, all individually reference-exact. The RWE gap `[0x56054,0x560d4)` is **128 file bytes**, including 12 structural zero bytes and 116 unimplemented program bytes (one additional runtime init function), and is NOT counted as reconstructed code.
- Next RWE candidate begins at file `0x56120`, after three additional NOP words at `[0x56114,0x56120)` that are source-layout boundary instructions/padding. The remaining CDSP image spans multiple large code/data ELF segments and is not fully reconstructed or newly authenticated.
- The Git branch is still an Agent 05 (`a02618f…`) descendant, not checkpoint50 integration. Publish this as a **provisional** checkpoint for rebase/cherry-pick onto Agent 20-01's actual integrated HEAD.

## Checkpoint 11 — fill CDSP RWE callback loader gap, 264 contiguous executable bytes plus 12 linker fill

- **116 new independently generated source-code bytes** at file `[0x56060,0x560d4)` with physical `[0x96cc8060,0x96cc80d4)`, source mapped VMA `[0xc00c8060,0xc00c80d4)`. All emitted bytes compare exactly with the private original pinned CDSP firmware.
- The decoder-visible routine stores a callback to GP global `0xc036488c`, saves R1:0 and adjusts SP, makes eight **address-checked extended calls** to `0x96ccd8e0`, `0x96cc9788`, `0x96cca8fc`, `0x96cc81e0`, `0x96cc82c8`, `0x96ccda30`, `0x96cca8e8`, `0x96dc3344`, and ends with a local self-branch. It uses the V66 compound packet form for an extended call and stack-pair transfer.
- Dedicated typed Rust encoders reconstruct the extended GP store, 24-bit bounded PC-relative call fields with explicit parse markers, and stack-pair load/store addressing modes. Unit tests validate every eight call encoding and reject unsupported immediate offsets; no OEM opcodes are pasted in from a reference.
- LLVM6 with semantically independent Hexagon source `tools/agent20-10-cdsp-rwe-loader.s` reproduces the **112 following bytes exactly**, but shortens the initial forced-extended two-word GP store to one word. `tools/verify-agent20-10-cdsp-rwe-loader.py` explicitly checks that equivalence, decodes **all 29 original instruction words** via Rizin V66 and verifies the original full 116-byte interval with zero mismatches. No claim is made that LLVM6's shortened GP-store packet is instruction-byte identical.
- `build_rwe_source_and_layout_prefix()` creates a byte-identical **276-byte contiguous image layout** `[0x56000,0x56114)`, carefully accounting for **264 independently source-generated executable bytes** and **12 structural zeros** `[0x56054,0x56060)`. These structural bytes are not counted as reconstructed executable.
- New unique independent code since Agent05 across all disjoint regions: 1,200-byte complete controller segment + 264-byte RWE executable prefix = **1,464 source-generated executable bytes**. The remaining 12 bytes are tracked solely as layout fill, and other enormous CDSP executable/zero-filled ELF regions are still unimplemented.
- Current work still descends from old Agent05 `a02618f…`, not unavailable checkpoint50-integrated head. OEM signature, final full image or hardware behavior remain out of scope.

## Checkpoint 12 — callback-registry cleanup at CDSP RWE 0x561a0, 1,528 executable bytes

- Added **64 independently source-generated Hexagon V66 bytes** `[0x561a0,0x561e0)` (VMA `[0xc00c81a0,0xc00c81e0)`, physical `[0x96cc81a0,0x96cc81e0)`). Their output matches the pinned private CDSP reference exactly, zero differing bytes. This is disjoint from `[0x56000,0x56114)` until `[0x56114,0x561a0)` is authored; no implicit original bytes or zeros count as code.
- The routine reads two absolute callback pointers through forced **two-word GP-relative V66 addressing**, allocates an eight-byte frame, conditionally branches if callback pointers are null, prepares `R1:0=(4,R16)`, invokes R2, clears its pointer using R2.new, then performs a paired deallocation/return.
- Checked typed encoders validate packet-relative offsets from the **packet start**, not the individual `if (cmp.eq(R2.new,#0))` consumer; the first branch to `0x96cc81dc` uses a `0x28`-byte packet-relative displacement, the second branch to `0x96cc81d4` uses `0x14` bytes. Rizin V66 confirms 16 original instruction words and both branch targets.
- An independently authored LLVM6 Hexagon semantic source reassembles **11 words to the exact original encodings**; LLVM6 chooses one-word GP loads rather than the two-word forced-extended source forms and inserts an extra zero extender before the final R2.new store. `tools/verify-agent20-10-cdsp-rwe-registry.py` checks and documents these differences, independently tests the equivalent LLVM semantics, decodes the entire original-order 16-word typed output with Rizin V66 and verifies the private reference. It does **not** misrepresent LLVM6's 60-byte output as byte-for-byte identical to the source's original 64 bytes.
- Total **unique independently generated executable code** across disjoint CDSP ranges: completed controller segment 1,200 bytes + RWE prefix 264 code bytes + callback-registry cleaner 64 = **1,528 bytes**. Structural 12-byte linker fill stays separate. The RWE gap `[0x56114,0x561a0)` contains 140 bytes not yet source-generated (including three NOP words); it will be reconstructed independently next.
- Git parent is still Agent05 `a02618f...`, not checkpoint-50. This is a portable provisional checkpoint until a verified integrated parent becomes available. A complete CDSP image, original OEM signing material, and real-hardware behavioral validation are out of scope.

## Checkpoint 13 — close RWE callback-dispatch gap; verified entire 480-byte prefix

- Added **128 independently encoded Hexagon V66 executable bytes** at CDSP MBN file `[0x56120,0x561a0)`, ELF virtual `[0xc00c8120,0xc00c81a0)`, physical `[0x96cc8120,0x96cc81a0)`. Generated bytes compare exactly with pinned source `cdsp.mbn`, with **zero byte differences**.
- The 32 four-byte instruction words implement frame allocation, forced-extended absolute callback-registry loads, callback-selection branches, an indexed callback lookup, a hardware `loop0`, `.new` conditional stores, guarded indirect calls and `dealloc_return`. Checked typed Rust encoders validate register classes, immediate fields, packet-relative branches and `.new` consumer distance; no original opcode arrays are embedded as output.
- The preceding **12 bytes** `[0x56114,0x56120)` encode three deterministic Hexagon NOPs. They are classified as **layout/alignment**, not novel source-built callback instructions. With the existing `[0x56054,0x56060)` 12-byte structural zero fill, the entire RWE file interval `[0x56000,0x561e0)` is **480 contiguous byte-identical bytes**, accounted as **456 independently generated executable code + 24 structural layout/NOP bytes**. The independently reconstructed short controller `[0x55000,0x554b0)` adds another 1,200 executable bytes, bringing this agent's independently authored **disjoint executable total to 1,656 bytes** (1,200+456), and no other CDSP image content is implied by that total.
- `tools/agent20-10-cdsp-rwe-dispatch.s` expresses the full function independently in LLVM6 Hexagon assembly. LLVM6 gives the same **128-byte semantic output**, but chooses a compact first registry load, inserts an extender for R1=0x320 and reorders two guarded packets; the verifier checks **23 unchanged original instruction words**, preserves and discloses all other forms. Rizin V66 decodes all **32 source-built original-order words**, verifies packet boundaries and five key branch/loop destinations. No claim is made that LLVM6's raw 128 bytes exactly match all original 128 bytes.
- Reproduce the emitter and full layout:

  ```sh
  cargo run --offline --locked -p firmware-tools --bin cdsp-late-callback-dispatch -- emit /tmp/cdsp-56120.bin
  cargo run --offline --locked -p firmware-tools --bin cdsp-late-callback-dispatch -- verify /private/cdsp.mbn
  cargo run --offline --locked -p firmware-tools --bin cdsp-late-callback-dispatch -- emit-prefix /tmp/cdsp-rwe-480.bin
  cargo run --offline --locked -p firmware-tools --bin cdsp-late-callback-dispatch -- verify-prefix /private/cdsp.mbn
  python3 tools/verify-agent20-10-cdsp-rwe-dispatch.py \
    --llvm-mc /private/llvm6/bin/llvm-mc --rz-asm /private/rz-asm \
    --generated /tmp/cdsp-56120.bin --prefix /tmp/cdsp-rwe-480.bin \
    --reference /private/cdsp.mbn
  ```

- **Tests:** 213 Rust tests and 23 Python tests pass; offline release build and direct reference check pass. The next distinct callback registry starts at file `0x561e0` (physical `0x96cc81e0`) and is not yet rebuilt. Checkpoint 50's original Git base is still unavailable, so this branch remains provisional on Agent 05's older CDSP ancestor.

## Checkpoint 14 — complete CDSP RWE callback-table sweeper, 1,736 executable bytes

- Added **80 independently constructed Hexagon V66 instruction bytes**, file `[0x561e0,0x56230)`, VMA `[0xc00c81e0,0xc00c8230)`, physical `[0x96cc81e0,0x96cc8230)`. Source output matches the pinned private OEM reference **byte-for-byte**.
- The callback sweeper initializes a global byte flag (via an original forced-extender GP load and predicated byte store), conditionally calls the dispatcher at physical `0x96cc8120`, walks a scaled function-pointer array, invokes callback R2 and loops on the callback-return flag in stack storage. Packet-relative backward branches target `0x96cc820c`; the routine restores a frame and returns at `0x5622c`.
- An independent LLVM6 semantic source reconstructs the same routine in **72 bytes**: it shortens the original forced GP byte load and conditional relative call, so it must **not** be claimed as exact byte-level independent LLVM output. Sixteen of the 20 original instruction words independently match unchanged, and Rizin V66 confirms all 20 source-built original-order packet words, branch destinations and opcode semantics. The remaining instructions are checked with narrow typed V66 operand encoders, not firmware byte arrays.
- **Complete contiguous RWE prefix** is now `[file 0x56000,0x56230)` = **560 byte-identical file bytes**, accounted strictly as **536 independently generated executable code bytes** and 24 alignment/NOP/fill bytes. Separate short controller executable `[0x55000,0x554b0)` contributes 1,200 additional source-generated bytes, so this agent has **1,736 disjoint executable bytes** verified. No authentication or unknown payload bytes are counted as rebuilt firmware.
- Reproducible verification:

  ```sh
  cargo run --offline --locked -p firmware-tools --bin cdsp-late-callback-sweeper -- emit /tmp/cdsp-rwe-sweeper-80.bin
  cargo run --offline --locked -p firmware-tools --bin cdsp-late-callback-sweeper -- verify /private/cdsp.mbn
  cargo run --offline --locked -p firmware-tools --bin cdsp-late-callback-sweeper -- emit-prefix /tmp/cdsp-rwe-560.bin
  cargo run --offline --locked -p firmware-tools --bin cdsp-late-callback-sweeper -- verify-prefix /private/cdsp.mbn
  python3 tools/verify-agent20-10-cdsp-rwe-sweeper.py \
    --llvm-mc /path/to/llvm6/bin/llvm-mc --rz-asm /path/to/rz-asm \
    --generated /tmp/cdsp-rwe-sweeper-80.bin --prefix /tmp/cdsp-rwe-560.bin \
    --reference /private/cdsp.mbn
  ```

- **Tests:** 215 Rust tests, 23 Python tests, offline workspace release build, reference comparison and independent decoder verification passed. Next unimplemented function begins file `0x56230` (physical `0x96cc8230`) and is unrelated to the completed dispatcher. Checkpoint-50 integration remains unavailable; branch parentage is the original Agent05 `a02618f…` provisional base.

## Checkpoint 15 — callback-poll and fallback function; 1,888 executable bytes

- Added 152 independently emitted Hexagon V66 executable bytes at `cdsp.mbn` file `[0x56230,0x562c8)` (ELF virtual `[0xc00c8230,0xc00c82c8)`, physical `[0x96cc8230,0x96cc82c8)`). Source-generated bytes compare exactly to the pinned private CDSP reference: **0 mismatches**.
- The routine performs conditional callback registration and a frame-linked multiplexer: it tests R0 for `-1`, saves R17:16/R19:18/R21:20, invokes the controller service at physical `0x96dfd9a0`, computes `R17=mpyi(R16,R17)`, checks a GP-relative callback status byte at `0xc0364128`, optionally calls the dispatcher at `0x96cc8120`, traverses callback pointers through R20, and loops over R2.new results until either completion or fallback. It then returns with three saved register pairs restored.
- `build_rwe_prefix_through_poll()` is **712 byte-identical file bytes** at `[0x56000,0x562c8)`, accounted as **688 independently reconstructed executable code bytes and 24 separate layout NOP/fill bytes**. The short controller executable `[0x55000,0x554b0)` adds 1,200 code bytes, making **1,888 independently generated executable bytes in two disjoint ranges** (plus 24 layout bytes).
- `tools/agent20-10-cdsp-rwe-poll.s` is an independently authored Hexagon v60/V66-compatible LLVM6 source. LLVM6 produces **144 bytes** vs the OEM's 152 (two forced extenders in the original), with **32 of 38 original instruction words** exactly matching. A conditional branch displacement also changes because the source is shortened. This is a semantic cross-check, **not** an assertion that the LLVM6 text reproduces the original bytes in full. Rizin V66 independently verifies all **38 original packet-order words** and all significant call/loop destinations. A source-built Rust emitter with checked operands preserves the actual firmware packet widths.
- The callback emitter checks direct-call split displacements (the upper field is `(delta >> 15)`, not `>>16`), and unit tests reject unsupported addresses/registers, GP stores and invalid backedges. This fixes a previous local test failure.
- To reproduce verification (original reference is private, never copied into Git):

  ```sh
  cargo run --offline --locked -p firmware-tools --bin cdsp-late-callback-poll -- emit /tmp/cdsp-rwe-poll-152.bin
  cargo run --offline --locked -p firmware-tools --bin cdsp-late-callback-poll -- verify /private/cdsp.mbn
  cargo run --offline --locked -p firmware-tools --bin cdsp-late-callback-poll -- emit-prefix /tmp/cdsp-rwe-712.bin
  cargo run --offline --locked -p firmware-tools --bin cdsp-late-callback-poll -- verify-prefix /private/cdsp.mbn
  python3 tools/verify-agent20-10-cdsp-rwe-poll.py \
    --llvm-mc /private/llvm6/bin/llvm-mc --rz-asm /private/rz-asm \
    --generated /tmp/cdsp-rwe-poll-152.bin --prefix /tmp/cdsp-rwe-712.bin \
    --reference /private/cdsp.mbn
  ```

- This **provisional** branch descends from the old Agent05 CDSP checkpoint17, not from the still-unavailable checkpoint-50 integrated branch. Independent source reconstruction covers only the explicitly listed ranges, does not prove real hardware behavior, and does not provide or imply a newly signed Qualcomm image. The next CDSP callback function begins at file `0x562c8`.

## Checkpoint 16 — CDSP callback completion and aligned RWE prefix

- Added **80 independently generated V66 executable bytes** at CDSP file `[0x562c8,0x56318)`, corresponding to ELF virtual `[0xc00c82c8,0xc00c8318)` and physical `[0x96cc82c8,0x96cc8318)`. Original private firmware comparison is exact (0 differing bytes).
- Source-level operations: signed `R0=-2` guard, 8-byte frame save, repeated calls to the shared poll helper at `0x96cc8230`, a forced GP-relative word load at `0xc028c538`, an extended `0xfffe`/`add #1` duplex, controller call at `0x96cca640`, two register-pair manipulations, frame restore, and branch to `0x96ccc7a0`.
- Two alignment `nop` words at file `[0x56318,0x56320)` are reconstructed **layout**, not independently authored executable program logic. Exact contiguous RWE file prefix `[0x56000,0x56320)` = **800 bytes**, accounted as **768 independently generated executable bytes + 32 explicitly identified NOP/fill bytes**.
- The other disjoint late-CDSP `R E` controller segment `[0x55000,0x554b0)` is still source-generated 1,200 executable bytes. Thus this agent's total independently source-emitted executable coverage is **1,968 bytes**; no invented or OEM-imported code bytes are counted.
- LLVM6 independently assembles a semantic fixture in 84 bytes versus 88 original bytes including two alignment words (compiler moves a backward call and shortens the forced GP load). Thirteen original instruction words are unchanged in the compiler's semantic assembly; this is **not** a claim of exact LLVM6 full binary output. Rizin V66 independently decodes all 20 original packet-order source-built words, including the physical direct-call and final branch destinations.
- Checked encoder tests reject unsupported register/value/branch offsets, with explicit overflow and packet marker rules. Reproduce using `cargo run --offline --locked -p firmware-tools --bin cdsp-late-callback-completion -- verify-prefix /private/cdsp.mbn`, or direct two-assembler check:

  ```sh
  python3 tools/verify-agent20-10-cdsp-rwe-finish.py \
    --llvm-mc /private/llvm6/bin/llvm-mc --rz-asm /private/rz-asm \
    --generated /private/callback-finish-80.bin \
    --prefix /private/cdsp-rwe-800.bin --reference /private/cdsp.mbn
  ```

- Next missing CDSP executable code begins file `0x56320` (physical `0x96cc8320`), a separate RNG/callback routine. Checkpoint-50 integration remains unavailable: this branch still descends from the original Agent05 CDSP checkpoint 17 and is a **provisional portable patch**.

## Checkpoint 17 — controller seed/state-update function prologue, 2,036 executable bytes

- Independently reconstructed `[file 0x56320,0x56364)` = **68 Hexagon V66 executable bytes** (physical `[0x96cc8320,0x96cc8364)`, VMA `[0xc00c8320,0xc00c8364)`). Byte comparison against pinned CDSP reference is exact.
- The new routine checks a GP-relative initialization pointer at `0xfe0ad660`, sets up a 48-byte stack frame, saves live registers, conditionally initializes three seed-state globals at `0xc0364158`, `0xc0364150`, `0xc0364148`, and branches to exit at physical `0x96cc8438` when already initialized. Full function continues beyond `0x56364` and **is not yet marked complete**.
- LLVM6 semantic source yields **64 bytes**, while the original uses **68** due to a forced 32-bit GP load and different packet scheduling. Thirteen source instruction encodings match LLVM6 unchanged; this does **not** establish binary-identical LLVM assembly. Rizin V66 independently validates all 17 original-order packet words and the physical pointer/branch targets.
- Contiguous RWE prefix `[0x56000,0x56364)` = **868 file bytes**, including **836 generated executable bytes and 32 distinct NOP/fill bytes**. The separate complete short controller program `[0x55000,0x554b0)` contributes another **1,200 source-derived executable bytes**, so Agent 20-10 total executable coverage is now **2,036 bytes** across disjoint regions, with no OEM authentication or opaque executable payload counted.
- Focused checked-operand unit tests reject invalid GP pointers, register numbers, conditional branch locations and unsupported frame sizes. Run `cdsp-late-rng-prologue verify-prefix` with the pinned private MBN, plus `tools/verify-agent20-10-cdsp-rng-prefix.py` with LLVM6 and Rizin V66. Next missing code: file `0x56364` within the same seed-update routine.

## Checkpoint 18 — seeded callback registration and GP status writes, 2,128 executable bytes

- Added **92 original-byte-matching independently encoded Hexagon V66 executable bytes** at CDSP file `[0x56364,0x563c0)`, ELF VMA `[0xc00c8364,0xc00c83c0)`, physical `[0x96cc8364,0x96cc83c0)`. Every byte is generated from checked source-level operand fields and explicit packet markers, not copied OEM executable bytes.
- Semantic operations: read `UPCYCLELO` into R22, invoke timing routine at physical `0x96dc0540`, conditionally load/update GP byte at `0xc0364140`, run guarded poll callback at `0x96cc8230`, update the three seed-state words addressed through R19/R18/R20 and set the completed-init byte to R23. Branch from the `P0.new` packet at `0x96cc837c` to `0x96cc83c0` when already initialized.
- LLVM6 independently assembles an 84-byte semantic equivalent to the source's 92-byte original-order V66 packet layout: it omits two forced GP immexts and adjusts several packet markers/call placements. **Fifteen unchanged instruction words** are verified against its semantic compilation. Rizin V66 independently verifies all **23** original packet words, physical branch/call targets and GP addresses. Direct pinned reference comparison passes with zero differences; LLVM6 is not misrepresented as an unchanged full binary reproduction.
- Contiguous RWE file prefix is now `[0x56000,0x563c0)` = **960 file bytes**, consisting of **928 independently emitted executable bytes plus 32 NOP/fill bytes**. The other disjoint, fully rebuilt 1,200-byte controller segment yields **2,128 cumulative independently generated CDSP executable bytes** in this agent.
- Unit tests check checked operand ranges and packet-relative branch targets; all direct emitted bytes are compared against original `cdsp.mbn`. Reproduce with `cdsp-late-rng-init verify-prefix /private/cdsp.mbn` and `tools/verify-agent20-10-cdsp-rng-init.py --llvm-mc /private/llvm6/bin/llvm-mc --rz-asm /private/rz-asm --generated /private/rng-state-92.bin --prefix /private/cdsp-rwe-960.bin --reference /private/cdsp.mbn`.
- Next code at file `0x563c0`: the data-dependent shift/XOR state-update loop and a callback dispatch/return path; the full function starting at `0x56320` remains **incomplete**. This branch remains **provisional** because it derives from Agent05's CDSP checkpoint 17, not checkpoint 50 integration.

## Checkpoint 19 — complete seeded CDSP controller update and return, 2,264 executable bytes

- Source-encoded the final **136 Hexagon V66 executable bytes** at CDSP MBN
  file `[0x563c0,0x56448)` (physical `[0x96cc83c0,0x96cc8448)`, ELF virtual
  `[0xc00c83c0,0xc00c8448)`). Direct comparison to the pinned private
  `cdsp.mbn` is exact, with **zero mismatching bytes**. The full seeded-state
  function `[0x56320,0x56448)` is now independently emitted as **296 executable bytes**.
- The new code uses the documented 32-bit V66 xorshift/insert instructions,
  checked Hexagon `immext` assignment, `.new` word stores for all three state
  slots, a `P0.new` backward branch from physical `0x96cc841c` to
  `0x96cc8368`, two physical controller service calls (`0x96dc0680`,
  `0x96dc4acc`), stack-spill and packed duplex ABI restore instructions.
  The checked operand encoder validates register IDs, branch bounds, packet
  placements and callback targets; unsupported operands are rejected.
- Source-generated **contiguous RWE file prefix** is `[0x56000,0x56448)` =
  **1,096 bytes**, comprising **1,064 executable source bytes and 32
  separately identified layout NOP/fill bytes**. The other disjoint completed
  controller segment `[0x55000,0x554b0)` contributes 1,200 executable bytes,
  for **2,264 independently generated code bytes** in two disjoint regions.
  Structural NOPs are not counted as functional executable code.
- Independent LLVM6 semantic source `tools/agent20-10-cdsp-rwe-seed-update.s`
  compiles to 136 bytes, with **24 original-order matching instruction-word
  encodings**. LLVM6 reschedules an extended GP assignment and uses different
  packet parse markers and branch placement for some words; it is not claimed
  to be an exact, unchanged full LLVM6 binary reassembly.
  `tools/verify-agent20-10-cdsp-rng-update.py` independently checks those
  24 unchanged LLVM word encodings, all **34 original source-built words**
  decoded by Rizin V66 in their authentic packet order, both physical call
  targets and the conditional backedge, and full byte equality of the
  296-byte function and 1,096-byte RWE file prefix.
- Verified with `cargo test --workspace --offline --locked` (**225 Rust tests**),
  Python discovery (**23 tests**), and the offline workspace release build.
  Reproduce via:

  ```sh
  cargo run --offline --locked -p firmware-tools --bin cdsp-late-rng-update -- verify /private/cdsp.mbn
  cargo run --offline --locked -p firmware-tools --bin cdsp-late-rng-update -- verify-seed /private/cdsp.mbn
  cargo run --offline --locked -p firmware-tools --bin cdsp-late-rng-update -- verify-prefix /private/cdsp.mbn
  python3 tools/verify-agent20-10-cdsp-rng-update.py \
    --llvm-mc /private/llvm6/bin/llvm-mc --rz-asm /private/rz-asm \
    --generated /private/rng-update-136.bin \
    --whole /private/seed-full-296.bin --prefix /private/cdsp-rwe-1096.bin \
    --reference /private/cdsp.mbn
  ```

- This is **still a provisional branch from Agent05 CDSP checkpoint 17**, not
  a checkpoint-50-rooted integrated firmware. Original signed payload remains
  outside Git; no OEM-authenticated new CDSP file or hardware behavior is
  claimed. **Next missing CDSP executable code starts at file offset `0x56448`**
  (physical `0x96cc8448`).

## Checkpoint 20 — CDSP seed handoff function and contiguous 1,124-byte RWE layout

- Added a complete **28-byte Hexagon V66 seeded-controller handoff** at
  `[0x56448,0x56464)` (physical `[0x96cc8448,0x96cc8464)`, ELF virtual
  `[0xc00c8448,0xc00c8464)`). All seven emitted 32-bit instruction words
  match the pinned original `cdsp.mbn`, with zero differing bytes.
- Checked semantic operations: physical call at `0x96cc8448` targeting
  `0x96dc28ac`, extended pointer assignment and zero-frame duplex,
  `R1:0=combine(1,0xc0226160)`, frame deallocation, and physical tail jump
  from `0x96cc8460` to `0x96cce74c`. The original continuation/final packet
  markers are modeled explicitly. The emitted instruction-word encoding
  does not require OEM opcode tables or imported reference payload.
- The independent LLVM6 semantic source reassembles all 28 bytes with
  **five unchanged instruction words** and two differing, documented Hexagon
  continuation marker choices. A mistaken LLVM fixture relative call value
  was corrected to the actual `0xfa464` physical byte displacement.
  Rizin V66 confirms all seven source-built words and both control transfers.
- Verified source-built RWE prefix `[0x56000,0x56464)` = **1,124 bytes**:
  **1,092 executable bytes + 32 NOP/fill layout bytes**. The other disjoint
  completed 1,200-byte executable controller segment makes cumulative
  independently rebuilt source code **2,292 bytes**, excluding the 32-byte
  structural fill. The next unimplemented CDSP routine begins at `0x56464`.
- Unit tests reject unsupported physical branch targets, operand registers,
  and wrong duplex sizes. Run `cdsp-late-seed-handoff verify-prefix` with the
  private pinned CDSP, and `tools/verify-agent20-10-cdsp-seed-handoff.py` with
  LLVM6/Rizin V66 for independent confirmation. Complete offline Rust,
  Python, and release checks pass before publication.
- This branch is still **provisional from Agent05's old CDSP checkpoint 17**,
  not from unavailable checkpoint-50-integrated history; it contains neither
  original OEM signed payload nor a hardware-tested new firmware binary.

## Checkpoint 21 — lock-free CDSP atomic counter, 2,376 executable bytes

- Independently constructed **84 Hexagon V66 executable bytes** at MBN file
  `[0x56464,0x564b8)`, virtual `[0xc00c8464,0xc00c84b8)` and physical
  `[0x96cc8464,0x96cc84b8)`, directly matching the pinned original with
  zero byte differences. The instruction at `0x564b4` is a separate one-word
  `jumpr LR` helper; it is included in the contiguous source-generated range.
- Models the shared atomic-counter update through checked instructions:
  `R3=memw(R2=0xc0363e00)`, zero/new-value forward branch, two
  `memw_locked` compare-and-swap retry loops (`0x96cc8474`, `0x96cc84a0`),
  signed bound checks and the indexed `R2=memw(R3<<2+0xc0363e08)` output
  store. Branches are encoded relative to their **packet starts** rather
  than an arbitrary instruction address. Reject unsupported registers,
  addresses, offsets and atomic loop targets.
- The compiler independently assembles an **88-byte semantic LLVM6 version**
  instead of the original 84 bytes because it does not choose the compact V66
  `R2=##addr; R3=memw(R2)` fused instruction and it changes several parse
  markers and consequent branch displacements. **16 original instruction
  words are unchanged** in LLVM6's output; these are checked explicitly.
  Rizin V66 independently decodes all **21 source-generated original-order
  instruction words** and confirms the conditional retry branch targets.
- New source-built contiguous CDSP RWE `[0x56000,0x564b8)` = **1,208 file
  bytes**, of which **1,176 source-reconstructed executable bytes** and
  **32 structural layout NOP/fill**. Together with the separate complete
  1,200-byte CDSP controller segment `[0x55000,0x554b0)`, the agent's
  independently reconstructed executable coverage totals **2,376 bytes**.
- All tested checks pass: offline Rust workspace tests, Python discovery,
  workspace release build, direct pinned reference comparison, Rizin V66,
  and independent LLVM6 fixture. To reproduce:

  ```sh
  cargo run --offline --locked -p firmware-tools --bin cdsp-late-atomic-counter -- verify /private/cdsp.mbn
  cargo run --offline --locked -p firmware-tools --bin cdsp-late-atomic-counter -- verify-prefix /private/cdsp.mbn
  python3 tools/verify-agent20-10-cdsp-atomic-counter.py \
      --llvm-mc /private/llvm6/bin/llvm-mc --rz-asm /private/rz-asm \
      --generated /private/atomic-counter-84.bin --prefix /private/cdsp-rwe-1208.bin \
      --reference /private/cdsp.mbn
  ```

- **Next unimplemented function starts at CDSP file `0x564b8`**, physical
  `0x96cc84b8`. This work remains provisional on Agent05's older CDSP base;
  no original vendor ELF/authentication bytes are embedded or OEM signing
  asserted.

## Checkpoint 22 — CDSP callback broker and three trap0 operations, 2,512 executable bytes

- Independently source-rebuilt **136 byte-identical Hexagon V66 executable
  bytes** at CDSP MBN file `[0x564b8,0x56540)`, virtual
  `[0xc00c84b8,0xc00c8540)`, physical `[0x96cc84b8,0x96cc8540)`.
  The complete generated function and enlarged RWE file prefix compare
  exactly with the pinned private original `cdsp.mbn`, with **0 byte mismatches**.
- The function saves R17:16/R19:18, reads a GP-relative status word
  at `0xc028c028`, calls the shared controller service `0x96ddf3a0`,
  performs guarded state updates and three `trap0(#0)` calls with selectors
  1, 6 and 2, then restores the pair frame and returns. Branches are
  packet-relative, using original V66 `R2.new` producer/consumer semantics.
- LLVM6 independently assembles the equivalent function in **128 bytes**:
  compiler changes one extended GP load/store selection, packet markers and
  branch immediates compared with the original 136-byte V66 layout.
  The verified matching subset has **24 unchanged instruction words**;
  Rizin V66 independently confirms all **34 generated original-order words**,
  the physical callback, guarded branches and all three traps. The compiler
  fixture is *not* misrepresented as a complete byte-identical assembly.
- Contiguous RWE file reconstruction now `[0x56000,0x56540)` = **1,344 bytes**:
  **1,312 independently encoded executable bytes + 32 distinctly identified
  layout NOP/fill**. Adding the disjoint complete 1,200-byte controller
  segment `[0x55000,0x554b0)` gives **2,512 total source-generated CDSP
  executable bytes** for this agent, excluding all structural fill.
- Added checked, narrow V66 op encoders for the extended GP load/store,
  status predicate/branch, duplex register pair saves, trap0 and new-value
  state writes. Regression tests reject unsupported branch targets, trap
  selectors, register values and offsets. All Rust tests, Python tests,
  offline workspace release build and independent decoder checks pass.
  Reproduce with:

  ```sh
  cargo run --offline --locked -p firmware-tools --bin cdsp-late-callback-service -- verify /private/cdsp.mbn
  cargo run --offline --locked -p firmware-tools --bin cdsp-late-callback-service -- verify-prefix /private/cdsp.mbn
  python3 tools/verify-agent20-10-cdsp-callback-service.py \
    --llvm-mc /private/llvm6/bin/llvm-mc --rz-asm /private/rz-asm \
    --generated /private/callback-service-136.bin \
    --prefix /private/cdsp-rwe-1344.bin --reference /private/cdsp.mbn
  ```

- **Next unimplemented CDSP code** starts file `0x56540` (physical
  `0x96cc8540`). This Git branch is still based on the old Agent05 CDSP
  checkpoint 17, not the unavailable checkpoint-50 integration branch.
  No OEM signature, full CDSP image or hardware execution is implied.

## Checkpoint 23 — CDSP GP-state multiplier and backedge, 2,552 executable bytes

- Added **40 independently encoded Hexagon V66 executable bytes** at CDSP
  MBN file `[0x56540,0x56568)`, ELF virtual `[0xc00c8540,0xc00c8568)`,
  physical `[0x96cc8540,0x96cc8568)`; direct pinned-original comparison
  reports **zero byte differences**.
- The routine reads a forced-extended GP word at `0xc028c02c`, initializes
  its state through a packed `R3=1; R4=R3` duplex, multiplies by the checked
  extended constant `0x015a4e35`, branches backward to physical `0x96cc8548`
  while the predicate is active, and finally writes GP state and caller output.
- LLVM6 independently assembles a **32-byte semantic** program. It shortens
  the two forced GP immexts and consequently changes one branch displacement,
  while the duplex, multiplier extender and result store have unchanged word
  encodings. The verifier checks the exact permitted marker/branch deltas.
  Rizin V66 independently decodes **all 10 original 32-bit packet words**.
  LLVM6 full binary byte-identical reconstruction is *not* claimed.
- The agent's contiguous RWE output `[0x56000,0x56568)` now measures **1,384
  source-built file bytes**, of which **1,352 are reconstructed executable
  bytes** and **32 are separately identified NOP/fill bytes**. Adding the
  independent 1,200-byte controller executable `[0x55000,0x554b0)` yields
  **2,552 independently authored source-code bytes across disjoint regions**.
- `cdsp-late-gp-multiplier` CLI emits only the source-derived interval or the
  joined prefix; its `verify`/`verify-prefix` commands compare byte-exactly
  with the pinned private reference. Regression tests reject unsupported
  GP addresses, multiplier values and branch targets. Independent check:

  ```sh
  python3 tools/verify-agent20-10-cdsp-gp-multiplier.py \
    --llvm-mc /private/llvm6/bin/llvm-mc --rz-asm /private/rz-asm \
    --generated /private/gp-multiplier-40.bin \
    --prefix /private/cdsp-rwe-1384.bin --reference /private/cdsp.mbn
  ```

- **Next unimplemented region** begins at file offset `0x56568` (physical
  `0x96cc8568`). All contributions remain provisional on Agent 05's older
  CDSP base, not a checkpoint-50-integrated repository; no copied OEM
  authentication or verified hardware behavior is claimed.
