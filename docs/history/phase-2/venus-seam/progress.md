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
  cargo run --offline --locked -p firmware-tools --example venus_seam -- verify /private/venus.mbn
  ```

- The independent Clang A32 assembly of instructions and separate jump table, with an externally observed disassembly, reproduces **all 348 bytes** without raw instruction words in the Rust emitter.
- Next missing instruction is `0x1815c`. No implied padding or completion of the entire 992-byte seam.

## Remaining to complete

Code `[0x1815c,0x18330)`, four-target table `[0x18330,0x18340)`, code `[0x18340,0x183dc)` and one diagnostic pointer `[0x183dc,0x183e0)`. Full expected length: 992 bytes.

## Checkpoint 02 — assigned seam fully reconstructed

- **Complete exact span:** ARM32 runtime `[0x18000, 0x183e0)`, **992 bytes**, file offsets `[0x1c000,0x1c3e0)`.
- Code/data boundaries:
  - `0x18000–0x18010`: codec-mode dispatch header, 4 instructions.
  - `0x18010–0x1802c`: seven-entry stream geometry jump table, 7 **data** words.
  - `0x1802c–0x18330`: active format preparation and codec command-ring/geometry calculation, 193 ARM32 instructions.
  - `0x18330–0x18340`: four-entry pixel geometry jump table, 4 **data** words.
  - `0x18340–0x183dc`: geometry packing and epilogue, 39 ARM32 instructions.
  - `0x183dc–0x183e0`: diagnostic string address `0x000e3d16`, 1 **data** word.
- **236 independently encoded ARM32 instructions** (944 bytes), **12 named non-instruction data words** (48 bytes). Total 992 bytes, with zero differences from the pinned original `venus.mbn`.
- The original function was entered at `0x17fd0` (Agent 01's final 48 bytes). Its return is a real `pop {...,pc}` at `0x183d8`, then a data pointer, followed by Agent 03's next function starting precisely at `0x183e0`. Nothing from Agent 01 or Agent 03 is overwritten.
- Independently assembled **all 992 bytes** with Clang/LLVM mnemonic input and separate `.word` data pools: **zero byte differences**. The Clang verification inputs are kept outside Git, since they were generated from the private firmware reference disassembly.
- Standalone smoke commands with the original image outside Git:

  ```sh
  cargo run --offline --locked -p firmware-tools --example venus_seam -- verify /private/venus.mbn
  cargo run --offline --locked -p firmware-tools --example venus_seam -- emit /tmp/venus-seam-992.bin
  cargo run --offline --locked -p firmware-tools --example venus_seam -- verify-joined-late /private/venus.mbn
  cargo run --offline --locked -p firmware-tools --example venus_seam -- emit-joined-late /tmp/venus-seam-late-40056.bin
  ```

- The optional joined verification **also independently emits Agent 03's 39,064-byte preexisting late fragment** and checks adjacency: `[0x18000,0x21c78)` = 40,056 consecutive bytes. Of these, **992 bytes belong to this agent**; the remaining 39,064 bytes are *not* new Agent 20-02 work.
- SHA-256 of 992-byte output: `22b98ab433e327ce69242c582ee90c85a1530b09bf6e39c67a98c30ea36c9002`.
- Tests: focused checked-range/table tests, offline Rust workspace suite, offline release build, Python unit tests, independent Clang mnemonic assembly, strict binary comparisons, `git diff --check` and Git bundle verification.

### Integration conditions

1. Obtain real checkpoint-50 ancestor `b7ed4b587633b9b475a40f5a3ace5cc90db620ff` — it was not provided in this execution session.
2. Integrate Agent 01 final forward `[0x12e5c,0x18000)` and Agent 03 late `[0x183e0,0x21c78)` into a genuinely checkpoint-50-based tree. Bring this agent's `venus_runtime_seam.rs`, CLI example and tests via the portable patch; add the `pub mod venus_runtime_seam;` registration (the only shared source edit).
3. Verify the existing 33,936-byte boot prefix, Agent 02-old 9,596-byte bridge, checkpoint-50 33,872-byte service and Agent 01 forward before claiming `[runtime 0,0x18000)` is rebuilt.
4. Verify the newly continuous range `[0,0x21c78)` against the exact pinned `venus.mbn`, and rebuild broader image metadata separately.

**Do not claim full firmware completion, new OEM authentication or high-level semantic recovery:** this assignment is a byte-identical typed A32 re-encoding, with matching control flow at the instruction level. Wider firmware regions remain incomplete.

## Reproducible independent A32 round-trip

A standalone script is included in this bundle:

```sh
cargo run --offline --locked -p firmware-tools --example venus_seam -- emit /tmp/venus-seam-992.bin
python3 tools/venus/verify-venus-seam-clang-roundtrip.py /private/venus.mbn \
  --emitted /tmp/venus-seam-992.bin
```

It creates temporary Clang/LLVM disassembly from the privately supplied MBN,
then independently reassembles the **236 ARM32 mnemonics** and handles **12
non-instruction data words** as explicit jump tables and a diagnostic literal.
It compares that result with the reference and, optionally, the typed-Rust
emitter. The script does **not** bundle the proprietary firmware or any code
listing in Git. This method corroborates **instruction-byte identity**, not
recovered high-level source semantics.

Final tested results in this session:

- `cargo test --workspace --offline --locked`: **189 Rust tests passed**.
- `python3 -m unittest discover -s tests -p 'test_*.py'`: **13 Python tests passed**.
- `cargo build --workspace --release --offline --locked`: **passed**.
- Direct 992-byte Rust-output comparison: **0 differences**.
- Independent Clang A32 reassembly: **0 differences**.
- Joined 40,056-byte seam + existing Agent 03 late output: **0 differences**.
- `git diff --check` and Git bundle verification: **passed**.

The adjacent source-code ranges are independently verified at the boundaries,
but the required original checkpoint 50 has not been supplied, so this branch
remains explicitly provisional until Integration Agent 20-01 merges it into a
true checkpoint-50-rooted repository.
