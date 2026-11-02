// SPDX-License-Identifier: GPL-3.0-only
// Complete controller callback lookup, route status through custom or
// platform handlers and update the per-frame pass/fail counters.
.text
.globl modem_late_aux_large_controller_phase46
modem_late_aux_large_controller_phase46:
.set no_more_reports,modem_late_aux_large_controller_phase46+0x6c
.set skip_custom_trace,modem_late_aux_large_controller_phase46+0x3c
.set platform_report,modem_late_aux_large_controller_phase46-0x212c
.set process_previous,modem_late_aux_large_controller_phase46-0xe2c
{
  r1 = #0
  r3 = r23
  r4 = memw(r16+#12)
  r2 = memw(r29+#28)
}
{
  r2 = memw(r2+#4)
  r0 = memw(r29+#88)
}
{
  r2 = add(r21,r2)
  callr r4
}
{
  p0 = cmp.eq(r0,#0)
  if (p0.new) jump:t no_more_reports
}
{
  r4 = memw(r19+#0)
  if (cmp.eq(r4.new,#0)) jump:nt skip_custom_trace
}
{
  r1:0 = combine(##507576841,#3)
  r2 = #551
}
{
  callr r4
  r3 = ##507578058
  memw(r29+#0) = r23
}
skip_custom_trace:
{
  call platform_report
  r0 = ##507578058
  memw(r29+#0) = r23
}
{jump process_previous}
{
  r2 = memw(r17+#8)
  r24 = memw(r29+#76)
}
{
  p0 = cmp.eq(r2,#0)
  r2 = memw(r29+#60)
}
{
  r2 = add(r2,#1)
  memw(r29+#60) = r2.new
}
{r2 = memw(r29+#56)}
{if (!p0) r2 = add(r2,#1)}
{memw(r29+#56) = r2}
