// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 controller-result diagnostics / callback delivery.
.text
.globl modem_late_aux_large_controller_phase19
modem_late_aux_large_controller_phase19:
.set no_aux_callback, modem_late_aux_large_controller_phase19 + 0x20
.set continued_controller_processing, modem_late_aux_large_controller_phase19 + 0x3c
.set trace_sink, modem_late_aux_large_controller_phase19 + 0xae8
.set controller_log, modem_late_aux_large_controller_phase19 + 0x3dd0
{
  p0 = r3
  if (!p0.new) jump:t no_aux_callback
  r2 = memw(r2+#0)
}
{
  r2 = #402
  r3 = ##507577439
  memw(r29+#0) = r2
}
{ call trace_sink }
{ jump continued_controller_processing }
no_aux_callback:
{
  r1:0 = combine(##507576841,#2)
  r2 = #402
  memw(r29+#0) = r2
}
{
  call controller_log
  r3 = ##507577439
}
