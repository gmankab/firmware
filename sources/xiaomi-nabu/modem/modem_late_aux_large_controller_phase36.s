// SPDX-License-Identifier: GPL-3.0-only
// Controller 64-bit resource bound, conditional frame reporting, and
// input-descriptor finalization. Hexagon V60 source with symbolic calls.
.text
.globl modem_late_aux_large_controller_phase36
modem_late_aux_large_controller_phase36:
.set valid_resource_size, modem_late_aux_large_controller_phase36 + 0x78
.set alternate_resource_report, modem_late_aux_large_controller_phase36 + 0x4c
.set finalize_resource_report, modem_late_aux_large_controller_phase36 + 0x68
.set trace_callback, modem_late_aux_large_controller_phase36 + 0x510
.set alternate_callback, modem_late_aux_large_controller_phase36 + 0x37f8
.set record_finalizer, modem_late_aux_large_controller_phase36 + 0x7030
{
  p0 = !cmp.gtu(r6,r24)
  if (p0.new) jump:t valid_resource_size
}
{
  r3 = memw(r29+#52)
  r5:4 = memd(r29+#64)
}
{
  r6 += add(r3,#-1)
  r4 = r24
  r1:0 = combine(r5,r6)
  r2 = add(r3,r24)
}
{
  r27:26 = sub(r1:0,r5:4)
  r3 = memw(r29+#84)
  memd(r29+#64) = r5:4
}
{
  p0 = r3
  if (!p0.new) jump:t alternate_resource_report
  if (!p0.new) memd(r29+#8) = r27:26
  if (p0.new) memw(r29+#4) = r6
}
{
  r2 = #488
  r3 = ##507577833
  memw(r29+#0) = r2
}
{
  call trace_callback
  memd(r29+#8) = r27:26
}
{jump finalize_resource_report}
alternate_resource_report:
{
  r1:0 = combine(##507576841,#2)
  r2 = #488
  memw(r29+#0) = r2
}
{
  r3 = ##507577833
  memw(r29+#4) = r6
}
{call alternate_callback}
finalize_resource_report:
{
  call record_finalizer
  r0 = add(r23,r24)
  r1 = #0
  r2 = r26
}
