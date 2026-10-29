// SPDX-License-Identifier: GPL-3.0-only
// Advance controller frames and emit the first bounded diagnostic, preserving
// conditional callback dispatch and the original new-value memory stores.
.text
.globl modem_late_aux_large_controller_phase40
modem_late_aux_large_controller_phase40:
.set after_resource_pass, modem_late_aux_large_controller_phase40 + 0x26c
.set fallback_report, modem_late_aux_large_controller_phase40 + 0x5c
.set report_done, modem_late_aux_large_controller_phase40 + 0x70
.set platform_log, modem_late_aux_large_controller_phase40 + 0x36ac
.set lookup_record, modem_late_aux_large_controller_phase40 + 0x7574
.set device_report, modem_late_aux_large_controller_phase40 + 0x3c4
{
  jump after_resource_pass
  memw(r17+#164) = r0
}
{
  r1:0 = combine(##507576841,#2)
  r2 = #523
  memw(r29+#0) = r0
}
{call platform_log}
{
  r23 = memw(r24+#0)
  memw(r17+#200) = r23.new
}
{
  call lookup_record
  r0 = r23
}
{
  r0 += add(r23,#1)
  r2 = memw(r29+#84)
  memw(r24+#0) = r0.new
}
{
  p0 = r2
  if (!p0.new) r3 = ##507577979
  if (!p0.new) jump:t fallback_report
}
{
  r2 = #528
  r3 = ##507577979
  memw(r29+#0) = r0
}
{call device_report}
{jump report_done}
fallback_report:
{
  r1:0 = combine(##507576841,#2)
  r2 = #528
  memw(r29+#0) = r0
}
{call platform_log}
