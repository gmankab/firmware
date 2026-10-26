// SPDX-License-Identifier: GPL-3.0-only
.text
.globl modem_late_aux_large_controller_phase19_next3
modem_late_aux_large_controller_phase19_next3:
.set callback_unavailable, modem_late_aux_large_controller_phase19_next3 + 0x2c
.set previous_scan, modem_late_aux_large_controller_phase19_next3 - 0x56c
.set notify_controller, modem_late_aux_large_controller_phase19_next3 - 0x186c
.set no_error, modem_late_aux_large_controller_phase19_next3 + 0x70
{
  r4 = memw(r19+#0)
  if (cmp.eq(r4.new,#0)) jump:nt callback_unavailable
}
{
  r1:0 = combine(##507576841,#3)
  r2 = #409
  r3 = memw(r17+#28)
}
{
  r3 = memw(r3+#12)
  memw(r29+#0) = r3.new
}
{
  r3 = ##507577494
  callr r4
}
{
  r0 = ##507577494
  r2 = memw(r17+#28)
}
{
  call notify_controller
  r2 = memw(r2+#12)
  memw(r29+#0) = r2.new
}
{jump previous_scan}
callback_unavailable:
{
  r2 = memw(r17+#28)
  r3 = memw(r29+#84)
}
{
  p0 = r3
  if (!p0.new) r3 = ##507577536
  r2 = memw(r2+#4)
}
{if (!p0) jump:nt no_error}
