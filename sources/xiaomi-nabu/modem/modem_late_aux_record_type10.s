// SPDX-License-Identifier: GPL-3.0-only
// Require the signed record halfword at byte 42 to equal the expected frame
// class (32). Report unsigned field contents on failure, preserving the
// reporter's original packet and frame restore semantics.
.text
.globl modem_late_aux_record_type10
modem_late_aux_record_type10:
.set valid_type10,modem_late_aux_record_type10+0x54
.set no_handler,modem_late_aux_record_type10+0x38
.set error_report,modem_late_aux_record_type10-0x255c
{r4 = memh(r16+#42)}
{
  p0 = cmp.eq(r4,#32)
  if (p0.new) jump:t valid_type10
}
{
  r5 = memw(r2+#0)
  if (cmp.eq(r5.new,#0)) jump:nt no_handler
}
{
  r2 = zxth(r4)
  r1:0 = combine(##507576841,#3)
  memw(r29+#0) = r2.new
}
{
  r3 = ##507578747
  callr r5
  r2 = #153
}
{r4 = memuh(r16+#42)}
no_handler:
{
  r0 = ##507578747
  r2 = zxth(r4)
  memw(r29+#0) = r2.new
}
{call error_report}
{
  r0 = #-1
  r17:16 = memd(r29+#8)
  dealloc_return
}
