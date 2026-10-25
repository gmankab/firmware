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
