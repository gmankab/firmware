// SPDX-License-Identifier: GPL-3.0-only
// The next controller-record guard validates word five (record status).
// A valid record follows the next fragment; invalid values are reported by
// the installed error callback, with original signed error return frame.
.text
.globl modem_late_aux_record_type7
modem_late_aux_record_type7:
.set valid_type7,modem_late_aux_record_type7+0x44
.set no_handler,modem_late_aux_record_type7+0x30
.set error_report,modem_late_aux_record_type7-0x2488
{
  r3 = memw(r16+#20)
  if (cmp.eq(r3.new,#1)) jump:t valid_type7
}
{
  r4 = memw(r2+#0)
  if (cmp.eq(r4.new,#0)) jump:nt no_handler
}
{
  r1:0 = combine(##507576841,#3)
  r2 = #147
  memw(r29+#0) = r3
}
{
  r3 = ##507578656
  callr r4
}
{r3 = memw(r16+#20)}
no_handler:
{
  call error_report
  r0 = ##507578656
  memw(r29+#0) = r3
}
{
  r0 = #-1
  r17:16 = memd(r29+#8)
  dealloc_return
}
