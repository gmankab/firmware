// SPDX-License-Identifier: GPL-3.0-only
// Hexagon controller state machine, semantic instructions in original packet order.
.text
.globl wlan_controller_state_extended
agent13_session_state_prefix:
// offset 0x0
{ 	r2 = memw(r29+#12) }
// offset 0x4
{ 	p0 = r2
// offset 0x8
if (!p0.new) r1 = add(r18,#0)
// offset 0xc
if (p0.new) r20 = add(r26,#0)
// offset 0x10
if (p0.new) jump:nt 0x61c }
// offset 0x14
{ 	r2 = addasl(r26,r16,#2) }
// offset 0x18
{ 	r2 = memw(r2+#-4) }
// offset 0x1c
{ 	call 0x15f748
// offset 0x20
r0 = memw(r2+#36) }
// offset 0x24
{ 	jump 0x554 }
