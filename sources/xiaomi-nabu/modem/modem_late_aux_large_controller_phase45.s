// SPDX-License-Identifier: GPL-3.0-only
// Signed descriptor decrement, missing-record branch and direct/fallback
// status code 548 callbacks. Preserve an exceptional producer/branch packet
// using checked V60 parse-bit and relative-branch reconstruction.
.text
.globl modem_late_aux_large_controller_phase45
modem_late_aux_large_controller_phase45:
.set fallback_report, modem_late_aux_large_controller_phase45 + 0x24
.set report_done, modem_late_aux_large_controller_phase45 + 0x44
.set device_report, modem_late_aux_large_controller_phase45 + 0x208
.set platform_log, modem_late_aux_large_controller_phase45 + 0x34f0
{r2 += add(r21,#-1)}
{
  p0 = r3
  if (!p0.new) jump:t fallback_report
}
{
  r2 = #548
  r3 = ##507578022
  memw(r29+#4) = r2
}
{
  call device_report
  memw(r29+#8) = r23
  memw(r29+#0) = r21
}
{jump report_done}
fallback_report:
{
  r1:0 = combine(##507576841,#2)
  r2 = #548
  memw(r29+#4) = r2
}
{
  r3 = ##507578022
  memw(r29+#8) = r23
  memw(r29+#0) = r21
}
{call platform_log}
