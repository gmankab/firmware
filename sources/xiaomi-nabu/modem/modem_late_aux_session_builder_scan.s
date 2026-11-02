// SPDX-License-Identifier: GPL-3.0-only
// Determine the number of codec input descriptor records, walking the
// bounded record table with Hexagon hardware loop0/ :endloop0 semantics.
.text
.globl modem_late_aux_session_builder_scan
modem_late_aux_session_builder_scan:
.set invalid_input,modem_late_aux_session_builder_scan-0x90
.set iterate_records,modem_late_aux_session_builder_scan+0x1c
.set finish_record_count,modem_late_aux_session_builder_scan+0x54
.set after_loop,modem_late_aux_session_builder_scan+0x50
.set loop_body,modem_late_aux_session_builder_scan+0x40
{
  r2 = memh(r17+#16)
  if (!cmp.eq(r2.new,#2)) jump:t invalid_input
}
{
  memw(r16+#12) = #1
  r3 = memuh(r17+#44)
}
{
  p0 = cmp.eq(r3,#0)
  if (!p0.new) jump:t iterate_records
  jump finish_record_count
  if (p0.new) r18 = #0
}
iterate_records:
{
  r6 = add(r3,#-1)
}
{
  p1 = cmp.gtu(r3,#1)
  r2 = #0
  r4 = memw(r17+#28)
}
{
  loop0(loop_body,r6)
  r5 = add(r17,r4)
}
{
  r3 = add(r5,#32)
  r4 = memw(r5+#0)
}
{if (!p1) jump:nt after_loop}
{
  r4 = add(r2,#1)
  p0 = cmp.eq(r4,#1)
}
loop_body:
{
  if (p0) r2 = add(r4,#0)
  r3 = add(r3,#32)
  r5 = memw(r3+#0)
}
{
  r4 = add(r2,#1)
  p0 = cmp.eq(r5,#1)
}:endloop0
after_loop:
{r18 = mux(p0,r4,r2)}
