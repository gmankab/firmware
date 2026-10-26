// SPDX-License-Identifier: GPL-3.0-only
// Reconstructed Hexagon V66 source, using mnemonic-level packet semantics.
// File [0x106f8c,0x107060); runtime [0xb00c6f8c,0xb00c7060).
// Find session record by identifier and call status hooks. Binary equivalence is not an on-device execution test.
.text
.globl agent14_late_window_record_lookup
agent14_late_window_record_lookup:
pkt_0000:
// offset +0x0000, 2 words
{ call 1959900; allocframe(#0x20) }
pkt_0008:
// offset +0x0008, 3 words
{ r18 = r0; r0 = ##0x6820; r20 = memw(r1+#0x0) }
pkt_0014:
// offset +0x0014, 1 words
{ r1 = #0x3e12 }
pkt_0018:
// offset +0x0018, 1 words
{ r17 = memw(r20+#0x4); 	r2 = memw(r20+#0x8) }
pkt_001c:
// offset +0x001c, 2 words
{ call 1393164; memw(r29+#0x4) = r2; 	memw(r29+#0x0) = r17 }
pkt_0024:
// offset +0x0024, 4 words
{ call -806032; r1:0 = combine(r17,##-0x4fd487d0); r2 = memw(r20+#0x8) }
pkt_0034:
// offset +0x0034, 2 words
{ jump pkt_0048; r16 = memw(r18+#0xc) }
pkt_003c:
// offset +0x003c, 2 words
{ r2 = memuh(r16+#0x0); if (cmp.eq(r2.new,r17)) jump:nt pkt_006c }
pkt_0044:
// offset +0x0044, 1 words
{ r16 = memw(r16+#0x4) }
pkt_0048:
// offset +0x0048, 1 words
{ p0 = cmp.eq(r16,#0x0); if (!p0.new) jump:t pkt_003c }
pkt_004c:
// offset +0x004c, 4 words
{ call 1393116; r0 = ##0x6828; r1 = #0x3e40 }
pkt_005c:
// offset +0x005c, 3 words
{ call 1959920; r0 = ##-0x4fd487c4 }
pkt_0068:
// offset +0x0068, 1 words
{ jump -806108 }
pkt_006c:
// offset +0x006c, 2 words
{ call -3864; r1:0 = combine(r17,r18) }
pkt_0074:
// offset +0x0074, 4 words
{ r19 = r0; r1:0 = combine(r17,r18); r2 = memw(r20+#0x8); memw(r16+#0x80) = r2.new }
pkt_0084:
// offset +0x0084, 1 words
{ call -3888 }
pkt_0088:
// offset +0x0088, 2 words
{ r17 = r0; p0 = cmp.eq(r19,#0x0); if (!p0.new) jump:nt pkt_00a0 }
pkt_0090:
// offset +0x0090, 2 words
{ r1:0 = combine(#0x1,r16); p1 = cmp.eq(r17,#0x0); if (p1.new) jump:nt pkt_00a0 }
pkt_0098:
// offset +0x0098, 1 words
{ call 1959860 }
pkt_009c:
// offset +0x009c, 1 words
{ jump -2688 }
pkt_00a0:
// offset +0x00a0, 1 words
{ if (p0) jump:nt pkt_00b4 }
pkt_00a4:
// offset +0x00a4, 2 words
{ r1:0 = combine(#0x0,r16); p0 = cmp.eq(r17,#0x0); if (!p0.new) jump:nt pkt_00b4 }
pkt_00ac:
// offset +0x00ac, 1 words
{ call 1959840 }
pkt_00b0:
// offset +0x00b0, 1 words
{ jump -2708 }
pkt_00b4:
// offset +0x00b4, 3 words
{ r1 = #0x3e11; r0 = ##0x6830; 	memw(r29+#0x0) = r17 }
pkt_00c0:
// offset +0x00c0, 1 words
{ call 1393000 }
pkt_00c4:
// offset +0x00c4, 3 words
{ call 1959816; r1:0 = combine(r17,##-0x4fd487b8) }
pkt_00d0:
// offset +0x00d0, 1 words
{ jump -803860 }
