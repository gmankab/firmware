# HALO PM transport, known ISA evidence and unresolved questions

Agent 20-15, Xiaomi Nabu CS35L41, 2026-10-25. Scope is **packed PM only**.

## Reference and ownership

* Pinned `cs35l41-dsp1-spk-prot.wmfw`: 34,236 bytes, SHA-256
  `d1bea27e41001024eb3878276292b5bcc0cda4484ffced0880e12c03ec9bac2c`.
* Program section `WMFW_HALO_PM_PACKED` (type 0x10): record header
  `[0x2784,0x278c)`, payload `[0x278c,0x76c4)` (**20,280 bytes**), DSP offset
  zero. PM payload SHA-256
  `0edacef35965ea2bd8c709f8cad73fd08714bb47922065d25e3b9ba1bf34c517`.
* Program storage contains **4,056 40-bit transport words**, or **1,014**
  blocks of four DSP words. Each 20-byte block produces exactly **five**
  big-endian 32-bit AHB register writes; the whole PM section produces **5,070**.
  Host byte addresses start at `0x03800000`, last write starts `0x03804f34`,
  and the write span ends at `0x03804f38` (exclusive).
* Agent 16 owns packed XM/YM coefficient data; this branch must not change
  that region, the general WMFW metadata builders, or the central firmware
  CLI. No private binary or extracted DSP program is stored in Git.

## Public transport evidence

Linux's maintained `cs_dsp.c` function `cs_dsp_halo_region_to_reg` maps HALO
packed PM with the transport formula `mem->base + offset * 5`. Linux's
`include/sound/cs35l41.h` maps the corresponding packed program register
window to base `0x03800000` and last register at `0x03804fe8`.

* https://kernel.googlesource.com/pub/scm/linux/kernel/git/torvalds/linux.git/+/142fdd7bb7095c114d027b1ee36878a67b869228/drivers/firmware/cirrus/cs_dsp.c
* https://code.googlesource.com/linux/torvalds/linux/+/24a0b5e196cf70ccff97bc0add6fa7178ad50cc4/include/sound/cs35l41.h

The byte-level representation is **not** an instruction encoding: a five-byte
storage unit can cross two aligned 32-bit AHB registers. For aligned starts
at DSP words divisible by four, the correct complete transfer unit is
20 bytes = five host registers. A 40-bit storage word can be separated into
**two mathematical 20-bit bit fields**, but without an instruction-length
oracle they must not be labelled two HALO operations. The existing
`wmfw_halo_memory` API's nominal `base+offset*5` is a logical packed-byte
address; the new `window_for_word` API computes the *actual aligned host
registers* containing the five bytes.

Datasheets for the configurable Cirrus HALO core describe separate program
banks of 40-bit storage words and reference a separately provided **software
programming guide** for custom algorithms. HALO instances vary by product;
a 2013 Cirrus "32-bit Audio DSP Assembly Programmer's Guide" applies to a
prior DSP family and is **not evidence** for CS35L41 HALO opcodes.

* https://statics.cirrus.com/pubs/proDatasheet/CS48L32_DS1219F4.pdf
* https://www.alldatasheet.es/html-pdf/1446093/CIRRUS/CS48L32/18827/73/CS48L32.html

We found public PM loaders, Linux register descriptions, and CSPL runtime
mailbox interfaces; they support AHB/container mapping, **not** a correct
opcode dictionary, execution unit architecture, branch displacement
encoding, 20-bit-vs-40-bit instruction-length discriminator, or valid code
entrypoints for this exact CS35L41 revision.

## Verified, strictly limited analysis

* Rust module `wmfw_halo_pm_transport.rs` handles full blocks, 40-bit word
  bounds, five host register writes, byte-lane mapping and bounded edits.
* An independent Python implementation uses `struct.unpack(">5I", block)`
  and `int.from_bytes(..., "big")` to check every transport block.
* All 160 possible individual bit positions in a 20-byte block round-trip
  without loss, tested by Rust and Python. Unaligned host transfers are
  rejected rather than silently supplying missing bytes.
* Pinned reference inventory: 3,701 distinct storage word values, 29 zero
  storage words, 4,056 total. Word repetitions or 20-bit halves alone do
  not prove data-vs-code, branch, opcode, function boundary or behavior.
* The full reference PM can be parsed and emitted to 32-bit AHB register
  values and immediately reassembled **with zero differences**. This is a
  transport property; the input bytes are externally supplied OEM content.
* **0 / 20,280 independent executable bytes**, and no traceable `HALO`
  instruction has been encoded. Existing WMFW 3,984 independently authored
  container metadata bytes belong to earlier Agent 08 work, not to this task.

## Blockers to actual instruction reconstruction

Needed: an applicable Cirrus CS35L41 HALO instruction-set programming guide
or legitimate HALO assembler/disassembler plus source fixtures; an execution
trace or trusted annotated assembly/object file that independently identifies
instruction lengths, program-counter units, branch targets and operand
fields; and proof the evidence applies to this firmware revision. A live
runtime registry helps locate data controls but **does not** decode program
opcodes. No such ISA oracle or an original HALO firmware source tree is
included with the supplied project archives.

Absent those inputs, continuing to transcribe arbitrary 40-bit blobs or
padding unknown memory with zeros would provide false "recompiled"
coverage. The correct checkpoint is a reproducible transport implementation
and an evidence-backed blocker report, **not** a flashable DSP binary.

## Candidate ISA reconstruction acceptance criteria (Checkpoint 02)

A future legitimate independent HALO assembler may generate 20-bit and/or
40-bit encoded instruction candidates. The new API records these as
*untrusted hypotheses* indexed by 40-bit storage word and 20-bit half;
it verifies bit-identity to an externally pinned PM reference only after
an evidence ID has been provided for human verification. Passing this
numerical equality check does not prove that a half-word encodes a real
HALO instruction or that its control-flow semantics match.

The companion `assemble_candidate_storage()` API fails on any uncovered
20-bit half-slot. In particular, **zero-padded undecoded storage is
never accepted as a complete recompiled DSP program**. Synthetic tests
exercise mixed 20/40-bit candidates and a full 4,056-word independent
fixture, while the genuine reference audit reports zero source candidates
and the first missing candidate at PM storage word zero, high half.
