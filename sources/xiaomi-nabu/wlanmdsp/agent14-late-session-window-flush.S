// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x1075c4,0x107634), runtime [0xb00c75c4,0xb00c7634).
// Initialize descriptor snapshots, publish controller status, then tail-call timer acknowledgment.
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_session_window_flush
agent14_late_session_window_flush:
pkt_0000:
// +0x0: 2 encoded word(s)
{ call 1958324; allocframe(#0x38) }
pkt_0008:
// +0x8: 3 encoded word(s)
{ r4 = add(r29,#0xc); r0 = #0x1; r6 = add(r29,#0x18); memw(r29+#0x1c) = r0 }
pkt_0014:
// +0x14: 3 encoded word(s)
{ r16 = r1; r18 = memw(gp+#0x78c); memw(r4+#0x8) = #0x0 }
pkt_0020:
// +0x20: 1 encoded word(s)
{ memw(r4+#0x4) = #0; 	memw(r29+#0xc) = r6 }
pkt_0024:
// +0x24: 2 encoded word(s)
{ call 1062776; memw(r29+#0x20) = r2; 	memw(r29+#0x24) = r3 }
pkt_002c:
// +0x2c: 2 encoded word(s)
{ call 1400984; r17 = r0 }
pkt_0034:
// +0x34: 4 encoded word(s)
{ r2 = r0; r1:0 = combine(#0x43,r17); r5:4 = combine(#0x0,#0x0); memw(r29+#0x4) = #0x0 }
pkt_0044:
// +0x44: 2 encoded word(s)
{ call 1404236; memw(r29+#0x0) = #0x0 }
pkt_004c:
// +0x4c: 2 encoded word(s)
{ call -742216; r0 = add(r18,#0x1020) }
pkt_0054:
// +0x54: 3 encoded word(s)
{ call -812; r2 = #0x0; r1 = add(r29,#0xc); r0 = r16 }
pkt_0060:
// +0x60: 3 encoded word(s)
{ call -742244; r0 = add(r18,#0x1030); r1 = #0x1 }
pkt_006c:
// +0x6c: 1 encoded word(s)
{ jump 1958436 }
