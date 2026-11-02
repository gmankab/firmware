// SPDX-License-Identifier: GPL-3.0-only
// Hexagon controller initialization callbacks and conditional diagnostics.
.text
.globl modem_late_aux_large_controller_phase22
modem_late_aux_large_controller_phase22:
.set platform_result, modem_late_aux_large_controller_phase22 - 0x1918
.set shared_scan, modem_late_aux_large_controller_phase22 - 0x618
.set error_message, modem_late_aux_large_controller_phase22 + 0x50
.set trace_sink, modem_late_aux_large_controller_phase22 + 0x9d8
{
  r3 = ##507577589
  memw(r29+#4) = r26
  memw(r29+#0) = #0
}
{callr r4}
{
  r0 = ##507577589
  memw(r29+#4) = r26
  memw(r29+#0) = #0
}
{call platform_result}
{jump shared_scan}
{
  p0 = r2
  if (!p0.new) r3 = ##507577614
  if (!p0.new) jump:t error_message
}
{
  call trace_sink
  r2 = #412
  r3 = ##507577614
}
