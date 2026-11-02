// SPDX-License-Identifier: GPL-3.0-only
// Validate the active controller record and dispatch an optional callback.
.text
.globl modem_late_aux_large_controller_phase25
modem_late_aux_large_controller_phase25:
.set valid_session, modem_late_aux_large_controller_phase25 + 0x48
.set absent_callback, modem_late_aux_large_controller_phase25 + 0x34
.set platform_reporting, modem_late_aux_large_controller_phase25 - 0x19dc
.set resource_list_loop, modem_late_aux_large_controller_phase25 - 0x6dc
{
  r2 = r23
  r4 = r24
  r3 = memw(r16+#72)
  if (!cmp.eq(r3.new,#0)) jump:t valid_session
}
{
  r4 = memw(r19+#0)
  if (cmp.eq(r4.new,#0)) jump:nt absent_callback
}
{
  r1:0 = combine(##507576841,#3)
  r2 = #430
}
{
  r3 = ##507577708
  callr r4
  memw(r29+#0) = #0
}
{
  call platform_reporting
  r0 = ##507577708
  memw(r29+#0) = #0
}
{jump resource_list_loop}
