// SPDX-License-Identifier: GPL-3.0-only
// Source-assembled Hexagon V60 controller handle resolver, file
// [0x447078,0x447100),
// VMA [0x1e401078,0x1e401100). Locate the indexed controller
// record via hardware loop0, perform guarded diagnostics when missing
// and return the resolved address through original frame restoration.
// Packet boundaries correspond to the actual program; immext is assembled
// from symbolic absolute operand constants, never from vendor opcode bytes.
.text
.globl modem_late_aux_resolve_handle
modem_late_aux_resolve_handle:
.set ext_1e4007a0, modem_late_aux_resolve_handle + (-2264)
.set ext_1e401324, modem_late_aux_resolve_handle + (684)
{
  r17:16 = combine(r1,r0)
  memd(r29+#-16) = r17:16
  allocframe(#16)
}
{
  r3 = memw(r16+#32)
}
{
  p0 = cmp.gt(r3,#0)
  if (!p0.new) jump:nt .L1e4010ac
  if (p0.new) r2 = memw(r16+#28)
}
{
  loop0(.L1e401090,r3)
}
.L1e401090:
{
  r3 = memw(r2+#0)
  if (cmp.gtu(r3.new,r17)) jump:t .L1e4010a4
}
{
  r4 = memw(r2+#12)
}
{
  r4 = add(r4,r3)
  if (!cmp.gtu(r17,r4.new)) jump:nt .L1e4010ec
}
.L1e4010a4:
{
  r2 = add(r2,#28)
  nop } 	:endloop0
.L1e4010ac:
{
  r4 = memw(##507617104)
  if (cmp.eq(r4.new,#0)) jump:nt .L1e4010d0
}
{
  r1:0 = combine(##507576347,#3)
  r2 = #923
}
{
  callr r4
  r3 = ##507576730
  memw(r29+#0) = r17
}
.L1e4010d0:
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
.L1e4010ec:
{
  r3 = sub(r17,r3)
  r2 = memw(r2+#4)
  r17:16 = memd(r29+#8)
}
{
  r0 = add(r2,r3)
  dealloc_return
}
{
  nop
}
