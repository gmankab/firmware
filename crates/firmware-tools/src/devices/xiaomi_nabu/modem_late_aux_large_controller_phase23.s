// SPDX-License-Identifier: GPL-3.0-only
// Controller report and descriptor-state initialization after callback.
.text
.globl modem_late_aux_large_controller_phase23
modem_late_aux_large_controller_phase23:
.set skip_post_callback_log, modem_late_aux_large_controller_phase23 + 0x14
.set controller_report, modem_late_aux_large_controller_phase23 + 0x3c74
{jump skip_post_callback_log}
{
  r1:0 = combine(##507576841,#2)
  r2 = #412
}
{call controller_report}
{
  memw(r29+#76) = r24
  memw(r29+#48) = r27
}
{memw(r29+#80) = r26}
{r2 = memw(r17+#28)}
{
  r3 = memw(r2+#4)
  memw(r2+#8) = r3.new
}
{
  r3 = memw(r17+#28)
  r5 = memw(r29+#84)
}
{p0 = r5}
