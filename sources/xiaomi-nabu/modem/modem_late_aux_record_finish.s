// SPDX-License-Identifier: GPL-3.0-only
// Validated controller-record success path: return selected slot's data
// pointer and original frame restoration, after the field guards above.
.text
.globl modem_late_aux_record_finish
modem_late_aux_record_finish:
{
  r2 = add(r16,r3)
  r0 = #0
}
{
  r2 = memw(r2+#16)
  memw(r1+#0) = r2.new
}
{
  r17:16 = memd(r29+#8)
  dealloc_return
}
