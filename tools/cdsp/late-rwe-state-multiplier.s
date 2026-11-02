// SPDX-License-Identifier: GPL-3.0-only
// Independent CDSP V66 semantic source for GP-state multiplier.
.text
.space 0x540
.global cdsp_state_multiplier
cdsp_state_multiplier:
{ r3=memw(gp+##0xc028c02c) }
loop:
{ r3=#1
  r4=r3 }
{ r3+=mpyi(r4,##0x15a4e35) }
{ p0=cmp.eq(r4,#0)
  if(p0.new) jump:t loop }
{ jumpr lr }
{ memw(gp+##0xc028c02c)=r3 }
{ memw(r0+#0)=r4 }
