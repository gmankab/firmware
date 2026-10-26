// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 session tagged-state checks, diagnostics and previous loop return.
// Source-derived interval [0x447e80, 0x447ee0).
.text
.globl modem_late_aux_large_controller_phase16
modem_late_aux_large_controller_phase16:
.set resource_mode_error, modem_late_aux_large_controller_phase16 + 0x44c
.set unsupported_buffer_mode, modem_late_aux_large_controller_phase16 + 0x2fc
.set controller_state_fail, modem_late_aux_large_controller_phase16 + 0x344
.set no_controller_log, modem_late_aux_large_controller_phase16 + 0x4c
.set shared_controller_report, modem_late_aux_large_controller_phase16 - 0x16e0
.set resource_list_loop, modem_late_aux_large_controller_phase16 - 0x3e0
{
  r2 = memw(r22+#-24)
  if (!cmp.eq(r2.new,#1)) jump:t resource_mode_error
}
{r2 = memw(r22+#0)}
{
  if (p0.new) r2 = add(r25,#0)
  if (p0.new) r4 = add(r26,#0)
  p0 = tstbit(r2,#0)
  if (!p0.new) jump:t unsupported_buffer_mode
}
{
  r3 = memw(r16+#60)
  if (!cmp.eq(r3.new,#0)) jump:t ##controller_state_fail
}
{
  r4 = memw(r19+#0)
  if (cmp.eq(r4.new,#0)) jump:nt no_controller_log
}
{
  r1:0 = combine(##507576841,#3)
  r2 = #425
}
{
  r3 = ##507577686
  callr r4
  memw(r29+#0) = #0
}
no_controller_log:
{
  call shared_controller_report
  r0 = ##507577686
  memw(r29+#0) = #0
}
{jump resource_list_loop}
