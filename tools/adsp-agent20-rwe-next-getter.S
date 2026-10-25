// SPDX-License-Identifier: GPL-3.0-only
// First independent callable helper after the ADSP RWE sparse linker layout.
// ADSP MBN [0x10330,0x1034c), physical [0x8be0d330,0x8be0d34c).
// Read two 64-bit, GP-relative codec controller records and return.
.text
.globl adsp_rwe_next_getter
adsp_rwe_next_getter:
{ immext(#0xf000d680); r3:2=memd(##0xf000d688) }
{ immext(#0xf000d680); r5:4=memd(##0xf000d690) }
{ jumpr lr; immext(#0xf000d680); r0=memw(##0xf000d6b0) }
