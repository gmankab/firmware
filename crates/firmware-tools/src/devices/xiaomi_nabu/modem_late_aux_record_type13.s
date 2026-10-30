// SPDX-License-Identifier: GPL-3.0-only
// Reject a record width exceeding the public 16-unit descriptor bound.
// Validate the callback pointer before logging observed 16-bit width.
.text
.globl modem_late_aux_record_type13
modem_late_aux_record_type13:
.set valid_type13,modem_late_aux_record_type13+0x48
.set no_handler,modem_late_aux_record_type13+0x2c
.set error_report,modem_late_aux_record_type13-0x2634
{
  r6 = #16
  if (cmp.gtu(r6.new,r5)) jump:t valid_type13
}
{
  r6 = memw(r2+#0)
  if (cmp.eq(r6.new,#0)) jump:nt no_handler
}
{
  r1:0 = combine(##507576841,#3)
  r2 = #159
}
{
  callr r6
  r3 = ##507578845
  memw(r29+#0) = r5
}
{r4 = memuh(r16+#44)}
no_handler:
{
  r0 = ##507578845
  r2 = zxth(r4)
  memw(r29+#0) = r2.new
}
{call error_report}
{
  r0 = #-1
  r17:16 = memd(r29+#8)
  dealloc_return
}
