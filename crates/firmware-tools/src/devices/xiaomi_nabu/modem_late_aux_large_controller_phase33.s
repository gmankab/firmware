// SPDX-License-Identifier: GPL-3.0-only
// Select normal/alternate controller size report and forward descriptor
// to external callback, preserving V60 new-value conditional stores.
.text
.globl modem_late_aux_large_controller_phase33
modem_late_aux_large_controller_phase33:
.set alternate_report, modem_late_aux_large_controller_phase33 + 0x2c
.set after_report, modem_late_aux_large_controller_phase33 + 0x50
.set report_callback, modem_late_aux_large_controller_phase33 + 0x62c
.set platform_report, modem_late_aux_large_controller_phase33 + 0x3914
{r2 = memw(r29+#84)}
{
  p0 = r2
  if (!p0.new) jump:t alternate_report
  if (p0.new) memd(r29+#16) = r25:24
  if (p0.new) memw(r29+#0) = r22
}
{
  r2 = #480
  r3 = ##507577730
}
{
  memw(r29+#8) = r6
  memw(r29+#4) = r23
}
{call report_callback}
{jump after_report}
alternate_report:
{
  r1:0 = combine(##507576841,#2)
  r2 = #480
}
{
  r3 = ##507577730
  memd(r29+#16) = r25:24
  memw(r29+#8) = r6
}
{
  call platform_report
  memw(r29+#4) = r23
  memw(r29+#0) = r22
}
