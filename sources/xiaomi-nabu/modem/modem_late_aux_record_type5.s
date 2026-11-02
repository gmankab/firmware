// SPDX-License-Identifier: GPL-3.0-only
// Next compact firmware-record field (#5): two permitted byte values;
// invalid values are sent to the controller's report callback before
// the existing signed-error epilogue. The accepted path continues beyond
// this source fragment at modem file offset 0x448bd8.
.text
.globl modem_late_aux_record_type5
modem_late_aux_record_type5:
.set valid_type5,modem_late_aux_record_type5+0x4c
.set no_handler,modem_late_aux_record_type5+0x38
.set error_report,modem_late_aux_record_type5-0x23ec
{
  r4 = #2
  r3 = memub(r16+#5)
}
{r5 = add(r3,#-1)}
{
  r5 = and(r5,#255)
  if (cmp.gtu(r4,r5.new)) jump:t valid_type5
}
{
  r4 = memw(r2+#0)
  if (cmp.eq(r4.new,#0)) jump:nt no_handler
}
{
  r1:0 = combine(##507576841,#3)
  r2 = #143
  memw(r29+#0) = r3
}
{
  r3 = ##507578628
  callr r4
}
{r3 = memub(r16+#5)}
no_handler:
{
  call error_report
  r0 = ##507578628
  memw(r29+#0) = r3
}
{
  r0 = #-1
  r17:16 = memd(r29+#8)
  dealloc_return
}
