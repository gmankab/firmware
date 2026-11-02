// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 controller session continuation with a bounded hardware loop.
// Source-generated partial instruction range [0x447d38, 0x447d90).
.text
.globl modem_late_aux_large_controller_phase13
modem_late_aux_large_controller_phase13:
.set callback_alloc, modem_late_aux_large_controller_phase13 + 0x4040
.set retry_current_session, modem_late_aux_large_controller_phase13 - 0x2c
{
  r1:0 = combine(##507576841,#2)
  r2 = #332
}
{call callback_alloc}
{jump retry_current_session}
{
  r6 = add(r2,#-1)
  p1 = cmp.gtu(r2,#1)
}
{
  r3 = #0
  r4 = memw(r21+#28)
}
{
  loop0(scan_body,r6)
  r5 = add(r21,r4)
}
{
  r2 = add(r5,#32)
  r4 = memw(r5+#0)
}
{
  if (!p1) jump:nt scan_done
  r4 = add(r3,#1)
  p0 = cmp.eq(r4,#1)
}
scan_body:
{
  if (p0) r3 = add(r4,#0)
  r2 = add(r2,#32)
  r5 = memw(r2+#0)
}
{
  nop
  r4 = add(r3,#1)
  p0 = cmp.eq(r5,#1)
}:endloop0
scan_done:
{r23 = mux(p0,r4,r3)}
{
  r22 = asl(r23,#5)
  r2 = memw(r19+#36)
}
{r0 = r22}
