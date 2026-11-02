// SPDX-License-Identifier: GPL-3.0-only
// Guard optional controller callback against missing stream metadata, record
// the invalid-result diagnostic and rejoin the earlier resource scanner.
.text
.globl modem_late_aux_large_controller_phase44
modem_late_aux_large_controller_phase44:
.set next_phase, modem_late_aux_large_controller_phase44 + 0xfc
.set scan_record_result, modem_late_aux_large_controller_phase44 + 0x48
.set skip_logging, modem_late_aux_large_controller_phase44 + 0x38
.set notifier, modem_late_aux_large_controller_phase44 - 0x209c
.set previous_scan, modem_late_aux_large_controller_phase44 - 0xd9c
{
  p0 = cmp.eq(r22,#0)
  if (!p0.new) jump:nt next_phase
}
{
  r2 = memw(r16+#12)
  if (cmp.eq(r2.new,#0)) jump:nt next_phase
}
{r23 = memw(r29+#92)}
{
  r2 = memw(r24+#0)
  if (!cmp.gt(r23,r2.new)) jump:t scan_record_result
}
{
  r4 = memw(r19+#0)
  if (cmp.eq(r4.new,#0)) jump:nt skip_logging
}
{
  r1:0 = combine(##507576841,#3)
  r2 = #546
}
{
  callr r4
  r3 = ##507577986
  memw(r29+#0) = r23
}
skip_logging:
{
  call notifier
  r0 = ##507577986
  memw(r29+#0) = r23
}
{jump previous_scan}
scan_record_result:
{
  r2 = r23
  r3 = memw(r29+#84)
}
