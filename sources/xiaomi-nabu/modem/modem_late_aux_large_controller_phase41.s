// SPDX-License-Identifier: GPL-3.0-only
// Second bounded controller record advance with diagnostic and platform
// callback fallback. Source statements and branch targets are symbolic.
.text
.globl modem_late_aux_large_controller_phase41
modem_late_aux_large_controller_phase41:
.set fallback_report, modem_late_aux_large_controller_phase41 + 0x3c
.set report_done, modem_late_aux_large_controller_phase41 + 0x50
.set platform_log, modem_late_aux_large_controller_phase41 + 0x363c
.set lookup_record, modem_late_aux_large_controller_phase41 + 0x7504
.set device_report, modem_late_aux_large_controller_phase41 + 0x354
{r23 = memw(r24+#0)}
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
  if (!p0.new) r3 = ##507577982
  if (!p0.new) jump:t fallback_report
}
{
  r2 = #531
  r3 = ##507577982
  memw(r29+#0) = r0
}
{call device_report}
{jump report_done}
fallback_report:
{
  r1:0 = combine(##507576841,#2)
  r2 = #531
  memw(r29+#0) = r0
}
{call platform_log}
