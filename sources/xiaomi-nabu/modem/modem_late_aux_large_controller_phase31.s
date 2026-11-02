// SPDX-License-Identifier: GPL-3.0-only
// Controller format-specific error handling, selected destination mapping,
// and short trace callback packet. All calls use symbolic relative targets.
.text
.globl modem_late_aux_large_controller_phase31
modem_late_aux_large_controller_phase31:
.set use_default_mapping, modem_late_aux_large_controller_phase31 + 0x340
.set resume_dispatch, modem_late_aux_large_controller_phase31 + 0x354
.set platform_trace, modem_late_aux_large_controller_phase31 + 0x6fc
{
  memw(r24=##507614244) = r0
  r2 = memw(r29+#84)
}
{
  p0 = r2
  if (!p0.new) r3 = ##507577940
  if (!p0.new) jump:t use_default_mapping
}
{
  r2 = #523
  r3 = ##507577940
  memw(r29+#0) = r0
}
{call platform_trace}
{jump resume_dispatch}
{
  r4 = asl(r7,#5)
  r8 = memw(r29+#56)
  r26 = memw(r18+#-8)
}
