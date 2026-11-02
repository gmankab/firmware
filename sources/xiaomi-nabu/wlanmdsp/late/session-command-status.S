// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x107c34,0x107c8c), runtime [0xb00c7c34,0xb00c7c8c).
// Publish status using a validated descriptor and forward the result to the status handler.
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_session_command_status
agent14_late_session_command_status:
pkt_0000:
// +0x0: 2 encoded word(s)
{ call 1956660; allocframe(#0x30) }
pkt_0008:
// +0x8: 4 encoded word(s)
{ r19:18 = combine(r1,r0); r17:16 = combine(r3,r2); r0 = add(r29,#0x3); r1 = #0x0 }
pkt_0018:
// +0x18: 2 encoded word(s)
{ call -810148; r2 = #0x15; 	r20 = memw(r18+#0x8) }
pkt_0020:
// +0x20: 4 encoded word(s)
{ r1:0 = combine(#0x18,##0xae8); r2 = memub(r20+#0x3); memb(r29+#0x0) = r2.new }
pkt_0030:
// +0x30: 2 encoded word(s)
{ r2 = memw(r18+#0xb8); memb(r29+#0x1) = r2.new }
pkt_0038:
// +0x38: 3 encoded word(s)
{ r2 = add(r29,#0x0); r5:4 = memd(r18+#0x0); memb(r29+#0x2) = r19 }
pkt_0044:
// +0x44: 2 encoded word(s)
{ r3 = memw(r18+#0xc0); memd(r29+#0x8) = r5:4 }
pkt_004c:
// +0x4c: 2 encoded word(s)
{ call 1387752; memw(r29+#0x4) = r3; 	memd(r29+#0x10) = r17:16 }
pkt_0054:
// +0x54: 1 encoded word(s)
{ jump 1956796 }
