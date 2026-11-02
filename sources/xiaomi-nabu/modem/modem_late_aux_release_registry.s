// SPDX-License-Identifier: GPL-3.0-only
// Source-encoded reconstruction of a Hexagon V60 function, file
// [0x447410,0x447448),
// VMA [0x1e401410,0x1e401448), based on architectural instructions.
// Packet boundaries correspond to the actual program; immext is assembled
// from symbolic absolute operand constants, never from vendor opcode bytes.
.text
.globl modem_late_aux_release_registry
modem_late_aux_release_registry:
.L1e401410:
{
  memd(r29+#-16) = r17:16
  allocframe(#8)
}
{
  r2 = memw(r0+#20)
  if (cmp.eq(r2.new,#0)) jump:nt .L1e401444
}
{
  r2 = add(r2,#-1)
  memw(r0+#20) = r2.new
}
{
  p0 = cmp.eq(r2,#0)
  if (!p0.new) jump:t .L1e401444
}
{
  r16 = memw(r0+#124)
  if (cmp.eq(r16.new,#0)) jump:nt .L1e401444
}
.L1e401430:
{
  r0 = memw(r16+#4)
}
{
  p0 = cmp.eq(r0,#0)
}
{
  if (!p0) call .L1e401410
}
{
  r16 = memw(r16+#0)
  if (!cmp.eq(r16.new,#0)) jump:t .L1e401430
}
.L1e401444:
{
  r17:16 = memd(r29+#0)
  dealloc_return
}
