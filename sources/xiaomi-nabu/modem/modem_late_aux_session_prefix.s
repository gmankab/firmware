// SPDX-License-Identifier: GPL-3.0-only
// Source-level Hexagon V60 session-validation prologue, file
// [0x446e90,0x446ee8),
// VMA [0x1e400e90,0x1e400ee8), independently assembled.
// Partial function: checks format index and callback validity before the
// next instruction `r3 = framekey`, which the available LLVM6 assembler
// cannot encode. Stop here instead of inserting an unverified machine word.
// Packet boundaries correspond to the actual program; immext is assembled
// from symbolic absolute operand constants, never from vendor opcode bytes.
.text
.globl modem_late_aux_session_prefix
modem_late_aux_session_prefix:
.set ext_1e400420, modem_late_aux_session_prefix + (-2672)
.set ext_1e400ee8, modem_late_aux_session_prefix + (88)
.set ext_1e400fa8, modem_late_aux_session_prefix + (280)
.set ext_1e40101c, modem_late_aux_session_prefix + (396)
.set ext_1e403ba0, modem_late_aux_session_prefix + (11536)
{
  r16 = r1
  memd(r29+#-16) = r17:16
  allocframe(#48)
}
{
  r19 = r0
  r0 = r16
  memd(r29+#32) = r19:18
  memd(r29+#24) = r21:20
}
{
  call ext_1e403ba0
  r23:22 = bitsplit(r19,#8)
  memd(r29+#16) = r23:22
}
{
  r21 = ##16777215
  r17 = r0
  memw(r29+#12) = #0
}
{
  p0 = cmp.eq(r23,r21)
  if (p0.new) jump:nt .L1e400ee0
}
{
  p0 = cmp.eq(r23,#0)
  if (!p0.new) jump:nt .L1e400ecc
}
{
  p0 = cmp.eq(r22,#0)
  if (p0.new) jump:nt ext_1e400ee8
}
.L1e400ecc:
{
  call ext_1e400420
  r0 = r19
}
{
  r18 = r0
  if (!cmp.eq(r18.new,#0)) jump:t ext_1e400fa8
}
{
  r0 = #0
  jump ext_1e40101c
}
.L1e400ee0:
{
  r2 = add(r22,#-253)
  if (cmp.gtu(r2.new,#2)) jump:t .L1e400ecc
}
