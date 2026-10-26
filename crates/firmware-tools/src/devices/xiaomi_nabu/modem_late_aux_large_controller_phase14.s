// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 session frame callbacks and controller resource installation.
// Source-only file offsets [0x447d90,0x447e1c); function continues later.
.text
.globl modem_late_aux_large_controller_phase14
modem_late_aux_large_controller_phase14:
.set resource_callback_logger, modem_late_aux_large_controller_phase14 - 0x15f0
.set status_scanner_loop, modem_late_aux_large_controller_phase14 - 0x2f0
.set skip_allocation, modem_late_aux_large_controller_phase14 + 0x8c
{callr r2}
{
  p0 = cmp.eq(r0,#0)
  if (!p0.new) jump:t next_allocation
  memw(r17+#36) = r0
}
{
  r4 = memw(r19+#0)
  if (cmp.eq(r4.new,#0)) jump:nt reported_primary
}
{
  r1:0 = combine(##507576841,#3)
  r2 = #363
}
{
  callr r4
  r3 = ##507577366
  memw(r29+#0) = r22
}
reported_primary:
{
  call resource_callback_logger
  r0 = ##507577366
  memw(r29+#0) = r22
}
{jump status_scanner_loop}
next_allocation:
{
  r2 = memw(r17+#8)
  memw(r17+#40) = r23
}
{
  p0 = cmp.eq(r2,#0)
  r2 = memw(r19+#36)
}
{r18 = mux(p0,#1,r23)}
{r22 = +mpyi(r18,#28)}
{
  r0 = r22
  callr r2
}
{
  p0 = cmp.eq(r0,#0)
  if (!p0.new) jump:t skip_allocation
  memw(r17+#28) = r0
}
{
  r4 = memw(r19+#0)
  if (cmp.eq(r4.new,#0)) jump:nt reported_secondary
}
{
  r1:0 = combine(##507576841,#3)
  r2 = #373
}
{
  callr r4
  r3 = ##507577401
  memw(r29+#0) = r22
}
reported_secondary:
{
  call resource_callback_logger
  r0 = ##507577401
  memw(r29+#0) = r22
}
{jump status_scanner_loop}
