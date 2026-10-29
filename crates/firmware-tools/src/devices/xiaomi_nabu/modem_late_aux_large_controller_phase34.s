// SPDX-License-Identifier: GPL-3.0-only
// Dispatch selected controller result callbacks, update frame descriptors,
// and conditionally return to the main resource scanner. Hexagon V60.
.text
.globl modem_late_aux_large_controller_phase34
modem_late_aux_large_controller_phase34:
.set valid_callback_result, modem_late_aux_large_controller_phase34 + 0x4c
.set optional_callback_empty, modem_late_aux_large_controller_phase34 + 0x38
.set platform_notifier, modem_late_aux_large_controller_phase34 - 0x1d14
.set previous_resource_scanner, modem_late_aux_large_controller_phase34 - 0xa14
{
  r1:0 = combine(r23,r20)
  r2 = #0
  r3 = memw(r16+#36)
}
{callr r3}
{
  p0 = cmp.eq(r23,r0)
  if (p0.new) jump:t valid_callback_result
  if (!p0.new) r18 = memw(r29+#80)
}
{
  r4 = memw(r19+#0)
  if (cmp.eq(r4.new,#0)) jump:nt optional_callback_empty
}
{
  r1:0 = combine(##507576841,#3)
  r2 = #482
}
{
  r3 = ##507577589
  memw(r29+#4) = r18
  memw(r29+#0) = r23
}
{callr r4}
optional_callback_empty:
{
  r0 = ##507577589
  memw(r29+#4) = r18
  memw(r29+#0) = r23
}
{call platform_notifier}
{jump previous_resource_scanner}
valid_callback_result:
{
  r23 = sub(#0,r27)
  r3 = memw(r16+#32)
  r2 = memw(r29+#44)
}
{
  r23 += add(r2,r26)
  r2 = r24
}
{
  r1:0 = combine(r23,r20)
  callr r3
}
{
  r26 = r0
  r1:0 = memd(r29+#64)
}
