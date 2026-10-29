// SPDX-License-Identifier: GPL-3.0-only
// Validate record byte #6 (type/codec-control flag), preserving the two
// existing diagnostic dispatch paths and original error return frame.
.text
.globl modem_late_aux_record_type6
modem_late_aux_record_type6:
.set valid_type6,modem_late_aux_record_type6+0x50
.set no_handler,modem_late_aux_record_type6+0x34
.set error_report,modem_late_aux_record_type6-0x2438
{
  r3 = memb(r16+#6)
  if (cmp.eq(r3.new,#1)) jump:t valid_type6
}
{
  r4 = memw(r2+#0)
  if (cmp.eq(r4.new,#0)) jump:nt no_handler
}
{
  r2 = and(r3,#255)
  r1:0 = combine(##507576841,#3)
  memw(r29+#0) = r2.new
}
{
  r3 = ##507578656
  callr r4
  r2 = #145
}
{r3 = memub(r16+#6)}
no_handler:
{
  r0 = ##507578656
  r2 = and(r3,#255)
  memw(r29+#0) = r2.new
}
{call error_report}
{
  r0 = #-1
  r17:16 = memd(r29+#8)
  dealloc_return
}
