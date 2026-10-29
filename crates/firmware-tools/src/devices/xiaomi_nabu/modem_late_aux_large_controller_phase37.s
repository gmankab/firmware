// SPDX-License-Identifier: GPL-3.0-only
// Restore temporary frame registers and check optional callback, including
// the signed decrement/accumulate packet that older LLVM cannot disassemble.
.text
.globl modem_late_aux_large_controller_phase37
modem_late_aux_large_controller_phase37:
.set no_result_callback,modem_late_aux_large_controller_phase37 + 0x320
.set alternate_result,modem_late_aux_large_controller_phase37 + 0x50
{
  r26 = memw(r29+#80)
  r27 = memw(r29+#48)
}
{r6 = memw(r29+#52)}
{
  r2 = memw(r16+#12)
  if (cmp.eq(r2.new,#0)) jump:nt ##no_result_callback
}
{
  r2 = r24
  r0 = r24
  r3 = memw(r29+#84)
}
{r2 += add(r6,#-1)}
{
  p0 = r3
  if (!p0.new) jump:t alternate_result
}
