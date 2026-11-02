// SPDX-License-Identifier: GPL-3.0-only
// Reconstructed Hexagon V66 source, using mnemonic-level packet semantics.
// File [0x106e8c,0x106f8c); runtime [0xb00c6e8c,0xb00c6f8c).
// Update normalized stream state and report status. Binary equivalence is not an on-device execution test.
.text
.globl agent14_late_window_state_update
agent14_late_window_state_update:
pkt_0000:
// offset +0x0000, 2 words
{ call 1960172; allocframe(#0x18) }
pkt_0008:
// offset +0x0008, 2 words
{ call 1398620; r16 = r0 }
pkt_0010:
// offset +0x0010, 4 words
{ r19:18 = combine(r1,r0); r0 = ##0x6810; r2 = memw(r16+#0x44) }
pkt_0020:
// offset +0x0020, 4 words
{ call 1393416; r1 = #0x2451; r2 = sub(r18,r2); memw(r29+#0x0) = r2.new }
pkt_0030:
// offset +0x0030, 3 words
{ r0 = ##-0x4fd487e8; r2 = memw(r16+#0x44) }
pkt_003c:
// offset +0x003c, 2 words
{ call -803456; r1 = sub(r18,r2) }
pkt_0044:
// offset +0x0044, 3 words
{ r1:0 = combine(#0x0,#0x0); r3 = #0x177; r2 = memw(r16+#0x44) }
pkt_0050:
// offset +0x0050, 1 words
{ r2 = sub(r18,r2) }
pkt_0054:
// offset +0x0054, 2 words
{ r2 = lsr(r2,#0x3); if (cmp.gtu(r3,r2.new)) jump:t pkt_00ac }
pkt_005c:
// offset +0x005c, 2 words
{ memw(r16+#0x44) = #0x0; memd(r16+#0x38) = r1:0 }
pkt_0064:
// offset +0x0064, 1 words
{ call 1398528 }
pkt_0068:
// offset +0x0068, 4 words
{ call -504; r0 = r16; memw(r16+#0x40) = r0; memw(r16+#0x48) = #0x0 }
pkt_0078:
// offset +0x0078, 3 words
{ call -444; r0 = r16; memw(r16+#0x4c) = r0 }
pkt_0084:
// offset +0x0084, 3 words
{ r17 = #0x3e8; r2 = memub(r16+#0x16); memw(r16+#0x50) = r0 }
pkt_0090:
// offset +0x0090, 4 words
{ p0 = tstbit(r2,#0x1); if (p0.new) jump:t pkt_00c4; if (!p0.new) r0 = memub(r16+#0x0); memw(r16+#0x54) = #0x1 }
pkt_00a0:
// offset +0x00a0, 1 words
{ call -1384 }
pkt_00a4:
// offset +0x00a4, 2 words
{ jump pkt_00c4; memb(r16+#0x16) = setbit(#0x1) }
pkt_00ac:
// offset +0x00ac, 2 words
{ r3 = #0x3e9; r2 = memw(r16+#0x48) }
pkt_00b4:
// offset +0x00b4, 2 words
{ r17 = sub(##0x3e8,r2) }
pkt_00bc:
// offset +0x00bc, 2 words
{ p0 = cmp.gtu(r3,r17); if (!p0.new) r17 = #0x3e8 }
pkt_00c4:
// offset +0x00c4, 3 words
{ r1 = #0x2451; r0 = ##0x6818; 	memw(r29+#0x0) = r17 }
pkt_00d0:
// offset +0x00d0, 1 words
{ call 1393240 }
pkt_00d4:
// offset +0x00d4, 4 words
{ call -803608; r1:0 = combine(r17,##-0x4fd487dc); r16 = add(r16,#0x18) }
pkt_00e4:
// offset +0x00e4, 2 words
{ call 1395180; r0 = r16 }
pkt_00ec:
// offset +0x00ec, 2 words
{ r2 = #0x3e8; r4 = #0x0; 	r0 = r16 }
pkt_00f4:
// offset +0x00f4, 2 words
{ r3:2 = mpyu(r17,r2); call 1960044 }
pkt_00fc:
// offset +0x00fc, 1 words
{ jump 1395032 }
