// SPDX-License-Identifier: GPL-3.0-only
// V66 source for physical memory controller setup, ADSP [0x676c,0x67a8).
.text
.globl adsp_rwe_physical_controller
adsp_rwe_physical_controller:
{ nop; r1=#0 } // V66 r0=CFGBASE; packet continuation
{ r0=insert(r1,#12,#20) }
{ r0=asl(r0,#5); r2=#8 }
{ nop } // V66 r0=memw_phys(r2,r0)
{ r0=insert(r1,#12,#20) }
{ r0=asl(r0,#5); r2=#68 }
{ r0=add(r0,#-256) }
{ nop } // V66 r0=memw_phys(r2,r0)
{ immext(#0xf0003c80); memw(r1=##0xf0003c9c)=r0 }
{ dccleana(r1) }
{ jumpr lr }
