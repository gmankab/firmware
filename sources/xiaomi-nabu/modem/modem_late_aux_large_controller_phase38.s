// SPDX-License-Identifier: GPL-3.0-only
// Emit ordinary/alternate controller error-size callbacks from the selected
// 64-bit frame accumulator. Hexagon V60 instructions and symbolic branches.
.text
.globl modem_late_aux_large_controller_phase38
modem_late_aux_large_controller_phase38:
.set completed_report, modem_late_aux_large_controller_phase38 + 0x40
.set trace_callback, modem_late_aux_large_controller_phase38 + 0x468
.set alternate_callback, modem_late_aux_large_controller_phase38 + 0x3750
{
  r3:2 = memd(r29+#64)
  memw(r29+#4) = r2
}
{
  r2 = #494
  r3 = ##507577862
  r1 = r3
}
{
  call trace_callback
  memw(r29+#0) = r6
  memd(r29+#8) = r1:0
}
{jump completed_report}
{
  r2 = #494
  r5:4 = memd(r29+#64)
  memw(r29+#4) = r2
}
{r1 = r5}
{
  r3 = ##507577862
  memw(r29+#0) = r6
}
{call alternate_callback}
{
  r1:0 = combine(##507576841,#2)
  memd(r29+#8) = r1:0
}
