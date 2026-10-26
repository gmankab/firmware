# Agent 20-03 — Venus late continuation

## Base and ancestry

Agent 20-03 owns the Venus ARM runtime interval **`[0x21c78,0x40000)`**, whose first ELF text `PT_LOAD` file offsets are `[0x25c78,0x44000)`. The provided original checkpoint-50 (`b7ed4b587633b9b475a40f5a3ace5cc90db620ff`) / post-integration base is *not mounted* in this session. This branch is therefore a **portable extension of prior Agent 03 checkpoint-23 (`a3ffc58da64d8f96a65b9a48f4bed24592ba0ac1`)**. It is not a verified descendant of checkpoint 50 or a merged ten-agent tree. The integration owner should apply/cherry-pick once an authentic checkpoint-50 integration base exists.

Original reference: private `map220v-nabu-firmware/venus.mbn`, SHA-256 `9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085`. No proprietary firmware, signed metadata or executable reference bytes are committed.

## Checkpoint 01 — four adjacent decoder routines

- Newly source-generated ARM32 runtime region: **`[0x21c78,0x22040)`**, 968 contiguous bytes, corresponding to MBN file `[0x25c78,0x26040)`.
- **221 typed ARM instructions** reconstructed into `venus_runtime_mid.rs` using checked ARM encoders and named register operands. **21 named data words** preserved separately, not misclassified as opcodes.
- Functions: image-format size allocation/branch, three-field descriptor size helper, primary descriptor comparison, record-count comparison.
- Code/data intervals: 12-handler jump table `[0x21c98,0x21cc8)`, diagnostic word `[0x21eb8,0x21ebc)`, four diagnostics each `[0x21f7c,0x21f8c)` and `[0x22030,0x22040)`.
- `cargo run --offline --locked -p firmware-tools --example agent20_03_venus_mid -- verify /private/venus.mbn` checks exact bytes and reports the first mismatch if any.
- Independent Clang/LLVM A32 mnemonic reassembly (with relocation linking, separately typed non-instruction words) also matches all 968 bytes.
- The next unreconstructed instruction address is `0x22040`; no overlap with the original Agent 03 region ending at `0x21c78`.
- Instruction-level byte equivalence is not evidence of recovered high-level business logic, device execution, or OEM signing. The overall firmware remains incomplete.

## Checkpoint 02 — codec descriptor preparation and stride calculations

- Source-generated contiguous runtime: **`[0x21c78,0x2265c)` = 2,532 bytes**, reference MBN file `[0x25c78,0x2665c)`.
- **601 typed A32 instructions** and **32 non-executable data words**. Added an independently encoded ARM `UMULL`/`SMULL` checked helper and explicit tests; no raw instruction bytes.
- Complete descriptor flag initialization `[0x22040,0x22164)` and codec stride/resource allocator `[0x22164,0x2265c)`. The latter has eleven literal constants `[0x22630,0x2265c)` including reciprocal-multiply constants and diagnostic pointers.
- Original `venus.mbn` direct byte comparison: zero mismatches across the complete 2,532-byte interval.
- Independent Clang/LLVM assembler/linker round-trip: 2,532/2,532 bytes, including classification of all five literal pools, pass. Verifier script `tools/agent20-03-a32-mnemonic-roundtrip.py` takes a disassembly derived from a locally held reference and does not import OEM instruction words as source.
- Next unreconstructed runtime address `0x2265c`, at the start of the next function.

## Checkpoint 03 — input vectors, channel registration and codec flags

- Runtime `[0x21c78,0x22a44)` = **3,532 contiguous independently emitted bytes** (MBN `[0x25c78,0x26a44)`): 845 A32 instructions and 38 non-executable data words.
- Newly completed `[0x2265c,0x22a44)` = 244 ARM instructions and 6 explicit data words. Distinct code paths implement input-vector packing, controller command/callback setup and channel state inspection.
- New literal pools: controller ID `[0x227dc,0x227e0)`, diagnostic pointers `[0x229d8,0x229e4)`, codec table offsets `[0x22a3c,0x22a44)`.
- Direct private-reference MBN comparison and independent Clang/LLVM assembler/linker round-trip both passed **all 3,532 bytes with zero mismatch**. The next function starts at `0x22a44`.

## Checkpoint 04 — codec controller command emission

- Contiguous VMA `[0x21c78,0x22fac)` = **4,916 independently source-encoded bytes**, MBN file `[0x25c78,0x26fac)`. Original/reference diff: zero mismatches.
- Typed **1,174 ARM32 instructions** and **55 explicitly typed data words**, including 17 data words `[0x22f68,0x22fac)` for diagnostics, control tables and peripheral/resource pointers.
- Newly completed `[0x22a44,0x22fac)` includes data validation, buffer configuration, fault logging, and branch back to platform callback `0x2a158`.
- Independent Clang/LLVM mnemonic reassembly and linker relocation handling independently confirm all 4,916 bytes; next function begins `0x22fac`.

## Checkpoint 05 — codec register-state loop

- Source-generated `[0x21c78,0x2326c)` = **5,620 contiguous byte-identical bytes**, private reference MBN file `[0x25c78,0x2726c)`.
- **1,342 ARM32 instructions** and **63 classified data words** total. Newly reconstructed `[0x22fac,0x2326c)` includes 168 A32 operations and 8 literal words `[0x2324c,0x2326c)` for peripheral base addresses and diagnostic tables.
- Verified each new byte twice: typed Rust generated output versus private original, and independent Clang/LLVM mnemonic assembly plus relocation linking versus the generated output. No unknown source filler or original instruction opcodes imported.
- Next routine begins at runtime `0x2326c`.

## Checkpoint 06 — controller validation and logging helpers

- Runtime `[0x21c78,0x23840)` = **7,112 contiguous byte-identical bytes**, MBN `[0x25c78,0x27840)`.
- **1,699 typed ARM32 instructions** and **79 explicitly separated non-instruction data words** in total.
- Newly reconstructed `[0x2326c,0x23840)` comprises 357 A32 instructions and 16 data words in nine literal pools; code includes per-plane bounds checking, buffer event logging, MMIO/pool registration, vector lookup and callback configuration.
- Independent Clang/LLVM mnemonic reassembly (including condition-specific PC-relative literal loads and byte-postindex updates) matches all 7,112 bytes.
- Next uncovered code begins at `0x23840`, a function that branches through controller format tables.

## Checkpoint 07 — buffer allocation modes and callback message

- VMA `[0x21c78,0x23cf0)` = **8,312 independently emitted byte-identical bytes**, MBN file `[0x25c78,0x27cf0)`.
- Total **1,974 named/checked ARM A32 instructions**, **104 classified data words**.
- New `[0x23840,0x23cf0)` includes a seven-way format selection table `[0x238ac,0x238c8)`, descriptor-size enforcement, error callback paths, 18 separate non-instruction diagnostics/control addresses `[0x23c60,0x23ca8)`, and a complete 72-byte command-message helper `[0x23ca8,0x23cf0)`.
- Independently assembled using Clang + LLD and compared all generated bytes against pinned Venus MBN. Next unresolved runtime entry is `0x23cf0` (inside this agent's exclusive `[0x21c78,0x40000)` range).
- The emitted bytes are instruction-level re-encoding, not source-level semantic completeness or signed deployable firmware. The exact checkpoint-50 ancestor remains unavailable; branch is a portable extension of earlier Agent03 checkout.

## Checkpoint 08 — nineteen-way codec mode validator

- New contiguous span `[0x21c78,0x24cd4)` = **12,380 source-encoded bytes**, MBN file `[0x25c78,0x28cd4)`, byte-identical against the pinned reference.
- Totals: **2,926 ARM32 instructions** and **169 named non-instruction data words**, independently assembled with Clang/LLD and compared to original and Rust emitter.
- New routine `[0x23cf0,0x24cd4)` (4,068 bytes) has nineteen explicit jump targets `[0x23d44,0x23d90)`, then codec size/frame profile validation, error logging and 46 separately identified diagnostic/offset values `[0x24c1c,0x24cd4)`.
- The next uncovered code begins at runtime `0x24cd4`, an independent record-copy routine. This remains a portable branch because checkpoint 50 was not supplied; no OEM signing or high-level source semantics have been established.

## Checkpoint 09 — bounded pixel-plane record transfer

- Contiguous VMA `[0x21c78,0x24f90)` = **13,080 generated byte-identical bytes**, reference MBN `[0x25c78,0x28f90)`.
- Total **3,091 ARM A32 instructions** and **179 explicitly classified non-instruction words**, independently assembled with Clang/LLD.
- New routine `[0x24cd4,0x24f90)` reconstructs dimension checks, supported byte-width formats, byte-indexed record lookup, and 10 literal diagnostic values at `[0x24f68,0x24f90)`. Exactly 165 new instructions and 10 data words.
- First uncovered function begins at `0x24f90`, still inside this agent's reservation. No new OEM signature or flashable image is produced.

## Checkpoint 10 — codec output records and replayable literal-pool ledger

- VMA `[0x21c78,0x253fc)` = **14,212 byte-identical source-encoded bytes**, reference MBN `[0x25c78,0x293fc)`.
- Totals: **3,326 typed A32 instructions** and **227 explicit non-instruction data words**, across 28 data pools. Independent Clang/LLD pass confirms all generated bytes and the reference.
- New functions: indexed codec-mode handler `[0x24f90,0x25118)` (20-entry branch table plus 14 literal words), per-session record synchronization `[0x25118,0x252a8)` (8 data words), and four-output-plane callback dispatcher `[0x252a8,0x253fc)` (6 words).
- `tools/agent20-03-venus-mid-ranges.json` records all jump-table and constant-data intervals separately. `tools/agent20-03-verify-all.py` uses it to drive independent A32 reassembly; it rejects partial/overlapping pools. It reads original MBN only for independent verification, never for the Rust generator.
- Next uncovered function begins at `0x253fc`.

## Checkpoint 11 — capture state machine

- Runtime `[0x21c78,0x25700)` contains **14,984 independently source-encoded and byte-identical bytes**; reference file range `[0x25c78,0x29700)`.
- Totals: **3,505 ARM32 instructions and 241 separately identified data words**, independently rebuilt using Clang/LLD and confirmed against the original MBN.
- New completed capture-state routine `[0x253fc,0x25700)` contributes 179 checked typed A32 operations and a 14-word diagnostic pool `[0x256c8,0x25700)`, with buffer state transitions, callback conditions and flags. Next routine begins at `0x25700`.

## Checkpoint 12 — peripheral register access and callback wrappers

- Contiguous `[0x21c78,0x25b1c)` = **16,036 source-encoded and byte-identical bytes**, reference MBN `[0x25c78,0x29b1c)`.
- Full region contains **3,729 checked typed ARM A32 instructions and 280 non-instruction words**, identified across 57 literal pools; Clang/LLD reassembly independently matches every byte.
- New `[0x25700,0x25b1c)` contributes 224 ARM32 instructions and 39 literal words in 28 pools, covering controller-register getters/setters, hardware-vendor table lookups, stream logging, and platform initialization/teardown callback wrappers.
- Next uncovered function begins at runtime `0x25b1c`; prior `[0x21c78,0x25b1c)` code is complete as A32 encoding, not a full high-level firmware source decompilation or signed firmware image.

## Checkpoint 13 — controller interrupt slots and MMIO initialization

- Contiguous, directly byte-verified VMA **`[0x21c78,0x25d14)` = 16,540 bytes**, MBN file **`[0x25c78,0x29d14)`**. All bytes match pinned SHA-256 Venus reference (`9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085`).
- New `[0x25b1c,0x25d14)` = **504 bytes**: 111 typed ARM32 instructions and **15 non-instruction literal words**. Implements assigning/removing indexed callback slots, controller mask/acknowledge/reset and hardware startup fields. Named literal pools at `[0x25b58,0x25b5c)`, `[0x25c0c,0x25c24)`, `[0x25c5c,0x25c60)`, `[0x25c88,0x25c8c)`, `[0x25cfc,0x25d14)`.
- Entire emitter now comprises **3,840 A32 instructions and 295 typed data words in 62 pools**. Independent Clang/LLD reconstruction (using separately classified mnemonic/data intervals) reproduces all 16,540 bytes exactly; no executable reference bytes are copied into the Rust generator. `tools/agent20-03-venus-mid-ranges.json` includes all new pools.
- Next missing runtime instruction is `0x25d14`, within Agent 20-03's exclusive `[0x21c78,0x40000)` reservation.

## Checkpoint 14 — platform IRQ dispatch initialization and shutdown

- Source-generated `[0x21c78,0x26014)` = **17,308 consecutive byte-identical bytes**, MBN file `[0x25c78,0x2a014)`.
- Newly re-encoded `[0x25d14,0x26014)` = **768 bytes** comprising **176 checked ARM32 instructions** and **16 explicitly classified data words** (13 platform callback addresses/diagnostics `[0x25f90,0x25fc4)`, 1 pending-indicator address at `0x25fe4`, 2 IRQ clear/address fields `[0x2600c,0x26014)`). Covers registration of handler IDs, allocation/rollback, status clearing and callback notification.
- Full fragment: **4,016 A32 instructions, 311 non-instruction words in 65 named pools**. Clang/LLD independent mnemonic reassembly reproduced every byte. Reference SHA-256 pinned in the earlier checkpoints; OEM signing and hardware behavior remain unverified.
- Next missing instruction/runtime function starts at `0x26014`, still below the exclusive reservation upper bound `0x40000`.

## Checkpoint 15 — pending-event dispatch and register-state synchronization

- Continuous exact source-generated interval `[0x21c78,0x2634c)` = **18,132 bytes**; MBN file `[0x25c78,0x2a34c)`.
- New `[0x26014,0x2634c)` = **824 bytes**: **201 ARM32 instructions** and **five literal address/data words** `[0x26338,0x2634c)`.
- Implemented pending IRQ bit loop, prioritized callback dispatch, repeated state/resource interrogation, and synchronization of controller command status before exiting the dispatcher; data constants are separate from executable instructions.
- Full interval independent Clang/LLD round-trip verifies **4,217 instructions and 316 data words** in 66 disjoint literal pools, zero byte mismatches. Next uncovered address `0x2634c`.

## Checkpoint 16 — IRQ release, accessor wrappers and controller-line masks

- Contiguous VMA `[0x21c78,0x265e4)` = **18,796 byte-identical typed bytes**, private MBN file `[0x25c78,0x2a5e4)`.
- Newly encoded `[0x2634c,0x265e4)` = **664 bytes**: **151 A32 instructions and 15 separate non-instruction words** in twelve literal pools. First function releases IRQ channels for six hardware sources; nine regular-index accessor wrappers are emitted from a semantic selector/field table; final two routines initialize paired IRQ lines with conditional channel masks.
- Cumulative independent mnemonic reassembly matches **4,368 instructions and 331 data words in 78 pools**; all previous code remains byte-identical to pinned `venus.mbn`. Next uncovered routine begins at `0x265e4`.

## Checkpoint 17 — IRQ masks, indexed channel accessors and controller reset

- Contiguous typed A32 runtime `[0x21c78,0x2693c)` = **19,652 exact bytes** (MBN file `[0x25c78,0x2a93c)`).
- Newly generated `[0x265e4,0x2693c)` = **856 bytes**, **195 A32 instructions and 19 named MMIO/data words in seven pools**, covering enable/clear IRQ masks for channel classes and DMA clocks, platform handler dispatching, pending-state acknowledgement, repeated MMIO read barriers and final hardware reset path.
- Independently rebuilt in Clang/LLVM the entire cumulative source span: **4,563 ARM instructions and 350 data words in 85 separately classified literal pools**; no mismatched bytes.
- Next uncovered function begins at `0x2693c`.

## Checkpoint 18 — controller handshake and IRQ power-state transitions

- Contiguous `[0x21c78,0x26c60)` = **20,456 byte-identical bytes** (file `[0x25c78,0x2ac60)`).
- New `[0x2693c,0x26c60)` = **804 bytes**, **183 A32 instructions and 18 non-instruction words** in three separate literal pools `[0x26aa0,0x26ac4)`, `[0x26af0,0x26af8)`, `[0x26c44,0x26c60)`.
- Handles controller sleep/resume handshake, IRQ pending state, status bit acknowledgement and repeated controller MMIO polling. Source-generated A32 is independent of the OEM binary, which is read only by verifiers.
- Cumulative independent Clang/LLVM round-trip matches **4,746 instructions, 368 data words in 88 pools**, with zero byte differences. Next missing function begins at `0x26c60`.

## Checkpoint 19 — controller MMIO reset and IRQ power-state transition

- Contiguous `[0x21c78,0x27180)` = **21,768 byte-identical typed bytes**, reference file `[0x25c78,0x2b180)`.
- New `[0x26c60,0x27180)` = **1,312 bytes**, **306 A32 instructions and 22 named non-instruction values** in two pools (`[0x26dcc,0x26de0)`, `[0x2713c,0x27180)`). Includes repeated MMIO flush/read barriers, controller power/IRQ reset, event-channel handshake flags and diagnostic callback paths.
- Cumulative independent LLVM/Clang verification passes all **5,052 instructions and 390 data words across 90 pools** without byte differences. The next uncovered routine is `0x27180`.

## Checkpoint 20 — controller readiness and descriptor bounds

- Runtime `[0x21c78,0x273a4)` = **22,316 generated, byte-identical bytes**, reference MBN `[0x25c78,0x2b3a4)`.
- New `[0x27180,0x273a4)` = **548 bytes**, with **122 A32 instructions** and **15 separately identified MMIO/diagnostic words** in three literal pools. It implements controller readiness probes, per-stream descriptor range checks and IRQ availability transitions.
- Cumulative independent Clang/LLD mnemonic round-trip reproduces all **5,174 instructions and 405 non-instruction words in 93 pools**, zero byte differences. First missing routine begins at `0x273a4`.

## Checkpoint 21 — queued IRQ barriers and codec resource shutdown

- Contiguous A32 source-generated `[0x21c78,0x27684)` = **23,052 reference-exact bytes**, file `[0x25c78,0x2b684)`.
- New `[0x273a4,0x27684)` = **736 bytes**, **170 ARM instructions** and **14 named MMIO/diagnostic data words** in five pools. Implements posted-write readback waits, controller queue polling and teardown, and per-codec resource shutdown helpers.
- Independent Clang/LLD reassembly confirms the complete cumulative **5,344 instructions and 419 data words in 98 pools** with zero differences. Next uncovered codec routine starts at runtime `0x27684`.

## Checkpoint 22 — codec IRQ queue selectors and resynchronization

- Byte-identical typed A32 `[0x21c78,0x278fc)` = **23,684 consecutive bytes** (file `[0x25c78,0x2b8fc)`).
- New `[0x27684,0x278fc)` = **632 bytes**, **144 A32 instructions and 14 named MMIO/diagnostic address words** across five literal pools at `[0x276a8,0x276ac)`, `[0x276e8,0x276f0)`, `[0x27714,0x27718)`, `[0x27828,0x27840)`, and `[0x278e8,0x278f8)`; final `bx lr` at `0x278f8` is executable code.
- Handles codec IRQ window assignment, queue source selection, event/callback attachment and synchronization of hardware controller channel state.
- Cumulative Clang/LLD independent mnemonic reassembly matches **5,488 instructions and 433 data words across 103 literal pools**, zero byte mismatches against pinned reference. Rust 194 tests, Python 13 tests and the release build pass.
- Next uncovered runtime address is `0x278fc`; fragment is neither complete independent high-level firmware source nor flashable/OEM-signed firmware.

## Checkpoint 23 — controller-frame MMIO and active callback binding

- Full contiguous A32 interval `[0x21c78,0x27ca4)` = **24,620 exact generated bytes**, reference MBN file offsets `[0x25c78,0x2bca4)`.
- Newly reconstructed `[0x278fc,0x27ca4)` = **936 bytes**, consisting of 220 typed instructions and 14 classified non-instruction literal words `[0x27c6c,0x27ca4)`.
- Writes codec controller command registers, handles host resource assignments, configures callbacks and mode-sensitive DMA/status paths, and selects input/output queue state. The pool contains hardware addresses, branch targets, diagnostic pointers and two IEEE float calibration words, not executable A32 instructions.
- Independent Clang/LLVM full-region mnemonic reassembly matches **5,708 instructions and 447 data words in 104 pools** with zero byte differences. The three absolute external BLX targets result in non-STT_FUNC ARM interworking link warnings, but linked bytes match the vendor reference; this is not device execution proof.
- Full Rust/Python suites and release build pass. Next missing runtime instruction at `0x27ca4`.

## Checkpoint 24 — codec frame policy and controller-state reporting

- Contiguous, byte-identical typed Venus interval `[0x21c78,0x280f0)` = **25,720 bytes**, MBN file `[0x25c78,0x2c0f0)`.
- New `[0x27ca4,0x280f0)` = **1,100 bytes** comprising **259 A32 instructions and 16 separate literal words**. Controller frame configuration uses seven MMIO/callback words at `[0x27ea4,0x27ec0)`; the subsequent multichannel state reporter uses nine words `[0x280cc,0x280f0)`.
- Reconstructed typed A32 covers frame and codec IRQ queue allocation, hardware MMIO transactions, record count and timeout reporting, and conditional callback registration. Data literals remain separated from executable code.
- Independent Clang/LLD linked mnemonic reassembly matches the complete fragment: **5,967 instructions and 463 data words in 106 literal pools**, zero byte differences. Three external BLX relocations raise linker symbol-type interworking warnings but do not affect exact output.
- Rust 194 tests, Python 13 tests and release build pass. Next uncovered function begins at `0x280f0`.

## Checkpoint 25 — codec DMA input planes and descriptor-registry helpers

- Contiguous typed A32 runtime `[0x21c78,0x28568)` = **26,864 byte-identical bytes**, original MBN `[0x25c78,0x2c568)`.
- Newly reconstructed `[0x280f0,0x28568)` = **1,144 bytes**, **262 A32 instructions** and **24 independent data words** in four pools: plane reset `[0x28290,0x282b0)`, DMA bindings `[0x28450,0x28478)`, queue-channel map `[0x28510,0x28520)`, and register diagnostics `[0x28560,0x28568)`.
- Includes controller plane descriptors, codec DMA queues, platform callback registration and bounded channel reporting, with literal tables separated from actual executable A32 instructions.
- Full Clang/LLVM mnemonic reassembly independently matches **6,229 instructions and 487 non-instruction data words** across **110 literal pools**. Three external BLX linker warnings remain harmless for byte identity, not a hardware semantics claim.
- All 194 Rust tests, 13 Python tests and release build pass; next executable routine begins at `0x28568`.

## Checkpoint 26 — packed codec plane transfer sequencer

- Contiguous typed A32 region `[0x21c78,0x28744)` = **27,340 byte-identical bytes** (original MBN file `[0x25c78,0x2c744)`).
- New `[0x28568,0x28744)` = **476 bytes**, **119 typed executable A32 instructions and zero literal data words**. Implements bounded codec segment reads, 16-bit/32-bit status packing, two-pass DMA/stream register writes and completion frame restore.
- Independent Clang/LLVM full-region reassembly matches **6,348 instructions and 487 distinct data words across 110 classified literal pools**, zero byte differences. Three external BLX symbol typing link warnings do not change emitted bytes.
- All 194 Rust tests, 13 Python tests and release build pass. Next function begins at `0x28744`.

## Checkpoint 27 — active codec-port and MMIO stream registration

- Full continuous typed A32 `[0x21c78,0x28c2c)` = **28,596 reference-exact bytes** (MBN file `[0x25c78,0x2cc2c)`).
- New `[0x28744,0x28c2c)` = **1,256 bytes**, **298 A32 instructions** and **16 separately identified literal words** `[0x28bec,0x28c2c)` comprising MMIO registry pointers, platform handlers and two calibration scalar pairs.
- Includes active codec queue frame setup, hardware MMIO register updates, codec callback registration and conditional status/error reporting. All opaque data values remain separate from executable instructions.
- Independent Clang/LLVM reassembly matches the entire fragment: **6,646 A32 instructions and 503 non-instruction data words across 111 pools**, zero byte differences. Nine external BLX relocation typing warnings are non-fatal; linked bytes match reference exactly.
- The full 194 Rust and 13 Python tests and release build pass. Next uncovered function starts at `0x28c2c`.

## Checkpoint 28 — codec frame teardown and output channel closure

- Contiguous typed A32 `[0x21c78,0x290a4)` = **29,740 byte-identical bytes**, reference file `[0x25c78,0x2d0a4)`.
- New `[0x28c2c,0x290a4)` = **1,144 bytes**, **267 typed ARM32 instructions** plus **19 classified hardware/data words** in pools `[0x28e7c,0x28e9c)`, `[0x28fd4,0x28ff0)` and `[0x29094,0x290a4)`. Three adjacent routines release host/codec channels, clear stale buffer control flags and report completion.
- Full independent LLVM/Clang mnemonic rebuild matches **6,913 instructions and 522 data words across 114 literal pools**, zero byte differences. Nine linker non-STT_FUNC external `BLX` target warnings do not affect equality.
- Rust 194 tests, Python 13 tests and release build pass; next missing codec routine begins at runtime `0x290a4`.

## Checkpoint 29 — codec callback release and MMIO reset

- Continuous source-generated `[0x21c78,0x2929c)` = **30,244 byte-identical bytes**, reference MBN file `[0x25c78,0x2d29c)`.
- New `[0x290a4,0x2929c)` = **504 bytes**, **115 typed A32 instructions** and **11 named MMIO/diagnostic data words** in pools `[0x291d4,0x291f0)` and `[0x2928c,0x2929c)`. Includes controller and output-channel release handlers with queue-state rechecks.
- Full Clang/LLVM independent mnemonic linked reconstruction matches **7,028 instructions and 533 non-instruction data words across 116 classified pools**, zero differences. Existing absolute external BLX symbol warnings affect only symbol metadata, not emitted bytes.
- All 194 Rust tests, 13 Python tests and release build pass. Next missing routine begins at `0x2929c`.

## Checkpoint 30 (recreated from available checkpoint 29) — primary MMIO status reporter

- Source-generated `[0x21c78,0x29474)` = **30,716 bytes**. New `[0x2929c,0x29474)` is **472 bytes**, consisting of **114 typed A32 instructions** and **4 named diagnostic-address words** at `[0x29464,0x29474)`.
- Writes per-channel codec status registers, prepares logs, and conditionally captures another controller bank. Functions/ABI were inferred from instruction behavior; output is instruction-byte identity, **not** proof of source-level semantic equivalence or new OEM authentication.
- Strictly reference-exact from a private `venus.mbn`, including all earlier code; independent Clang/LLD round-trip driven by `agent20-03-venus-mid-ranges.json` is required for release.
- The previous conversation reported checkpoint 30, but that bundle was **not** mounted in the present session. This is a new explicitly recreated checkpoint on top of the mounted checkpoint 29 branch, not a claim that the missing previous checkpoint archive was retrieved.

## Checkpoint 31 (recreated from available checkpoint 29) — IRQ readback and acknowledgment

- Continuous source-generated `[0x21c78,0x296f0)` = **31,352 bytes**, MBN file `[0x25c78,0x2d6f0)`. New `[0x29474,0x296f0)` = **636 bytes**, consisting of **151 ARM32 typed instructions** and **8 separately identified MMIO/control words** in seven literal pools.
- Seven compact routines implement codec/status IRQ reads, DMA interrupt acknowledgement, and queue notification. Pool locations: `0x29490`, `0x294ec`, `0x2954c`, `0x295ac`, `0x29604`, `0x29690–0x29698`, and `0x296ec`.
- Total `[0x21c78,0x296f0)` contains **7,293 instructions and 545 data words**, with no copied OEM instruction encodings. The original function flow, comments and opcode equivalence are not proof of independent high-level semantic decompilation or device execution.
- Existing checkpoint-31 Git bundle from a previous session was not mounted; this is a byte-identical recreation on top of recovered checkpoint 29. The next missing ARM runtime address is `0x296f0`.

## Checkpoint 32 — priority-based codec IRQ acknowledgment

- Contiguous `[0x21c78,0x29850)` = **31,704 reference-exact bytes** (MBN `[0x25c78,0x2d850)`). The new `[0x296f0,0x29850)` contains **86 typed A32 instructions** and **2 MMIO controller data words** `[0x29848,0x29850)`.
- Handles prioritized interrupt pending bits, conditional command-register writes and host interrupt-counter synchronization before a callback. The final return branches to previously reconstructed queue notify at `0x292c` (external target).
- Strict full-fragment reference comparison and independent Clang/LLD assembler comparison are required. Subsequent pending helper begins at `0x29850`.

## Checkpoint 33 — DMA completion, encoder error masks and capture metadata

- Byte-exact typed ARM32 coverage `[0x21c78,0x29ae4)` = **32,364 consecutive bytes** (MBN file `[0x25c78,0x2dae4)`). New `[0x29850,0x29ae4)` = **660 bytes**, comprising **156 instructions and nine separate host-address/diagnostic words** in three constant pools.
- Source models DMA completion threshold callbacks, warning-mask readback and capture resource initialization with allocation/error-log paths. Pools are `[0x298d8,0x298e0)`, `[0x29940,0x29944)`, and `[0x29acc,0x29ae4)`. No OEM opcode words are embedded as instructions.
- Independent mnemonic reassembly and complete fragment comparison must pass before release; next missing routine begins at `0x29ae4`.

## Checkpoint 34 — capture-resource configuration and MMIO toggles

- Runtime `[0x21c78,0x2a198)` = **34,080 consecutive byte-identical bytes**, MBN file `[0x25c78,0x2e198)`. New `[0x29ae4,0x2a198)` = **1,716 bytes**: **410 typed ARM32 instructions** and **19 separate MMIO/diagnostic address words**.
- Major initialization routine `[0x29ae4,0x2a158)` constructs controller descriptor buffers and logs allocation/report failures. The 17-word pool `[0x2a114,0x2a158)` contains host pointers, MMIO addresses and message pointers; two small register writer wrappers use single-word data pools at `[0x2a168,0x2a16c)` and `[0x2a188,0x2a18c)`.
- The helper at `[0x2a18c,0x2a198)` tail-branches into the next function. No reference executable bytes are copied into the Rust source. Next function starts at `0x2a198`; the generated bytes are not complete firmware or proof of hardware functionality.

## Checkpoint 35 — codec format policy, three switch tables and diagnostic pointers

- Contiguous typed A32 region `[0x21c78,0x2a878)` = **35,840 exact bytes** (MBN `[0x25c78,0x2e878)`). New `[0x2a198,0x2a878)` = **1,760 bytes** comprising **335 typed ARM32 instructions** and **105 non-instruction address/data words** in four pools.
- The three branch-target tables live at `[0x2a2b4,0x2a2f8)` (17 entries), `[0x2a31c,0x2a354)` (14 entries), and `[0x2a378,0x2a390)` (6 entries). Last `[0x2a768,0x2a878)` contains 68 MMIO/diagnostic/capability words. They are modeled as data; arbitrary bit patterns in them may disassemble as ARM instructions but are **not** executable.
- Performs codec-format dispatch, validates command metadata and routes controller resource/diagnostic statuses. Byte-exact A32 re-encoding is not high-level semantic or hardware behavior verification. Next routine begins at runtime `0x2a878`.

## Checkpoint 36 — frame submission and codec capture release

- Exact A32 region `[0x21c78,0x2aad8)` = **36,448 contiguous bytes** (MBN file `[0x25c78,0x2ead8)`). New `[0x2a878,0x2aad8)` = **608 bytes**, containing **146 typed instructions** and **six non-executable data words**.
- First routine selects an active frame descriptor, clamps/forwards the stream to the shared codec packet builder, and returns through a tail branch to `0x2a198`. Second group prepares buffers and frees codec capture resources, including host callbacks and explicit diagnostic data.
- Literal pools `[0x2a98c,0x2a990)` and `[0x2aac4,0x2aad8)` are never decoded as instructions. First missing routine begins at `0x2aad8`. This is instruction-level reconstruction only; no new OEM authentication or hardware behavior has been established.

## Checkpoint 37 — nine-way control-message dispatcher

- Strict source-emitted A32 `[0x21c78,0x2b13c)` = **38,084 byte-identical bytes** (MBN `[0x25c78,0x2f13c)`). New `[0x2aad8,0x2b13c)` = **1,636 bytes** consisting of **367 ARM32 instructions** and **42 structured data words**.
- Dispatcher has a nine-way branch table `[0x2ab50,0x2ab74)` with repeated route targets, plus 33 separate MMIO pointers, diagnostic addresses and controller state words `[0x2b0b8,0x2b13c)`.
- Handles command messages, controller synchronization, callback/clock state transitions and error reporting. This is typed instruction-level re-encoding only; no claim of independent source-language equivalence, OEM authentication or hardware execution.
- Next missing code starts `0x2b13c`.

## Checkpoint 38 — MMIO configuration and service message checks

- Continuous typed A32 `[0x21c78,0x2b304)` = **38,540 exact bytes** (MBN `[0x25c78,0x2f304)`). The additional `[0x2b13c,0x2b304)` is **456 bytes**, **105 instructions and nine data words**.
- Three compact routines configure the codec IRQ/MMIO bank and validate service commands with explicit error branches. MMIO constant pools at `[0x2b174,0x2b178)` and `[0x2b1c4,0x2b1cc)` and the six-word diagnostic table at `[0x2b2ec,0x2b304)` are deliberately excluded from executable instruction count.
- Next routine begins at `0x2b304`. This is instruction-level byte identity, not source-language behavioral proof or a signed firmware.

## Checkpoint 39 — codec startup and CP15 synchronization

- Continuous typed ARM `[0x21c78,0x2b474)` = **38,908 byte-identical bytes**. New `[0x2b304,0x2b474)` = **368 bytes**, **80 A32 instructions** and **12 controller address/data words**.
- Setup restores saved MMIO registers, resets IRQ state, and executes an explicitly checked ARMv7 CP15 `MCR p15, #0, r4, c7, c0, #4` barrier at `0x2b3e4`. The twelve host/MMIO/diagnostic pointers `[0x2b444,0x2b474)` are data, not executable instructions.
- The next function begins at runtime `0x2b474`. Source-to-reference equality does not imply independent high-level behavioral equivalence or a signed image.

## Checkpoint 40 — codec event routing with two switch tables

- Full byte-matched interval `[0x21c78,0x2b774)` = **39,676 typed bytes** (original MBN `[0x25c78,0x2f774)`). New `[0x2b474,0x2b774)` = **768 bytes**, **167 ARM32 instructions**, **25 handler-target data words**.
- A larger event processing function begins at `0x2b474`; this checkpoint covers its register/status setup, callback routing and branch destinations, including two tables `[0x2b6e4,0x2b720)` (15 entries) and `[0x2b74c,0x2b774)` (10 entries). Remaining case bodies are still missing beyond `0x2b774`; this fragment is incomplete as an independently callable function.

## Checkpoint 41 — frame routing and three codec jump tables

- Continuous A32 runtime `[0x21c78,0x2ba68)` = **40,432 bytes** exact against `venus.mbn`; additional `[0x2b774,0x2ba68)` = **756 bytes**, **169 instructions and 20 switch-table address words**.
- Buffer-class and format-level checks select dedicated handlers from three tables: `[0x2b944,0x2b968)` (9 entries), `[0x2b988,0x2b9a4)` (7), and `[0x2ba58,0x2ba68)` (4). Function continues after `0x2ba68` into payload handlers and eventual return; current checkpoint should not be considered standalone callable code.

## Checkpoint 42 — frame message processing and error-path callbacks

- Runtime `[0x21c78,0x2be44)` = **41,420 contiguous bytes** identical to the pinned `venus.mbn`. New `[0x2ba68,0x2be44)` = **988 typed ARM32 instruction bytes** (247 A32 instructions and no literal data words).
- Handles codec buffer type allocation, input descriptor selection, frame limits, callback dispatch and error reporting. This is still a section of the larger codec event function, whose later control flow continues beyond `0x2be44`.

## Checkpoint 43 — codec buffer cleanup and nine-way stream selector

- Generated A32 `[0x21c78,0x2c024)` = **41,900 contiguous byte-identical bytes**, with new `[0x2be44,0x2c024)` **480 bytes** comprising **111 instructions and 9 literal handler addresses**.
- Clears and releases two reference-counted buffer sets using an indirect ARM `BLX`, constructs diagnostics and introduces a nine-entry stream-type dispatch table `[0x2bfd0,0x2bff4)`; completion handlers beyond `0x2c024` remain unimplemented.

## Checkpoint 44 — codec event-class handlers and fourteen-way routing

- Continuous byte-identical A32 `[0x21c78,0x2c5ac)` = **43,316 bytes**. New `[0x2c024,0x2c5ac)` is **1,416 bytes**: **336 instructions and 18 data words**.
- Includes fourteen-entry handler table `[0x2c13c,0x2c174)` plus codec capability/error diagnostic pools `[0x2c568,0x2c574)` and `[0x2c5a8,0x2c5ac)`. Controller message processing remains a larger continuing function after `0x2c5ac`.

## Checkpoint 45 — category-specific codec event handlers

- Contiguous A32 byte identity `[0x21c78,0x2c950)` = **44,248 bytes**. New `[0x2c5ac,0x2c950)` = **932 bytes**, **217 A32 instructions and 16 separately identified pointer/diagnostic data words** in ten small pools.
- A wide array of message-category handlers routes codec buffers into platform callback targets; the interleaved pools carry diagnostic identifiers, hardware masks and invalid-reference offsets rather than executable ARM instructions. The surrounding event routine continues beyond this checkpoint.

## Checkpoint 46 — codec request setup and handler branch table

- Contiguous typed A32 match `[0x21c78,0x2cc94)` = **45,084 bytes**. Newly rebuilt `[0x2c950,0x2cc94)` = **836 bytes**: **189 A32 instructions + 20 separate data words**.
- Handler lookup table `[0x2cb24,0x2cb48)` holds nine runtime target addresses, with two trailing diagnostic addresses through `0x2cb50`. Six other short data pools remain separately classified and are tested as such.
- No opaque executable bytes imported; this is instruction-level re-encoding, not high-level functional decompilation or signed firmware. Next instruction address `0x2cc94`.

## Checkpoint 47 — codec class handlers and diagnostic table

- Contiguous typed A32 match `[0x21c78,0x2d19c)` = **46,372 bytes**. Newly rebuilt `[0x2cc94,0x2d19c)` = **1,288 bytes**: **237 A32 instructions and 85 separate data words**.
- Controller class dispatch branches to several prior codec allocation handlers and updates state; the long non-instruction diagnostic/map table `[0x2d090,0x2d19c)` contains 67 words.
- Literal/data pools are explicit; only typed operations emit executable instructions. This is instruction-level equivalence, not fully recovered high-level source or newly authenticated firmware. First missing runtime address `0x2d19c`.

## Checkpoint 48 — resource initialization and descriptor helpers

- Contiguous byte identity `[0x21c78,0x2d3f4)` = **46,972 bytes**, new `[0x2d19c,0x2d3f4)` = **600 bytes**: **140 ARM32 instructions and 10 data words**.
- Reconstructed a controller initialization routine, bounded resource-descriptor lookup and two small register-condition helpers. Four non-instruction data pools remain identified and tested separately; cross-region calls and branch targets retain original locations.
- First uncovered address `0x2d3f4`, a new controller-configuration function. This is byte-identical instruction re-encoding, not high-level source recovery or newly signed firmware.

## Checkpoint 49 — command register initialization

- Continuous A32 region `[0x21c78,0x2ddfc)` = **49,540 exact bytes**; new `[0x2d3f4,0x2ddfc)` = **2568 bytes** with **632 re-encoded ARM32 instructions** and **10 separate data words**.
- Initializes per-channel command register sets and optional surface planes; encodes ARMv7 `SMULBB` through checked operand fields. A 10-word literal pool begins at `0x2ddd4`, following the function's `POP {..,pc}` at `0x2ddd0`.
- First uncovered executable address: `0x2ddfc` (a new controller initializer). Instruction-byte reconstruction is not source-level behavioral recovery or newly signed firmware.

## Checkpoint 50 — full encoder resource initialization

- Contiguous reference-matched A32 span `[0x21c78,0x2ea50)` = **52,696 bytes**, new `[0x2ddfc,0x2ea50)` = **3,156 bytes** comprising **782 ARM32 instructions** and **7 data words**.
- Builds codec surface descriptor slots, validates the run-state, lays out multiple command lists and uses the public `SMULBB` formula. Seven constant/address words at `[0x2ea34,0x2ea50)` are separate, following the original return at `0x2ea30`. This is an instruction-byte reconstruction, not high-level behavioral equivalence or newly authenticated code.
- Next uncovered executable routine starts `0x2ea50`.

## Checkpoint 51 — packed bitstream and callback bootstrap

- Generated continuous `[0x21c78,0x2ebbc)` = **53,060 exact bytes**, newly added `[0x2ea50,0x2ebbc)` = **364 bytes**, with **86 A32 instructions** and **5 controller address words**.
- Transcribes the input packer (`0x2ea50–0x2eb30`) using checked halfword loads/stores, register-specified shifts and compares; followed by callback registration (`0x2eb30–0x2eba8`). The five trailing values are non-instruction constant pool `[0x2eba8,0x2ebbc)`.
- Next uncovered runtime function begins at `0x2ebbc`. Instruction-level equivalence is not high-level semantic recovery or OEM signing.

## Checkpoint 52 — event publication and 64-bit counter

- Contiguous A32 region `[0x21c78,0x2ec94)` = **53,276 exact bytes**, new `[0x2ebbc,0x2ec94)` = **216 bytes** comprised of **50 instructions** and **4 named data words**.
- Rebuilds a controller message publisher and 64-bit event sequence update with architecturally typed `LDRD`, `ADDS`, checked `ADC #0` and `STRD` instructions. Four literal address words `[0x2ec84,0x2ec94)` remain separate from executable instructions.
- The next original routine has a nonstandard single `SUB SP,SP,#8` at `0x2ec94` preceding its `PUSH`, and must retain that prologue intact. No borrowed opaque instruction bytes, no OEM-authenticated output claimed.

## Checkpoint 53 — controller stream event monitor

- Source-generated `[0x21c78,0x2efc0)` = **54,088 exact bytes**, newly added `[0x2ec94,0x2efc0)` = **812 bytes**: **187 A32 instructions** and **16 data words**.
- Original function starts at `0x2ec94` with an 8-byte stack reservation before its saved-register frame, monitors active source classes and emits diagnostic records. The separate 16-word data pool `[0x2ef78,0x2efb8)` is followed by two code words leading into the next function at `0x2efc0`.
- First missing code after this checkpoint begins at `0x2efc0`; referential equivalence is not device execution or a new OEM signature.

## Checkpoint 54 — controller ring buffer helpers

- Contiguous source-generated `[0x21c78,0x2f394)` = **55,068 bytes**, newly rebuilt `[0x2efc0,0x2f394)` = **980 bytes** with **232 typed ARM32 instructions** and **13 data words**.
- Reconstructs two separate ring-buffer request/response routines and their buffer-length validation, copy paths and completion flags. Literal pools at `[0x2f1c8,0x2f1e4)` and `[0x2f37c,0x2f394)` contain 7 and 6 data words, respectively, each following a function's branch/return.
- Next original function begins at `0x2f394`. Exact A32 machine-code agreement is not high-level behavioral recovery or new OEM firmware authentication.

## Checkpoint 55 — ring buffer counter, descriptor slots and completion

- Contiguous A32 reconstruction `[0x21c78,0x2f5b0)` = **55,608 exact bytes**; added `[0x2f394,0x2f5b0)` = **540 bytes** with **129 typed ARM32 instructions** and **6 non-executable data words**.
- Includes four short ring-slot/callback routines, and the original single-predicate `BX` fast path. Data pools at `[0x2f3d8,0x2f3dc)` and `[0x2f578,0x2f58c)` are modeled separately.
- First missing original instruction `0x2f5b0`. No bootable/signed OEM image claimed.

## Checkpoint 56 — DMA controller initialization and resource barriers

- Contiguous typed A32 interval `[0x21c78,0x2f7d0)` = **56,152 exact bytes**; newly added `[0x2f5b0,0x2f7d0)` = **544 bytes**, **117 instructions** and **19 data words**.
- Builds controller DMA input mappings, conditional queue/resource barriers, and trivial buffer getters. Five literal address words at `[0x2f658,0x2f66c)` and fourteen at `[0x2f788,0x2f7c0)` are separately modeled.
- The next actual function starts at `0x2f7d0`. Only instruction-byte equality is verified, not OEM signature, hardware boot or high-level behavior.

## Checkpoint 57 — MMIO state polls and counters

- Contiguous typed A32 reconstruction `[0x21c78,0x2f96c)` = **56,564 exact bytes**, newly added `[0x2f7d0,0x2f96c)` = **412 bytes**: **90 ARM32 instructions** and **13 data words**.
- Includes MMIO-controller state predicates, version/mode tests and three per-channel counter helpers. Ten interspersed literal pools are each explicitly classified as data, preventing OEM address words from being interpreted as instructions.
- Next routine begins at `0x2f96c`. This is instruction-byte re-encoding only, not independently recovered high-level semantics or an OEM-signed firmware.

## Checkpoint 58 — IRQ register-bank logging and mask controls

- Contiguous typed A32 reconstruction `[0x21c78,0x2fbc4)` = **57,164 exact bytes**; newly reconstructed `[0x2f96c,0x2fbc4)` = **600 bytes**, **131 instructions** and **19 literal data words**.
- Includes controller register-bank reporting, interrupt mask shutdown, getters and indexed IRQ enable/disable functions. Five separate pools contain MMIO and diagnostic pointers (a register-list LDMIB is an instruction, not a literal).
- Next unknown executable starts at `0x2fbc4`. This is source-level typed A32 *instruction encoding* verification, not a recovered high-level firmware implementation or OEM signature.

## Checkpoint 59 — IRQ routing, controller state and MMIO commands

- Contiguous typed A32 reconstruction `[0x21c78,0x2fcbc)` = **57,412 exact bytes**, new `[0x2fbc4,0x2fcbc)` = **248 bytes**, **58 instructions** and four literal words.
- Implements the route register writes and conditionally handles disabled controller states. Pool `[0x2fcac,0x2fcbc)` is MMIO and diagnostic data, not executable ARM instructions.
- The next routine begins at `0x2fcbc`, outside this checkpoint. Byte-for-byte instruction encoding is not behavioral proof or signing.

## Checkpoint 60 — Codec plane layout and frame stride conversion

- Full contiguous A32 region `[0x21c78,0x3027c)` = **58,884 exact bytes**; new function `[0x2fcbc,0x3027c)` = **1472 bytes** (**358 ARM32 instructions** and ten literal data words).
- Emits codec plane dimensions, interlacing/sample-stride calculations, 32-entry command array and command packer. The table `[0x30254,0x3027c)` is explicitly non-executable data.
- The next function starts at `0x3027c`. This is instruction-byte re-encoding and deterministic span generation, not independent high-level codec semantics or new OEM signing.

## Checkpoint 61 — Codec controller register serializer

- Contiguous typed A32 reconstruction `[0x21c78,0x311c0)` = **62,792 exact bytes**; new large function `[0x3027c,0x311c0)` = **3908 bytes** with **965 ARM32 instructions** and **12 literal data words**.
- Re-encodes model/format descriptor fields, range checking, controller command registers and retry/diagnostic paths; `[0x31190,0x311c0)` is controller offsets and diagnostic data, not ARM instructions.
- Next independent routine starts at `0x311c0`. Instruction-byte equivalence is not a recovered high-level implementation or authenticated firmware.

## Checkpoint 62 — Controller command lists and frame buffer types

- Contiguous typed A32 reconstruction `[0x21c78,0x3192c)` = **64,692 exact bytes**, new `[0x311c0,0x3192c)` = **1900 bytes**, **462 ARM32 instructions** and **13 data words**.
- Includes protocol/format conversion, cache-write/MMIO setup, input descriptor loops, shared queue entry registration and a seven-word resource/log pointer pool.
- Four literal pools are kept out of the instruction stream; controller list fill and command payloads use typed ARM op encoders. Next handler begins at `0x3192c`.

## Checkpoint 63 — Post-reset controller register-bank replication

- Contiguous typed A32 reconstruction `[0x21c78,0x31b80)` = **65,288 exact bytes**; newly added `[0x3192c,0x31b80)` = **596 bytes**, **143 ARM32 instructions** and **6 register-address data words**.
- Includes two controller bank synchronization loops and a descriptor-to-MMIO register-array transfer. Pointer literals reside in three explicit data pools; no OEM opcode sequences or implicit padding.
- Next function begins at `0x31b80`. This is exact instruction encoding, not a semantic or signed firmware reconstruction.

## Checkpoint 64 — Codec input capability flags and per-channel modes

- Contiguous typed A32 reconstruction `[0x21c78,0x31d80)` = **65,800 exact bytes**; new `[0x31b80,0x31d80)` = **512 bytes** containing **119 ARM32 instructions** and **9 data words**.
- Updates per-channel byte-mode flags using ARM32 writeback addressing and optionally sends controller diagnostic callbacks. `[0x31d40,0x31d64)` is MMIO/diagnostic data.
- Next routine begins at `0x31d80`; no reconstruction is claimed past this boundary or of OEM signature bytes.

## Checkpoint 65 — controller allocation, MMIO diagnostics and initial status messages

- Full contiguous typed A32 interval `[0x21c78,0x32190)` = **66,840 exact reference bytes**, MBN file `[0x25c78,0x36190)`.
- New `[0x31d80,0x32190)` = **1,040 bytes**, **231 explicitly typed ARM32 instructions** and **29 non-instruction literal words** across seven named pools: `[0x31db0,0x31db8)`, `[0x31dc4,0x31dc8)`, `[0x31e80,0x31e94)`, `[0x31f60,0x31f74)`, `[0x31fe0,0x31ff0)`, `[0x32054,0x32064)`, `[0x32170,0x32190)`.
- Includes a controller-status reporter, a tail-call status getter, allocation success/failure logging, controller release probes, and structured initial-state diagnostics. Data words resembling ARM instructions are explicitly emitted as data, never treated as executable A32.
- Full generated fragment matches pinned Venus MBN reference SHA-256 `9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085` with zero byte differences; independent Clang/LLVM relocation-aware reassembly of all **1,040 new bytes** passes. Generated fragment SHA-256 `99559b545ce3e1a727be1a745e4c40b3486a0dec4cbeef942856217238811166`.
- Whole Rust workspace **232 tests**, Python **13 tests** pass. Optimized release build was started but timed out in this execution environment and is **not claimed passed** for this checkpoint.
- First uncovered executable routine begins at `0x32190` (within agent 20-03 exclusive reservation `[0x21c78,0x40000)`). The source is instruction-byte re-encoding, not a high-level semantic codec reimplementation and not signed/flashable firmware.

## Checkpoint 66 — controller format-table copies and stream setup

- Contiguous typed A32 region `[0x21c78,0x32550)` = **67,800 exact bytes**; new `[0x32190,0x32550)` = **960 bytes**, **205 instructions** and **35 named data words** in eight literal pools.
- Reconstructed the controller startup-check loop, message emission and error callbacks, paired active/inactive controller table transfers using `LDM`/`STM` register-list copies, and initial stream setup diagnostics. The first function is a 32-byte rolling compare against a controller magic value, not blindly copied reference words.
- Separate non-instruction data pools: `[0x32278,0x32290)`, `[0x32368,0x3238c)`, `[0x323d8,0x323ec)`, `[0x3241c,0x32424)`, `[0x32464,0x32470)`, `[0x324d0,0x324e0)`, `[0x3250c,0x32518)`, `[0x32544,0x32550)`.
- Reference `venus.mbn` pinned SHA-256 `9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085`. Full Rust generation and independent Clang/LLVM assembler pass for all 960 new bytes with zero differences. **233 Rust and 13 Python tests** passed. Release-mode compilation is not verified for this checkpoint.
- Next function begins at runtime `0x32550`. The implementation proves operand-level A32 byte agreement, not high-level codec semantics or OEM signing.

## Checkpoint 67 — codec controller resource buffers and error unwinding

- Contiguous typed A32 `[0x21c78,0x329f8)` = **68,992 byte-identical bytes**, new `[0x32550,0x329f8)` = **1,192 bytes**, **284 instructions** and **14 separately identified data words** in `[0x329c0,0x329f8)`.
- Reconstructs an allocator for four controller buffer classes, callback attachment, MMIO register fields, configuration checks and rollback on allocation failures. Controller diagnostic pointers and source offsets are held in a named data pool. No vendor instruction bytes or inferred zeros enter the generated fragment.
- Direct whole-fragment private `venus.mbn` comparison and independent Clang/LLVM relocation-aware reconstruction of the 1,192 new bytes both pass with zero mismatches. Rust workspace **234 tests** and Python **13 tests** pass. Release-build status remains **unconfirmed**, not claimed passed.
- Next function begins at runtime `0x329f8`. This is byte-exact typed A32 instruction re-encoding, not high-level firmware semantics, hardware execution or OEM authentication.

## Checkpoint 68 — codec-mode classification and three bounded jump tables

- Typed A32 contiguous `[0x21c78,0x32c9c)` = **69,668 byte-identical bytes**. New partial controller-mode function prefix `[0x329f8,0x32c9c)` = **676 bytes**, **150 instructions** and **19 jump-address data words**.
- Distinct jump tables `[0x32b74,0x32b8c)` (six targets), `[0x32ba8,0x32bc4)` (seven targets), `[0x32c84,0x32c9c)` (six targets) are kept out of the executable instruction region and modeled as named data. Builds initial controller data-plane and codec-class selection with indexed PC dispatch; the function itself continues after the verified boundary.
- Direct full-fragment comparison to pinned reference `venus.mbn` reports **zero byte differences**; independent Clang/LLVM reassembly of all 676 new bytes passes. Rust workspace **235** tests and Python **13** tests pass. Release build was not confirmed for this checkpoint.
- First missing code begins at runtime `0x32c9c`. This remains typed ARM32 instruction encoding, not high-level functional decompilation or OEM-authenticated firmware.

## Checkpoint 69 — completed codec-class dispatch and diagnostic tables

- Contiguous typed A32 `[0x21c78,0x33488)` = **71,696 byte-identical bytes**. New `[0x32c9c,0x33488)` = **2,028 bytes**, **475 instructions** and **32 distinct data words**.
- Completes the controller codec-class state machine begun in checkpoint 68: a seven-way switch at `[0x32d18,0x32d34)` and extended codec command/format selection, error reporting, frame payload checks and callbacks. The final 25-word offset/diagnostic table `[0x33424,0x33488)` is explicitly data; 32-bit values that naive ARM objdump renders as `andeq`, `ldrsbeq`, or unknown instructions are not misclassified as executable code.
- Full typed emitter verifies byte-for-byte against pinned `venus.mbn` (`9d4af65d…a2091445a085`), and independent Clang/LLVM A32 reassembly of all **2,028** new bytes confirms **zero mismatches**.
- **236 Rust tests** and **13 Python tests** passed. Optimized release build remains unconfirmed for the current series of checkpoints.
- First uncovered runtime function starts at `0x33488`, safely below this agent's exclusive `0x40000` end. This establishes instruction-byte identity, not recovered high-level codec behavior or new OEM authentication.

## Checkpoint 70 — controller event-classification loop

- Contiguous typed ARM32 `[0x21c78,0x33abc)` = **73,284 reference-exact bytes**. New `[0x33488,0x33abc)` = **1,588 bytes**, consisting of **378 independently encoded A32 instructions** and a distinct **19-word literal/diagnostic data table** at `[0x33a70,0x33abc)`.
- Reconstructs controller event classification, callback selection and state/format validation. Literal words are structured data, not decoded instructions or copied executable bytes.
- Whole-fragment Rust emitter exactly matches pinned `venus.mbn` and independent Clang/LLVM assembler comparison confirms all 1,588 newly rebuilt bytes.
- **237 Rust tests** and **13 Python tests** passed. Release build **not verified** for this checkpoint.
- Generated fragment SHA-256 `8b709ee54c7a69ba600c9bf895a4bee92b88c8c635710a3c7a7b1b7b8f96b9d8`. Next unimplemented entry begins at `0x33abc`, below Agent 20-03's exclusive `0x40000` boundary. Instruction-byte agreement does not prove behavioral equivalence or OEM authentication.

## Checkpoint 71 — stream callback queues and input validation

- Full contiguous typed ARM32 `[0x21c78,0x33e10)` = **74,136 byte-identical bytes**. New `[0x33abc,0x33e10)` = **852 bytes**: **197 instructions** and **16 non-instruction literal data words** across six pools (`0x33c90–0x33ca8`, `0x33ce8–0x33cf0`, `0x33d30–0x33d38`, `0x33d78–0x33d80`, `0x33dc0–0x33dc8`, and `0x33e08–0x33e10`).
- Rebuilds two codec stream callback queue drainers and five guarded stream/mode validation helpers. Device diagnostic pointers and codec class IDs are modeled as data rather than A32 instructions.
- Direct generated-fragment match against pinned Venus MBN and independent Clang/LLVM reassembly both pass **all 852 new bytes** with zero mismatches. **237 Rust tests** and **13 Python tests** pass. Release build not verified for this checkpoint.
- Generated fragment SHA-256: `c1ec7d06950813ef757d55f305e915ec223ca0b4f355428c9e4c0a211f8031de`. First uncovered routine starts at runtime `0x33e10`; source is ARM32 instruction encoding, not complete semantic decompilation or a signed firmware image.

## Checkpoint 72 — codec stream event dispatch and validation

- Contiguous typed ARM32 `[0x21c78,0x34088)` = **74,768 exact bytes**. Newly generated `[0x33e10,0x34088)` = **632 bytes**, comprising **139 instructions and 19 data words**, in three non-instruction pools: `[0x33e54,0x33e6c)` (six handler pointers), `[0x33eb8,0x33ed4)` (seven handler pointers) and `[0x34070,0x34088)` (six controller constants).
- Implements priority-based event dispatch, codec channel callbacks, conditional input-plane validation, and diagnostic paths. Repeated handler targets are deliberately preserved; OEM opcode byte arrays are not embedded in source.
- Direct whole-fragment comparison against pinned Venus MBN and independent Clang/LLVM mnemonic reconstruction verify all **632 newly generated bytes** with zero differences.
- **238 Rust tests** and **13 Python tests** pass; release build is not claimed. The generated fragment SHA-256 is `b08d1c32cc146ea50feac6b3b70cca35d06fb3d02bb3204c3c6fc4fd633430bf`.
- First missing routine begins at `0x34088`. Instruction-byte encoding equivalence is not high-level decompilation or a signed firmware artifact.

## Checkpoint 73 — controller frame synchronization and descriptor allocation

- Contiguous typed ARM32 `[0x21c78,0x344c0)` = **75,848 exact bytes**, new `[0x34088,0x344c0)` = **1,080 bytes** with **263 ARM32 instructions** and **seven literal data words**.
- Reconstructed frame-type/MMIO selector and controller count update, a short codec-channel reset function, plus guarded descriptor allocation and parameter initialization. Separate diagnostic pools `[0x343a8,0x343bc)` (five words) and `[0x344b8,0x344c0)` (two words) remain data.
- Full fragment matches pinned `venus.mbn`, and independent Clang/LLVM reassembly of all 1,080 new bytes agrees exactly. **239 Rust tests** and **13 Python tests** pass; release build not independently confirmed for this checkpoint.
- SHA-256 of cumulative typed fragment: `df86236938128b83e7ed1d9265b18406ce91f23ce89938089657ad41bfa95436`. Next function begins at runtime `0x344c0`. This is instruction-level byte equivalence, not high-level source semantics or signed OEM firmware.

## Checkpoint 74 — codec frame-state and plane-format updates

- Contiguous typed ARM32 `[0x21c78,0x347a0)` = **76,584 byte-identical bytes**, new `[0x344c0,0x347a0)` = **736 bytes**, **177 ARM32 instructions** and **seven non-instruction diagnostic-address data words** in `[0x34784,0x347a0)`.
- Reconstructs controller-selected codec stream-plane metadata and format/status transitions, updates MMIO output state and invokes the original callback path. Data values are emitted separately from executable A32 instructions.
- Whole generated region matches the pinned Venus MBN with zero byte differences; independent Clang/LLVM reassembly reproduces all 736 new bytes exactly. **240 Rust** and **13 Python tests** pass; optimized release build unconfirmed.
- SHA-256 of generated fragment `c9cf88dd0bd974573628f411b096b3ae1abccc27c0c59ac55c149382242b8456` (verify digest before publication); next function begins at `0x347a0`.

## Checkpoint 75 — controller stream callback and packet management

- Contiguous typed ARM32 `[0x21c78,0x34ca0)` = **77,864 exact bytes**, new `[0x347a0,0x34ca0)` = **1,280 bytes**: **299 A32 instructions** and **21 literal data words** in four separately identified pools (`0x34888–0x3489c`, `0x34b0c–0x34b24`, `0x34bb0–0x34bc4`, `0x34c8c–0x34ca0`).
- Handles controller stream-format validation, three indirect callback dispatcher helpers, stale-packet cleanup and stream response construction. Data words are distinct from the ARM instruction stream.
- Generated fragment matches pinned Venus MBN exactly; independent Clang/LLVM assembler confirms all 1,280 new bytes without mismatch. **241 Rust tests**, **13 Python tests** pass. Release build not verified for this increment.
- Full fragment SHA-256 `0b3640e6666f748dcb850c4ff3d76632ef9406cc702cf5e1e0baba9eb101cfec`. The next routine begins at runtime `0x34ca0` below our exclusive upper limit `0x40000`.

## Checkpoint 76 — publish codec stream update and controller callback

- Contiguous typed ARM32 `[0x21c78,0x34fac)` = **78,644 exact bytes**, new `[0x34ca0,0x34fac)` = **780 bytes** comprising **188 A32 instructions** and seven controller format/diagnostic data words at `[0x34f90,0x34fac)`.
- Constructs codec output commands, validates active stream state, applies controller register flag updates and executes the optional callback. The register-postindexed memory operation is reconstructed using a checked public ARM32 encoder, not a copied opcode literal.
- Full generated region equals the original Venus reference, and independent Clang/LLVM reassembly confirms all 780 new bytes. **242 Rust tests** and **13 Python tests** pass. Release build not verified for this checkpoint.
- Full fragment SHA-256 `c68f9439ae5f8e281fbaa709fefd4c0d3f90760e031ab3b2d7e6c88fbc826a4d`. Next runtime routine begins at `0x34fac`; byte identity does not establish high-level semantics or OEM authentication.

## Checkpoint 77 — codec streaming resource and input format initialization

- Full contiguous typed ARM32 `[0x21c78,0x3548c)` = **79,892 reference-identical bytes**. New `[0x34fac,0x3548c)` = **1,248 bytes**, **306 encoded A32 instructions** and six non-instruction controller/data pointers `[0x35474,0x3548c)`.
- Implements selected codec stream session setup, input format/size collection, controller command generation and cleanup/diagnostic callbacks. Literal words stay separate from ARM instructions.
- Whole-region comparison with pinned Venus MBN and an independent Clang/LLVM assembly of all 1,248 newly generated bytes both pass with zero byte differences. **243 Rust tests** and **13 Python tests** pass; release build not verified.
- Generated fragment SHA-256 `71f5c24fc6261091550e66b0f2d324005d98373f48a950506b52ccf958e93fbb`. Next original routine begins at runtime `0x3548c`; instruction-byte agreement is not recovered high-level semantics or an OEM-authenticated image.

## Checkpoint 78 — codec exponent, aligned buffer accounting and queue initialization

- Full byte-identical, source-encoded A32 interval **`[0x21c78,0x35670)` = 80,376 bytes**; reference MBN file `[0x25c78,0x39670)`.
- New `[0x3548c,0x35670)` = **484 bytes**, **118 typed ARM32 instructions and 3 named literal data words** at `[0x3553c,0x35540)`, `[0x355d8,0x355dc)`, `[0x35648,0x3564c)`. The routines compute log2/bit-length, perform multiply/high-half scaling, align allocator addresses, then initialize ring-buffer queue state.
- Existing checked long multiply and ADC/carry encoders are reused; zero executable opcode arrays copied. Exact full-region generation and reference comparison pass. Independent Clang/LLVM assembly reproduces the complete 484-byte new interval.
- SHA-256 of generated contiguous 80,376 bytes: `f2ad196ae196fc633bc0efc676abfbe5eac4f96571af1c884c99f67178e2d03e`.
- Function boundary at **`0x35670`**; next queue enqueue routine starts there. Instruction-byte equivalence is not proof of reverse-engineered high-level semantics, OEM authentication or hardware execution.

## Checkpoint 79 — bounded controller ring queue and descriptor transfers

- Full source-encoded A32 fragment `[0x21c78,0x3580c)` = **80,788 byte-identical bytes**; Venus MBN file offsets `[0x25c78,0x3980c)`.
- New function group `[0x35670,0x3580c)` = **412 bytes**, **103 explicitly typed ARM32 instructions, zero data words**, implementing 32-entry bounded controller enqueue/dequeue operations, paired 64-bit `STRD` descriptor writes, IRQ owner synchronization callbacks, and queue-count wraparound.
- End address `0x3580c` coincides with the start of the next event classifier, which contains an 18-way dispatch table; this table is **not** treated as executable by this checkpoint.
- Full fragment independently compared against reference Venus SHA-256 `9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085`; no mismatches. Standalone Clang/LLVM reassembly of all 412 new bytes also matches. A guarded opt-in Rust unit test can emit/compare the entire interval using `NABU_VENUS_REFERENCE` and `NABU_VENUS_EMIT` without bundling OEM firmware bytes.
- Results demonstrate exact A32 instruction re-encoding, not behaviorally verified source-level decompilation or flashable/authenticated firmware.

## Checkpoint 80 — 18-way frame dispatch and input-descriptor initialization

- Complete contiguous typed A32 fragment `[0x21c78,0x35bec)` = **81,780 byte-identical bytes**, MBN file `[0x25c78,0x39bec)`.
- Newly re-encoded `[0x3580c,0x35bec)` = **992 bytes**, **227 A32 instructions** and **21 non-executable data words**. The controller event-class function dispatches through 18 explicit handler addresses at `[0x35830,0x35878)`, guards 17 input types, initializes channel size/format fields, and updates stream accounting. Subsequent initialization helpers emit the default 0x01010200 codec format word and two separate diagnostic-address literals.
- Literal pools: `[0x35830,0x35878)` (18 handler pointers), `[0x35a2c,0x35a30)` (warning), `[0x35b2c,0x35b30)` (format), `[0x35be8,0x35bec)` (warning). No reference opcodes or unknown padding are imported into the typed output.
- Both direct reference comparison and independent Clang/LLVM mnemonic reassembly pass for the entire newly generated 992-byte interval; SHA-256 of full fragment: `1e430261ad796d23c5033b4ee34b330fbbbca834c577e4c6ade0f4f98f9d8761`.
- Next function starts at runtime `0x35bec`, still within Agent 20-03 `[0x21c78,0x40000)` reservation. This demonstrates operand-level binary equivalence, not a new high-level source implementation or OEM signing.

## Checkpoint 81 — controller descriptor consumption and ring accounting

- Contiguous typed ARM32 `[0x21c78,0x35ee4)` = **82,540 exact bytes**, source file `[0x25c78,0x39ee4)`.
- New `[0x35bec,0x35ee4)` = **760 bytes**, **187 typed A32 instructions** and three explicitly modeled non-instruction MMIO/diagnostic address words at `[0x35d40,0x35d44)`, `[0x35df0,0x35df4)`, `[0x35ea0,0x35ea4)`.
- Reconstructed descriptor pointer, queue count/index, callback synchronization, wraparound, 2D stride math, buffer deallocation/zeroing and fault-reporting helpers. The instructions are auditable public architecture encodings, not source-level proof of semantic equivalence.
- Complete 82,540-byte emitter and standalone independent Clang/LLVM reassembly of the new 760 bytes report **zero mismatches** versus the original pinned Venus MBN. Generated-fragment SHA-256: `9e1261ae69b74219d6ba01d28d2179834b0b09ff6a91dfa2aa664a2a2304bcfa`.
- Next runtime function begins at `0x35ee4`, below Agent 20-03's exclusive `0x40000` reservation boundary.

## Checkpoint 82 — typed ring entry lookup/update and packet delivery

- Contiguous generated ARM32 `[0x21c78,0x36048)` = **82,896 reference-exact bytes**, file `[0x25c78,0x3a048)`.
- New `[0x35ee4,0x36048)` = **356 bytes**, **86 A32 instructions** and three non-executable diagnostic values `[0x35fac,0x35fb8)`. Implements descriptor search and in-place update, queue allocation retry and guarded delivery of a selected packet to its destination.
- Full fragment was regenerated using typed ARM32 operations, byte-compared against pinned `venus.mbn` and independently reassembled with Clang/LLVM: **zero byte differences**.
- Generated SHA-256: `1c2e88a039551e830218d09354fa773bccf31be992d24454eee81d093160d1e4`. The next input-mode controller initializer starts at `0x36048`.
- No claim of high-level source-code or hardware-execution equivalence; this remains an unsignable, nonflashable partial firmware segment.

## Checkpoint 83 — full controller input-mode defaults and format register tables

- Full contiguous typed A32 fragment `[0x21c78,0x3701c)` = **86,948 reference-identical bytes**, MBN file `[0x25c78,0x3b01c)`.
- New controller initialization routine `[0x36048,0x3701c)` adds **4,052 bytes**: **996 typed ARM32 instructions** and a separately modeled **17-word** controller data table `[0x36fd8,0x3701c)`.
- Reconstructed the complete initialization path that configures stream mode, channel masks, format identifiers, default frame dimensions, multiple plane byte/halfword fields, callback pointers, hardware register sets, and large zero/enable-controlled state arrays. It ends at the actual saved-register return (`pop ... pc`) at `0x36fd4`, not an invented return instruction.
- Two checked local A32 ISA encoders cover register-indexed halfword transfer and signed byte pre-indexed writeback; focused tests match those publicly specified architecture fields. No OEM executable opcode arrays are copied into the source.
- Direct private Venus reference comparison reports zero mismatches for the complete fragment. Independent Clang/LLVM mnemonic reassembly confirms **all 4,052 new bytes**. Generated SHA-256: `cc9ea751ca1da0bdcf0fcee200649543b876339ba96b2d466e9ec7207a218446`.
- The next source function begins at runtime address `0x3701c`, within Agent 20-03's exclusive reservation ending at `0x40000`.

## Checkpoint 84 — stream quantization and interpolated hardware format selection

- Complete typed ARM32 `[0x21c78,0x37180)` = **87,304 consecutive reference-identical bytes**, original MBN `[0x25c78,0x3b180)`.
- New `[0x3701c,0x37180)` = **356 bytes**, **83 A32 instructions** and **six diagnostic/scaling literal data words** `[0x37168,0x37180)`.
- Reconstructs a codec-format table interpolation/scaling helper with **eight ARM-to-Thumb BLX immediate calls**. The return at `0x37164` is verified; the trailing pool holds fixed-point factors and controller addresses, not instructions.
- Exact full-fragment reference comparison passes and independent LLVM/Clang mnemonic reassembly confirms all 356 bytes. LLVM/Lld emits benign `R_ARM_CALL` warnings for external non-STT_FUNC BLX branch symbols; both emitted bytes and target offsets match the original. Instruction binary equivalence does not establish hardware behavior.
- Next function begins at `0x37180`, below the reserved `0x40000` boundary.

## Checkpoint 85 — codec input-mode classification and fixed defaults

- Verified source-encoded ARM32 `[0x21c78,0x37be8)` = **89,968 byte-identical bytes**; original MBN `[0x25c78,0x3bbe8)`. New `[0x37180,0x37be8)` = **2,664 bytes**, **656 typed A32 instructions** and **ten controller threshold/diagnostic literal data words** `[0x37b24,0x37b4c)`.
- The main classifier has distinct codec channel fallbacks and branches, returns via original POP/PC at `0x379c0` and has reachable tail cases through `0x37b20`. After its literal pool are two fixed configuration helpers `[0x37b4c,0x37be8)`, each terminating in original `BX LR`; no artificial return was inserted.
- Added a checked local A32 register-offset `LDRSH` pre-indexed-writeback encoder, with tested public ISA fields (`0xe1b560f1`, `0xe1b650f0`) rather than reference opcode literal reuse. Reused public `arm32::transfer_byte_preindexed_writeback` for byte accesses.
- Full region directly matches the pinned `venus.mbn` source, zero differing bytes. Independent Clang/LLVM reassembly matches the entire newly generated 2,664-byte interval with no differences; linker's external BLX `R_ARM_CALL` warnings do not change encoded bytes. 253 Rust tests (across workspace crates) and 13 Python tests pass. An optimized release build has not been independently confirmed for this checkpoint.
- Whole generated region SHA-256: `e7201d50391e37bb32b6da3a9ca0a7ad69e9fa334113550d93a4b6ef48268db3`. Next routine entry `0x37be8` (file `0x3bbe8`) remains in the Agent 20-03 reserved range `[0x21c78,0x40000)`.
- This is instruction-level re-encoding and independent byte validation, not complete high-level source recovery, OEM authentication, or hardware/flashability validation.

## Checkpoint 86 — format-plane controller validation and channel dispatch

- Entire typed Venus A32 fragment `[0x21c78,0x38a44)` = **93,644 bytes**, file-backed MBN `[0x25c78,0x3ca44)`. The new interval `[0x37be8,0x38a44)` = **3,676 bytes**, consisting of **904 explicitly generated ARM32 instructions** and **15 non-instruction controller/scaling constants** `[0x38a08,0x38a44)`.
- A large format-control routine selects codec plane input/channel layouts, checks format-dependent frame widths, and issues MMIO callback updates. It includes original POP/PC return at `0x38938` plus reachable alternate-case branches through `0x38a04` before the literal pool. No synthetic return or arbitrary zero-fill bytes were introduced.
- Added checked register-indexed `LDRSH` and `LDRH`/`STRH` operands locally, reusing the public `arm32` byte writeback encoder. Dedicated tests verify the signed halfword encoding. The 15-word pool includes mask/reciprocal and diagnostic pointers; several decode as plausible ARM instructions but remain **data**.
- Full **93,644 bytes** directly match pinned `venus.mbn`, no differing bytes; separate Clang/LLVM mnemonic reassembly exactly matches all 3,676 new bytes. Two benign external interworking relocation warnings have no effect on the emitted machine bytes. SHA-256 of generated fragment: `ab677edfd426ce344225c4fe626783e65abd66b17173a9a90f090d0f353f98f3`.
- `cargo test --workspace --offline --locked -q` passes **255 Rust tests** in this branch; Python suite passes **13 tests**. An optimized release build has not been independently completed for checkpoint 86.
- Next runtime routine starts at `0x38a44` (MBN file `0x3ca44`). Reserved Agent 20-03 upper bound is `0x40000`. Matching opcode bytes does not imply source-level behavioral correctness, hardware validation, or OEM authentication.

## Checkpoint 87 — controller codec formats and descriptor counter

- New typed A32 `[0x38a44,0x38f70)` = **1,324 bytes**, consisting of **293 instructions** and two diagnosed non-instruction literal pools: nine words `[0x38bc0,0x38be4)` and 29 words `[0x38e4c,0x38ec0)`. Total source-emitted interval `[0x21c78,0x38f70)` = **94,968 contiguous reference-identical bytes**, file offsets `[0x25c78,0x3cf70)`.
- One controller function selects a codec resource and emits channel defaults, another initializes format-specific MMIO registers with three codec mode options, and two smaller helpers update an indexed 64-bit descriptor counter with carry and access a codec entry. Original `POP {..., PC}` and `BX LR` returns are encoded, without synthetic padding. The literal pools contain runtime diagnostic addresses, not ARM instructions.
- Whole-fragment comparison against pinned `venus.mbn` succeeds with zero byte differences; independently assembled Clang/LLVM mnemonic span matches all **1,324 new bytes**. `cargo test --workspace --offline --locked` passes **256 Rust tests**, Python test discovery **13 passed**; optimized release build not confirmed. Full fragment SHA-256: `d6fe89ce4632f6ca0cbfe8e13f2f821dedc6ff5059f299d82d49c3ba0b489212`.
- Next original routine begins at runtime `0x38f70` (file `0x3cf70`), below Agent 20-03's exclusive upper limit `0x40000`. These are exact instruction-byte encodings, not independently proven high-level semantics or newly OEM-signed firmware.

## Checkpoint 88 — codec plane dimensions and format flags

- Entire typed A32 `[0x21c78,0x39490)` = **96,280 contiguous reference-identical bytes**, MBN original offsets `[0x25c78,0x3d490)`. New `[0x38f70,0x39490)` = **1,312 bytes** comprising **325 ARM32 instructions** and **three clearly identified non-executable data words** `[0x39484,0x39490)`.
- Reconstructs codec plane-geometry calculations, signed frame dimensions, stream stride and buffer size accounting, input bit-depth routing and packed format flags. It uses signed/unsigned shift operations and controlled register-indexed memory transfers; interworking BLX calls are encoded from original register and target semantics.
- Original main function POP/PC return at `0x39368`, reachable alternative cases to `0x39410` and flag formatter ends with `BX LR` at `0x39480`; the three-word data pool follows the latter, not part of executable ARM code.
- Full generated fragment and independent Clang/LLVM A32 assembly of the 1,312 new bytes both match pinned `venus.mbn` exactly, zero mismatches. Eight harmless external BLX relocation type warnings from `ld.lld` did not affect emitted bytes. **257 Rust tests** and **13 Python tests** pass. Release build unconfirmed.
- Whole generated fragment SHA-256: `76426b0adcba36d6fb2a4bbc11411961391d1d638f14b7f5bc7741074c216258`. Next ARM32 routine begins `0x39490`, file offset `0x3d490`; reserved Agent 20-03 upper bound `0x40000` is not approached/overlapped.
- Byte-identical instruction re-encoding is not an independently verified high-level implementation, device boot validation or OEM authentication.

## Checkpoint 89 — codec capture state and descriptor accessors

- Complete typed source-encoded Venus A32 `[0x21c78,0x39bd0)` = **98,136 contiguous reference-identical bytes**, original firmware `[0x25c78,0x3dbd0)`; new `[0x39490,0x39bd0)` = **1,856 bytes**, **462 typed instructions** and two diagnostic pointers, one at each `[0x3996c,0x39970)` and `[0x39ac0,0x39ac4)`.
- The reconstructed code calculates codec stream/capture constraints and shifts packed signed plane dimensions, initializes controller geometry/status fields and accesses indexed descriptor slots under lock-protected callbacks. The original function returns at `0x39968`, subsequent accessor routines terminate at original return sites, and the last function tail-calls the runtime callback at `0x39bcc` immediately before the next initializer at `0x39bd0`.
- The register-offset signed halfword load is generated from checked public A32 bitfields. Both literal words are kept distinct from the executable stream. Direct full-region byte comparison and independent LLVM/Clang instruction assembly pass for all **1,856 added bytes** (one harmless external branch/interworking warning).
- `cargo test --workspace --offline --locked` passes **258 Rust tests**; **13 Python tests** pass. Release build remains unconfirmed. Generated fragment SHA-256 `9651f5255f65fa87381073ce546d0282f4d8b7522a8e5aa4f7dc24ac31a29e4e`.
- Next routine begins at runtime `0x39bd0` (MBN `0x3dbd0`). This is opcode-byte agreement, not semantic/hardware equivalence or OEM signing.

## Checkpoint 90 — codec-record flushing and format conversion callbacks

- Typed A32 region `[0x21c78,0x39f68)` = **99,056 byte-identical bytes**; reference MBN `[0x25c78,0x3df68)`.
- Newly reconstructed `[0x39bd0,0x39f68)` = **920 bytes** with **203 architecture-typed A32 instructions** and **27 noninstruction data words** in six pools: `[0x39cc8,0x39ccc)`, `[0x39d00,0x39d20)`, `[0x39dd0,0x39df0)`, `[0x39eec,0x39ef0)`, `[0x39f0c,0x39f2c)`, `[0x39f64,0x39f68)`. The three eight-way handler tables contain repeated legitimate jump targets; these are preserved as data rather than ARM instructions.
- Record flush resets active count and prints diagnostic data; two type conversion callbacks map codec identifiers to output layout codes with fallback diagnostics. Original return sites and callbacks are preserved. **No reference opcode byte arrays, implicit unknown zeros or artificial return instructions were introduced.**
- Standalone source-generated fragment `cargo run --example agent20_03_venus_mid -- emit /tmp/venus-99056.bin` matched original Venus (`sha256=9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085`) with **zero byte differences**. Independent Clang/LLVM mnemonic assembler comparison of all 920 new bytes also passed. **260 Rust and 13 Python tests passed**; release build not yet confirmed for checkpoint 90.
- Next original function begins at runtime **`0x39f68`** (file `0x3df68`) within exclusive `[0x21c78,0x40000)` range. Opcode equivalence is not high-level semantic source proof or OEM signing. Branch remains portable on prior Agent 03 history, not checkpoint 50.

## Checkpoint 91 — stream initialization prelude and diagnostic text pointers

- Complete source-emitted fragment `[0x21c78,0x3a2e4)` = **99,948 reference-identical bytes**; original MBN file offsets `[0x25c78,0x3e2e4)`.
- New interval `[0x39f68,0x3a2e4)` = **892 bytes**: **76 typed ARM32 instructions** and **147 named diagnostic-string address words** `[0x3a098,0x3a2e4)`. The latter decode nonsensically as ARM instructions if not classified as literal data; none is counted as independently written program code. The main function continues at `0x3a2e4`, so no return instruction is invented here.
- Public checked indexed A32 `LDRH`/`STRH` register-transfer encoder is tested against architecture instruction fields. This module checks the exact number and boundaries of diagnostic pointers.
- Direct source-generated Rust comparison and independently linked Clang/LLVM reconstruction of the entire 892 bytes agree **with zero differences** against the pinned `venus.mbn` SHA-256 `9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085`.
- **262 Rust and 13 Python tests pass**. The optimized release build is not confirmed for this checkpoint. This work is A32 opcode re-encoding, not recovered high-level source semantics, independently signed firmware or functional hardware validation.
- Next original executable instruction begins at runtime `0x3a2e4` (original MBN file `0x3e2e4`), still within Agent 20-03's exclusive `[0x21c78,0x40000)` scope.

## Checkpoint 92 — stack-backed codec stream configuration matrix, first block

- Contiguous typed A32 `[0x21c78,0x3aa00)` = **101,768 byte-identical bytes**, MBN file `[0x25c78,0x3ea00)`.
- New `[0x3a2e4,0x3aa00)` = **1,820 typed executable bytes, 455 ARM instructions**, zero new data words. The code constructs a large descriptor matrix on the stack, using backward PC-relative references into the preceding 147-word diagnostic table; that table remains data and its previous byte identity is rechecked in the complete output.
- Full Rust emitter matches the pinned reference (zero byte differences); independent Clang/LLVM ARM assembler reproduces all 1,820 bytes. **263 Rust and 13 Python tests passed**. Release build not confirmed.
- Current function continues after `0x3aa00`: this is a verified partial-function checkpoint without an invented return. The source remains on the earlier portable branch, not checkout of checkpoint 50; no signing/hardware test is claimed.

## Checkpoint 93 — controller stream parameter matrix continuation

- Full typed Venus interval `[0x21c78,0x3b800)` = **105,352 reference-exact bytes**, MBN `[0x25c78,0x3f800)`.
- New `[0x3aa00,0x3b800)` = **3,584 bytes**, **896 typed A32 instructions**, **zero new data words**. The compiler-like routine populates channel-specific command slots in a stack-backed matrix, repeatedly referencing the pre-existing explicit diagnostic address table. No vendor instruction arrays or unknown data fillers are imported.
- Direct reference check of the full generated region and independent Clang/LLVM A32 assembly of all 3,584 new bytes pass with zero mismatches. **264 Rust and 13 Python tests passed**; optimized release build not yet confirmed for this checkpoint.
- The main controller initializer remains incomplete after `0x3b800`. Next code starts at runtime `0x3b800`; exclusive reservation ends at `0x40000`. Byte agreement is not high-level semantics or OEM authentication.

## Checkpoint 94 — matrix writer and second controller diagnostic pool

- Complete typed A32 region `[0x21c78,0x3c53c)` = **108,740 reference-identical bytes**, MBN file `[0x25c78,0x4053c)`.
- New partial controller initialization interval `[0x3b800,0x3c53c)` = **3,388 bytes**, comprising **692 explicitly typed A32 instructions** `[0x3b800,0x3c2d0)` and **155 diagnostic data-pointer words** `[0x3c2d0,0x3c53c)`. The code branches over this literal pool to resume at its end. No plausible-looking vendor strings are treated as instructions.
- Full Rust emitter matched the pinned `venus.mbn` reference with zero differences; independent LLVM/Clang reassembly of all 3,388 new bytes also matched. **265 Rust and 13 Python tests passed**. Release build not confirmed.
- The large stream initializer is still incomplete; its executable instructions resume at `0x3c53c`. This represents byte-accurate public A32 operand encoding and OEM data addresses, not high-level semantic reconstruction, authentication or flashability.

## Checkpoint 95 — stream controller register matrix completion, first part

- Full typed A32 `[0x21c78,0x3dc00)` = **114,568 original-reference-exact bytes**, MBN file `[0x25c78,0x41c00)`.
- New `[0x3c53c,0x3dc00)` = **5,828 bytes**, **1,457 public ARM32 instructions** and zero interspersed noninstruction words. This block populates controller registers, callback descriptor arrays and stream format metadata, referring backward to the two previously separated literal pools.
- Direct Rust emission and Clang/LLVM mnemonic reassembly match every byte. **266 Rust and 13 Python tests passed**; optimized release build not independently verified.
- This checkpoint stops at `0x3dc00` inside a large function. Remaining control-flow cleanup and a trailing literal pool precede the next function. Source bytes are exact but high-level semantics and OEM signing are not established.

## Checkpoint 96 — complete controller stream initializer, return and diagnostic pool

- Verified typed Venus `[0x21c78,0x3dfec)` = **115,572 byte-identical bytes**, MBN file offsets `[0x25c78,0x41fec)`.
- New `[0x3dc00,0x3dfec)` = **1,004 bytes**, **154 ARM32 instructions** and **97 classified data words** across nine disjoint literal/jump-table intervals. It handles descriptor type dispatch and signed byte/halfword copying, finishes at the true POP/PC return at `0x3def8`, and follows with a 60-word diagnostic pool `[0x3defc,0x3dfec)`.
- Three independently typed opcode pools encode 10-way input data types, 10-way output types and 7-way payload-copy dispatch. Checked ARMv7 signed byte/halfword preindexed writeback helper includes ISA field tests; no opaque reference instructions are pasted.
- Complete fragment direct source/reference comparison and an independent Clang/LLVM reassembly of all 1,004 new bytes match with **zero differences**. **268 Rust and 13 Python tests passed**; optimized release build not confirmed.
- Next function begins at runtime `0x3dfec` (MBN file `0x41fec`) below the assigned upper bound `0x40000`. Byte equality is not new high-level source equivalence, hardware execution or OEM signing.

## Checkpoint 97 — codec descriptor publication and queue advance

- Contiguous typed Venus `[0x21c78,0x3e304)` = **116,364 byte-identical bytes**, MBN `[0x25c78,0x42304)`.
- New `[0x3dfec,0x3e304)` = **792 bytes**: 196 typed ARM32 instructions and two explicit non-executable diagnostic words at `[0x3e2c4,0x3e2c8)` and `[0x3e300,0x3e304)`.
- Reconstructed original saved-register return and guarded queue/capacity branch. No vendor instruction data was pasted into the Rust emitter.
- Direct source vs reference and independently linked Clang/LLVM mnemonic reassembly produce zero byte differences. Test results are recorded in checkpoint log; not proof of whole-firmware signing or functional behavior.
- Next uncovered runtime entry `0x3e304`; assigned upper limit `0x40000`. The portable branch does **not** descend from checkpoint-50.
