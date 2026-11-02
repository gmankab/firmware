// SPDX-License-Identifier: GPL-3.0-only
// Source-encoded reconstruction of a Hexagon V60 function, file
// [0x4472d0,0x447324),
// VMA [0x1e4012d0,0x1e401324), based on architectural instructions.
// Packet boundaries correspond to the actual program; immext is assembled
// from symbolic absolute operand constants, never from vendor opcode bytes.
.text
.globl modem_late_aux_remove_controller
modem_late_aux_remove_controller:
.set ext_1e4043a8, modem_late_aux_remove_controller + (12504)
{
  r16 = r0
  r0 = #0
  memd(r29+#-16) = r17:16
  allocframe(#8)
}
{
  r2 = memw(r16+#28)
  r1 = memw(r16+#4)
}
{
  call ext_1e4043a8
  r2 = memw(r2+#4)
}
{
  r3 = memw(r16+#160)
  r2 = memw(r16+#156)
}
{
  p0 = cmp.eq(r3,#0)
  if (!p0.new) jump:t .L1e40130c
  if (p0.new) memw(##507617056) = r2
}
{
  p0 = cmp.eq(r2,#0)
  if (p0.new) jump:nt .L1e401320
}
// Keep the original two 32-bit instructions instead of LLVM's compact
// load/store duplex. The checked emitter joins their packet parse fields.
{
  memw(r2+#20) = #0
}
{
  r17:16 = memd(r29+#0)
}
{
  dealloc_return
}
.L1e40130c:
{
  p0 = cmp.eq(r2,#0)
  memw(r3+#16) = r2
  if (!p0.new) r2 = memw(r16+#156)
}
{
  if (!p0) r3 = memw(r16+#160)
  if (!p0) memw(r2+#20) = r3.new
}
.L1e401320:
{
  r17:16 = memd(r29+#0)
  dealloc_return
}
