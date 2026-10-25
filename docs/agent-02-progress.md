# Agent 02 — Venus boot/service bridge

This branch reconstructs only Venus ARM A32 runtime addresses `[0x8490, 0xaa0c)`;
the MBN file offset for these instructions is runtime `+0x4000`. It does **not**
rewrite the forward service-region modules owned by Agent 01. Missing bytes
are never filled with reference slices or artificial padding.

## Base and integration note

Requested baseline: checkpoint 50 (`b7ed4b5`). That bundle was not mounted
in the Agent 02 environment. The available `recompile-firmware.bundle`
contains boot-prefix commit `1ef3787` and current parent `69ebf0d`. Its
verified 33,936-byte boot/runtime prefix is the same initial prefix described
at checkpoint 50. The changes here are deliberately isolated: merge them
onto checkpoint 50 with `git cherry-pick`, resolving only the addition of
`mod.rs` registration. Do **not** treat this branch as a direct descendant
of the missing checkpoint-50 commit.

## Checkpoint 01 — scoped register/table access (`0x8490–0x883c`)

- New separately authored ARM32 fragment: **940 consecutive bytes**, runtime
  `[0x8490,0x883c)` / file `[0xc490,0xc83c)`; 231 instructions and **four**
  clearly identified literal words.
- Functions: name-hash lookup and bounded index traversal, pointer resolution,
  compatibility filtering, scoped lookups, and record selection in a section.
- Source module: `venus_runtime_bridge_lookup.rs`, with the strict layout
  and boot-prefix concatenation in `venus_runtime_bridge.rs`.
- Full boot+bridge checked region: **34,876** consecutive bytes beginning at
  file `0x4000`; gap remaining until existing service start `0xaa0c` is
  `[0x883c,0xaa0c)`, **8,656 bytes**.
- No proprietary instruction bytes embedded in source; literal data words are
  `0x000e359a`, `0x000e35bd`, `0x000e35ea`, `0x001039ec`.
- Branch destinations into code outside this fragment are preserved, not
  executable in a standalone fragment. The fragment cannot be flashed.

### Reproduce exact verification

```sh
cargo run --offline --locked -p firmware-tools --bin venus_bridge -- \
  emit /tmp/venus-bridge.bin
cargo run --offline --locked -p firmware-tools --bin venus_bridge -- \
  verify /private/venus.mbn
cargo run --offline --locked -p firmware-tools --bin venus_bridge -- \
  verify-joined /private/venus.mbn
python3 tools/agent-02-a32-roundtrip.py /private/venus.mbn \
  --file-offset 0xc490 --vma 0x8490 --length 0x3ac \
  --data-range 0x8614:0x8618 --data-range 0x8684:0x8688 \
  --data-range 0x8834:0x883c
```

For all further checkpoints, continue at **exactly** `0x883c` and add only
consecutive verified instructions or named data words to bridge layout.

## Checkpoint 02 — record classification and queue state (`0x883c–0x8c00`)

- Adds **964** exact bytes (226 ARM32 instructions and 15 non-executable
  words: 8 format jump targets, 6 error/context words, 1 queue-head address).
- New region module: `venus_runtime_bridge_records.rs`, with record-header
  validation, type dispatch, attribute extraction and queue-state access.
- **Cumulative bridge:** 1,904 contiguous bytes, runtime `[0x8490,0x8c00)`,
  file `[0xc490,0xcc00)`; independently regenerated early boot+bridge:
  **35,840 contiguous bytes** (`0x4000–0xcc00` in the MBN).
- **Remaining gap:** `[0x8c00,0xaa0c)` = **7,692 bytes**. The original
  reference is only read by the opt-in verification command.

```sh
cargo run --offline --locked -p firmware-tools --bin venus_bridge -- verify /private/venus.mbn
cargo run --offline --locked -p firmware-tools --bin venus_bridge -- verify-joined /private/venus.mbn
python3 tools/agent-02-a32-roundtrip.py /private/venus.mbn \
  --file-offset 0xc83c --vma 0x883c --length 0x3c4 \
  --data-range 0x899c:0x89bc --data-range 0x8b94:0x8bac \
  --data-range 0x8bd8:0x8bdc
```

## Checkpoint 03 — queue global state and codec slots (`0x8c00–0x8e98`)

- Adds **664 byte-identical bytes**, including 152 A32 instructions and 14
  explicitly distinguished global-pointer/diagnostic data words.
- Routines cover queue initialization, live queue pointers, bounded memory
  movement, linked slot acquisition and basic codec-state accounting.
- New module: `venus_runtime_bridge_state.rs` with strict address checks.
- Cumulative Agent 02 bridge: **2,568 bytes** `[0x8490,0x8e98)`;
  contiguous generated boot+bridge: **36,504 bytes**, file `[0x4000,0xce98)`.
- Remaining exact gap `[0x8e98,0xaa0c)` is **7,028 bytes**.
- Independent `clang`/`llvm-objdump`/`llvm-objcopy` round-trip:

```sh
python3 tools/agent-02-a32-roundtrip.py /private/venus.mbn \
  --file-offset 0xcc00 --vma 0x8c00 --length 0x298 \
  --data-range 0x8c1c:0x8c24 --data-range 0x8c48:0x8c50 \
  --data-range 0x8c58:0x8c5c --data-range 0x8c80:0x8c88 \
  --data-range 0x8d44:0x8d50 --data-range 0x8e7c:0x8e88 \
  --data-range 0x8e94:0x8e98
```

Validation: exact byte comparison against the private Venus MBN, combined
boot+bridge comparison, 164 offline Rust tests, 13 Python tests and release
build. The integrated image remains partial and is not flashable.

## Checkpoint 04 — pending interrupts, slot queues and device IRQ (`0x8e98–0x9400`)

- An additional **1,384 contiguous bytes**, 329 typed A32 instructions and
  17 separately identified pointer/error-message literal words, generated by
  `venus_runtime_bridge_slots.rs`.
- Reconstructed interrupt snapshots, linked slot bookkeeping, platform
  synchronization and the beginning of the device IRQ handler. Some branch
  targets lie beyond this interval and are not yet independently executable.
- **Bridge cumulative:** **3,952 bytes**, runtime `[0x8490,0x9400)` /
  file `[0xc490,0xd400)`; with the previously authored boot prefix the
  joined contiguous region is **37,888 exact bytes**, file `[0x4000,0xd400)`.
- Remaining bridge target: `[0x9400,0xaa0c)` = **5,644 bytes**.
- Independent assembly round-trip from Clang/LLVM reproduces the entire new
  interval using these *non-instruction data* ranges:

```sh
python3 tools/agent-02-a32-roundtrip.py /private/venus.mbn \
  --file-offset 0xce98 --vma 0x8e98 --length 0x568 \
  --data-range 0x8ec0:0x8ec4 --data-range 0x9110:0x9124 \
  --data-range 0x913c:0x9140 --data-range 0x9258:0x9264 \
  --data-range 0x92c8:0x92d8 --data-range 0x934c:0x9358
```

## Checkpoint 05 — IRQ flag transitions and readiness (`0x9400–0x970c`)

- Another **780 contiguous verified bytes**, 182 ARM32 instructions and 13
  explicitly distinguished non-instruction data words.
- Routines finish device IRQ paths, enable/clear guarded controller flag
  masks and poll two platform readiness pointers. Branches to other routines
  are encoded but this is not executable independently as firmware.
- **Cumulative gap bridge:** 4,732 exact bytes `[0x8490,0x970c)` /
  file `[0xc490,0xd70c)`. With the existing 33,936-byte boot/runtime
  prefix, **38,668 contiguous bytes** are reconstructed and byte-identical,
  file `[0x4000,0xd70c)`.
- Remaining bridge `[0x970c,0xaa0c)` is **4,864 bytes**.
- Reproducible independent LLVM/Clang verification:

```sh
python3 tools/agent-02-a32-roundtrip.py /private/venus.mbn \
  --file-offset 0xd400 --vma 0x9400 --length 0x30c \
  --data-range 0x94e8:0x94f4 --data-range 0x9534:0x9538 \
  --data-range 0x95e0:0x95e8 --data-range 0x96bc:0x96d0 \
  --data-range 0x9704:0x970c
```

## Checkpoint 06 — resource descriptors, aligned and strided copies (`0x970c–0x9b50`)

- Adds **1,092 byte-identical bytes**, 259 independently authored typed A32
  instructions and 14 explicitly identified literal/data words. One word is
  the recognizable `0xa5a5a5a5` sentinel; this is non-executable data.
- Reconstructed platform callback dispatch, descriptor initialization,
  aligned/strided buffer copy wrappers and resource disabling/queue updates.
  `arm32::DataOp::Teq` was needed for the flag-only parity test at `0x9b28`
  and was added in an isolated encoder commit with unit tests.
- **Cumulative bridge:** 5,824 exact bytes `[0x8490,0x9b50)` /
  file `[0xc490,0xdb50)`. Joined early boot+bridge **39,760** consecutive
  byte-identical bytes `[0x4000,0xdb50)`; still not complete firmware.
- Remaining gap `[0x9b50,0xaa0c)` is **3,772 bytes**.
- Clang/LLVM round-trip on the additional span (excluding literal pools):

```sh
python3 tools/agent-02-a32-roundtrip.py /private/venus.mbn \
  --file-offset 0xd70c --vma 0x970c --length 0x444 \
  --data-range 0x9734:0x9738 --data-range 0x98a0:0x98bc \
  --data-range 0x9968:0x996c --data-range 0x9a18:0x9a1c \
  --data-range 0x9ac8:0x9acc --data-range 0x9b44:0x9b50
```

## Checkpoint 07 — resource release and controller queues (`0x9b50–0x9fc8`)

- Adds **1,144 exact bytes**: 259 original-operation A32 instructions,
  27 named data words in seven explicit pools.
- Functions cover resource flag clearing, neighboring-list unlinking,
  reference-count decrement, controller queue initialization, and queue-head
  access. Branches to reconstructed or later external routines are preserved.
- **Cumulative bridge:** 6,968 contiguous bytes, runtime `[0x8490,0x9fc8)` /
  file `[0xc490,0xdfc8)`. With the 33,936-byte initial boot/runtime prefix:
  **40,904 consecutive byte-identical bytes**, MBN file `[0x4000,0xdfc8)`.
- **Remaining missing bridge:** `[0x9fc8,0xaa0c)`, **2,628 bytes**.
- Independent LLVM/Clang round-trip with separate literal ranges:

```sh
python3 tools/agent-02-a32-roundtrip.py /private/venus.mbn \
  --file-offset 0xdb50 --vma 0x9b50 --length 0x478 \
  --data-range 0x9bc8:0x9bd0 --data-range 0x9d9c:0x9db4 \
  --data-range 0x9e20:0x9e2c --data-range 0x9ea8:0x9eb4 \
  --data-range 0x9ec4:0x9ec8 --data-range 0x9ed8:0x9edc \
  --data-range 0x9f9c:0x9fc8
```

## Checkpoint 08 — controller IRQ queues and priority lists (`0x9fc8–0xa44c`)

- Adds **1,156 exact bytes**, containing 267 typed A32 instructions and 22
  separate controller-pointer, error and list-head literal words in thirteen
  identified non-instruction data regions.
- Implements controller queue/head operations, priority list insertion,
  resource removal, current-service selection, interrupt-state checks and
  guarded retry loops. Calls to out-of-range routines are not complete
  standalone firmware implementations.
- **Cumulative bridge:** 8,124 byte-identical bytes `[0x8490,0xa44c)` /
  file `[0xc490,0xe44c)`. Early boot + bridge: **42,060** consecutive
  reference-identical bytes from MBN file offset `0x4000` through `0xe44c`.
- **Remaining gap:** `[0xa44c,0xaa0c)` = **1,472 bytes**.
- Independent Clang/LLVM reassembly precisely distinguishes all thirteen
  non-executable data ranges:

```sh
python3 tools/agent-02-a32-roundtrip.py /private/venus.mbn \
  --file-offset 0xdfc8 --vma 0x9fc8 --length 0x484 \
  --data-range 0x9fd8:0x9fdc --data-range 0xa01c:0xa024 \
  --data-range 0xa034:0xa038 --data-range 0xa040:0xa044 \
  --data-range 0xa098:0xa0a0 --data-range 0xa0e0:0xa0e8 \
  --data-range 0xa0f4:0xa0f8 --data-range 0xa1ac:0xa1b4 \
  --data-range 0xa1c0:0xa1c4 --data-range 0xa228:0xa230 \
  --data-range 0xa27c:0xa288 --data-range 0xa32c:0xa334 \
  --data-range 0xa398:0xa3a0
```

## Checkpoint 09 — bridge complete through service boundary (`0xa44c–0xaa0c`)

- Adds the **final 1,472 byte-identical bytes**: 340 typed ARM32
  instructions and 28 separately modeled literal/global-pointer words.
- Routines cover queue-balance adjustment, resource changes, creation of
  codec/video-session descriptors, forwarding calls into the existing Venus
  service code, and session/controller validation. The code branches into
  later routines; this independently authored interval is **not flashable**.
- **Agent 02 bridge target fully reconstructed:** 9,596 consecutive bytes,
  runtime `[0x8490,0xaa0c)` and MBN file `[0xc490,0xea0c)`.
- **Verified contiguous boot + bridge:** 43,532 bytes, runtime
  `[0x1000,0xaa0c)`, MBN file `[0x4000,0xea0c)`.
- **Remaining Agent 02 gap:** zero bytes. The independently reconstructed
  *later* service starts at runtime `0xaa0c`, and its attachment to this
  bridge must be verified in the integrator's checkpoint-50-based branch.
  This checkout intentionally does not contain that service-source tree.
- New independent ELF/A32 Clang/LLVM assembler check of 340 instructions,
  28 data words matches the reference exactly, using:

```sh
python3 tools/agent-02-a32-roundtrip.py /private/venus.mbn \
  --file-offset 0xe44c --vma 0xa44c --length 0x5c0 \
  --data-range 0xa44c:0xa454 --data-range 0xa520:0xa524 \
  --data-range 0xa5b4:0xa5bc --data-range 0xa638:0xa640 \
  --data-range 0xa69c:0xa6a4 --data-range 0xa7e8:0xa800 \
  --data-range 0xa85c:0xa864 --data-range 0xa870:0xa874 \
  --data-range 0xa888:0xa88c --data-range 0xa89c:0xa8a0 \
  --data-range 0xa8ac:0xa8b0 --data-range 0xa904:0xa908 \
  --data-range 0xa914:0xa918 --data-range 0xa990:0xa99c \
  --data-range 0xa9d0:0xa9d4 --data-range 0xaa08:0xaa0c
```

## Checkpoint 10 — independent full-gap validation and integration handoff

The Agent 02 target is **complete**. Source-generated contiguous Venus bridge
runtime `[0x8490,0xaa0c)` is **9,596 exact bytes**, consisting of **2,245**
encoded ARM32 instructions plus **154** distinguished non-instruction words.
It joins the existing **33,936-byte** early boot/runtime prefix into **43,532
contiguous independently emitted bytes**, MBN file `[0x4000,0xea0c)`.

The new one-command independent verifier cross-checks the **entire gap in one
Clang/LLVM reconstruction**, with the full ordered list of literal regions:

```sh
PATH=/path/to/offline-rust/bin:$PATH \
  bash tools/agent-02-verify-full-gap.sh /private/venus.mbn
```

To generate the complete boot-and-bridge output without OEM bytes:

```sh
cargo run --offline --locked -p firmware-tools --bin venus_bridge -- \
  emit-joined /tmp/venus-boot-bridge.bin
```

To also verify a separately generated **Agent 01/Checkpoint 50 service**
fragment, provide its emitted binary as the optional second argument:

```sh
bash tools/agent-02-verify-full-gap.sh /private/venus.mbn \
  /tmp/venus-service-region.bin
```

This verifies that the supplied service bytes follow the bridge without gaps
and match the reference. It does not infer the provenance of supplied service
bytes, does not create missing authentication, and does not make the firmware
flashable.

**Integration onto checkpoint 50:** This branch is based on the available
older boot-prefix commit `69ebf0d`, **not** on the requested checkpoint 50
commit `b7ed4b5`. Fetch this agent bundle, then cherry-pick the Agent 02
commits in their original order, rather than merging unrelated default main
histories. Preserve the separate ARM32 helper commits (`TEQ` and the
register-indexed halfword transfer). If checkpoint 50 already implements one
of these helpers, resolve the overlap by retaining a single consistent checked
implementation and rerun all tests. Do not apply this agent's `docs/STATUS.md`
from an older tree; it was not modified. The only shared module registration
change is `xiaomi_nabu/mod.rs`; the agent does not alter the forward service
planner.

**Final acceptance:** zero remaining bytes in the exclusive Agent 02 gap;
whole-bridge byte comparison, whole-bridge independent assembler comparison,
joined prefix byte comparison, offline Rust and Python tests, and release
build pass. A new integration verification is required on the checkpoint-50-
based branch before claiming continuous coverage through its service area.
