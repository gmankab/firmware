// SPDX-License-Identifier: GPL-3.0-only
// Source-level Hexagon V60 diagnostic and guarded callback helpers, file
// [0x446e1c,0x446e90),
// VMA [0x1e400e1c,0x1e400e90), independently assembled.
// The first helper serializes diagnostics in a temporary stack buffer;
// the second invokes an installed callback only when its argument and
// selector pass the required checks.
// Packet boundaries correspond to the actual program; immext is assembled
// from symbolic absolute operand constants, never from vendor opcode bytes.
.text
.globl modem_late_aux_diagnostic_helpers
modem_late_aux_diagnostic_helpers:
.set ext_1e405dd0, modem_late_aux_diagnostic_helpers + (20404)
.set ext_1e40a5c8, modem_late_aux_diagnostic_helpers + (38828)
{
  nop
}
{
  r16 = r2
  r2 = r3
  memd(r29+#-16) = r17:16
  allocframe(#272)
}
{
  r0 = add(r29,#8)
  r1 = #256
  r3 = add(r29,#280)
  r4 = add(r29,#280)
}
{
  call ext_1e40a5c8
  memw(r29+#4) = r4
}
{
  r3:2 = combine(r16,##507576347)
  r1 = #2
  r0 = add(r29,#8)
}
{
  call ext_1e405dd0
}
{
  r17:16 = memd(r29+#264)
  dealloc_return
}
.L1e400e60:
{
  memd(r29+#-16) = r17:16
  allocframe(#8)
}
{
  r16 = r0
  if (cmp.eq(r16.new,#0)) jump:nt .L1e400e8c
}
{
  call .L1e400e60
  r0 = memw(r16+#0)
}
{
  r2 = memw(r16+#128)
  if (cmp.eq(r2.new,#0)) jump:nt .L1e400e8c
}
{
  r3 = memw(r16+#16)
  if (cmp.eq(r3.new,#1)) jump:t .L1e400e8c
}
{
  callr r2
}
{
  r17:16 = memd(r29+#0)
  dealloc_return
}
.L1e400e8c:
{
  r17:16 = memd(r29+#0)
  dealloc_return
}
