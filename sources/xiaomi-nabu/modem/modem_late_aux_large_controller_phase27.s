// SPDX-License-Identifier: GPL-3.0-only
// Check selected controller callback, report buffer descriptor and return
// to the earlier resource scan. Hexagon V60 source, no OEM opcode arrays.
.text
.globl modem_late_aux_large_controller_phase27
modem_late_aux_large_controller_phase27:
.set no_callback, modem_late_aux_large_controller_phase27 + 0x50
.set callback_empty, modem_late_aux_large_controller_phase27 + 0x34
.set controller_notify, modem_late_aux_large_controller_phase27 - 0x1a88
.set prior_resource_scan, modem_late_aux_large_controller_phase27 - 0x788
{p0 = cmp.eq(r0,#0); if (p0.new) jump:t no_callback}
{
  r4 = memw(r19+#0)
  if (cmp.eq(r4.new,#0)) jump:nt callback_empty
}
{
  r1:0 = combine(##507576841,#3)
  r2 = memw(r17+#28)
}
{
  r5 = add(r2,mpyi(#28,r18))
  r3 = ##507577494
  r2 = #443
}
{
  callr r4
  r5 = memw(r5+#12)
  memw(r29+#0) = r5.new
}
{
  r0 = ##507577494
  r2 = memw(r17+#28)
}
{r2 = add(r2,mpyi(#28,r18))}
{
  call controller_notify
  r2 = memw(r2+#12)
  memw(r29+#0) = r2.new
}
{jump prior_resource_scan}
