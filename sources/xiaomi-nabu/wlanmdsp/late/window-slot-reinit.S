// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x1076e4,0x107710), runtime [0xb00c76e4,0xb00c7710).
// Reset a session slot summary and confirm controller state.
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_window_slot_reinit
agent14_late_window_slot_reinit:
pkt_0000:
// +0x0: 3 encoded word(s)
{ r0 = r1; memw(r29+#-0x10) = r0; allocframe(#0x18) }
pkt_000c:
// +0xc: 3 encoded word(s)
{ r3 = add(r29,#0x0); r1 = add(r29,#0x0); r2 = #0x0; memw(r29+#0x14) = r2 }
pkt_0018:
// +0x18: 2 encoded word(s)
{ r3 = add(r29,#0xc); memw(r3+#0x8) = #0; 	memw(r3+#0x4) = #0 }
pkt_0020:
// +0x20: 2 encoded word(s)
{ call -1912; memw(r29+#0x0) = r3 }
pkt_0028:
// +0x28: 1 encoded word(s)
{ dealloc_return }
