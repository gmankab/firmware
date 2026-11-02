// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 controller descriptor initialization, [0x447e1c,0x447e80).
.text
.globl modem_late_aux_large_controller_phase15
modem_late_aux_large_controller_phase15:
.set process_resource, modem_late_aux_large_controller_phase15 + 0x7794
.set after_frame_reset, modem_late_aux_large_controller_phase15 + 0x20
.set no_controller, modem_late_aux_large_controller_phase15 + 0xc4
.set late_diagnostic, modem_late_aux_large_controller_phase15 + 0xb2c
{
  call process_resource
  r1 = #0
  r2 = r22
}
{
  p0 = cmp.gt(r18,#0)
  if (!p0.new) jump:nt after_frame_reset
}
{
  loop0(init_frame_loop,r18)
  r3:2 = combine(#-1,#4)
}
init_frame_loop:
{r4 = memw(r17+#28)}
{
  r2 = add(r2,#28)
  memw(r4+r2<<#0) = r3
}:endloop0
after_frame_reset:
{memw(r17+#32) = r18}
{r22 = memw(r21+#28)}
{r4 = memuh(r21+#44)}
{
  r2 = memw(r17+#8)
  if (cmp.eq(r2.new,#0)) jump:nt no_controller
}
{
  if (!p0.new) r18 = #0
  if (!p0.new) r23 = add(r16,#76)
  p0 = cmp.eq(r4,#0)
  if (p0.new) jump:nt late_diagnostic
}
{
  r22 += add(r21,#24)
  r25 = add(r16,#64)
  memw(r29+#76) = r24
  memw(r29+#48) = r27
}
{
  r24 = add(r16,#80)
  memw(r29+#80) = r26
}
{
  r26 = add(r16,#68)
  r27 = #0
}
