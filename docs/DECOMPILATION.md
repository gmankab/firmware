# Firmware decompilation: practical handbook

This handbook describes a repeatable path from an **unchanged vendor firmware file** to a documented, testable Rust implementation. The process applies to Xiaomi Nabu and to future devices; hardware-specific examples are identified as such. It intentionally separates (a) locating and decoding instruction bytes, (b) understanding the program's behavior, (c) writing Rust source, and (d) proving a rebuilt file matches the reference.

> **There is no universal firmware decompiler.** Firmware images may contain several instruction sets, relocation records, microcontroller programs, DSP code, memory initialization, certificates, and padding. A disassembler may decode random bytes into plausible instructions. Validate processor identity and segment boundaries before trusting any pseudocode.

## 1. Prepare an isolated, repeatable environment

Fedora dependencies:

```sh
sudo dnf install -y \
  binutils xxd gcc gcc-c++ make pkgconf-pkg-config zstd \
  cmake meson ninja-build flex bison expat-devel libdrm-devel \
  rust cargo rustfmt java-latest-openjdk-devel
```

Debian/Ubuntu dependencies:

```sh
sudo apt-get update
sudo apt-get install -y \
  git ca-certificates curl file binutils xxd python3 \
  build-essential pkg-config zstd xz-utils \
  cmake meson ninja-build flex bison libexpat1-dev libdrm-dev
```

Install a Rust toolchain (`rustc`, `cargo` and optionally `rustfmt`). The current source builds with Rust 1.99.1. For headless Ghidra install **JDK 21 or newer as supported by the release**; Ghidra 12.1.4 requires JDK 21. Consult the [12.1.4 setup guide](https://www.ghidradocs.com/12.1.4_PUBLIC/GettingStarted.html).

Use a predictable directory layout **outside the Git checkout**:

```sh
mkdir -p "$HOME/firmware-work"/{original,analysis,tools,rebuild}
chmod 700 "$HOME/firmware-work/original" "$HOME/firmware-work/analysis"
```

Store complete original images, source package archives, private disassemblies, Ghidra databases, cryptographic signatures, and output comparisons there. Record the exact version/commit and SHA-256 of every decompiler or toolchain you use:

```sh
sha256sum /path/to/original-firmware > "$HOME/firmware-work/original/SHA256SUMS"
file /path/to/original-firmware
xxd -l 128 /path/to/original-firmware
```

**Avoid working on a mutable firmware file.** A reference image modified in-place makes later comparisons meaningless. Copy it once, hash it, and treat that copy as read-only.

## 2. Acquire and verify the exact reference revision

For Xiaomi Nabu, use the firmware payload contained in the corresponding postmarketOS package, not an arbitrarily named upstream clone. In the verified Nabu package, the Novatek file differs from one upstream checkout. Extract the package to a private directory. For an Alpine APK that is actually a gzip-compressed tarball:

```sh
file /path/to/firmware.apk
mkdir -p "$HOME/firmware-work/original/apk"
tar -tzf /path/to/firmware.apk | head -40
tar -xzf /path/to/firmware.apk \
  -C "$HOME/firmware-work/original/apk" --no-same-owner
```

Check the archive type before choosing `tar -z`; not every distribution or vendor uses the same format. Do not trust filenames alone. Verify each extracted file with SHA-256 (and, where available, the package manager's verified package metadata/signatures). The Nabu reference manifest is at `reference/xiaomi-nabu/firmware.sha256`:

```sh
cd "$HOME/firmware-work/original/apk/usr/lib/firmware"
sha256sum -c /path/to/checkout/reference/xiaomi-nabu/firmware.sha256
```

For another device, record **model, board revision, SoC, firmware package revision, file path, length, SHA-256, provenance, known loading driver**, and any expected sidecar files. A mismatched revision invalidates a claimed byte-identical result even if the functionality seems similar.

### Verify Qualcomm MBNv6 hashes before decompiling executable segments

A Qualcomm ELF32 MBNv6 image contains a signed hash segment distinct from executable segments. The public format places a 48-byte header, optional QTI/OEM metadata, a SHA-384 segment-hash table, and QTI/OEM signatures and certificate chains in a specific order. Use the device-independent verifier to establish which file-backed segments are covered by the table:

```sh
cargo run --offline --locked -p firmware-tools -- \
  verify-mbnv6-hashes /private/firmware.mbn
```

The Nabu A640 example contains three SHA-384 table entries: a digest of its 148-byte ELF header and program-header table, a zero digest for the hash segment, and the digest of the 1,968-byte QRISC/IR3 program. Its MBNv6 metadata is 120 bytes, represented in Rust by named image-policy fields. Compare full lengths and hash-table indices; **do not infer that ELF wrapper `e_machine` identifies every embedded microcode ISA**.

To implement a new signed format, create a checked layout of the header, metadata, hash entries, signer material and padding. Generate hash entries **from rebuilt source data**, not from a copied vendor hash table. In the final step separately validate OEM/QTI signatures and hardware trust policy; a matching SHA-384 table by itself proves neither. Never fill missing signature or certificate bytes with arbitrary placeholders and describe the result as working firmware.

Public reference: Qualcomm's *Secure Boot and Image Authentication Technical Overview*, plus public MBNv6 implementations in U-Boot/coreboot. See [Building](BUILDING.md) for verification tiers and the hardware acceptance boundary.

## 3. Identify the actual file format and processor

Start with inexpensive, nondestructive inspection:

```sh
file /private/firmware-image
xxd -l 256 /private/firmware-image
readelf -hW /private/firmware-image       # only if ELF
readelf -lW /private/firmware-image       # program headers / loadable ranges
readelf -SW /private/firmware-image       # sections, if present
```

Use the workspace's format inspector for supported images:

```sh
cargo run --offline --locked -p firmware-tools -- \
  inspect-image /private/firmware-image
```

It distinguishes a subset of ELF32LE, Cirrus WMFW, text configuration and candidate Novatek images. It does **not** establish the instruction set of arbitrary raw binaries.

Recommended triage questions:

1. Is this an executable, a compressed payload, a metadata-only file, a certificate/hash table, or a mixture?
2. Does its ELF `e_machine` describe the instructions in *all* loadable segments, or merely a wrapper used by a loader?
3. Where are `PT_LOAD`, executable-marked `PF_X`, BSS-only and vendor-defined hash segments?
4. What is the device-side execution architecture, instruction width and endianness? Check **more than one** independent source (ISA documentation, Linux driver, hardware manual, known firmware, disassembly).
5. Are virtual/physical addresses relocated by a loader? Does the processor enter through an ELF entry point or a different firmware-specific address?
6. Which bytes are authentication metadata or other unmodifiable signed regions?

For ELF32LE images with the Nabu-style header layout, the following script inventories **file-backed** byte ranges, without copying code:

```sh
python3 tools/analysis/elf-byte-budget.py \
  /private/firmware-tree /tmp/firmware-byte-budget.json
```

The script is also useful on a single file from another device. `PF_X` means a loader marked a segment executable; it does **not** mean that every byte in that segment is machine code. Scan for data tables and embedded programs.

## 4. Choose a decoder by *actual* architecture

| Actual content | Primary decoder / inspector | Notes |
| --- | --- | --- |
| Adreno A5xx/A6xx/A7xx QRISC command processor | Mesa Freedreno `qrisc-disasm`, `qrisc-asm` | May contain embedded IR3 instructions and data tables |
| Adreno IR3 shader programs | Mesa Freedreno `ir3-disasm` | Separate 64-bit instruction slots; GPU generation matters |
| Qualcomm Hexagon QDSP6 | Ghidra native Hexagon or compatible SLEIGH module; LLVM Hexagon tools | Choose exact ISA generation; not all plugins support V4 |
| ARM32/AArch64 | Ghidra, `llvm-objdump`, GNU `objdump` where supported, Rizin | Confirm Thumb/ARM mode, code addresses and relocation |
| Cirrus HALO WMFW | `firmware-tools` WMFW inspector + Cirrus/driver format sources | Container parsing comes before DSP-specific instruction decoding |
| Raw Novatek touch controller | Datasheet, hardware/ROM documentation, confirmed ISA-specific disassembler | Never guess ARM, 8051 or RISC-V from a few valid-looking opcodes |
| Text configuration | Text/JSON parser | No executable decompilation needed |

**Critical Nabu example:** `a640_zap.mbn` has an ELF wrapper identifying machine 164 (Hexagon), but its executable-marked 1,968-byte payload is Adreno QRISC code with embedded IR3 shader slots. Earlier Hexagon decoders gave widespread invalid instructions. The fact that a decoder accepts some words is not proof that it is the right architecture.

## 5. Ghidra headless decompilation of ELF firmware

Install Ghidra from [NSA's Ghidra releases](https://github.com/NationalSecurityAgency/ghidra/releases) and verify the release checksum. The official [headless-analyzer guide](https://github.com/NationalSecurityAgency/ghidra/blob/master/Ghidra/RuntimeScripts/support/analyzeHeadlessREADME.md) documents `-import`, `-scriptPath`, `-postScript`, processor selection, timeouts and project options.

For a single ELF, first try direct headless analysis:

```sh
GHIDRA=/opt/ghidra_12.1.4_PUBLIC    # adjust to the installed directory
mkdir -p /private/ghidra/projects
"$GHIDRA/support/analyzeHeadless" /private/ghidra/projects first_image \
  -import /private/firmware.elf \
  -analysisTimeoutPerFile 120 \
  -scriptPath "$PWD/tools/ghidra" \
  -postScript DecompileFirmware.java /private/ghidra/functions.c 2000 120 \
  -max-cpu 2 -deleteProject \
  > /private/ghidra/analysis.log 2>&1
```

Our `DecompileFirmware.java` exports bounded C-like function listings and failures. Its output is **not original source code**. Import diagnostics and explicit error lines are important evidence. The number of discovered functions can be low or misleading if the language is wrong.

For a directory containing ELF firmware from *any* device, a wrapper performs import, hashing, bounded processing and per-image error logging:

```sh
tools/decompile-all.sh \
  /private/extracted-device-firmware \
  "$GHIDRA" \
  /private/ghidra-batch
```

Only files with ELF magic are automatically imported. Other raw binaries require an intentional processor/loader choice, for example `-loader BinaryLoader`, `-loader-baseAddr`, and `-processor` with a **supported Ghidra language ID** (check the installed language list). Do not silently assign a guessed ISA.

You can extend the time budget for large images without embedding timeouts in code:

```sh
ANALYSIS_SEC=600 DECOMP_SEC=420 MAX_FUNCS=50000 \
  tools/decompile-all.sh /private/firmware.elf "$GHIDRA" /private/ghidra-large
```

To repeat Ghidra on a difficult file while skipping function addresses already present in an earlier export:

```sh
tools/continue-decompilation.sh \
  /private/firmware.elf "$GHIDRA" \
  /private/ghidra-batch/images/NAME-HASH/functions.c \
  /private/ghidra-second-pass
```

Manually review decompiled function boundaries, unresolved branches, constant data misidentified as code, stack conventions, abnormal calling conventions and indirect jump tables. Ghidra cannot restore the original Rust/C build flags or vendor source design from a stripped image. For Hexagon V66+, consult the [Hexagon SLEIGH implementation](https://github.com/CUB3D/ghidra-hexagon-sleigh) and check compatibility with your Ghidra version; its documented support starts at **V66**, so don't assume it handles a V4-labelled wrapper.

## 6. Mesa Freedreno: QRISC and IR3 (Adreno)

Mesa supplies **different tools for different execution engines**. Its [Freedreno QRISC README](https://gitlab.freedesktop.org/mesa/mesa/-/blob/main/src/freedreno/qrisc/README.rst) describes the ISA, assembler, command-processor control flow and issues in reassembling firmware with embedded tables and relocations. The [Mesa Freedreno guide](https://docs.mesa3d.org/drivers/freedreno.html) explains practical disassembly of SQE firmware. The [ISASPEC guide](https://docs.mesa3d.org/drivers/freedreno/isaspec.html) documents the XML-based instruction fields.

### Build the free-software tools

On Ubuntu 24.04 or Debian 13, build against your host rather than installing Ubuntu's glibc over Debian. The project verified Mesa **26.2.4** with SHA-256 `bce5f7fbebb934373b86c999a064d52fb5065878dc57f287f95346648ec832e9`:

```sh
VERSION=26.2.4
curl --fail --location --output "/tmp/mesa-${VERSION}.tar.xz" \
  "https://archive.mesa3d.org/mesa-${VERSION}.tar.xz"
printf '%s  %s\n' \
  bce5f7fbebb934373b86c999a064d52fb5065878dc57f287f95346648ec832e9 \
  "/tmp/mesa-${VERSION}.tar.xz" | sha256sum -c -
mkdir -p /tmp/mesa-source
tar -xJf "/tmp/mesa-${VERSION}.tar.xz" -C /tmp/mesa-source
meson setup /tmp/mesa-build "/tmp/mesa-source/mesa-${VERSION}" \
  -Dtools=freedreno -Dbuild-tests=false \
  -Dgallium-drivers=[] -Dvulkan-drivers=[] -Dplatforms=[] \
  -Dglx=disabled -Degl=disabled -Dgles1=disabled -Dgles2=disabled \
  -Dgbm=disabled -Dopengl=false -Dllvm=disabled -Dvideo-codecs=[]
ninja -C /tmp/mesa-build \
  src/freedreno/qrisc/qrisc-asm \
  src/freedreno/qrisc/qrisc-disasm \
  src/freedreno/isa/ir3-disasm
```

Building Mesa may require additional system packages or Python Meson dependencies depending on distribution and version; consult the output of `meson setup` rather than treating a dependency failure as a binary-analysis failure. These are **analysis tools**, not signed-firmware generators.

### Decode QRISC correctly

Extract the *exact* executable microcode bytes from the ELF instead of feeding the entire container to a raw firmware disassembler. The A640-specific helper records the segment offset and a candidate embedded shader range:

```sh
BIN=/tmp/mesa-build/src/freedreno/qrisc
OUT=/private/a640-decoded
mkdir -p "$OUT"
cargo run --offline --locked -p firmware-tools -- \
  extract-a640-isa /private/a640_zap.mbn "$OUT"
```

In raw mode, QRISC firmware identifiers are hexadecimal integers, and supported decoding families can differ from the vendor image ID. For the Nabu A640 case the tested decoder fallback is **A630 0x6ee**, not the original A640 0x6ff:

```sh
export RNN_PATH=/tmp/mesa-source/mesa-26.2.4/src/freedreno/registers
"$BIN/qrisc-disasm" -r -g 6ee -v "$OUT/a640-qrisc-pf_x.bin" \
  > "$OUT/qrisc.txt" 2> "$OUT/qrisc.err"
```

`-r` means raw disassembly in this tested version; `-g` is the QRISC firmware identifier, **not** the IR3 GPU name. Check `qrisc-disasm -h` for flags in other releases. For Nabu's A640 image a single driver script also probes three related A6xx families and writes a decoder-coverage report:

```sh
tools/analyze-a640-mesa.sh \
  /private/a640_zap.mbn \
  /tmp/mesa-build/src/freedreno/qrisc \
  /private/a640-mesa-output
```

That helper works with the previously tested packaged toolkit, or the uninstalled Mesa build directory when `RNN_PATH` points at the Mesa source `src/freedreno/registers` XML directory. It locates `ir3-disasm` in the neighboring `isa` build directory; `IR3_DISASM=/absolute/path` can override the path. Check the output log for missing runtime libraries.

### Decode embedded IR3 shader slots

IR3 instruction slots are eight bytes each and must be separated from QRISC words before decoding. For the detected A640 candidate shader:

```sh
/tmp/mesa-build/src/freedreno/isa/ir3-disasm \
  -g FD640 "$OUT/a640-candidate-ir3.bin" \
  > "$OUT/ir3.txt" 2> "$OUT/ir3.err"
```

Use `FD640`, **not** `640`, in Mesa's IR3 GPU identifier syntax. In the reference A640 shader, one of 64 slots remains unrecognized by the tested decoder; don't turn its raw 64-bit value into a pseudo-ISA without independent hardware evidence. Do not infer an entire shader's purpose from a run of zero-initialization instructions.

### Assemble and test before translating

First verify that the assembler works with Mesa's **public sample firmware**, not OEM-derived instructions:

```sh
SRC=/tmp/mesa-source/mesa-26.2.4
"$BIN/qrisc-disasm" -u "$SRC/src/freedreno/tests/reference/qrisc_test.fw" \
  > /tmp/public-qrisc.asm
"$BIN/qrisc-asm" /tmp/public-qrisc.asm /tmp/public-reassembled.fw
cmp -- "$SRC/src/freedreno/tests/reference/qrisc_test.fw" \
  /tmp/public-reassembled.fw
```

The workspace also has a **synthetic**, original QRISC program that tests its typed Rust assembler against Mesa. Run `tools/verify-qrisc-against-mesa.sh` with the Mesa QRISC assembler path and register database path. The tool versions tested with Nabu may include a four-byte prefix in some custom raw reassembly outputs; account for the raw-file format only after comparing against a public fixture. Never “fix” an unknown difference by silently inserting original executable words into public Rust source.

When reconstructing a program from semantics, use labels for calls, branches and embedded-table locations. Mesa's QRISC README warns that preemption routines reference tables relative to `CP_SQE_INSTR_BASE`; changing layout can require label-based relocations. Disassembly and exact reassembly are validation steps, **not a substitute** for separately authored algorithms.

## 7. Qualcomm Hexagon DSP firmware

For modem, ADSP, CDSP and WLAN DSP images whose executable instructions really are Hexagon, note the specific architecture revision (`v66`, etc.), verify segment-relative addresses, and combine:

- Ghidra's Hexagon language/decompiler where supported; record plugin and processor version.
- LLVM tools (see [`llvm-objdump` command guide](https://llvm.org/docs/CommandGuide/llvm-objdump.html)). LLVM tools may require explicit `--mcpu`/`--mattr` settings and old releases may support ISA versions removed later.
- Rizin's independent control-flow and reference analysis, using the [Rizin handbook](https://book.rizin.re/src/analysis/intro.html).

For an independent binary-header and disassembly analysis with Rizin, first check which architecture its loader chooses; use `rz-bin` to inspect an ELF even when Ghidra can import it:

```sh
rz-bin -I /private/cdsp.mbn
# Interactive review; inside Rizin use: aa, afl, s <address>, pd 20
rizin /private/cdsp.mbn
```

`aa` analyzes functions and basic blocks, `afl` lists discovered functions, `s` seeks to a confirmed code address and `pd 20` prints twenty instructions. If Rizin reports a wrong architecture or no analysis, choose the correct architecture explicitly from its installed plugins; don't assume that forcing an ISA makes the output valid. Preserve function addresses and unresolved instructions for cross-checking against Ghidra and LLVM. The [Rizin command reference](https://book.rizin.re/src/analysis/intro.html) details the options.

Example for a confirmed ELF containing Hexagon V66 code, with a tool build that supports that target:

```sh
llvm-objdump --disassemble --triple=hexagon --mcpu=hexagonv66 \
  /private/cdsp.mbn > /private/cdsp-llvm.txt
```

`llvm-objdump` generally disassembles executable **sections**; stripped `.mbn` ELF files may have only program headers. In that case, extract file-backed `PT_LOAD` segments with their correct addresses and use a processor-aware raw loader; don't interpret an empty `llvm-objdump` result as absence of executable code. For Hexagon, bundle syntax, duplex instructions, constant extenders, hardware loops, relocations and ABI conventions matter. LLVM's generated assembly does not automatically recover C or Rust functions.

Avoid treating older Hexagon `e_flags` on GPU-related ELF wrappers as proof the GPU runs Hexagon code. The Nabu A640 false positive is a concrete example of why a second decoder and semantic sanity checks matter.

## 8. ARM, Cirrus WMFW, Novatek, and unknown formats

**ARM/Thumb/AArch64:** use `readelf -h -A`, compatible `llvm-objdump` or Ghidra ARM language, and manually confirm ARM-vs-Thumb entry bits and instruction boundaries. Resolve linker relocation, interworking veneers, vector tables, exception handlers, and device MMIO registers before reconstructing routines. For kernel-loaded DSPs, a vendor-defined container may carry ARM code at offsets not represented as ELF sections.

### ARM firmware with program headers but no executable sections

A Qualcomm `.mbn` may be a valid ARM ELF with file-backed `PT_LOAD` program
headers but **no section table**. In that case, plain `llvm-objdump -d`
can print no instructions even when loadable machine code exists. Confirm the
ELF header, program headers, segment file offset and load address with
`readelf -h -l`. Extract **only** a verified code interval; the example
below is the known Venus runtime boundary, not a generic layout assumption.

```sh
ELF=/private/venus.mbn
mkdir -p /private/venus-analysis
python3 - "$ELF" /private/venus-analysis/runtime.bin <<'PYTHON'
import pathlib
import sys

# This is the first unimplemented region in the checked example.
# For other images, derive offset and virtual address from PT_LOAD.
file_offset = 0x4000 + 0x6afc
length = 2048
source = pathlib.Path(sys.argv[1]).read_bytes()
if file_offset + length > len(source):
    raise SystemExit("code range exceeds firmware")
pathlib.Path(sys.argv[2]).write_bytes(source[file_offset:file_offset + length])
PYTHON
cat > /private/venus-analysis/raw.S <<'ASM'
.syntax unified
.arm
.text
.global entry
entry:
.incbin "/private/venus-analysis/runtime.bin"
ASM
clang --target=armv7a-none-eabi -c /private/venus-analysis/raw.S \
  -o /private/venus-analysis/raw.o
llvm-objdump -d --adjust-vma=0x6afc /private/venus-analysis/raw.o \
  > /private/venus-analysis/disassembly.txt
```

The `.incbin` wrapper is a **private disassembly aid**, not firmware source:
it does not perform reconstruction or give redistribution permission.
`--adjust-vma` assigns the known runtime address for branch-target reading.
Use `p_vaddr + (file_offset - p_offset)` for an ELF segment with a nonzero
load address. This raw ARM view cannot identify Thumb transitions,
relocations, literal-pool data, or function boundaries on its own; check
all those independently, and do not commit its OEM-derived listings.

Reconstruct a few understood instructions using typed encoders, then run:

```sh
cargo run --offline --locked -p firmware-tools -- \
  verify-venus-contiguous-boot /private/venus.mbn
```

The verifier compares the entire **27,388-byte** generated prefix at file
offset `0x4000`, not merely the last routine. Unit-test new encoder forms
against public ARM examples (including invalid operand tests), and treat
matching bytes as an encoding result rather than proof of correct hardware
behavior.

**Cirrus WMFW:** identify `WMFW` header, version, HALO core and region descriptors first; the Linux driver definitions in [`include/linux/firmware/cirrus/wmfw.h`](https://github.com/torvalds/linux/blob/master/include/linux/firmware/cirrus/wmfw.h) help describe region types. Inspect with `firmware-tools inspect-image`, verify with `firmware-tools verify-wmfw-structure`, and reconstruct non-executable headers with `firmware-tools rebuild-wmfw-header`. Those steps do not decode HALO DSP opcodes. A HALO-compatible ISA decoder and algorithm mapping are required before writing executable replacements.

**Novatek touch controller:** checksum and partition-marker recognition only establish probable container boundaries. Locate the controller's actual CPU core, endianness, ROM/vector semantics, DMA and update requirements from reliable vendor documentation or a verified independent decoder. Do not claim a recovered program from a generic decompiler guessing an MCU ISA.

**Unknown formats:** begin with `file`, `xxd`, entropy/compression tests, strings, embedded offsets, checksums and relevant kernel/bootloader source. Require evidence for where instruction regions start and how they are loaded. Build a small deterministic parser with tests before generating pseudocode.

## 9. From decoded instructions to independently authored Rust

Disassembly is a map of what *may* be happening, not a source file. A defensible implementation process for each routine:

1. Record reference **firmware SHA-256, file offsets, CPU addresses, decoder/version, input segments and recognized instruction coverage**. Unknown or raw words must be counted, not hidden.
2. Derive its purpose from instruction effects and public register/interface descriptions. Trace input/output state, loops, MMIO, memory regions, interrupt behavior and call conventions. Document uncertain invariants.
3. Write typed Rust operations or algorithmic functions instead of copying opcode arrays or decompiler-generated pseudocode. Existing examples are the generic `qrisc` and `ir3` crates and Nabu's A640 device module.
4. Create **synthetic** unit tests independent of the OEM reference. For an assembler, test field ranges, signed branches, register numbering, label resolution, bad inputs, and round-trip properties.
5. Compare independently generated fragments with the reference *only in a separate verification step*. Report disjoint byte ranges and explain every mismatch.
6. Implement deterministic segment placement and fixed data structures. Linker layout, padding, debug/proprietary metadata, compiler optimizations and ISA scheduling all affect byte equality.
7. Proceed to full-image comparison only when every input-backed segment is accounted for. A complete byte-identical result needs exact hashes, headers, data, executable sections, sidecars and any authentication material.

For multiple architectures, use one Rust workspace with separate reusable ISA crates, a target-specific module for each device, and an explicit image builder/validator. Native Rust/LLVM code generation for a target may be possible but is **not** sufficient for exact OEM instruction scheduling. For unsupported targets, typed encoders and a carefully verified linker/assembler may be required.

## 10. Prove coverage and preserve honest results

For the entire file:

```sh
cmp -- /private/original.bin /tmp/independent-rebuild.bin
sha256sum /private/original.bin /tmp/independent-rebuild.bin
wc -c /private/original.bin /tmp/independent-rebuild.bin
```

For incomplete work, list every byte interval separately: **metadata**, **independently authored executable operations**, **OEM-derived disassembly reassembly**, **unimplemented data**, and **signed authentication**. Never add overlapping intervals or count reassembled OEM instruction bytes as independently authored Rust output. Use different tests for code generation, container validation, and behavior on a device.

If exact bytes still differ, prioritize determining *why*: opcode semantics, encoding aliases (`ret`/`iret`), instruction width, unexpected embedded data, branch displacement, alignment, original data, instruction scheduling, absolute offsets, or signatures. Fix the generator's semantics, not the hash checker. Do not use opaque verbatim instruction arrays to hide unfinished decoding.

## 11. Security, boot testing and licensing

Qualcomm's [MDT loader](https://github.com/torvalds/linux/blob/master/drivers/soc/qcom/mdt_loader.c) and secure-processor services illustrate why a Linux firmware load may involve separate authentication. Recompilation does not grant a signing key or disable remote-processor signature checks. An image that changes authenticated bytes may be rejected even if Linux itself boots.

An existing byte-identical image includes its existing signature bytes, but copying a vendor signature is not evidence that the source is GPL-licensed. Building an unsigned replacement is a different question from verifying a byte-identical OEM image. Do not flash intermediate snippets. Test only with a suitable recovery procedure and document actual hardware behavior before claiming that a complete image works.

Finally, distinguish *machine-code decoding*, *source authorship*, and *redistribution permission*. Changing the language of an OEM-derived program is not automatically a clean-room rewrite. Perform a separate legal review before publishing rebuilt proprietary algorithms as GPLv3.

## 12. Recovering additional IR3 categories and enforcing exact image gaps

For newer Adreno images, Mesa's MIT-licensed ISA definitions are particularly useful when a standard disassembler reports an unknown opcode. Inspect `src/freedreno/isa/ir3-common.xml` together with the appropriate `ir3-catN.xml`; each XML bitset documents field widths, fixed bit patterns, generation restrictions, register classes and operand forms. **Do not** infer an operation from its three-bit category alone. A partially matching field layout can describe an invalid instruction.

The device-independent `ir3` crate implements selected category-0 control, category-1 moves, category-2 integer scalar operations and category-6 local stores. A new encoder should validate every field width, reject unsupported register types, use typed operands and have synthetic tests that do not incorporate vendor-derived opcode arrays. Compare a newly generated instruction with the exact source revision only after its semantics are understood, keeping the original reference outside Git.

For example, all 64 Nabu A640 IR3 slots are now generated from typed encoders and match the original bytes. The final category-7-like synchronization slot uses a legacy compatibility encoding which the tested Mesa decoder rejects. Its **bytes** match, but its **semantics have not been validated**; do not infer verified behavior or clean-room provenance from that match.

The reusable `ImagePlan` in `firmware-tools` places generated fragments at checked offsets, rejects overlapping regions, reports every missing range and will emit a contiguous image **only when coverage is complete**. This is safer than concatenating snippets or zero-padding unknown code. To inspect the current target:

```sh
cargo run --offline --locked -p firmware-tools -- a640-coverage
```

Firmware containers often have additional non-executable load segments, signatures, certificate chains, hash tables and padding. Even a complete and byte-identical executable stream is not equivalent to a complete `.mbn` file or authenticated, usable firmware. Maintain separate verification for each level.

## Sources and primary reference material

- [Ghidra Headless Analyzer documentation](https://github.com/NationalSecurityAgency/ghidra/blob/master/Ghidra/RuntimeScripts/support/analyzeHeadlessREADME.md)
- [Mesa Freedreno GPU documentation](https://docs.mesa3d.org/drivers/freedreno.html)
- [Mesa ISASPEC encoding guide](https://docs.mesa3d.org/drivers/freedreno/isaspec.html) and the source-tree `src/freedreno/isa/ir3-cat0.xml` / `ir3-cat1.xml`
- Mesa source-tree `src/freedreno/qrisc/README.rst`, `qrisc.xml`, and public test fixtures
- [LLVM object disassembly documentation](https://llvm.org/docs/CommandGuide/llvm-objdump.html)
- [Rizin analysis handbook](https://book.rizin.re/src/analysis/intro.html)
- [Ghidra Hexagon language implementation](https://github.com/CUB3D/ghidra-hexagon-sleigh) (verify ISA/version compatibility)
- [Linux Qualcomm MDT firmware loader](https://github.com/torvalds/linux/blob/master/drivers/soc/qcom/mdt_loader.c)
- [Linux Cirrus WMFW definitions](https://github.com/torvalds/linux/blob/master/include/linux/firmware/cirrus/wmfw.h)
