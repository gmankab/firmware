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
