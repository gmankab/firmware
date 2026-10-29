// SPDX-License-Identifier: GPL-3.0-only
// Fourth controller record and diagnostic callback; source-encoded Hexagon
// V60 packets with symbolic external call and branch destinations.
.text
.globl modem_late_aux_large_controller_phase43
modem_late_aux_large_controller_phase43:
.set lookup_record, modem_late_aux_large_controller_phase43 + 0x7460
.set device_report, modem_late_aux_large_controller_phase43 + 0x2b0
.set platform_log, modem_late_aux_large_controller_phase43 + 0x3598
.set report_done, modem_late_aux_large_controller_phase43 + 0x158
.set fallback_report, modem_late_aux_large_controller_phase43 + 0x40
{
  r23 = memw(r24+#0)
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
  if (!p0.new) r3 = ##507577982
  if (!p0.new) jump:t fallback_report
}
{
  r2 = #538
  r3 = ##507577982
  memw(r29+#0) = r0
}
{
  call device_report
  r24 = r25
}
{jump report_done}
fallback_report:
{
  r1:0 = combine(##507576841,#2)
  r2 = #538
  memw(r29+#0) = r0
}
{
  call platform_log
  r24 = r25
}
{jump report_done}
