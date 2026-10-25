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
