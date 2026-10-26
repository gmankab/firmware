// SPDX-License-Identifier: GPL-3.0-only
// Independent LLVM6 Hexagon scalar/branch semantics. The original OEM
// byte sequence and scheduling are validated separately using Rizin.
.text
.space 0x784
.globl controller_mode_guard
controller_mode_guard:
{ p0=cmp.eq(r23,#0)
  if(p0.new) jump:nt upper_exit }
{ r4=#33
  if(cmp.gtu(r4.new,r3)) jump:nt upper_exit }
{ r24=lsr(r22,#12) }
{ r19=and(r19,##0xfff00) }
{ r17=#32 }
{ r25=#8 }
{ r26=#1 }
{ r3=cl0(r23) }
{ r2=add(r17,#-31)
  if(!cmp.gtu(r25,r2.new)) jump:nt capacity_exit }
.space 0x5c
upper_exit:
.space 0x9c
capacity_exit:
{ nop }
