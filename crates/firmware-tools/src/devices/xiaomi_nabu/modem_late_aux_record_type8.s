// SPDX-License-Identifier: GPL-3.0-only
// Controller record halfword at offset 16 must have the required low bits;
// malformed records are sent to an optional reporter and then rejected.
.text
.globl modem_late_aux_record_type8
modem_late_aux_record_type8:
.set valid_type8,modem_late_aux_record_type8+0x48
.set no_handler,modem_late_aux_record_type8+0x34
.set error_report,modem_late_aux_record_type8-0x24cc
{r3 = memuh(r16+#16)}
{
  r4 = setbit(r3,#0)
  if (cmp.eq(r4.new,#3)) jump:t valid_type8
}
{
  r4 = memw(r2+#0)
  if (cmp.eq(r4.new,#0)) jump:nt no_handler
}
{
  r1:0 = combine(##507576841,#3)
  r2 = #149
  memw(r29+#0) = r3
}
{
  r3 = ##507578687
  callr r4
}
{r3 = memuh(r16+#16)}
no_handler:
{
  call error_report
  r0 = ##507578687
  memw(r29+#0) = r3
}
{
  r0 = #-1
  r17:16 = memd(r29+#8)
  dealloc_return
}
