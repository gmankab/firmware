// SPDX-License-Identifier: GPL-3.0-only
// Original high-level Hexagon operations independently assembled with LLVM6.
// LLVM6 schedules calls before duplex pointer-copy instructions; that is an
// LLVM packet placement difference, not a change to the pointer/handle logic.
.text
.space 0x670
retry:
{ nop }
.space 0x24
.global cdsp_late_controller_transfer
cdsp_late_controller_transfer:
{ r1=r16
  r2=memw(r19+#0)
  call #0x8988 }
{ r0=memw(r2+#16) }
{ r1=r17
  r2=memw(r19+#0)
  call #0x7ddc }
{ r3:2=combine(#0,#-1)
  r0=memw(r2+#16) }
{ r16=r0
  if (!cmp.eq(r16.new,#0)) jump:t retry }
{ jump error }
.space 0x1fc
error:
{ nop }
