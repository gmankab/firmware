// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 controller record field #4 validation. Preserve the short
// success branch to the subsequent field #5 check and the original
// malformed-field diagnostic callback/return sequence.
.text
.globl modem_late_aux_record_type4
modem_late_aux_record_type4:
.set valid_type4,modem_late_aux_record_type4+0x58
.set callback_missing,modem_late_aux_record_type4+0x38
.set error_report,modem_late_aux_record_type4-0x2394
{
  r3 = memb(r16+#4)
  if (cmp.eq(r3.new,#1)) jump:t valid_type4
}
{
  r4 = memw(r2+#0)
  if (cmp.eq(r4.new,#0)) jump:nt callback_missing
}
{
  r2 = and(r3,#255)
  r1:0 = combine(##507576841,#3)
  memw(r29+#0) = r2.new
}
{
  r3 = ##507578597
  r2 = #140
  memw(r29+#4) = #1
}
{callr r4}
{r3 = memub(r16+#4)}
callback_missing:
{
  r0 = ##507578597
  r2 = and(r3,#255)
  memw(r29+#0) = r2.new
}
{
  call error_report
  memw(r29+#4) = #1
}
{
  r0 = #-1
  r17:16 = memd(r29+#8)
  dealloc_return
}
