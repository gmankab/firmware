# Agent 19: consumer evidence for structured Novatek DLM data

## Scope and method

The pinned reference is the 196,608-byte postmarketOS image, SHA-256
`8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5`.
The ILM file range `[0x200,0x1ac80)` maps to runtime address zero; thus an ILM
runtime address plus `0x200` gives its firmware-file offset. DLM file offsets
map to runtime addresses by subtracting `0x2000`.

Consumer inspection used GNU binutils 2.30 NDS32 objdump and the EX9 instruction
table at reference file `[0x17dc0,0x185c0)`. The 2,048-byte table was matched
against this pinned image before decoding. Binary slices, ELF decoder wrappers,
and complete disassembly remain private scratch analysis, outside Git. No ILM
instructions are authored or credited to Agent 19 by this inspection.

## Packed 50-entry, 11-bit index records

The routine at runtime `[0xc7ea,0xc87a)` (file `[0xc9ea,0xca7a)`) decodes two
selected records into 50 halfword indices each. The following actions establish
the wire format independently of the data-pattern analysis.

| Runtime address | File offset | Observed operation and implication |
| --- | --- | --- |
| `0xc7ee`, `0xc7f0` | `0xc9ee`, `0xc9f0` | Read the configuration record's word at displacement `0x50`. |
| `0xc7f6`, `0xc7fa` | `0xc9f6`, `0xc9fa` | Mask that base-offset field to 15 bits. |
| `0xc7f2`, `0xc7fc`, `0xc7fe` | `0xc9f2`, `0xc9fc`, `0xc9fe` | Use `0x48` (72) bytes per selected packed record. |
| `0xc800`–`0xc80c` | `0xca00`–`0xca0c` | Multiply the masked base offset by four and add DLM SRAM base `0x20000`. |
| `0xc812`–`0xc824` | `0xca12`–`0xca24` | Read one low byte per entry into halfword output; loop until 50 (`0x32`) entries are copied. |
| `0xc828`, `0xc82c` | `0xca28`, `0xca2c` | Begin the high-bit word array at record displacement `0x34` (52), proving the two-byte gap after the 50 low bytes. |
| `0xc832`, `0xc83a` | `0xca32`, `0xca3a` | Read one 32-bit high-bit word from each of the two selected records. |
| `0xc830`, `0xc844` | `0xca30`, `0xca44` | Multiply the within-word field index by three. |
| `0xc84c`, `0xc850`, `0xc852` | `0xca4c`, `0xca50`, `0xca52` | Shift the word by that amount, extract bits 0–2, and add the result shifted left eight to the low-byte value. |
| `0xc85a`–`0xc864` | `0xca5a`–`0xca64` | Apply the same high-three-bit reconstruction to the second selected record. |
| `0xc86e` | `0xca6e` | Process ten high fields per 32-bit word. |
| `0xc872`, `0xc874` | `0xca72`, `0xca74` | Advance output by 20 bytes, looping until 100 output bytes are written: five words, ten indices each. |

The caller uses the resulting values as data indices. At runtime `0xc9ce`
(file `0xcbce`) it tests whether the decoded value is below `0x7ff`; the branch
at runtime `0xc9d2` skips the entry otherwise. Since the packed format has
eleven value bits, this establishes `0x7ff` as the inactive-entry sentinel.
At runtime `0xc9d8`, an active index addresses halfword data with index scale two.

This supports the semantic description **packed index schedules**, with explicit
inactive slots. The routine does not by itself identify the physical electrode
layout or every mode's algorithmic purpose.

### Descriptor-to-table cross-references

The consumer's base formula is:

`schedule_runtime_base = 0x20000 + 4 * (descriptor_word_at_0x50 & 0x7fff)`.

| Descriptor records in file | Base-field value after mask | Resolved schedule runtime/file start |
| --- | --- | --- |
| `0x22a64 + 0x8c*k`, `k=0..16` | `0x0a41` | `0x22904` / `0x24904` |
| `0x25230 + 0x8c*k`, `k=0..3` | `0x0f53` | `0x23d4c` / `0x25d4c` |
| `0x26a64 + 0x8c*k`, `k=0..16` | `0x1a41` | `0x26904` / `0x28904` |
| `0x29230 + 0x8c*k`, `k=0..3` | `0x1f53` | `0x27d4c` / `0x29d4c` |

These descriptor fields provide independent placement evidence for all four
reconstructed blocks. They are not themselves newly credited source bytes:
the other packed scalar fields and flags remain pinned external metadata.

## Gaussian half-kernel references

At runtime `0x192a` (file `0x1b2a`), initialization loads DLM runtime address
`0x2b7f4`, corresponding to file offset `0x2d7f4`. At runtime `0x192e` it stores
that pointer into the GP-relative slot at displacement `-4704`; initialization
calls this routine from runtime `0x8224` (file `0x8424`).

Consumers load the pointer from that same GP-relative slot, including runtime
`0x1a6c`, `0x1bd0`, `0x1d66`, and `0x1ed8`. Examples of table selection:

| Runtime operation | Selected file offset | Interpretation supported by emitted data |
| --- | --- | --- |
| Add `0x288` at `0x1ab8` | `0x2da7c` | Fourth 162-byte displacement from the base; Gaussian sigma 25.6. |
| Add `0x510` at `0x1a96` | `0x2dd04` | Eighth 162-byte displacement; Gaussian sigma 25.2. |
| Add `0x7a4` at `0x1c2c` | `0x2df98` | Wider Gaussian sigma 40, following the 12-byte scalar interlude. |
| Add `0x846` at `0x1ac0` | `0x2e03a` | Wider Gaussian sigma 80. |

The lookup routine at runtime `0x19ba` receives a selected table pointer; its
indexed signed-halfword reads at `0x1a0c` and `0x1a12` use an index scaled by two.
The following multiply/add and division combine those samples with the input
measurement. These reads corroborate interpreting the values as numeric
halfword lookup weights, not an instruction stream.

The exact mathematical reconstruction is separate from the consumer analysis:
81 samples in reverse distance order, `x=80..0`, serialized as little-endian u16,

`round(400000 / (sigma * sqrt(2*pi)) * exp(-x*x / (2*sigma*sigma)))`.

An independent 80-digit Decimal evaluation agrees with all six distinct upper
DLM kernel parameterizations. The smallest distance from a rounding half-point
among those samples is greater than `0.001583`, for sigma 40. Physical filter
behavior and touchscreen performance remain unvalidated.

## Uncredited candidate

File `[0x2d3b0,0x2d450)` has an exact 160-byte structured pattern: for `i=0..7`,
choose selector `16+i` then `31-i`; for each selector choose phase zero then one;
serialize a five-byte little-endian word
`0x8000064800 | (selector << 24) | (phase << 23)`.

This pattern is mathematically exact, but no consumer or documented command
format has yet established the meaning of the fixed operation bits. Leave it
uncredited until its semantics can be substantiated. Arbitrary zero tails,
calibration matrices, and other opaque control parameters likewise receive no
new independent-source credit.
