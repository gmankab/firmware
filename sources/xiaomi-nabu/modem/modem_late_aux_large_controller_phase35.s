// SPDX-License-Identifier: GPL-3.0-only
// Signed 64-bit controller frame comparison and diagnostic notifier path;
// all targets symbolic and all code assembled as Hexagon V60 instructions.
.text
.globl modem_late_aux_large_controller_phase35
modem_late_aux_large_controller_phase35:
.set same_size, modem_late_aux_large_controller_phase35 + 0x60
.set no_callback, modem_late_aux_large_controller_phase35 + 0x44
.set platform_notifier, modem_late_aux_large_controller_phase35 - 0x1d80
.set resource_scanner, modem_late_aux_large_controller_phase35 - 0xa80
{
  r27 = asr(r26,#31)
  r25 = r1
}
{
  p0 = cmp.eq(r25:24,r27:26)
  if (p0.new) jump:t same_size
  if (p0.new) r7:6 = memd(r29+#32)
}
{
  r4 = memw(r19+#0)
  if (cmp.eq(r4.new,#0)) jump:nt no_callback
}
{
  r0 = r24
  r2 = #484
  r3 = ##507577799
}
{
  r1:0 = combine(##507576841,#3)
  memw(r29+#0) = r26
  memd(r29+#8) = r1:0
}
{callr r4}
{r1:0 = memd(r29+#64)}
no_callback:
{
  r25 = r1
  r0 = ##507577799
  memw(r29+#0) = r26
}
{
  call platform_notifier
  memd(r29+#8) = r25:24
}
{jump resource_scanner}
