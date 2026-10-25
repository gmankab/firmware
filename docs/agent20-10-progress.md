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
