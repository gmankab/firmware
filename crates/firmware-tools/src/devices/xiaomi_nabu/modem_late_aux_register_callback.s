// SPDX-License-Identifier: GPL-3.0-only
// Source-encoded reconstruction of a Hexagon V60 function, file
// [0x447324,0x447388),
// VMA [0x1e401324,0x1e401388), based on architectural instructions.
// Packet boundaries correspond to the actual program; immext is assembled
// from symbolic absolute operand constants, never from vendor opcode bytes.
.text
.globl modem_late_aux_register_callback
modem_late_aux_register_callback:
.set ext_1e405df8, modem_late_aux_register_callback + (19156)
{
  r16 = ##507614240
  memd(r29+#-16) = r17:16
  allocframe(#8)
}
{
  r2 = r0
  r3 = memw(r16+#0)
  if (!cmp.gt(r3.new,#0)) jump:nt .L1e401364
}
// LLVM6 otherwise merges the r6 extended assignment with the following
// r4 load as an unrelated duplex.  Two source packets retain all three
// architecture instructions; the checked emitter joins the scalar r6's
// packet into the following r5/r4 duplex (parse marker only).
{
  r6 = ##507612164
}
{
  r5 = #0
  r4 = memw(r2+#4)
}
{
  jump .L1e401358
}
.L1e40134c:
{
  r6 = add(r6,#208)
  r5 = add(r5,#1)
  if (!cmp.gt(r3,r5.new)) jump:nt .L1e401364
}
.L1e401358:
{
  r7 = memw(r6+#0)
  if (cmp.eq(r7.new,r4)) jump:nt .L1e401380
}
{
  jump .L1e40134c
}
.L1e401364:
{
  r4 = #208
  r1 = #208
}
{
  r0 = add(##507612160,mpyi(r3,r4))
  call ext_1e405df8
  r3 = #208
}
{
  memw(r16+#0) += #1
}
.L1e401380:
{
  r17:16 = memd(r29+#0)
  dealloc_return
}
{
  nop
}
