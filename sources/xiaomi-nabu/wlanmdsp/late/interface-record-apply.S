// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x1063f4,0x10656c); runtime [0xb00c63f4,0xb00c656c).
// Apply validated interface record and report failures.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_interface_record_apply
agent14_late_interface_record_apply:
pkt_0000:
// offset +0x0000, 2 encoded words
{ call 1962868; allocframe(#0x28) }
pkt_0008:
// offset +0x0008, 1 encoded words
{ r20 = r1; 	r18 = r0 }
pkt_000c:
// offset +0x000c, 1 encoded words
{ r21 = memw(r20+#0x0) }
pkt_0010:
// offset +0x0010, 1 encoded words
{ r16 = memw(r21+#0x4) }
pkt_0014:
// offset +0x0014, 2 encoded words
{ call -808; r1 = r16 }
pkt_001c:
// offset +0x001c, 2 encoded words
{ r19 = r0; r3 = memw(r21+#0x10); 	r2 = memw(r21+#0x18) }
pkt_0024:
// offset +0x0024, 2 encoded words
{ p0 = cmp.gtu(r2,r3); if (!p0.new) jump:t pkt_0058; r4 = memw(r21+#0x14) }
pkt_002c:
// offset +0x002c, 3 encoded words
{ r1 = #0x3e43; r0 = ##0x6780; 	memw(r29+#0x8) = r4 }
pkt_0038:
// offset +0x0038, 2 encoded words
{ call 1396104; memw(r29+#0x4) = r2; 	memw(r29+#0x0) = r3 }
pkt_0040:
// offset +0x0040, 3 encoded words
{ r0 = ##-0x4fd4893c; r1 = memw(r21+#0x10); 	r2 = memw(r21+#0x18) }
pkt_004c:
// offset +0x004c, 2 encoded words
{ call 1962904; r3 = memw(r21+#0x14) }
pkt_0054:
// offset +0x0054, 1 encoded words
{ jump -802612 }
pkt_0058:
// offset +0x0058, 2 encoded words
{ r5 = add(r4,r2); if (cmp.gtu(r5.new,r3)) jump:nt pkt_002c }
pkt_0060:
// offset +0x0060, 1 encoded words
{ p0 = cmp.gtu(r5,r4); if (!p0.new) jump:nt pkt_002c }
pkt_0064:
// offset +0x0064, 2 encoded words
{ jump pkt_0078; r17 = memw(r18+#0xc) }
pkt_006c:
// offset +0x006c, 2 encoded words
{ r5 = memuh(r17+#0x0); if (cmp.eq(r5.new,r16)) jump:nt pkt_0098 }
pkt_0074:
// offset +0x0074, 1 encoded words
{ r17 = memw(r17+#0x4) }
pkt_0078:
// offset +0x0078, 1 encoded words
{ p0 = cmp.eq(r17,#0x0); if (!p0.new) jump:t pkt_006c }
pkt_007c:
// offset +0x007c, 3 encoded words
{ r1 = #0x3e41; r0 = ##0x6788; 	memw(r29+#0x0) = r16 }
pkt_0088:
// offset +0x0088, 1 encoded words
{ call 1396024 }
pkt_008c:
// offset +0x008c, 3 encoded words
{ r0 = ##-0x4fd48930; jump pkt_00c0 }
pkt_0098:
// offset +0x0098, 2 encoded words
{ r5 = memw(r17+#0x78); if (cmp.eq(r5.new,#0x0)) jump:nt pkt_00a8 }
pkt_00a0:
// offset +0x00a0, 2 encoded words
{ r6 = memw(r17+#0x7c); if (!cmp.gtu(r3,r6.new)) jump:t pkt_00cc }
pkt_00a8:
// offset +0x00a8, 3 encoded words
{ r1 = #0x3e41; r0 = ##0x6790; 	memw(r29+#0x0) = r16 }
pkt_00b4:
// offset +0x00b4, 1 encoded words
{ call 1395980 }
pkt_00b8:
// offset +0x00b8, 2 encoded words
{ r0 = ##-0x4fd48924 }
pkt_00c0:
// offset +0x00c0, 2 encoded words
{ call 1962788; r1 = r16 }
pkt_00c8:
// offset +0x00c8, 1 encoded words
{ jump -800884 }
pkt_00cc:
// offset +0x00cc, 2 encoded words
{ p0 = cmp.eq(r4,#0x0); if (p0.new) jump:nt pkt_00f4; if (!p0.new) r6 = memuh(r17+#0x66) }
pkt_00d4:
// offset +0x00d4, 2 encoded words
{ if (p0.new) r0 = add(r5,r4); p0 = tstbit(r6,#0); if (!p0.new) jump:nt 164 }
pkt_00dc:
// offset +0x00dc, 2 encoded words
{ r6 = memw(r17+#0x70); if (!cmp.eq(r6.new,r4)) jump:nt 156 }
pkt_00e4:
// offset +0x00e4, 2 encoded words
{ r6 = memw(r17+#0x6c); if (cmp.gtu(r3,r6.new)) jump:nt 148 }
pkt_00ec:
// offset +0x00ec, 2 encoded words
{ jump pkt_0128; r1 = memw(r20+#0xc) }
pkt_00f4:
// offset +0x00f4, 2 encoded words
{ p0 = cmp.eq(r3,#0x0); if (p0.new) jump:nt pkt_0138; if (p0.new) r2 = memw(r17+#0x70) }
pkt_00fc:
// offset +0x00fc, 2 encoded words
{ r2 = memw(r21+#0xc); memw(r17+#0x68) = r2.new }
pkt_0104:
// offset +0x0104, 3 encoded words
{ call 1401080; r2 = memw(r21+#0x10); memw(r17+#0x6c) = r2.new }
pkt_0110:
// offset +0x0110, 2 encoded words
{ r3 = memw(r17+#0x78); memw(r17+#0x74) = r0 }
pkt_0118:
// offset +0x0118, 1 encoded words
{ memh(r17+#0x66) = setbit(#0x0) }
pkt_011c:
// offset +0x011c, 1 encoded words
{ r4 = memw(r21+#0x14); 	r1 = memw(r20+#0xc) }
pkt_0120:
// offset +0x0120, 2 encoded words
{ r0 = add(r3,r4); r2 = memw(r21+#0x18) }
pkt_0128:
// offset +0x0128, 1 encoded words
{ call -803820 }
pkt_012c:
// offset +0x012c, 1 encoded words
{ r2 = memw(r21+#0x14); 	r3 = memw(r21+#0x18) }
pkt_0130:
// offset +0x0130, 2 encoded words
{ r2 = add(r3,r2); memw(r17+#0x70) = r2.new }
pkt_0138:
// offset +0x0138, 2 encoded words
{ r3 = memw(r17+#0x6c); if (!cmp.eq(r3.new,r2)) jump:t pkt_0144 }
pkt_0140:
// offset +0x0140, 1 encoded words
{ memh(r17+#0x66) = clrbit(#0x0) }
pkt_0144:
// offset +0x0144, 2 encoded words
{ call -1112; r1:0 = combine(r16,r18) }
pkt_014c:
// offset +0x014c, 1 encoded words
{ p0 = cmp.eq(r19,#0x0); if (!p0.new) jump:nt pkt_0160 }
pkt_0150:
// offset +0x0150, 1 encoded words
{ p1 = cmp.eq(r0,#0x0); if (p1.new) jump:nt pkt_0160 }
pkt_0154:
// offset +0x0154, 2 encoded words
{ call 1962640; r1:0 = combine(#0x1,r17) }
pkt_015c:
// offset +0x015c, 1 encoded words
{ jump 88 }
pkt_0160:
// offset +0x0160, 1 encoded words
{ if (p0) jump:nt pkt_0174 }
pkt_0164:
// offset +0x0164, 1 encoded words
{ p0 = cmp.eq(r0,#0x0); if (!p0.new) jump:nt pkt_0174 }
pkt_0168:
// offset +0x0168, 2 encoded words
{ call 1962620; r1:0 = combine(#0x0,r17) }
pkt_0170:
// offset +0x0170, 1 encoded words
{ jump 68 }
pkt_0174:
// offset +0x0174, 1 encoded words
{ jump 1962716 }
