// SPDX-License-Identifier: GPL-3.0-only
// Source-encoded reconstruction of a Hexagon V60 function, file
// [0x447388,0x447410),
// VMA [0x1e401388,0x1e401410), based on architectural instructions.
// Packet boundaries correspond to the actual program; immext is assembled
// from symbolic absolute operand constants, never from vendor opcode bytes.
.text
.globl modem_late_aux_find_controller
modem_late_aux_find_controller:
.set ext_1e4007a0, modem_late_aux_find_controller + (-3048)
.set ext_1e401324, modem_late_aux_find_controller + (-100)
{
  r17:16 = combine(r1,r0)
  memd(r29+#-16) = r17:16
  allocframe(#16)
}
{
  r2 = memw(r16+#32)
}
{
  p0 = cmp.gt(r2,#0)
  if (!p0.new) jump:nt .L1e4013c0
  if (p0.new) r3 = memw(r16+#28)
}
{
  loop0(.L1e4013a4,r2)
}
{
  r2 = add(r3,#8)
}
.L1e4013a4:
{
  r3 = memw(r2+#-8)
  if (cmp.gtu(r3.new,r17)) jump:t .L1e4013b8
}
{
  r4 = memw(r2+#4)
}
{
  r4 = add(r4,r3)
  if (!cmp.gtu(r17,r4.new)) jump:nt .L1e401400
}
.L1e4013b8:
{
  r2 = add(r2,#28)
  nop } 	:endloop0
.L1e4013c0:
{
  r4 = memw(##507617104)
  if (cmp.eq(r4.new,#0)) jump:nt .L1e4013e4
}
{
  r1:0 = combine(##507576347,#3)
  r2 = #947
}
{
  callr r4
  r3 = ##507576730
  memw(r29+#0) = r17
}
.L1e4013e4:
{
  call ext_1e4007a0
  r0 = ##507576730
  memw(r29+#0) = r17
}
{
  call ext_1e401324
  r0 = r16
}
{
  r0 = #0
  r17:16 = memd(r29+#8)
  dealloc_return
}
.L1e401400:
{
  r3 = sub(r17,r3)
  r2 = memw(r2+#0)
  r17:16 = memd(r29+#8)
}
{
  r0 = add(r2,r3)
  dealloc_return
}
