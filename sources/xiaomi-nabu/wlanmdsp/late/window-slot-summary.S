// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x107598,0x1075c4), runtime [0xb00c7598,0xb00c75c4).
// Initialize a slot summary and call the existing session queue helper.
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_window_slot_summary
agent14_late_window_slot_summary:
pkt_0000:
// +0x0: 3 encoded word(s)
{ r0 = r1; memw(r29+#-0x14) = r0; allocframe(#0x20) }
pkt_000c:
// +0xc: 3 encoded word(s)
{ r4 = add(r29,#0x4); r1 = add(r29,#0x4); r2 = #0x0; memw(r29+#0x18) = r2 }
pkt_0018:
// +0x18: 2 encoded word(s)
{ r6 = add(r29,#0x10); memw(r4+#0x8) = #0; 	memw(r4+#0x4) = #0 }
pkt_0020:
// +0x20: 2 encoded word(s)
{ call -3048; memw(r29+#0x4) = r6; 	memw(r29+#0x1c) = r3 }
pkt_0028:
// +0x28: 1 encoded word(s)
{ dealloc_return }
