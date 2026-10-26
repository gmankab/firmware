// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 source for session resource-list scan, file [0x4478a4,0x44795c).
// Original function base 0x447724; symbolic calls and jumps retain V60 offsets.
.text
.globl modem_late_aux_large_controller_phase3
modem_late_aux_large_controller_phase3:
.set session_base, modem_late_aux_large_controller_phase3 - 0x180
.set loop_empty, session_base + 0x300
.set loop_no_resource, session_base + 0x230
.set resource_available, session_base + 0x23c
.set report_invalid, session_base + 0x218
.set diagnostic_console, session_base - 0xf84
.set recovery_exit, session_base + 0x37c
{
  r23 = #-1
  r3 = memuh(r21+#44)
}
{
  if (p0.new) r22 = #0
  p0 = cmp.eq(r3,#0)
  if (p0.new) jump:nt loop_empty
  if (!p0.new) r2 = memw(r21+#28)
}
{ loop0(.Lscan_iter,r3) }
{ r2 += add(r21,#24) }
.Lscan_iter:
{ r3 = memw(r2+#0) }
{ r3 = and(r3,##117440512) }
{
  p0 = cmp.eq(r3,##33554432)
  if (!p0.new) jump:t loop_no_resource
}
{
  r22 = memw(r2+#-4)
  memw(r29+#76) = r24
}
{
  r3 = memw(r24+#0)
  r25 = memw(r2+#-20)
}
{
  r3 += add(r22,#-1)
  r4 = sub(#0,r3)
  r18 = memw(r2+#-8)
  r6 = memw(r16+#48)
}
{
  r24 = and(r3,r4)
  r5:4 = combine(r25,r20)
  r3:2 = combine(#1,#1)
  memw(r29+#0) = r20
}
{
  r1:0 = combine(r24,#0)
  callr r6
}
{
  r23 = r0
  if (!cmp.eq(r23.new,#-1)) jump:t resource_available
}
{
  r4 = memw(r19+#0)
  if (cmp.eq(r4.new,#0)) jump:nt report_invalid
}
{
  r1:0 = combine(##507576841,#3)
  r2 = #286
}
{
  r3 = ##507576995
  memw(r29+#4) = r25
  memw(r29+#0) = r22
}
{ callr r4 }
report_invalid:
{
  r0 = ##507576995
  memw(r29+#4) = r25
  memw(r29+#0) = r22
}
{ call diagnostic_console }
{ jump recovery_exit }
loop_no_resource:
{ r2 = add(r2,#32)
  nop }:endloop0
