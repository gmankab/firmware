// SPDX-License-Identifier: GPL-3.0-only
// Scan a bounded physical-word region and publish four cache-synced values,
// ADSP MBN [0x67e8,0x6818), physical [0x8be037e8,0x8be03818).
.text
.globl adsp_rwe_physical_scan
adsp_rwe_physical_scan:
{ immext(#0xf0003000); r0=memw(gp+##0xf0003004) }
{ r2=lsr(r0,#11); immext(#0xf0003ec0); r1=##0xf0003ee0 }
{ loop0(scan_loop,#4); immext(#0x7c0); r0=and(r0,##0x7ff) }
scan_loop:
{ nop } // checked V66 r3=memw_phys(r0,r2)
{ r0=add(r0,#4); memw(r1++#4)=r3 }:endloop0
{ jumpr lr }
