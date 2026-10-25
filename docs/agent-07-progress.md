# Agent 07 — `wlanmdsp.mbn` executable reconstruction

## Provenance / baseline limitation

Intended integration base is checkpoint 50, `b7ed4b587633b9b475a40f5a3ace5cc90db620ff`, branch `agent/07-wlanmdsp`. **That Git bundle/commit is not available in this conversation's mounted container**. The available `recompile-firmware.bundle` ends at `69ebf0d` and contains the compatible earlier ancestor `1ef37874e2993321700f4e934639fe447a26056d`. Work is rooted at that available ancestor and restricted to new, WLAN-specific source, tests, tools and this agent progress note. These commits are therefore intended for `git cherry-pick` on the actual checkpoint-50 repository; **do not claim they have the requested direct parent**.

The exact private WLAN reference is not checked into Git. It was extracted from the user's `nabu-downloads-ubuntu24.04-amd64.tar.zst.part{001,002,003.1,003.2,004,005.1,005.2,006}` archive in that explicit order and verified against `reference/xiaomi-nabu/firmware.sha256`:

- WLAN revision SHA-256: `87c2c7bb1fe97eaf007a947832feba0898644b4d6626ac8ef49d8b94dd82604b`.
- Little-endian ELF32, e_machine=164 (Qualcomm Hexagon DSP6), `e_flags=0x66`, entry address `0xb0000000`.
- Executable `PT_LOAD`: file offset `0x40000`, vaddr/paddr `0xb0000000`, `p_filesz=p_memsz=0x313d38`, `PF_R|PF_X`.
- Other mapped data starts at file offset `0x380000` (RW, much larger runtime `p_memsz`), whereas the NOTE and Qualcomm hash/auth metadata are **not program text**.

## Independent encoding and ISA limits

The supplied LLVM 6.0.1 `llvm-mc` (Hexagon target) can independently **decode** the opening instructions with v60, v62 and v65 selection; **assembling** this source requires `hexagonv62` or `hexagonv65` because LLVM6 rejects `framelimit` for v60. The file flag `0x66` indicates a newer Hexagon version, which this LLVM 6 assembler cannot target (`hexagonv66` rejected). However, the reconstructed fragment produces identical bytes across the supported assembler CPU choices (v62 and v65) **and** against the exact original firmware. That verifies the fragment, not the full instruction-set version.

`wlan-startup.S` is independently authored packetized Hexagon assembly, documenting:

- `[0xb0000000, 0xb00000a8)`: QuRT/TLS and global-pointer initialization, BSS cleanup calls, subsystem callback setup and idle loop.
- `[0xb00000a8, 0xb00000f8)`: zero-initialization routine, handling initial unaligned bytes, 8-byte aligned stores and trailing bytes. Hexagon packet grouping matters: each backward jump executes together with its associated memory store.
- `[0xb00000f8, 0xb0000100)`: extended relative branch to the existing runtime heap stub at vaddr `0xb0001140`; intentionally forced by the assembler's `##` immediate notation.
- `[0xb0000100, 0xb0000154)`: C-runtime constructor/finalizer initialization, including a duplex `r3/r1` constant packet. Source order is required for byte-identical VLIW encoding.

Build without OEM reference using:

```sh
python3 tools/build-wlanmdsp-startup.py /tmp/wlan-startup.bin \
  --llvm-mc /path/to/llvm6/bin/llvm-mc
```

Optionally supply `--reference /private/wlanmdsp.mbn` to compare the independently generated output to the *full hash-verified reference* at file offset `0x40000`. This rejects wrong revisions and the first differing byte with both file offset and vaddr. `--manifest /tmp/wlan-startup.json` records generated digest and coverage. The builder refuses unresolved text relocations, invalid ELF32, and raw embedded opcode directives (`.incbin`, `.word`, etc.).

### Checkpoint A: independently generated startup

- File range: `[0x40000,0x40154)`; virtual address `[0xb0000000,0xb0000154)`.
- Total: **340 bytes of independently written Hexagon assembly**, no original opcode words included.
- Exact output SHA-256: `11a77a2cd5e9a41e87f622bd559c19a4c2c9a1036c3071ed63b52e7fb9a688c1`.
- Comparison: **340/340 identical bytes; first differing offset: none**.
- Coverage excludes padding and every later program function; this is **not signed/bootable firmware**.

### Checkpoint B: deterministic PLT and first-page layout

The reference `DT_PLTGOT=0xb0340018`, `DT_PLTRELSZ=2172` and
`DT_RELAENT=12` identify **181** Hexagon lazy binding entries. Re-implementing
the actual PLT resolver and the 181 16-byte thunks from these ELF linker
parameters produces exactly `0x1b80 - 0x1000 = 2944` bytes:

- Resolver: 28 bytes of independently assembled Hexagon instructions.
- Resolver-to-thunk alignment: 20 **zero-layout bytes**, not executable code.
- Import stubs: 181 × 16 = 2896 bytes of independently generated instructions.
- Each thunk reads one indexed PLTGOT entry and tail-jumps to the resolved
  function pointer. The import-slot formula is `(PLTGOT+0x10 + 4*n) -
  (0xb0001030+16*n)` for PC-relative displacements, with executable-vaddr
  normalization applied by the build script.

The preceding `[0x154,0x1000)` gap is exactly **939 repetitions** of the
4-byte word `0x00c0007f`, structurally identified as *linker page-alignment
fill*, not a reconstructed function or a Hexagon `nop` opcode. The generation
rule models this observation; it is **not counted as executable logic**. Use
`--with-plt` to regenerate the full `0x1b80`-byte first region, and
`--emit-plt-assembly /tmp/wlan-plt.S` to inspect the generated PLT source.

```sh
python3 tools/build-wlanmdsp-startup.py /tmp/wlan-through-plt.bin   --with-plt --llvm-mc /path/to/llvm6/bin/llvm-mc   --reference /private/wlanmdsp.mbn
```

- File `[0x40000,0x41b80)`, vaddr `[0xb0000000,0xb0001b80)`: **7,040/7,040**
  exact bytes, SHA-256 `7ac9069ef3adba598f1058276e7c6e8ad57ffa1a3dbc87ed66cbe729c050f0c9`.
- Of these, independently authored executable code: 340 startup + 2924 PLT
  bytes = **3,264 bytes**. Structural linker fill: 3756 patterned bytes +
  20 zero-alignment bytes = **3,776 bytes**. These totals must never be
  represented as 7040 bytes of decompiled independent functional code.

## Remaining work

Continue with the first non-PLT routine at `0xb0001b80` (MBN offset `0x41b80`). The executable program is over 3.2 MiB; reconstructing the rest of its original functional source remains open. The signature/hash program header and the external DSP loader are also not independently reconstructed. Full image emission must be refused until all executable bytes, relocation/linker data and external authentication are accounted for.

### Checkpoint C: post-PLT runtime relocation hooks

The next exactly adjacent executable interval `[vaddr 0xb0001b80,0xb0001be4)`
(`wlanmdsp.mbn` file `[0x41b80,0x41be4)`) contains **100 bytes** of
independently assembled Hexagon instructions: two PC-relative stores of saved
register values into loader/QuRT runtime slots, extended tail branches to
pre-existing procedure-linkage stubs, seven real Hexagon `nop` packets, and a
two-word absolute store packet. `wlan-loader-hooks.S` models their control flow;
no vendor opcode word array or reference segment is embedded in the repository.

Build `--with-loader-hooks` implies the initial 0x1b80-byte startup, linker
fill, and PLT, and appends exactly 100 independently assembled executable
bytes. The emitted combined region is **7,140 bytes** (file
`[0x40000,0x41be4)`, virtual `[0xb0000000,0xb0001be4)`). Executable code
coverage is 3,364 bytes; the 3,776 linker fill/alignment bytes continue to
be accounted for separately. The result is compared against the exact
manifest-hash-matching private ELF reference with zero differences. Both
`hexagonv62` and `hexagonv65` LLVM6 assemblers emit identical instructions.

```sh
python3 tools/build-wlanmdsp-startup.py /tmp/wlan-through-hooks.bin \
  --with-loader-hooks --llvm-mc /path/to/llvm6/bin/llvm-mc \
  --reference /private/wlanmdsp.mbn --manifest /tmp/through-hooks.json
```

The next unimplemented function begins at `0xb0001be4`, file `0x41be4`.
This remains a disconnected partial executable and is not a valid signed or
flashable WLAN firmware image.

### Checkpoint D: bounded WLAN configuration table selector

The Hexagon table selector at runtime `[0xb0001be4,0xb0001c80)` / ELF file
`[0x41be4,0x41c80)` is reconstructed from independently written Hexagon
assembly (`wlan-config-lookup.S`, **156 bytes**). The original routine
searches a packed table of device/configuration tags, validates a requested
entry/count, and extracts packed bitfields into five words of caller-provided
metadata. Its matching/overflow paths return status values; no OEM
instruction-word arrays are included in source.

The assembler required careful reconstruction of **four-word VLIW packets**:
an immediate-extended mux shares a packet with another immediate-extended
conditional register load; the new-value conditional branches and stores must
share packets with their producers. After preserving these semantics,
`hexagonv62` and `hexagonv65` emit the exact same 156-byte output, matching
the private original at file offset `0x41be4` without a single differing byte.

`--with-config-lookup` implies startup, linker fill, all 181 generated PLT
stubs, and loader hooks. Combined region: **7,296 bytes** (file
`[0x40000,0x41c80)`, VMA `[0xb0000000,0xb0001c80)`) of which independently
reconstructed executable instructions total **3,520 bytes**. The other
3,776 bytes are explicit linker-layout fill, not code. A new test performs
strict whole-ELF-SHA verified byte comparison when the reference is supplied.

```sh
python3 tools/build-wlanmdsp-startup.py /tmp/wlan-through-lookup.bin \
  --with-config-lookup --llvm-mc /path/to/llvm6/bin/llvm-mc \
  --reference /private/wlanmdsp.mbn --manifest /tmp/through-lookup.json
```

The next unimplemented codec-format consumer begins at runtime
`0xb0001c80`, ELF offset `0x41c80`; executable coverage remains partial and
no full signed WLAN MBN is generated.

### Checkpoint E: bounded WLAN record-consumer entry

The next portion of the large packed-record consumer is independently
assembled as 148 verified bytes, file `[0x41c80,0x41d14)` and runtime
`[0xb0001c80,0xb0001d14)`. Source is
`sources/xiaomi-nabu/wlanmdsp/wlan-record-consumer-prefix.S`. This is
**explicitly an unfinished function**, not a complete callable routine.

The entry sets up its Hexagon stack frame and saved registers, checks
configuration parameters, reads global runtime state, verifies a bounded
record count, and initializes local working variables. Three external
calls/branches are represented as PC-relative branch instructions into known
function addresses; new-value packet grouping is preserved. Hexagon LLVM6
assembler for both `hexagonv62` and `hexagonv65` produces exactly the
reference 148 bytes (zero differences), without embedding original opcode
words. The next unimplemented bytes begin at runtime address `0xb0001d14`.

`--with-record-prefix` implies all preceding stages and builds a
**7,444-byte** exact executable-layout prefix (ELF file
`[0x40000,0x41d14)`). Executable instructions total **3,668 bytes**; the
3,776 linker-fill/alignment bytes remain separately identified. The
reference can be supplied strictly for verification, but is never copied
into the generated artifact at build time.

```sh
python3 tools/build-wlanmdsp-startup.py /tmp/wlan-through-consumer-prefix.bin \
  --with-record-prefix --llvm-mc /path/to/llvm6/bin/llvm-mc \
  --reference /private/wlanmdsp.mbn --manifest /tmp/consumer-prefix.json
```

The remaining ≈3.22 MiB Hexagon executable and all non-executable container
and external authentication fields are not reconstructed. This fragment is
not flashable WLAN firmware.

### Checkpoint F: expanded packet-consumer validation and metadata transfer

The same Hexagon packed-record consumer is now implemented through **300
byte-identical bytes**, covering runtime `[0xb0001c80,0xb0001dac)` and ELF
file `[0x41c80,0x41dac)`. This advances **152 real executable bytes** beyond
checkpoint E. The extended source checks the global record list, makes calls
to the existing index/size lookup, validates the packed 16-/18-/19-format
entry against caller capacity, initializes working stack fields, and begins
descriptor-size normalization.

The new-value packet rules are important: counter comparisons, predicate
branches, paired memory stores, and the group of four instructions at
`0x1d98` must be emitted as the correct Hexagon VLIW bundles. Branches to
unimplemented later handlers remain *relative targets*, not fabricated
function bodies. `wlan-record-consumer-prefix.S` ends at an original packet
boundary and does **not** pretend the larger consumer is complete.

The complete contiguous executable-layout prefix is **7,596 bytes**
(`0x40000–0x41dac` file and `0xb0000000–0xb0001dac` runtime). Independently
encoded executable instructions total **3,820 bytes**, alongside **3,776
separately modeled linker fill/alignment bytes**. Byte equality is checked
against the SHA-256-fixed WLAN reference, including earlier reconstructed
code; zero differences. Test builds with LLVM6 `hexagonv62` and `hexagonv65`
agree byte-for-byte. Next unimplemented runtime address: `0xb0001dac`.

```sh
python3 tools/build-wlanmdsp-startup.py /tmp/wlan-through-record-prefix.bin \
  --with-record-prefix --llvm-mc /path/to/llvm6/bin/llvm-mc \
  --reference /private/wlanmdsp.mbn --manifest /tmp/wlan-record-prefix.json
```

### Checkpoint G: three independent import-alias trampolines (disconnected)

The range `[0xb0001fa8,0xb0001fc0)` (file `[0x41fa8,0x41fc0)`) consists
of **three eight-byte Hexagon tail branches**, targeting the already rebuilt
PLT import stubs for `memset`, unsigned 32-bit division and `memmove`.
`sources/xiaomi-nabu/wlanmdsp/wlan-import-aliases.S` independently expresses
them as relative extended Hexagon branches, **not** encoded OEM opcode words.
Assembly under Hexagon v62/v65 and exact revision-checked reference comparison
confirm all **24/24** bytes, zero mismatches.

Crucially, this is a **disconnected** independently reconstructed region:
`[0xb0001dac,0xb0001fa8)` (508 runtime bytes) is still unimplemented. The
contiguous startup prefix remains exactly **7,596 bytes**; counting this
branch group separately raises the independent executable-code total from
3,820 to **3,844 bytes**, without falsely claiming longer contiguous coverage.
No added file contents are copied or filled with OEM reference instruction
bytes. All generated bytes are unsigned and not a flashable firmware.

Use `--import-aliases-only` for the disjoint artifact; reference verification
now supports an explicitly bounded `runtime_offset` within the executable
`PT_LOAD`, while preserving strict SHA-256 revision, ELF32 and load-segment
checks for all callers:

```sh
python3 tools/build-wlanmdsp-startup.py /tmp/wlan-import-aliases.bin \
  --import-aliases-only --llvm-mc /path/to/llvm6/bin/llvm-mc \
  --reference /private/wlanmdsp.mbn --manifest /tmp/wlan-aliases.json
```

Next work: complete remaining record-consumer body or independently recover
function-level routines after `0xb0001fc0` without confusing disjoint spans
with a contiguous firmware reconstruction.

### Checkpoint H: import-alias retry and fieldwise comparator

The disjoint import-alias range now extends from `0xb0001fa8` through
`0xb0002010` (ELF file `[0x41fa8, 0x42010)`): **104 independently assembled
and revision-verified Hexagon bytes**. The previously reconstructed 24-byte
alias thunks remain unchanged. The following 80 bytes, authored as
`wlan-postaliases-compare.S`, implement:

- A packetized retry branch from `0xb0001fc0` to the unfinished record
  consumer entry at `0xb0001c80`, preserving its paired `combine` operation.
- A fieldwise five-word record comparator at `0xb0001fc8–0xb0002010`, which
  returns equal/unequal using new-value comparisons and a shared return.
- Exact packet shape at runtime `0xb0001fcc–0xb0001fd4`: `r2 = r0` must
  precede `r0 = #0` in source to reconstruct the original duplex encoding.

`--import-aliases-with-compare` assembles all 104 bytes; it rejects combining
this disconnected region with the prior contiguous startup. Both LLVM6
`hexagonv62` and `hexagonv65` independently encode the 80 new bytes identically
and the 104-byte output matches the private hash-verified `wlanmdsp.mbn` with
zero mismatches. The contiguous startup prefix stays at **7,596 bytes**, of
which 3,820 bytes are independently encoded instructions and 3,776 bytes are
identified linker fill/alignment. Separately covered executable bytes now
measure **104**, including 24 import aliases and 80 retry/comparator bytes.

```sh
python3 tools/build-wlanmdsp-startup.py /tmp/alias-through-compare.bin \
  --import-aliases-with-compare --llvm-mc /private/llvm6/bin/llvm-mc \
  --reference /private/wlanmdsp.mbn --manifest /tmp/alias-compare-manifest.json
```

The unimplemented gap `[0xb0001dac,0xb0001fa8)` is still 508 bytes and the
following independent function begins at `0xb0002010`. The 104-byte artifact
is not a flashable firmware or an independently authenticated image.

### Checkpoint I: Hexagon hardware-loop record-array comparator

The disconnected executable region now covers **400 exact independently
assembled bytes**: runtime `[0xb0001fa8,0xb0002138)` and ELF file
`[0x41fa8,0x42138)`. The new 296-byte function at `0xb0002010` compares
record array entries, checks field counts and data ranges, and searches for
matching entries using Hexagon `loop0` and `loop1` hardware loops. It has
explicit mismatch/error exits and a shared reporting tail; it is a **complete
function**, unlike the unfinished earlier record consumer.

The implementation in `wlan-record-array-compare.S` was independently written
as packetized Hexagon assembly. Two loop-start targets must point to
source offsets `0x70` and `0x80`, not merely the comparison instructions;
incorrect loop targets assemble without error but change exact bytes.
Absolute far jumps are expressed as displacements normalized to this
independently assembled fragment's runtime start. The code contains no OEM
instruction byte arrays. The LLVM6 `hexagonv62` and `hexagonv65` encodings
are identical to the exact hash-verified reference at `[0x42010,0x42138)`.

```sh
python3 tools/build-wlanmdsp-startup.py /tmp/imports-and-array-compare.bin \
  --import-aliases-with-array-compare --llvm-mc /private/llvm6/bin/llvm-mc \
  --reference /private/wlanmdsp.mbn --manifest /tmp/wlan-array-compare.json
```

The contiguous initial startup prefix remains 7,596 layout bytes (3,820
independently encoded code plus 3,776 structural linker fill), and the
unimplemented `[0xb0001dac,0xb0001fa8)` gap remains 508 bytes. The adjacent
import-alias/comparator group is independently covered for 400 code bytes,
raising the separately reconstructed executable total to **4,220 bytes**.
Next function begins at `0xb0002138`. No complete signed WLAN MBN exists.

### Checkpoint J: record wrapper, interrupt-state guard and runtime table setup

The independent interval `[0xb0001fa8,0xb0002208)` / firmware file
`[0x41fa8,0x42208)` now contains **608 independently assembled and
reference-verified executable bytes**. The 208-byte extension includes a
76-byte comparison wrapper (`0x2138–0x2184`), a 28-byte conditional
interrupt-state guard (`0x2184–0x21a0`) and a 104-byte runtime-table setup
routine (`0x21a0–0x2208`). The comparison wrapper calls the earlier two
record comparators and forwards the result; the guard checks an interrupt
flag in QuRT state; the initialization routine installs global addresses,
performs setup calls, and preserves/restores a stack-saved register pair.

Rebuilding the runtime table requires Hexagon `:mem_noshuf` on a packet
containing a word load and a doubleword store. Without that annotation,
LLVM6 produces a different but legal instruction order. LLVM6 `hexagonv65`
assembles the correct original bytes; its `hexagonv62` target **rejects**
`mem_noshuf`, and the test explicitly records this architectural constraint
rather than implying v62 equivalence for all later code. The comparison
wrapper and IRQ-state guard still round-trip identically on v62/v65.

```sh
python3 tools/build-wlanmdsp-startup.py /tmp/imports-through-runtime-helpers.bin \
  --import-aliases-with-runtime-helpers \
  --llvm-mc /private/llvm6/bin/llvm-mc \
  --reference /private/wlanmdsp.mbn --manifest /tmp/wlan-runtime-helpers.json
```

The contiguous startup region remains **7,596 bytes**, comprising 3,820
independently encoded executable code bytes plus 3,776 linker fill/alignment.
This disconnected region comprises 608 independently encoded executable bytes,
for a **total of 4,428 independently reconstructed code bytes** without
counting unresolved gaps. The 508-byte `[0x1dac,0x1fa8)` gap remains open.
The next independent function begins at runtime `0xb0002208`. No complete
signed WLAN MBN is available from these fragments.

### Checkpoint K: close 508-byte record-consumer gap; join all prior regions

**The previously disconnected intervals are now one contiguous verified
executable layout region:** runtime `[0xb0000000,0xb0002208)` and WLAN ELF
file `[0x40000,0x42208)`, **8,712/8,712 byte-identical bytes** from
independently emitted assembly and deterministic linker-layout fill.

The formerly missing **508 bytes** at runtime `[0xb0001dac,0xb0001fa8)`
are now implemented as `wlan-record-consumer-body.S`. The body continues
from the original 300-byte consumer prefix, performs record count/format
checks, compares packed table entries, copies/merges the selected fields,
updates destination metadata and branches through the import-alias thunks.
It completes this packed-record consumer function. Notably, at runtime
`0xb0001de0` there is a Hexagon v66 arithmetic instruction
`r20 -= mpyi(r17,r16)` (firmware file offset `0x41de0`, relative body
`0x34`). The LLVM6 Hexagon v62/v65 disassembler reports it as unknown and
its assembler rejects that operand form. The supplied LLVM22 toolchain
accepts Hexagon v66, correctly assembles it, and produces the **exact 508
bytes** across the entire function. Independently, Rizin 0.9.1 recognizes
that operation by meaning, corroborating the ISA interpretation without
embedding the OEM instruction word in the source.

The full executable-layout prefix is reconstructed with two assembler
versions: LLVM6 v65 for the older well-supported startup, PLT and helper
routines; LLVM22 v66 for the new 508-byte consumer body. An explicit
`--llvm-mc-v66` CLI argument is mandatory when using
`--with-complete-record-consumer` or `--with-full-initial-region`; the
builder refuses to silently substitute unknown instructions or reference
bytes. LLVM22's x86_64 host tools require the bundled `libc++`, `libc++abi`
and `libunwind` runtime libraries in `LD_LIBRARY_PATH`.

```sh
LD_LIBRARY_PATH=/private/nabu-next-toolchain/ubuntu-runtime/flat \
python3 tools/build-wlanmdsp-startup.py /tmp/wlan-contiguous-prefix.bin \
  --with-full-initial-region \
  --llvm-mc /private/nabu-next-toolchain/llvm6/bin/llvm-mc \
  --llvm-mc-v66 /private/clang-llvm22/bin/llvm-mc \
  --reference /private/wlanmdsp.mbn \
  --manifest /tmp/wlan-contiguous-prefix.json
```

The 8,712 prefix layout bytes consist of **4,936 independently encoded
Hexagon executable bytes** and **3,776 identified fill/alignment bytes**.
The source's firmware-SHA binding ensures only the intended WLAN revision
passes opt-in reference checking. The next executable routine begins at
runtime `0xb0002208` (file `0x42208`); all later ELF load segments and
OEM authentication remain unrebuilt. This is not flashable firmware.

### Checkpoint L: Hexagon v66 format-state selector and three return trampolines

The contiguous independently generated WLAN executable-layout prefix extends
from virtual `[0xb0000000,0xb0002208)` to `[0xb0000000,0xb0002340)`;
file offsets `[0x40000,0x42340)` now comprise **9,024 byte-identical bytes**.
The 312 additional executable bytes at offsets `[0x42208,0x42340)` implement
codec format-state selection (NULL guard, tagged mode 0/1/2, global validation,
record reporting, and a direct fast path), with three relative tail-call
trampolines and six actual Hexagon `nop` instruction packets. Their source is
`wlan-runtime-format-handler.S`, assembled independently with LLVM22 v66.

No OEM executable opcode words are imported. Exactly **5,248 executable-code
bytes** are independently regenerated and **3,776 bytes** are identified
linker fill/alignment. Both strict firmware-SHA-matched reference comparison
and the source-level assembler byte comparison show **zero differences**.

```sh
LD_LIBRARY_PATH=/private/nabu-runtime/flat \
python3 tools/build-wlanmdsp-startup.py /tmp/wlan-9024.bin \
  --with-runtime-format-handler \
  --llvm-mc /private/llvm6/bin/llvm-mc \
  --llvm-mc-v66 /private/llvm22/bin/llvm-mc \
  --reference /private/wlanmdsp.mbn \
  --manifest /tmp/wlan-9024.json
```

SHA-256 for the independently generated 9,024 bytes:
`8fd925f9e455b4fdd8b339a0ab7119e0f708c1ae32ef745a6f4fef9df7a4e942`.
The next uncovered function begins at virtual **`0xb0002340`**, file offset
**`0x42340`**. Firmware beyond this prefix and external authentication remain
unreconstructed; this artifact is not a bootable/signed `wlanmdsp.mbn`.

### Checkpoint M: initialize WLAN runtime session and packed record table

The contiguous verified executable-layout prefix extends to
`[0xb0000000,0xb0002494)` / ELF file offsets `[0x40000,0x42494)`:
**9,364 generated bytes, exactly matching the pinned reference**.
`wlan-runtime-session-init.S` adds **340 independently generated instruction
bytes** at runtime `[0xb0002340,0xb0002494)` and completes a runtime session
initializer: two-iteration Hexagon hardware loop, metadata tag copying,
conditional diagnostics, and shared exit/tail-jump paths.

The source preserves the V66 packet memory ordering using `:mem_noshuf`
for the store and load at relative offsets `0xb0–0xb8`. Without this, LLVM22
legally exchanges instruction positions and changes two words. The final
Hexagon V66 LLVM22 assembler output matches all 340 original bytes exactly.

Command (requires external Hexagon-capable LLVM6 and LLVM22 tools):

```sh
LD_LIBRARY_PATH=/private/nabu-runtime/flat \
python3 tools/build-wlanmdsp-startup.py /tmp/wlan-9364.bin \
  --with-runtime-session-init \
  --llvm-mc /private/llvm6/bin/llvm-mc \
  --llvm-mc-v66 /private/llvm22/bin/llvm-mc \
  --reference /private/wlanmdsp.mbn \
  --manifest /tmp/wlan-9364.json
```

The now-verified prefix comprises **5,588 independently re-encoded Hexagon
instruction bytes** and **3,776 structural linker fill/alignment bytes**.
The next independent code begins at runtime **`0xb0002494`**, file offset
**`0x42494`**. The remaining executable program and container authentication
remain unreconstructed; this is neither the full firmware nor flashable.

### Checkpoint N: WLAN diagnostic and register-reporting wrappers

The verified executable-layout prefix now covers `0xb0000000–0xb00025a0`
(file `0x40000–0x425a0`), **9,632/9,632 identical bytes**. A new
268-byte Hexagon v66 source `wlan-runtime-reporting.S` encodes two complete
status-report routines, one local import-jump trampoline, and four compact
PC-relative tail branches. The report paths pass signed/unsigned metadata
through QuRT stack frames, emit diagnostics and update an atomic halfword.

One three-instruction VLIW packet originally assembled with a different
duplex ordering. Reordering *source instructions* to
`{ r16 = r1; r0 = ##48; memw(r29+#0) = r0 }` yields exactly the original
three instruction words, without raw opcode injection. The entire new
268-byte assembly independently matches the known reference.

```sh
LD_LIBRARY_PATH=/private/nabu-runtime/flat \
python3 tools/build-wlanmdsp-startup.py /tmp/wlan-9632.bin \
  --with-runtime-reporting \
  --llvm-mc /private/llvm6/bin/llvm-mc \
  --llvm-mc-v66 /private/llvm22/bin/llvm-mc \
  --reference /private/wlanmdsp.mbn \
  --manifest /tmp/wlan-9632.json
```

The prefix consists of **5,856 independently encoded executable bytes** plus
**3,776 linker-fill and PLT alignment bytes**. SHA-256:
`bb8a4ccb662ebeeab677ef2c36fcc5a45e6c3901c9f767a613fd2ffa1e6df50d`.
The next code begins at runtime `0xb00025a0` (ELF file `0x425a0`).
Remaining executable, loader tables and OEM authentication are incomplete;
this is not flashable firmware.
