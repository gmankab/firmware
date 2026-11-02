// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 controller buffer scanner, [0x447ee0,0x447f38).
.text
.globl modem_late_aux_large_controller_phase17
modem_late_aux_large_controller_phase17:
{
  if (!p0.new) r6 = #0
  if (!p0.new) r5 = add(r21,r22)
  p0 = cmp.eq(r4,#0)
  if (!p0.new) jump:t begin_loop
}
{
  r1:0 = combine(#-1,#-1)
  r3:2 = combine(#0,#0)
  jump loop_complete
}
begin_loop:
{
  loop0(inner_scan,r4)
  r9:8 = combine(#-1,#-1)
  r3:2 = combine(#0,#0)
  r13 = r6
}
inner_scan:
{
  r4 = memw(r5+#0)
  if (cmp.eq(r4.new,#1)) jump:t found_active_entry
}
{
  r1:0 = combine(r9,r8)
  jump next_entry
}
found_active_entry:
{
  r1 = #0
  r0 = memw(r5+#8)
  r4 = memw(r5+#20)
}
{
  p0 = cmp.gtu(r9:8,r1:0)
  r12 = add(r4,r0)
}
{
  r3:2 = maxu(r3:2,r13:12)
  if (!p0) r1:0 = combine(r9,r8)
}
next_entry:
{
  r9:8 = combine(r1,r0)
  r5 = add(r5,#32)
}:endloop0
loop_complete:
