// SPDX-License-Identifier: GPL-3.0-only
// Source-encoded reconstruction of a Hexagon V60 function, file
// [0x447234,0x4472d0),
// VMA [0x1e401234,0x1e4012d0), based on architectural instructions.
// Packet boundaries correspond to the actual program; immext is assembled
// from symbolic absolute operand constants, never from vendor opcode bytes.
.text
.globl modem_late_aux_insert_controller
modem_late_aux_insert_controller:
.set ext_1e4043a8, modem_late_aux_insert_controller + (12660)
.set ext_1e409b60, modem_late_aux_insert_controller + (35116)
{
  r16 = r0
  memd(r29+#-16) = r17:16
  allocframe(#16)
}
// Target uses two full-width instructions here; separate LLVM6 packets
// prevent its optimizer from using the shorter, non-equivalent-byte duplex.
{
  memd(r29+#0) = r19:18
}
{
  r17 = memw(r16+#4)
}
{
  r1:0 = combine(##507576461,r17)
  r18 = memw(r16+#28)
  memw(r16+#144) = r17
}
{
  r2 = memw(r16+#44)
}
{
  r3 = memw(r18+#4)
  memw(r16+#140) = r3.new
}
{
  call ext_1e409b60
  memw(r16+#152) = r16
  memw(r16+#148) = r2
}
{
  p0 = cmp.eq(r0,#0)
  if (p0.new) jump:nt .L1e40127c
  if (!p0.new) r2 = memw(r18+#4)
}
{
  call ext_1e4043a8
  r1:0 = combine(r17,#1)
}
.L1e40127c:
{
  r3 = ##507617056
  r2 = add(r16,#140)
}
{
  r4 = memw(r3+#0)
  if (cmp.eq(r4.new,#0)) jump:nt .L1e4012c4
}
{
  r3 = ##507617356
}
.L1e401298:
{
  r5 = r4
  r4 = memw(r4+#16)
  if (cmp.eq(r4.new,#0)) jump:nt .L1e4012ac
}
{
  p0 = !cmp.eq(r4,r3)
  if (p0.new) jump:t .L1e401298
}
.L1e4012ac:
{
  r3 = add(r5,#16)
  memw(r16+#160) = r5
  r4 = memw(r5+#16)
}
{
  p0 = cmp.eq(r4,#0)
  memw(r16+#156) = r4
  if (!p0.new) memw(r4+#20) = r2
}
.L1e4012c4:
// Preserve separate 32-bit store/load instructions before frame return.
{
  memw(r3+#0) = r2
}
{
  r17:16 = memd(r29+#8)
}
{
  r19:18 = memd(r29+#0)
  dealloc_return
}
