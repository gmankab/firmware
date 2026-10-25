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
