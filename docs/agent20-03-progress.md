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
