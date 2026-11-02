// SPDX-License-Identifier: GPL-3.0-only
// Tiny return stub plus checked runtime revision guard, [0x65f4,0x661c).
.text
.globl adsp_rwe_revision_guard
adsp_rwe_revision_guard:
{ jumpr lr }  // one-word standalone return stub
{ nop }       // checked V66 system register S27 / controller revision field
{ r0=and(r0,#255) }
{ p0=cmp.gt(r0,#85); if(!p0.new) jumpr:nt lr }
{ r0=#1; immext(#0xf0003c80); r2=memw(r1=##0xf0003ca0) }
{ p0=cmp.eq(r2,#0); if(!p0.new) jumpr:nt lr }
{ memw(r1+#0)=r0 }
