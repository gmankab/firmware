// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x105bac,0x105e70); runtime [0xb00c5bac,0xb00c5e70).
// Record validation, session lookup and status callback dispatch.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_record_validate_dispatch
agent14_late_record_validate_dispatch:
pkt_0000:
// offset +0x0000, 2 encoded words
{ call 1964944; allocframe(#0x30) }
pkt_0008:
// offset +0x0008, 4 encoded words
{ r19:18 = combine(r5,r1); r21:20 = combine(#0x0,r0); r17 = r4; if (cmp.eq(r17.new,#0x0)) jump:nt pkt_015c }
pkt_0018:
// offset +0x0018, 3 encoded words
{ r1 = r18; r2 = memw(gp+#0x78c); r0 = memw(r20+#0x3a8) }
pkt_0024:
// offset +0x0024, 1 encoded words
{ r2 = addasl(r2,r18,#0x2) }
pkt_0028:
// offset +0x0028, 2 encoded words
{ call 2468; r22 = memw(r2+#0x54) }
pkt_0030:
// offset +0x0030, 2 encoded words
{ r16 = r0; if (cmp.eq(r16.new,#0x0)) jump:nt pkt_015c }
pkt_0038:
// offset +0x0038, 1 encoded words
{ r23 = memw(r29+#0x38) }
pkt_003c:
// offset +0x003c, 2 encoded words
{ r2 = memub(r23+#0x6); if (cmp.eq(r2.new,#0x2)) jump:nt pkt_0058 }
pkt_0044:
// offset +0x0044, 3 encoded words
{ if (!p0.new) r2 = add(r22,##0x7ec); p0 = cmp.eq(r2,#0x4); if (!p0.new) jump:nt pkt_0064 }
pkt_0050:
// offset +0x0050, 2 encoded words
{ r2 = add(r22,#0x7d4); jump pkt_005c }
pkt_0058:
// offset +0x0058, 1 encoded words
{ r2 = add(r22,#0x7e0) }
pkt_005c:
// offset +0x005c, 2 encoded words
{ jump pkt_0074; memw(r2+#0x0) += #0x1 }
pkt_0064:
// offset +0x0064, 1 encoded words
{ memw(r2+#0x0) += #0x1 }
pkt_0068:
// offset +0x0068, 2 encoded words
{ r2 = memw(r16+#0x54); if (cmp.eq(r2.new,#0x0)) jump:nt pkt_0074 }
pkt_0070:
// offset +0x0070, 1 encoded words
{ memw(r16+#0x3c) += #0x1 }
pkt_0074:
// offset +0x0074, 1 encoded words
{ call 296000 }
pkt_0078:
// offset +0x0078, 2 encoded words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt pkt_00d4; if (p0.new) r2 = memub(r16+#0x16) }
pkt_0080:
// offset +0x0080, 4 encoded words
{ call pkt_017c; r1 = r16; r3:2 = combine(r19,r17); r24 = add(r22,#0x798) }
pkt_0090:
// offset +0x0090, 2 encoded words
{ r21 = r0; if (cmp.eq(r21.new,#0x1)) jump:nt pkt_00fc }
pkt_0098:
// offset +0x0098, 1 encoded words
{ p0 = cmp.eq(r21,#0x2); if (!p0.new) jump:nt pkt_0128 }
pkt_009c:
// offset +0x009c, 3 encoded words
{ r2 = #0x6; r3 = memub(r23+#0x6); if (cmp.eq(r3.new,#0x2)) jump:t pkt_00b0 }
pkt_00a8:
// offset +0x00a8, 2 encoded words
{ r2 = #0x5; p0 = cmp.eq(r3,#0x4); if (!p0.new) jump:nt pkt_0164 }
pkt_00b0:
// offset +0x00b0, 1 encoded words
{ r2 = add(r24,mpyi(#0xc,r2)) }
pkt_00b4:
// offset +0x00b4, 1 encoded words
{ memw(r2+#0x4) += #0x1 }
pkt_00b8:
// offset +0x00b8, 4 encoded words
{ r1 = #0x27; r3:2 = combine(r19,r17); r5:4 = combine(r18,#0x0); r0 = memw(r20+#0x244) }
pkt_00c8:
// offset +0x00c8, 2 encoded words
{ call 295932; r21 = #0x2 }
pkt_00d0:
// offset +0x00d0, 1 encoded words
{ jump pkt_0128 }
pkt_00d4:
// offset +0x00d4, 1 encoded words
{ p0 = tstbit(r2,#0); if (!p0.new) jump:t pkt_00e4 }
pkt_00d8:
// offset +0x00d8, 2 encoded words
{ r3 = memub(r23+#0x6); if (cmp.eq(r3.new,#0x2)) jump:nt pkt_0080 }
pkt_00e0:
// offset +0x00e0, 1 encoded words
{ p0 = cmp.eq(r3,#0x4); if (p0.new) jump:t pkt_0080 }
pkt_00e4:
// offset +0x00e4, 3 encoded words
{ p0 = !tstbit(r2,#0x1); r21 = #0x0; if (p0.new) jump:t pkt_0128 }
pkt_00f0:
// offset +0x00f0, 2 encoded words
{ r2 = memb(r23+#0x6); if (cmp.eq(r2.new,#0x1)) jump:t pkt_0080 }
pkt_00f8:
// offset +0x00f8, 1 encoded words
{ jump pkt_0128 }
pkt_00fc:
// offset +0x00fc, 3 encoded words
{ r21 = #0x1; r2 = memub(r23+#0x6); if (cmp.eq(r2.new,#0x2)) jump:nt pkt_011c }
pkt_0108:
// offset +0x0108, 1 encoded words
{ p0 = cmp.eq(r2,#0x4) }
pkt_010c:
// offset +0x010c, 1 encoded words
{ r2 = mux(p0,#0x5,#0x7) }
pkt_0110:
// offset +0x0110, 1 encoded words
{ r2 = add(r24,mpyi(#0xc,r2)) }
pkt_0114:
// offset +0x0114, 2 encoded words
{ jump pkt_0128; memw(r2+#0x8) += #0x1 }
pkt_011c:
// offset +0x011c, 2 encoded words
{ r21 = #0x1; r2 = add(r22,#0x7e8) }
pkt_0124:
// offset +0x0124, 1 encoded words
{ memw(r2+#0x0) += #0x1 }
pkt_0128:
// offset +0x0128, 2 encoded words
{ r2 = memw(r16+#0x54); if (cmp.eq(r2.new,#0x0)) jump:nt pkt_015c }
pkt_0130:
// offset +0x0130, 3 encoded words
{ r1 = #0x2452; r0 = ##0x6738; 	r2 = memw(r16+#0x3c) }
pkt_013c:
// offset +0x013c, 2 encoded words
{ r3 = memw(r16+#0x38); memw(r29+#0x4) = r3.new }
pkt_0144:
// offset +0x0144, 2 encoded words
{ call 1397956; memw(r29+#0x0) = r2 }
pkt_014c:
// offset +0x014c, 3 encoded words
{ r0 = ##-0x4fd489ec; r1 = memw(r16+#0x3c); 	r2 = memw(r16+#0x38) }
pkt_0158:
// offset +0x0158, 1 encoded words
{ call -801252 }
pkt_015c:
// offset +0x015c, 2 encoded words
{ r0 = r21 ; jump ##1964816 }
pkt_0164:
// offset +0x0164, 1 encoded words
{ r2 = add(r22,#0x7f0) }
pkt_0168:
// offset +0x0168, 1 encoded words
{ memw(r2+#0x0) += #0x1 }
pkt_016c:
// offset +0x016c, 2 encoded words
{ r2 = memw(r16+#0x54); if (cmp.eq(r2.new,#0x0)) jump:nt pkt_00b8 }
pkt_0174:
// offset +0x0174, 2 encoded words
{ jump pkt_00b8; memw(r16+#0x38) += #0x1 }
pkt_017c:
// offset +0x017c, 2 encoded words
{ call 1964588; allocframe(#0x38) }
pkt_0184:
// offset +0x0184, 2 encoded words
{ r17 = r1; r21:20 = combine(r2,r3) }
pkt_018c:
// offset +0x018c, 2 encoded words
{ r18 = memw(r17+#0x6c); r16 = memuh(r17+#0x0) }
pkt_0194:
// offset +0x0194, 2 encoded words
{ p0 = cmp.eq(r18,#0x0); if (p0.new) jump:nt pkt_022c; r19 = memw(r17+#0x78) }
pkt_019c:
// offset +0x019c, 3 encoded words
{ if (p0.new) r19 = #0x0; p0 = cmp.eq(r19,#0x0); if (p0.new) jump:nt pkt_022c; if (!p0.new) r2 = memuh(r17+#0x66) }
pkt_01a8:
// offset +0x01a8, 1 encoded words
{ p0 = tstbit(r2,#0); if (p0.new) jump:t pkt_022c }
pkt_01ac:
// offset +0x01ac, 2 encoded words
{ r2 = memw(r17+#0x90); if (!cmp.eq(r2.new,#0x0)) jump:t pkt_022c }
pkt_01b4:
// offset +0x01b4, 3 encoded words
{ call 1403024; r22 = memw(r17+#0x7c); memw(r17+#0xc) += #0x1 }
pkt_01c0:
// offset +0x01c0, 3 encoded words
{ r3 = #0x0; r2 = memw(r17+#0x74); r7:6 = memd(gp+#0xe78) }
pkt_01cc:
// offset +0x01cc, 1 encoded words
{ r1:0 = sub(r1:0,r3:2) }
pkt_01d0:
// offset +0x01d0, 1 encoded words
{ r5:4 = combine(r0,r1) }
pkt_01d4:
// offset +0x01d4, 1 encoded words
{ r5:4 = vlsrw(r5:4,#0x3) }
pkt_01d8:
// offset +0x01d8, 1 encoded words
{ r5 = insert(r1,#0x3,#0x1d) }
pkt_01dc:
// offset +0x01dc, 2 encoded words
{ r1:0 = mpyu(r5,r6); r9:8 = mpyu(r5,r7) }
pkt_01e4:
// offset +0x01e4, 1 encoded words
{ r2 = r1 }
pkt_01e8:
// offset +0x01e8, 2 encoded words
{ r1:0 = combine(r3,r2); r2 = r8 }
pkt_01f0:
// offset +0x01f0, 1 encoded words
{ r1:0 += mpyu(r4,r6) }
pkt_01f4:
// offset +0x01f4, 1 encoded words
{ r1:0 = add(r1:0,r3:2) }
pkt_01f8:
// offset +0x01f8, 2 encoded words
{ r2 = r1; r1:0 = combine(r18,r19) }
pkt_0200:
// offset +0x0200, 2 encoded words
{ r13:12 = combine(r3,r2); r2 = r9 }
pkt_0208:
// offset +0x0208, 1 encoded words
{ r13:12 += mpyu(r4,r7) }
pkt_020c:
// offset +0x020c, 2 encoded words
{ r7:6 = add(r13:12,r3:2); r3:2 = combine(r21,r22) }
pkt_0214:
// offset +0x0214, 1 encoded words
{ r7:6 = lsr(r7:6,#0x4) }
pkt_0218:
// offset +0x0218, 2 encoded words
{ call -1664; r5:4 = combine(r6,r20) }
pkt_0220:
// offset +0x0220, 2 encoded words
{ r0 = #0x2; p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt pkt_0288 }
pkt_0228:
// offset +0x0228, 1 encoded words
{ jump 1964636 }
pkt_022c:
// offset +0x022c, 3 encoded words
{ r1 = #0x3e45; r3 = memuh(r17+#0x66); r2 = memw(r17+#0x90) }
pkt_0238:
// offset +0x0238, 2 encoded words
{ r0 = and(r3,#0x1); memw(r29+#0x10) = r0.new }
pkt_0240:
// offset +0x0240, 3 encoded words
{ r0 = ##0x6750; memw(r29+#0xc) = r2; 	memw(r29+#0x8) = r19 }
pkt_024c:
// offset +0x024c, 2 encoded words
{ call 1397692; memw(r29+#0x4) = r18; 	memw(r29+#0x0) = r16 }
pkt_0254:
// offset +0x0254, 4 encoded words
{ r1:0 = combine(#0x5,##0xb02b767c); r4 = memuh(r17+#0x66); r2 = memw(r17+#0x6c) }
pkt_0264:
// offset +0x0264, 3 encoded words
{ r4 = and(r4,#0x1); r3 = memw(r17+#0x78); r5 = memw(r17+#0x90) }
pkt_0270:
// offset +0x0270, 1 encoded words
{ memw(r29+#0x10) = r4; 	memw(r29+#0xc) = r5 }
pkt_0274:
// offset +0x0274, 1 encoded words
{ memw(r29+#0x8) = r3; 	memw(r29+#0x4) = r2 }
pkt_0278:
// offset +0x0278, 2 encoded words
{ call -801556; memw(r29+#0x0) = r16 }
pkt_0280:
// offset +0x0280, 2 encoded words
{ r0 = #0x0 ; jump ##1964548 }
pkt_0288:
// offset +0x0288, 3 encoded words
{ r1 = #0x3e13; r0 = ##0x6758; 	r2 = memw(r17+#0x10) }
pkt_0294:
// offset +0x0294, 3 encoded words
{ r2 = add(r2,#0x1); r3 = memw(r17+#0xc); memw(r17+#0x10) = r2.new }
pkt_02a0:
// offset +0x02a0, 1 encoded words
{ memw(r29+#0x4) = r3; 	memw(r29+#0x8) = r2 }
pkt_02a4:
// offset +0x02a4, 2 encoded words
{ call 1397604; memw(r29+#0x0) = r16 }
pkt_02ac:
// offset +0x02ac, 3 encoded words
{ r1:0 = combine(r16,##-0x4fd48978); r2 = memw(r17+#0xc); 	r3 = memw(r17+#0x10) }
pkt_02b8:
// offset +0x02b8, 1 encoded words
{ call -801104 }
pkt_02bc:
// offset +0x02bc, 2 encoded words
{ r0 = #0x1 ; jump ##1964488 }
