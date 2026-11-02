// SPDX-License-Identifier: GPL-3.0-only
// ADSP V66 controller global init function, beginning at file offset 0x11450.
// The source is partial: no invented terminal return or copied OEM bytes.
.text
.globl adsp_controller_global_startup
adsp_controller_global_startup:
{ memd(r29+#-16)=r17:16; allocframe(#16) }
{ call read_controller_config; memd(sp+#0)=r19:18 }
{ r2=##0xfbbb }
{ r0=#-1 }
{ nop } // checked V66 CSwi
{ nop } // checked V66 ciad
{ r1=r2 }
{ nop } // checked V66 IMASK
.set read_controller_config, adsp_controller_global_startup+(0x8be115b0-0x8be0e450)
// Global MMIO pointers: first group is a four-word packet, second another.
{ r6=##0xf0021ae8; immext(#0xf01f10c0); r2=memw(gp+##0xf01f10c0) }
{ r4=##0xf01285c4; immext(#0xf02ca4c0); r3=memw(gp+##0xf02ca4fc) }
{ p0=!cmp.gtu(r3,r2); r3=add(r6,#480); immext(#0xf020a100); r1=memw(gp+##0xf020a108) }
