// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x107634,0x1076e4), runtime [0xb00c7634,0xb00c76e4).
// Format byte-indexed window list, emit normalized runtime command packet, then transfer control.
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_session_window_packet
agent14_late_session_window_packet:
pkt_0000:
// +0x0: 2 encoded word(s)
{ call 1958152; allocframe(#0x58) }
pkt_0008:
// +0x8: 4 encoded word(s)
{ r17 = r1; r19:18 = combine(r2,r4); r21:20 = combine(r0,r3); r23 = r29 }
pkt_0018:
// +0x18: 2 encoded word(s)
{ r24 = #0x0; r5 = memw(r17+#0x18) }
pkt_0020:
// +0x20: 1 encoded word(s)
{ r5 = add(r5,#0x7); 	r22 = memw(r17+#0x18) }
pkt_0024:
// +0x24: 2 encoded word(s)
{ r5 = and(r5,#-0x8); r2 = r22 }
pkt_002c:
// +0x2c: 2 encoded word(s)
{ r16 = sub(r29,r5); r29 = sub(r29,r5) }
pkt_0034:
// +0x34: 2 encoded word(s)
{ call -808640; r1:0 = combine(#0x0,r16) }
pkt_003c:
// +0x3c: 1 encoded word(s)
{ p0 = cmp.eq(r22,#0x0); if (p0.new) jump:nt pkt_0068 }
pkt_0040:
// +0x40: 3 encoded word(s)
{ loop0(pkt_004c,r22); r3:2 = combine(r16,##-0x7f7f7f7f) }
pkt_004c:
// +0x4c: 2 encoded word(s)
{ r1:0 = mpyu(r24,r2); r4 = add(r24,#0x1) }
pkt_0054:
// +0x54: 1 encoded word(s)
{ r1:0 = lsr(r1:0,#0x27) }
pkt_0058:
// +0x58: 2 encoded word(s)
{ r24 -= mpyi(r0,#0xff); memb(r3++#0x1) = r24.new }
pkt_0060:
// +0x60: 2 encoded word(s)
{ r24 = r4; nop }  :endloop0
pkt_0068:
// +0x68: 2 encoded word(s)
{ r2 = add(r30,#-0x58); r1 = #0x0 }
pkt_0070:
// +0x70: 3 encoded word(s)
{ call -808700; r0 = add(r2,#0x4); r2 = #0x14 }
pkt_007c:
// +0x7c: 4 encoded word(s)
{ r2 = add(r30,#-0x40); r0 = r17; r1 = add(r30,#-0x58); memw(r30+#-0x58) = r2.new }
pkt_008c:
// +0x8c: 3 encoded word(s)
{ r2 = #0x0; memw(r30+#-0x3c) = r21; memw(r30+#-0x30) = r19 }
pkt_0098:
// +0x98: 2 encoded word(s)
{ memw(r30+#-0x2c) = r20; memw(r30+#-0x34) = r18 }
pkt_00a0:
// +0xa0: 2 encoded word(s)
{ call -1652; memw(r30+#-0x4c) = r16 }
pkt_00a8:
// +0xa8: 2 encoded word(s)
{ r29 = r23; jump 1958204 }
