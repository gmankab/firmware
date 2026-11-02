// SPDX-License-Identifier: GPL-3.0-only
// Independent semantic assembly of CDSP V66 wrapper at file 0x562c8.
.text
.global cdsp_callback_finish
.space 0x2c8
cdsp_callback_finish:
{ r0=#-2
  memd(r29+#-16)=r17:16
  allocframe(#8)
  call #-0x98 }
{ r16=memw(gp+##0xc028c538)
  call #-0x98 }
{ r0=##0xfffe
  r17=add(r0,#1) }
{ call #0x23758 }
{ r2=add(r0,#1)
  r1:0=combine(r17,r16) }
{ call #-0xc4 }
{ r0=#-2
  call #-0xcc }
{ r0=##0xfffe
  r16=add(r0,#1) }
{ r1=add(r0,#1)
  r0=r16
  r17:16=memd(r29+#0)
  deallocframe }
{ jump #0x44a70 }
{ nop }
{ nop }
