// SPDX-License-Identifier: GPL-3.0-only
// Finish controller-session validation: register the descriptor result,
// preserve paired 64-bit parameters, optionally notify the callback and
// restore the original Hexagon V60 register frame.
.text
.globl modem_late_aux_large_controller_phase49
modem_late_aux_large_controller_phase49:
.set lookup_record,modem_late_aux_large_controller_phase49+0x6dec
.set resolve_handle,modem_late_aux_large_controller_phase49-0x197c
.set error_handler,modem_late_aux_large_controller_phase49-0xa40
.set skip_descriptor,modem_late_aux_large_controller_phase49+0x68
.set no_extra_data,modem_late_aux_large_controller_phase49+0x5c
.set no_callback,modem_late_aux_large_controller_phase49+0x80
{call lookup_record; r1 = r26}
{call resolve_handle}
{r0 = r17}
{memw(r17+#52) = r20}
{r1 = memw(r21+#24)}
{
  p0 = cmp.eq(r27,#0)
  if (p0.new) jump:nt skip_descriptor
  memw(r17+#48) = r0
}
{
  r3:2 = combine(#0,#0)
  r1:0 = memd(r27+#0)
}
{
  p0 = cmp.eq(r1:0,r3:2)
  if (!p0.new) jump:t no_extra_data
  if (p0.new) r5:4 = memd(r27+#8)
}
{
  p0 = cmp.eq(r5:4,r3:2)
  if (!p0.new) jump:t no_extra_data
  if (p0.new) r0 = memw(r27+#32)
}
{r1 = asr(r0,#31)}
{
  memd(r17+#192) = r1:0
  r0 = memw(r27+#44)
}
{r1 = asr(r0,#31)}
{jump skip_descriptor}
{memd(r17+#184) = r1:0}
no_extra_data:
{
  memd(r17+#184) = r1:0
  r1:0 = memd(r27+#8)
}
{memd(r17+#192) = r1:0}
skip_descriptor:
{
  r0 = memw(r29+#88)
  if (cmp.eq(r0.new,#0)) jump:nt no_callback
}
{
  r2 = memw(r16+#12)
  if (cmp.eq(r2.new,#0)) jump:nt no_callback
}
{r2 = memw(r16+#16)}
{callr r2}
no_callback:
{r0 = r17}
{
  r17:16 = memd(r29+#144)
  r19:18 = memd(r29+#136)
}
{
  r21:20 = memd(r29+#128)
  r23:22 = memd(r29+#120)
}
{
  r25:24 = memd(r29+#112)
  r27:26 = memd(r29+#104)
}
{dealloc_return}
{nop}
