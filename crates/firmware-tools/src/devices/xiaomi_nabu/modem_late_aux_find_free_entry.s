// SPDX-License-Identifier: GPL-3.0-only
// Source-assembled Hexagon V60 free-slot registry search, file
// [0x447028,0x447078),
// VMA [0x1e401028,0x1e401078). Search a linked controller registry
// with hardware loop0 packets, checking start/length and returning the
// selected object or null. This is a complete callable function.
// Packet boundaries correspond to the actual program; immext is assembled
// from symbolic absolute operand constants, never from vendor opcode bytes.
.text
.globl modem_late_aux_find_free_entry
modem_late_aux_find_free_entry:
{
  r0 = #0
  r2 = r0
}
{
  r3 = memw(##507617068)
  if (cmp.eq(r3.new,#0)) jump:nt .L1e40106c
}
.L1e401038:
{
  r5 = memw(r3+#32)
}
{
  p0 = cmp.gt(r5,#0)
  if (!p0.new) jump:nt .L1e401064
  if (p0.new) r4 = memw(r3+#28)
}
{
  loop0(.L1e401048,r5)
}
.L1e401048:
{
  r5 = memw(r4+#4)
  if (!cmp.gtu(r2,r5.new)) jump:t .L1e40105c
}
{
  r6 = memw(r4+#12)
}
{
  r5 = add(r5,r6)
  if (cmp.gtu(r5.new,r2)) jump:nt .L1e401070
}
.L1e40105c:
{
  r4 = add(r4,#28)
  nop } 	:endloop0
.L1e401064:
{
  r3 = memw(r3+#0)
  if (!cmp.eq(r3.new,#0)) jump:t .L1e401038
}
.L1e40106c:
{
  jumpr r31
}
.L1e401070:
{
  r0 = r3
  jumpr r31
}
{
  nop
}
