// SPDX-License-Identifier: GPL-3.0-only
// The signed record halfword at byte 44 must be nonzero. An absent value
// follows the installed diagnostic callback and the original error epilogue.
.text
.globl modem_late_aux_record_type11
modem_late_aux_record_type11:
.set valid_type11,modem_late_aux_record_type11+0x3c
.set no_handler,modem_late_aux_record_type11+0x28
.set error_report,modem_late_aux_record_type11-0x25b0
{
  r4 = memh(r16+#44)
  if (!cmp.eq(r4.new,#0)) jump:t valid_type11
}
{
  r4 = memw(r2+#0)
  if (cmp.eq(r4.new,#0)) jump:nt no_handler
}
{
  r1:0 = combine(##507576841,#3)
  r2 = #155
}
{
  r3 = ##507578777
  callr r4
}
no_handler:
{
  call error_report
  r0 = ##507578777
}
{
  r0 = #-1
  r17:16 = memd(r29+#8)
  dealloc_return
}
