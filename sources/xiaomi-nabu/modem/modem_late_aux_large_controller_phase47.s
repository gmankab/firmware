// SPDX-License-Identifier: GPL-3.0-only
// Scan codec-controller record limits, report missing registry state, then
// resume the earlier bounded loop using symbolic V60 branch destinations.
.text
.globl modem_late_aux_large_controller_phase47
modem_late_aux_large_controller_phase47:
.set scan_previous,modem_late_aux_large_controller_phase47-0x610
.set next_result,modem_late_aux_large_controller_phase47+0x48
.set skip_diagnostic,modem_late_aux_large_controller_phase47+0x38
.set platform_report,modem_late_aux_large_controller_phase47-0x2198
.set previous_scan,modem_late_aux_large_controller_phase47-0xe98
{
  r18 = add(r10,#32)
  r22 = add(r22,#1)
}
{
  r2 = memuh(r21+#44)
  if (cmp.gt(r2.new,r22)) jump:t ##scan_previous
}
{
  r2 = memw(r17+#44)
  if (!cmp.eq(r2.new,#0)) jump:t next_result
}
{
  r4 = memw(r19+#0)
  if (cmp.eq(r4.new,#0)) jump:nt skip_diagnostic
}
{
  r1:0 = combine(##507576841,#3)
  r2 = #557
}
{
  r3 = ##507578107
  callr r4
}
skip_diagnostic:
{
  call platform_report
  r0 = ##507578107
}
{jump previous_scan}
