// SPDX-License-Identifier: GPL-3.0-only
// Reconstructed Hexagon V66 source, using mnemonic-level packet semantics.
// File [0x107060,0x1071a8); runtime [0xb00c7060,0xb00c71a8).
// Validate buffered record bounds and publish completion. Binary equivalence is not an on-device execution test.
.text
.globl agent14_late_window_record_validate
agent14_late_window_record_validate:
pkt_0000:
// offset +0x0000, 2 words
{ call 1959704; allocframe(#0x20) }
pkt_0008:
// offset +0x0008, 3 words
{ r0 = ##0x6838; r18 = r0; r17 = r1 }
pkt_0014:
// offset +0x0014, 2 words
{ r1 = #0x3e14; r19 = memw(r17+#0x0) }
pkt_001c:
// offset +0x001c, 1 words
{ r2 = memw(r19+#0x10); 	r16 = memw(r19+#0x4) }
pkt_0020:
// offset +0x0020, 1 words
{ r3 = memw(r19+#0x14); 	r4 = memw(r19+#0xc) }
pkt_0024:
// offset +0x0024, 1 words
{ memw(r29+#0xc) = r4; 	memw(r29+#0x8) = r3 }
pkt_0028:
// offset +0x0028, 2 words
{ call 1392940; memw(r29+#0x4) = r2; 	memw(r29+#0x0) = r16 }
pkt_0030:
// offset +0x0030, 3 words
{ r1:0 = combine(#0x4,##0xb02b7854); r2 = memw(r19+#0x10); 	r4 = memw(r19+#0xc) }
pkt_003c:
// offset +0x003c, 1 words
{ r3 = memw(r19+#0x14); 	memw(r29+#0xc) = r4 }
pkt_0040:
// offset +0x0040, 1 words
{ memw(r29+#0x8) = r3; 	memw(r29+#0x4) = r2 }
pkt_0044:
// offset +0x0044, 2 words
{ call -806292; memw(r29+#0x0) = r16 }
pkt_004c:
// offset +0x004c, 2 words
{ jump pkt_0060; r18 = memw(r18+#0xc) }
pkt_0054:
// offset +0x0054, 2 words
{ r2 = memuh(r18+#0x0); if (cmp.eq(r2.new,r16)) jump:nt pkt_0084 }
pkt_005c:
// offset +0x005c, 1 words
{ r18 = memw(r18+#0x4) }
pkt_0060:
// offset +0x0060, 1 words
{ p0 = cmp.eq(r18,#0x0); if (!p0.new) jump:t pkt_0054 }
pkt_0064:
// offset +0x0064, 3 words
{ r1 = #0x3e41; r0 = ##0x6840; 	memw(r29+#0x0) = r16 }
pkt_0070:
// offset +0x0070, 1 words
{ call 1392868 }
pkt_0074:
// offset +0x0074, 3 words
{ call 1959704; r1:0 = combine(r16,##-0x4fd487a0) }
pkt_0080:
// offset +0x0080, 1 words
{ jump -803992 }
pkt_0084:
// offset +0x0084, 2 words
{ r2 = memw(r18+#0x80); if (!cmp.eq(r2.new,#0x0)) jump:nt pkt_011c }
pkt_008c:
// offset +0x008c, 1 words
{ r2 = memw(r19+#0x14); 	r3 = memw(r19+#0x10) }
pkt_0090:
// offset +0x0090, 2 words
{ r4 = memw(r18+#0x7c); if (!cmp.gtu(r2,r4.new)) jump:t pkt_00c8 }
pkt_0098:
// offset +0x0098, 3 words
{ r1 = #0x3e43; r0 = ##0x6850; 	memw(r29+#0x8) = r4 }
pkt_00a4:
// offset +0x00a4, 2 words
{ call 1392816; memw(r29+#0x4) = r2; 	memw(r29+#0x0) = r3 }
pkt_00ac:
// offset +0x00ac, 4 words
{ r0 = ##-0x4fd48788; r1 = memw(r19+#0x10); r3 = memw(r18+#0x7c) }
pkt_00bc:
// offset +0x00bc, 2 words
{ call 1959632; r2 = memw(r19+#0x14) }
pkt_00c4:
// offset +0x00c4, 1 words
{ jump -805904 }
pkt_00c8:
// offset +0x00c8, 2 words
{ r5 = add(r3,r2); if (cmp.gtu(r5.new,r4)) jump:nt pkt_0098 }
pkt_00d0:
// offset +0x00d0, 1 words
{ p0 = cmp.gtu(r5,r3); if (!p0.new) jump:nt pkt_0098 }
pkt_00d4:
// offset +0x00d4, 2 words
{ r1 = memw(r17+#0xc); if (cmp.eq(r1.new,#0x0)) jump:nt pkt_00fc }
pkt_00dc:
// offset +0x00dc, 1 words
{ r4 = memw(r18+#0x78) }
pkt_00e0:
// offset +0x00e0, 2 words
{ call -806928; r0 = add(r4,r3) }
pkt_00e8:
// offset +0x00e8, 3 words
{ call 1397928; r2 = memw(r19+#0xc); memw(r18+#0x6c) = r2.new }
pkt_00f4:
// offset +0x00f4, 2 words
{ jump 1959680; memw(r18+#0x74) = r0 }
pkt_00fc:
// offset +0x00fc, 4 words
{ call 1392728; r0 = ##0x6858; r1 = #0x3e50 }
pkt_010c:
// offset +0x010c, 3 words
{ call 1959552; r0 = ##-0x4fd4877c }
pkt_0118:
// offset +0x0118, 1 words
{ jump -806496 }
pkt_011c:
// offset +0x011c, 3 words
{ r1 = #0x3e41; r0 = ##0x6848; 	memw(r29+#0x0) = r16 }
pkt_0128:
// offset +0x0128, 1 words
{ call 1392684 }
pkt_012c:
// offset +0x012c, 3 words
{ call -804164; r1:0 = combine(r16,##-0x4fd48794) }
pkt_0138:
// offset +0x0138, 4 words
{ call -3072; r0 = ##-0x4fd488df; r1 = #0x2d8 }
