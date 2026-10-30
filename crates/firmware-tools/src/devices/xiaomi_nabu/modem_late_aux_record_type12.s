// SPDX-License-Identifier: GPL-3.0-only
// Normalize the header's unsigned width and verify its 5-bit-shifted
// aggregate fits the runtime controller's maximum allocation limit.
// On failure call the configured reporter and preserve the signed epilogue.
.text
.globl modem_late_aux_record_type12
modem_late_aux_record_type12:
.set valid_type12,modem_late_aux_record_type12+0x48
.set no_handler,modem_late_aux_record_type12+0x34
.set error_report,modem_late_aux_record_type12-0x25ec
{
  r5 = zxth(r4)
  r6 = memw(##507617080)
}
{
  r7 = addasl(r3,r5,#5)
  if (!cmp.gtu(r7.new,r6)) jump:t valid_type12
}
{
  r4 = memw(r2+#0)
  if (cmp.eq(r4.new,#0)) jump:nt no_handler
}
{
  r1:0 = combine(##507576841,#3)
  r2 = #157
}
{
  r3 = ##507578805
  callr r4
}
no_handler:
{
  call error_report
  r0 = ##507578805
}
{
  r0 = #-1
  r17:16 = memd(r29+#8)
  dealloc_return
}
