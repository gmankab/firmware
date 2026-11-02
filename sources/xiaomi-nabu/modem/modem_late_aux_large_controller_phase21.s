// SPDX-License-Identifier: GPL-3.0-only
// Conditional descriptor callback results and secondary controller dispatch.
.text
.globl modem_late_aux_large_controller_phase21
modem_late_aux_large_controller_phase21:
.set trace_sink, modem_late_aux_large_controller_phase21 + 0xa28
.set skip_secondary_trace, modem_late_aux_large_controller_phase21 + 0x28
.set platform_report, modem_late_aux_large_controller_phase21 + 0x3d10
.set not_registered, modem_late_aux_large_controller_phase21 + 0x7c
.set registration_failed, modem_late_aux_large_controller_phase21 + 0x64
{
  r2 = #410
  r3 = ##507577536
  memw(r29+#0) = r2
}
{call trace_sink}
{jump skip_secondary_trace}
{
  r1:0 = combine(##507576841,#2)
  r2 = #410
  memw(r29+#0) = r2
}
{call platform_report}
{r2 = #0}
{
  r1:0 = combine(#0,r20)
  r3 = memw(r16+#36)
}
{callr r3}
{
  p0 = cmp.eq(r0,#0); if (p0.new) jump:nt not_registered
  if (p0.new) r2 = memw(r29+#84)
}
{
  r4 = memw(r19+#0)
  if (cmp.eq(r4.new,#0)) jump:nt registration_failed
}
{
  r1:0 = combine(##507576841,#3)
  r2 = #411
}
