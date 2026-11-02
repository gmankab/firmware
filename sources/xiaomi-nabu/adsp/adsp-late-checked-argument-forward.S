// SPDX-License-Identifier: GPL-3.0-only
// Hexagon ADSP file [0x3efa8,0x3eff4): reject invalid queue modes and
// forward nine arguments to the platform's outgoing controller slot handler.
.text
.globl adsp_late_checked_argument_forward
adsp_late_checked_argument_forward:
{ p0 = cmp.eq(r2,#3); if (!p0.new) jump:nt forward }
{ r6 = #3; if (!cmp.gtu(r6.new,r3)) jump:nt forward }
{ r0 = #-1; jumpr r31 }
forward:
{ allocframe(#24) }
{ r6 = memw(r29+#52); memw(r29+#20) = r6.new }
{ r8 = memw(r29+#44); memw(r29+#12) = r8.new }
{ r12 = memw(r29+#36); memw(r29+#4) = r12.new }
{ r7 = memw(r29+#48); memw(r29+#16) = r7.new }
{ r9 = memw(r29+#40); memw(r29+#8) = r9.new }
{ call platform_forward; r13 = memw(r29+#32); memw(r29+#0) = r13.new }
{ dealloc_return }
.set platform_forward, adsp_late_checked_argument_forward+0xeb920
