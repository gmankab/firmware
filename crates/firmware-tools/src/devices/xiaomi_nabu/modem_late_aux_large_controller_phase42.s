// SPDX-License-Identifier: GPL-3.0-only
// Third controller record advance. Emits the resource counter and conditional
// diagnostic callback for stream code 536, retaining symbolic external calls.
.text
.globl modem_late_aux_large_controller_phase42
modem_late_aux_large_controller_phase42:
.set fallback_report, modem_late_aux_large_controller_phase42 + 0x40
.set report_done, modem_late_aux_large_controller_phase42 + 0x54
.set platform_log, modem_late_aux_large_controller_phase42 + 0x35ec
.set lookup_record, modem_late_aux_large_controller_phase42 + 0x74b4
.set device_report, modem_late_aux_large_controller_phase42 + 0x304
{
  r23 = memw(r24+#0)
  memw(r17+#204) = r23.new
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
  r2 = #536
  r3 = ##507577979
  memw(r29+#0) = r0
}
{call device_report}
{jump report_done}
fallback_report:
{
  r1:0 = combine(##507576841,#2)
  r2 = #536
  memw(r29+#0) = r0
}
{call platform_log}
