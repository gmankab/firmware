// SPDX-License-Identifier: GPL-3.0-only
// Advance to next controller slot; reset descriptor accumulator if no record
// remains, preserving the original Hexagon V60 loop packet and call targets.
.text
.globl modem_late_aux_large_controller_phase29
modem_late_aux_large_controller_phase29:
.set prior_scan, modem_late_aux_large_controller_phase29 - 0x44c
.set after_empty_record, modem_late_aux_large_controller_phase29 + 0x67c
{
  r22 = add(r22,#32)
  r27 = add(r27,#1)
  if (cmp.gt(r4,r27.new)) jump:t prior_scan
}
{
  r26 = memw(r29+#80)
  r27 = memw(r29+#48)
}
{r24 = memw(r29+#76)}
{
  r2 = memh(r21+#44)
  if (cmp.eq(r2.new,#0)) jump:nt after_empty_record
}
{
  r2 = #0
  r1 = #0
  r18 = memw(r21+#28)
  memw(r29+#60) = r2.new
}
{
  r18 += add(r21,#16)
  r2 = add(r21,r18)
  r22 = #0
  memw(r29+#28) = r2.new
}
{
  r2 = #0
  memd(r29+#64) = r1:0
}
{
  memw(r29+#56) = r2
  memw(r29+#80) = r26
}
