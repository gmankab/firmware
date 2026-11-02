// SPDX-License-Identifier: GPL-3.0-only
// Reject a missing record descriptor at word offset 28. Check the optional
// callback before emitting diagnostic 151; preserve the original error exit.
.text
.globl modem_late_aux_record_type9
modem_late_aux_record_type9:
.set valid_type9,modem_late_aux_record_type9+0x48
.set handler_present,modem_late_aux_record_type9+0x14
.set error_report_gate,modem_late_aux_record_type9+0x34
.set error_report,modem_late_aux_record_type9-0x2514
{
  r3 = memw(r16+#28)
  if (!cmp.eq(r3.new,#0)) jump:t valid_type9
}
{
  r4 = memw(r2+#0)
  if (!cmp.eq(r4.new,#0)) jump:t handler_present
}
{r2 = #0 ; jump error_report_gate}
handler_present:
{
  r1:0 = combine(##507576841,#3)
  r2 = #151
}
{
  r3 = ##507578715
  callr r4
  memw(r29+#0) = #0
}
{r2 = memw(r16+#28)}
error_report_gate:
{
  call error_report
  r0 = ##507578715
  memw(r29+#0) = r2
}
{
  r0 = #-1
  r17:16 = memd(r29+#8)
  dealloc_return
}
