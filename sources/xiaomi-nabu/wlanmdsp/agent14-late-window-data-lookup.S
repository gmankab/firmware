// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 window-data-lookup: register and buffer-state operations.
// File [0x107438,0x107568); runtime [0xb00c7438,0xb00c7568).
// Encoded-byte equivalence is not a proof of hardware-level behavior.
.text
.globl agent14_late_window_data_lookup
agent14_late_window_data_lookup:
pkt_0000:
// offset +0x0000, 2 words
{ call 1958720; allocframe(#0x28) }
pkt_0008:
// offset +0x0008, 2 words
{ r17:16 = combine(r0,r1); r18 = r2; 	r3:2 = combine(#0,#0x0) }
pkt_0010:
// offset +0x0010, 3 words
{ r1 = #0x2412; r0 = ##0x6878; 	memd(r29+#0x10) = r3:2 }
pkt_001c:
// offset +0x001c, 1 words
{ memw(r29+#0x4) = r18; 	memw(r29+#0x0) = r16 }
pkt_0020:
// offset +0x0020, 3 words
{ call 1391964; memd(r29+#0x8) = r3:2; memb(r29+#0x16) = r16 }
pkt_002c:
// offset +0x002c, 4 words
{ call -807236; r1:0 = combine(r16,##-0x4fd48700); r2 = r18 }
pkt_003c:
// offset +0x003c, 1 words
{ p0 = cmp.eq(r18,#0x0); if (p0.new) jump:nt pkt_009c }
pkt_0040:
// offset +0x0040, 2 words
{ call -5016; r0 = r17; 	r1 = and(r16,#255) }
pkt_0048:
// offset +0x0048, 1 words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt pkt_0110 }
pkt_004c:
// offset +0x004c, 4 words
{ call 1391920; r0 = ##0x6880; r1 = #0x2410 }
pkt_005c:
// offset +0x005c, 3 words
{ call -807292; r0 = ##-0x4fd486f4 }
pkt_0068:
// offset +0x0068, 4 words
{ r1:0 = combine(#0x1,#-0x1); r6 = add(r29,#0x8); r17 = #0x4; r3 = memw(gp+#0x78c) }
pkt_0078:
// offset +0x0078, 3 words
{ r1:0 = combine(#0x13,#0x2); r2 = ##0xb00c5bac; 	memd(r29+#0x8) = r1:0 }
pkt_0084:
// offset +0x0084, 4 words
{ r4 = #0x0; r5 = add(r29,#0x8); memb(r6+#0xf) = #0x7; memw(r6+#0x8) = #0x1 }
pkt_0094:
// offset +0x0094, 1 words
{ call -739664 }
pkt_0098:
// offset +0x0098, 1 words
{ jump pkt_00c4 }
pkt_009c:
// offset +0x009c, 4 words
{ call 1391840; r0 = ##0x6890; r1 = #0x2410 }
pkt_00ac:
// offset +0x00ac, 3 words
{ call -807372; r0 = ##-0x4fd486dc }
pkt_00b8:
// offset +0x00b8, 3 words
{ call -739004; r17 = #0x1; r0 = #0x13; r1 = and(r16,#255) }
pkt_00c4:
// offset +0x00c4, 4 words
{ r1:0 = combine(#0x6,r16); r3:2 = combine(#0x0,#0xb); r5:4 = combine(#0x0,r17); memw(r29+#0x4) = #0x0 }
pkt_00d4:
// offset +0x00d4, 2 words
{ call -735472; memw(r29+#0x0) = #0x0 }
pkt_00dc:
// offset +0x00dc, 4 words
{ r1:0 = combine(#0xb,r16); r3:2 = combine(#0x2,#0xb); r5:4 = combine(#0x0,r17); memw(r29+#0x4) = #0x0 }
pkt_00ec:
// offset +0x00ec, 2 words
{ call -735496; memw(r29+#0x0) = #0x0 }
pkt_00f4:
// offset +0x00f4, 4 words
{ r1:0 = combine(#0x29,r16); r3:2 = combine(#0x1,#0xb); r5:4 = combine(#0x0,r17); memw(r29+#0x4) = #0x0 }
pkt_0104:
// offset +0x0104, 2 words
{ call -735520; memw(r29+#0x0) = #0x0 }
pkt_010c:
// offset +0x010c, 1 words
{ jump 1958672 }
pkt_0110:
// offset +0x0110, 4 words
{ call 1391724; r0 = ##0x6888; r1 = #0x2440 }
pkt_0120:
// offset +0x0120, 3 words
{ call -807488; r0 = ##-0x4fd486e8 }
pkt_012c:
// offset +0x012c, 1 words
{ jump 1958640 }
