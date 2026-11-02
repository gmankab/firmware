// SPDX-License-Identifier: GPL-3.0-only
// Xiaomi Nabu ADSP QDSP6 reset/exception vector fragment.
//
// This is original source assembly, NOT an incbin of the OEM image.
// In the ELF image the first executable load has file offset 0x3000,
// vaddr 0xf0000000 and paddr/entry 0x8be00000. This fragment is only
// the first 0x24 file bytes. The rest of the bootstrap is unimplemented.
//
// Bootstrap vector 0 branches over six self-loop trap vectors to a
// startup branch. The startup code then initializes r0 to zero.
// The distant branch destination is a file-local ELF segment offset
// (0x1158 relative to this segment), NOT another generated routine.

.text
.p2align 2
.globl adsp_boot_vectors
adsp_boot_vectors:
{
  jump bootstrap_start
}
trap_vector_1:
{ jump trap_vector_1 }
trap_vector_2:
{ jump trap_vector_2 }
trap_vector_3:
{ jump trap_vector_3 }
trap_vector_4:
{ jump trap_vector_4 }
trap_vector_5:
{ jump trap_vector_5 }
trap_vector_6:
{ jump trap_vector_6 }
bootstrap_start:
{ jump bootstrap_stage2 }
{ r0 = #0 }

// Hexagon resolves this assembler symbol as an offset in the entry
// code section, without a linker-generated relocation or OEM bytes.
.set bootstrap_stage2, bootstrap_start + 0x113c
