# Agent 20-02: Venus codec-seam reconstruction

## Required provenance and scope

**Assigned interval:** ARM runtime `[0x18000,0x183e0)` (992 bytes), corresponding to private `venus.mbn` file offsets `[0x1c000,0x1c3e0)`. Agent 01 owns code before `0x18000`; Agent 03 begins a new routine at `0x183e0`.

**Critical integration caveat:** The actual checkpoint-50 Git bundle, commit `b7ed4b587633b9b475a40f5a3ace5cc90db620ff`, was not uploaded. This work therefore starts from the separately verified Agent 03 old-root bundle, HEAD `a3ffc58da64d8f96a65b9a48f4bed24592ba0ac1`, on `agent20/02-venus-seam-prep`. The Git history cannot honestly be represented as a descendant of checkpoint 50. Submit the uniquely named seam source as a portable patch to Integration Agent 20-01; register `pub mod venus_runtime_seam` in the integrated source as a separate glue operation if needed.

Reference image is external to Git and pinned by `reference/xiaomi-nabu/firmware.sha256` (Venus SHA-256 `9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085`). No OEM executable or authentication payload is committed.

## Checkpoint 01: stream mode jump table and geometry

- Typed A32 operations cover `[0x18000,0x1815c)`, **348 consecutive bytes**.
- Original controller function began at `0x17fd0` (Agent 01's final 48 bytes); this fragment continues the already-open frame and is **not** an independent callable routine.
- Seven **data** pointer words occupy `[0x18010,0x1802c)`, with explicit case labels `0x1802c,0x1802c,0x1815c,0x18050,0x18088,0x180c0,0x18104` (repeated targets intentional).
- Comparison command (reference explicitly provided; writer itself does not read firmware):

  ```sh
  cargo run --offline --locked -p firmware-tools --example agent20_02_venus_seam -- verify /private/venus.mbn
  ```

- The independent Clang A32 assembly of instructions and separate jump table, with an externally observed disassembly, reproduces **all 348 bytes** without raw instruction words in the Rust emitter.
- Next missing instruction is `0x1815c`. No implied padding or completion of the entire 992-byte seam.

## Remaining to complete

Code `[0x1815c,0x18330)`, four-target table `[0x18330,0x18340)`, code `[0x18340,0x183dc)` and one diagnostic pointer `[0x183dc,0x183e0)`. Full expected length: 992 bytes.
