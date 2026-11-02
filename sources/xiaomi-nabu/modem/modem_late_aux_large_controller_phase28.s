// SPDX-License-Identifier: GPL-3.0-only
// Selected controller result size, diagnostic callback and move to the next
// bounded indexed resource. Hexagon V60 source, no raw OEM opcodes.
.text
.globl modem_late_aux_large_controller_phase28
modem_late_aux_large_controller_phase28:
.set alternate_report, modem_late_aux_large_controller_phase28 + 0x30
.set next_record, modem_late_aux_large_controller_phase28 + 0x4c
.set reporting_callback, modem_late_aux_large_controller_phase28 + 0x818
.set alternate_callback, modem_late_aux_large_controller_phase28 + 0x3b00
{
  r2 = memw(r17+#28)
  r5 = memw(r29+#84)
}
{
  r3 = add(r2,mpyi(#28,r18))
  p0 = r5
}
{
  r2 = memw(r3+#4)
  r4 = memw(r3+#12)
}
{
  r3 = add(r2,r4)
  if (!p0) jump:nt alternate_report
}
{
  r2 = #447
  memw(r29+#4) = r3
  memw(r29+#0) = r2
}
{
  call reporting_callback
  r3 = ##507577659
  memw(r29+#8) = r4
}
{jump next_record}
alternate_report:
{
  r1:0 = combine(##507576841,#2)
  memw(r29+#4) = r3
  memw(r29+#0) = r2
}
{
  r2 = #447
  r3 = ##507577659
  memw(r29+#8) = r4
}
{call alternate_callback}
next_record:
{
  r18 = add(r18,#1)
  r4 = memuh(r21+#44)
}
