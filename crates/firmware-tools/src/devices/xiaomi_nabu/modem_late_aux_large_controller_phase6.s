// SPDX-License-Identifier: GPL-3.0-only
// Post-scan controller result dispatch, file [0x4479d0,0x447a24).
.text
.globl modem_late_aux_large_controller_phase6
modem_late_aux_large_controller_phase6:
.set controller_base, modem_late_aux_large_controller_phase6 - 0x2ac
.set controller_log, controller_base + 0x136c
.set resource_log, controller_base + 0x4654
.set else_report, controller_base + 0x2c8
.set shared_callback, controller_base + 0x2e4
.set error_packet, controller_base + 0x350
{
  r2 = #291
  r3 = ##507577067
  memw(r29+#4) = r2
}
{
  call controller_log
  memw(r29+#8) = r24
  memw(r29+#0) = r23
}
{ jump shared_callback }
else_report:
{
  r1:0 = combine(##507576841,#2)
  r2 = #291
  memw(r29+#4) = r2
}
{
  r3 = ##507577067
  memw(r29+#0) = r23
}
{ call resource_log }
shared_callback:
{
  r1:0 = combine(r24,r23)
  r3:2 = combine(r20,#1)
  r4 = memw(r16+#56)
}
{ callr r4 }
{
  p0 = cmp.eq(r0,#0)
  if (!p0.new) jump:t error_packet
  if (p0.new) r6 = memw(r16+#8)
  if (p0.new) r24 = memw(r29+#76)
}
