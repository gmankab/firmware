# Agent 06 — Qualcomm modem firmware

## Repository base and boundary

- Assignment baseline: **checkpoint 50, `b7ed4b587633b9b475a40f5a3ace5cc90db620ff`**.
- **Availability caveat:** checkpoint-50 bundle is not mounted in this agent's file environment. The actual available ancestor is `recompile-firmware.bundle`, HEAD `69ebf0d1c50859189a9196a21155c388ac914d7b`. Work lives on `agent/06-modem`, with isolated `modem_*` modules and a standalone `modem-rebuild` binary. Commit(s) must be cherry-picked onto checkpoint 50 by the integration agent. Do not report these as direct descendants of checkpoint 50.
- Reference firmware: `modem.mbn`, SHA-256 `6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782`. It is held **outside Git** in the user-supplied archive.

## Architecture, mapping and reference verification

`modem.mbn` has little-endian ELF32, e_machine `164` (Qualcomm Hexagon DSP6), e_flags `0x66`, and ELF e_entry `0x8e000000`. Its first executable `PT_LOAD` is file `[0x3000,0x5020)`, ELF virtual `[0xc0800000,0xc0802020)`, and physical `[0x8e000000,0x8e002020)`. Its next main R-X segment starts at file `0x7c000` (ELF virtual `0xc0890000`, physical `0x8e090000`), size `0x183cf0`. These address spaces are deliberately never conflated.

Rizin 0.9.1 Hexagon V6 decodes the entry's eight vector slots, privileged `S6` (SSR), `S18` (SYSCFG) register transfers, set-bit operations and cache-kill instructions. Historical LLVM 6.0.1 Hexagon MC (`hexagonv60`) independently assembles the vector jumps and set-bit operations; it does not accept several privileged operations in its syntax. For those, the typed encoder follows the Rizin/Hexagon instruction-field patterns, rather than embedding OEM instruction words.

## Checkpoint 01: entry/reset vector code

- Exact file offsets: `[0x3000,0x3044)` (**68 bytes**).
- Physical addresses: `[0x8e000000,0x8e000044)`; ELF virtual: `[0xc0800000,0xc0800044)`.
- 17 independently generated single-instruction Hexagon packets, no embedded reference opcode data. One vector dispatches to physical `0x8e00111c`; after privileged setup control transfers to physical `0x8e0005c4` (file offset `0x35c4`).
- `cargo run --release --offline --locked -p firmware-tools --bin modem-rebuild -- emit /tmp/entry.bin` emits the fragment.
- `cargo run --release --offline --locked -p firmware-tools --bin modem-rebuild -- verify /private/modem.mbn` reports exact byte agreement; the emitted 68-byte SHA-256 is `80611bd86bb0827c0bf6db324f898deec1d91658d4d5193ccf195ea1f499e7d4`.
- `verify-mbnv6-hashes` on the private original confirms the hash table for 20 segments, **not** a regenerated, separately signed image. No full modem firmware or signer keys have been reconstructed.
- `cargo test --workspace --offline --locked`, `cargo build --workspace --release --offline --locked`, and Python unit tests pass on the actual available baseline (Rust: existing tests plus 5 new focused tests; Python: 13 tests). `git diff --check` passes.

## Remaining work

The following file interval `[0x3044,0x35c4)` appears to contain a memory-map/translation table and is **not yet generated**; it must not be counted as executable or interpreted as linear Hexagon instructions. Additional startup code begins at physical `0x8e0005c4` (file `0x35c4`) and requires typed packet reconstruction. All other executable and data sections remain unimplemented, and OEM authentication cannot be independently reissued.

## Checkpoint 02: structurally reconstructed bootstrap manifest

- Added `modem_boot_manifest.rs`: endian-checked type/size fields for 13 memory mapping records, three small boot options, a 16-byte process name, two transfer regions, six copy records, two control records, a terminator, and `0x320` bytes of explicit `0x7f` ELF alignment fill.
- Exact file range `[0x3044,0x3500)`, size `0x4bc` = **1,212 bytes**, which are **not** claimed as executable code. The fields are independently generated from semantically named values. The record type meanings are provisional; their structure/lengths and addresses are exact.
- Combined independently emitted `[0x3000,0x3500)` is **1,280 byte-identical bytes** (68 executable, 412 structured metadata, 800 alignment fill). Output SHA-256 is `a59993f08867839ebaff4940ae6335da61e4d7059591e32982ead7e4eb57fe39`.
- Run `cargo run --release --offline --locked -p firmware-tools --bin modem-rebuild -- emit-layout /tmp/modem-entry-layout.bin`, and `... -- verify-layout /private/modem.mbn`.
- Following code begins at file `0x3500` (physical `0x8e000500`), and the first vector dispatches to file `0x35c4` (physical `0x8e0005c4`); these code islands are **not yet rebuilt**. Whole MBN completion, hash regeneration, and authentication are still out of scope.

## Checkpoint 03: Hexagon TLB/cache tail (disconnected code island)

- Reconstructed executable file `[0x3584,0x35c4)` (64 bytes, 16 Hexagon instruction words), physical `[0x8e000584,0x8e0005c4)`. These are **not** contiguous with the earlier `[0x3000,0x3500)` because the 132-byte `[0x3500,0x3584)` routine is not yet implemented.
- Typed ISA operations: privileged write of SYSCFG, CLRBITS, Hexagon `isync`, `ickill`, three conditional TLB-write exit paths and jumpr to r14/r13/r31; the initial 2-instruction packet uses `Continue`/`End` markers. The source contains no raw OEM opcodes.
- Both Rizin Hexagon V6 disassembly and the open ISA mask/op tables corroborate the encoding; `cargo run --release --offline --locked -p firmware-tools --bin modem-rebuild -- verify-tlb-tail /private/modem.mbn` reports exact byte equality.
- Combined verified **nonoverlapping** coverage: 132 independently rebuilt executable bytes (68 + 64), plus 412 metadata and 800 deterministic padding bytes. Total generated byte count **1,344** in two disjoint intervals. Do not call this a 1,344-byte contiguous prefix.

## Checkpoint 04: contiguous entry, manifest, TLB bootstrap through `0x35c4`

- Added `modem_tlb_init.s`: **196 bytes of original assembly source** covering executable file `[0x3500,0x35c4)`, physical `[0x8e000500,0x8e0005c4)`, with 49 Hexagon V6 instruction words. Named macros derive unsupported privileged instruction forms (`S6`/`S18`, `tlbp`, `tlbr`, `tlbw`, `ickill`) from public Hexagon V6 ISA bitfield patterns; no OEM instruction array is present.
- Historical LLVM 6 Hexagon MC assembles the standard packet instructions. One conditional packet is canonically reordered by LLVM; the checked assembler driver `tools/assemble-modem-tlb.py` reorders four generated **instructions**, resets their packet parse bits and rejects unexpected classes or scheduling. It never copies the reference machine instructions.
- The combined, now **fully contiguous** `[0x3000,0x35c4)` region totals `0x5c4` = **1,476 bytes**: **264 executable** (68 entry + 196 MMU) + **412 structured metadata** + **800 fixed alignment fill**. The `[0x3044,0x3500)` table is data, never counted as code. The combined SHA-256 is `8f0509f4c7e2a52cecbbeeb5290b71c4e2a02588f269fd756bd6bd27c5efd436`.
- Reproduce with historical LLVM 6, which is in the user-supplied next-toolchain archive:

  ```sh
  cargo run --offline --locked -p firmware-tools --bin modem-rebuild -- emit-layout /tmp/modem-entry-layout.bin
  python3 tools/assemble-modem-tlb.py --llvm-mc /path/to/llvm6/bin/llvm-mc \
    --prefix /tmp/modem-entry-layout.bin --output /tmp/modem-contiguous-1476.bin \
    --reference /private/modem.mbn
  ```

- The referenced `modem.mbn` SHA-256 must match `6f5cb32bc10e41abb40938d185805e788f5637a41d3ebcef278e4e169fc36782` before a reported match. The next instruction region begins at file `0x35c4` (`0x8e0005c4` physical), reached by the reset trampoline. All other ELF executable segments, data, hashes and signer blocks still need independent reconstruction.

## Checkpoint 05: post-TLB system register startup through `0x3644`

- Independently rebuilt Hexagon V60 system startup at file `[0x35c4,0x3644)` (128 executable bytes/32 architecture words), physical `[0x8e0005c4,0x8e000644)`, using new `modem_sys_boot.s`. Privileged `S18`, `S28`, `S7`, `S25`, `S23` and system-register pair writes to `S30d` and `S4d` use checked field formulas; standard instructions are compiled by LLVM 6 Hexagon MC. The pair-write semantics were cross-checked against the separately supplied Hexag00n V5/V55 instruction templates, not inferred solely from the OEM byte sequence.
- The packet-parse layout matters: two adjacent extended-immediate moves (`r0 = -4096`, `r26 = 0xc0800000`) are in **one** four-word packet. The destination of the last `call` at offset `0x3640` is symbolically expressed as `modem_post_tlb + 0xad4` (file offset `0x4098`), rather than inserted as an opaque opcode.
- `tools/assemble-modem-system.py` validates the ELF32 Hexagon assembler object, enforces the 128-byte region, optionally takes a previously generated 1,476-byte prefix, and checks the SHA-256 of the **entire** private modem reference before comparing any bytes. Four new Python tests check validation and source properties.
- The now contiguous independently rebuilt partial modem interval is file `[0x3000,0x3644)` (1,604 bytes): **392 executable bytes**, **412 structured metadata bytes**, and **800 explicitly modeled alignment bytes**. The region SHA-256 is `b804a6aa5a756c30931b5b57d0d522ee7076f725d7915767868b3caf42a401bc`; the new 128-byte routine SHA-256 is `763dea714e79c8516a04926414e3a0fefd873bb5042842398a0fce62e2d28d77`. There are **zero differences** at this exact reference revision.
- To regenerate: build `modem-rebuild emit-layout`, then `assemble-modem-tlb.py` with the generated prefix, then `assemble-modem-system.py --llvm-mc /path/to/llvm-mc --prefix /tmp/boot-1476.bin --output /tmp/boot-1604.bin --reference /private/modem.mbn`. Everything beyond file `0x3644` remains unimplemented, including most executable bytes of the first and later load segments and the authentication metadata. No complete/flashable image is claimed.
- Validation: `cargo test --workspace --offline --locked`, `cargo build --workspace --release --offline --locked`, Python unit tests (21 passing), `git diff --check`, revision hash, independent LLVM compilation, and full 1,604-byte comparison all succeeded.

## Checkpoint 06: early cache synchronization and resource lookup through `0x36e0`

- Independently rebuilt Hexagon V60 bootstrap continuation at file `[0x3644,0x36e0)` (**156 executable bytes**, 39 ISA words), physical `[0x8e000644,0x8e0006e0)`. The source `modem_boot_dispatch.s` reconstructs memory-mapped bootstrap state, conditional `l2kill`/`syncht`, revision detection by reading `S29`, and a pair of conditional resource-table lookups. This code depends on the prior 128-byte `modem_sys_boot.s` and its relocation-base registers; it is not a separate firmware.
- Historical LLVM 6 MC independently assembles the standard instructions. For the unsupported privileged read `S29` and L2-kill operation, checked Hexagon ISA bit-field macros are used. The external Hexag00n decoder corroborates the privileged register source as `S29` and interprets the cache instruction as `l2kill`.
- Preserving packet grouping is essential: the status-register read shares a packet with the following extended immediate, and predicate/branch operations share their instruction word and controller loads. The original packet parse-marker positions were independently regenerated, with no raw copied OEM instruction array.
- `tools/assemble-modem-dispatch.py` is a strict ELF32 Hexagon assembly output reader. It can emit the standalone 156-byte region or append it to a previously generated 1,604-byte prefix; it rejects a wrong modem reference hash, wrong prefix length or differing firmware byte. Three new Python tests check its boundaries and source representation.
- The initial contiguous modem interval is now `[0x3000,0x36e0)` (**1,760 bytes**): **548 executable bytes**, **412 bytes of structured metadata** and **800 fixed alignment bytes**. Its SHA-256 is `840d07f7e346cbd34768a0f87c9ddd61d282c294bf6398eb6a43ee7583a9590c` (zero reference differences). The next uncovered byte is file offset `0x36e0`, physical `0x8e0006e0`.
- Validation: LLVM 6 architecture assembly, revision hash, complete comparison, Rust workspace tests, offline release build, Python tests (24 passed), and `git diff --check` passed. This remains an early partial executable reconstruction: the remaining ELF load segments and OEM signature/certificate are not recreated, and the result is not independently signed or flashable.

## Checkpoint 07: four table-driven modem bootstrap resource searches

- Added 288 more independently reassembled executable bytes at file `[0x36e0,0x3800)` (physical `[0x8e0006e0,0x8e000800)`), completing four bounded image-map resource lookups. Each loop loads a count, walks parallel value/key tables via post-incremented pointers, matches the S29-derived selector in `r12`, and updates the corresponding controller status pointer. These are normal Hexagon v6 operations, with LLVM v6 assembly as independent corroboration.
- Extended `modem_boot_dispatch.s` and checked `tools/assemble-modem-dispatch.py`: LLVM 6's scheduler puts `if (p0.new) jump` before the corresponding `p0=cmp.eq(...)` in four legal three-word packets. The build now verifies expected instruction classes and packet-end markers, then restores producer-first order. It never copies original OEM opcodes and fails closed if an instruction class or layout changes. Unit tests verify the four restoration sites and malformed-length failures.
- The independent contiguous reconstruction now covers modem file `[0x3000,0x3800)` (**2,048 bytes** total): **836 independently reassembled executable bytes** (68 reset + 196 TLB + 128 privileged post-TLB + 444 dispatch/table lookup), **412 typed metadata bytes**, and **800 deterministic alignment-fill bytes**. SHA-256 `e045a32c274707a453edc89d49f25e9985856a5307af3709ab040a3bb9390caa`, with zero differences against the exact private modem reference.
- Reproduce using the earlier 1,604-byte prefix and `python3 tools/assemble-modem-dispatch.py --llvm-mc /path/to/llvm-mc --prefix /tmp/modem-boot-1604.bin --output /tmp/modem-boot-2048.bin --reference /private/modem.mbn`. When using the current branch, `assemble-modem-dispatch.py` emits 444 bytes without a prefix (rather than checkpoint 6's shorter 156-byte version).
- Rust workspace tests, offline release build, Python tests (25 passing), `git diff --check`, and revision-specific reference check all pass. Next uncovered byte is modem file `0x3800` (physical `0x8e000800`); the rest of the modem, the large later executable region, and OEM authentication remain unimplemented. The 2,048-byte artifact is **not a flashable firmware image**.

## Checkpoint 08: selected page registers, MMU/cache flags through `0x38a8`

- Reconstructed another 168 executable bytes at `[0x3800,0x38a8)` (physical `[0x8e000800,0x8e0008a8)`), including the controller revision byte, optional revision-byte correction, privileged system-register writes to `S60,S63,S62,S61,S59,S35`, bit 21 clearing of `S63`'s input, cache option flags and a synchronized `S18` enable.
- The Hexag00n V5/V55 disassembler expresses immediates in hexadecimal without a `0x` prefix. Original `cmp.eq(r4,#64)` therefore means decimal 100 (`0x64`), while `clrbit(r0,#15)` means index 21 (`0x15`). The LLVM6 assembly uses **decimal** `#100` and `#21` to emit the identical encodings. One three-instruction packet in this interval is reordered by LLVM's scheduler: branch before condition producer. The checked `restore_predicate_order` reverses that ordering, validates instruction class/packet markers and writes the same legal producer-first encoding; it contains no OEM opcode arrays.
- The independent contiguous modem executable/data bootstrap is now file `[0x3000,0x38a8)` (**2,216 bytes**): **1,004 independently regenerated Hexagon executable bytes**, 412 structured boot metadata, 800 fixed alignment bytes. The reconstructed total SHA-256 is `b823a1ba941678739b7908e1a4fc66974a86e633fdad24a5f7e3a8122058052d`, with no differences against the exact modem reference revision.
- `assemble-modem-dispatch.py` now compiles the combined 612-byte code section `[0x3644,0x38a8)`; when supplied its independently rebuilt 1,604-byte prefix it verifies and emits the 2,216-byte total. The four repeated search packets and one revision-branch packet are both checked against Hexagon architecture class masks before adjusting LLVM's scheduling.
- Full Rust workspace, offline release build, 25 Python tests, `git diff --check`, independent LLVM6 build, Hexag00n ISA evidence and all 2,216 original bytes check out. The first unknown byte is at file `0x38a8` (physical `0x8e0008a8`). No entire modem MBN or OEM signing credentials are reproduced.
