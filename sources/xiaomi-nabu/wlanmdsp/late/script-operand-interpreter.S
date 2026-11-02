// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x105744,0x105bac); runtime [0xb00c5744,0xb00c5bac).
// Parser of variable-size script operands, opcode predicates and bounded hardware loops.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_script_operand_interpreter
agent14_late_script_operand_interpreter:
pkt_0000:
// offset +0x0000, 2 encoded words
{ call 1966044; allocframe(#0x78) }
pkt_0008:
// offset +0x0008, 3 encoded words
{ r17:16 = combine(r1,r2); r19:18 = combine(r4,r0); r1 = #0x0; 	r0 = add(r29,#0x8) }
pkt_0014:
// offset +0x0014, 3 encoded words
{ r21:20 = combine(r5,r3); r27 = #0x0; r2 = #0x38; 	r22 = add(r29,#0x8) }
pkt_0020:
// offset +0x0020, 1 encoded words
{ call -800700 }
pkt_0024:
// offset +0x0024, 3 encoded words
{ r2 = #0xf; r0 = #0x1; memw(r29+#0x34) = r17; 	memw(r29+#0x38) = r16 }
pkt_0030:
// offset +0x0030, 3 encoded words
{ p0 = cmp.gtu(r2,r19); if (p0.new) jump:t ##pkt_0454; memw(r29+#0x40) = r19; 	memw(r29+#0x44) = r21 }
pkt_003c:
// offset +0x003c, 3 encoded words
{ r1:0 = combine(#0,#0x0); r23 = add(r17,#0x1); r25:24 = combine(r17,#-0x5); r2 = memub(r20+#0xe) }
pkt_0048:
// offset +0x0048, 3 encoded words
{ r2 = and(#0x3c,asl(r2,#0x2)); r3 = and(r2,#0xf0); memd(r29+#0x0) = r1:0 }
pkt_0054:
// offset +0x0054, 2 encoded words
{ p0 = cmp.eq(r3,#0x40); if (p0.new) memw(r29+#0x3c) = r2 }
pkt_005c:
// offset +0x005c, 2 encoded words
{ p0 = cmp.eq(r27,r17); if (p0.new) jump:nt pkt_0458 }
pkt_0064:
// offset +0x0064, 2 encoded words
{ p0 = cmp.eq(r27,r23); if (p0.new) jump:nt pkt_0460 }
pkt_006c:
// offset +0x006c, 4 encoded words
{ p0 = !cmp.gtu(r17,r27); if (!p0.new) r4 = #0x0; if (p0.new) jump:nt pkt_0458; if (!p0.new) r5 = memub(r18+r27<<#0x0) }
pkt_007c:
// offset +0x007c, 2 encoded words
{ r3 = extractu(r5,#0x2,#0x1); r26 = add(r27,#0x1) }
pkt_0084:
// offset +0x0084, 2 encoded words
{ if (p0.new) r27 = #0x0; p0 = cmp.eq(r3,#0x0); if (p0.new) jump:nt pkt_00d4 }
pkt_008c:
// offset +0x008c, 1 encoded words
{ r0 = #0x1; 	r2 = add(r3,#-0x1) }
pkt_0090:
// offset +0x0090, 3 encoded words
{ r27 += asl(r0,r2); if (!cmp.gtu(r17,r27.new)) jump:nt ##pkt_0454 }
pkt_009c:
// offset +0x009c, 3 encoded words
{ p1 = cmp.gtu(r26,r27); if (!p1.new) r27 = #0x0; if (p1.new) jump:nt pkt_0454 }
pkt_00a8:
// offset +0x00a8, 1 encoded words
{ r2 = lsl(#0x1,r2) }
pkt_00ac:
// offset +0x00ac, 1 encoded words
{ loop0(pkt_00b0,r2) }
pkt_00b0:
// offset +0x00b0, 3 encoded words
{ r4 = r27; r26 = add(r26,#0x1); r27 = memub(r18+r26<<#0x0) }
pkt_00bc:
// offset +0x00bc, 2 encoded words
{ r27 = insert(r4,#0x18,#0x8); nop } :endloop0
pkt_00c4:
// offset +0x00c4, 1 encoded words
{ r2 = sub(#0x4,r2) }
pkt_00c8:
// offset +0x00c8, 1 encoded words
{ r2 = asl(r2,#0x3) }
pkt_00cc:
// offset +0x00cc, 1 encoded words
{ r4 = asl(r27,r2) }
pkt_00d0:
// offset +0x00d0, 1 encoded words
{ r4 = asr(r4,r2) }
pkt_00d4:
// offset +0x00d4, 1 encoded words
{ r6 = #-0x1 }
pkt_00d8:
// offset +0x00d8, 3 encoded words
{ r6 += lsr(r5,#0x3); if (cmp.gtu(r6.new,#0x16)) jump:nt ##pkt_0458 }
pkt_00e4:
// offset +0x00e4, 4 encoded words
{ r7 = and(r5,#0x1); r2 = add(r29,#0x0); r8 = memw(r6<<#0x2+##0xb02b7568) }
pkt_00f4:
// offset +0x00f4, 2 encoded words
{ r2 |= asl(r7,#0x2); r6 = lsr(r5,#0x3) }
pkt_00fc:
// offset +0x00fc, 1 encoded words
{ jumpr r8 }
pkt_0100:
// offset +0x0100, 3 encoded words
{ p0 = cmp.gtu(r5,#0x1f); if (!p0.new) r3 = #0x0; if (p0.new) r3 = memw(r29+#0x4) }
pkt_010c:
// offset +0x010c, 3 encoded words
{ r3 = add(r3,r27); if (!cmp.gtu(r19,r3.new)) jump:nt ##pkt_0458 }
pkt_0118:
// offset +0x0118, 2 encoded words
{ r4 = ##-0x8000000 }
pkt_0120:
// offset +0x0120, 1 encoded words
{ r4 += asl(r6,#0x1b) }
pkt_0124:
// offset +0x0124, 1 encoded words
{ r4 = asr(r4,#0x1b) }
pkt_0128:
// offset +0x0128, 2 encoded words
{ r4 = memw(r4<<#0x2+##0xb02b75e0) }
pkt_0130:
// offset +0x0130, 1 encoded words
{ r5 = r4 }
pkt_0134:
// offset +0x0134, 3 encoded words
{ r5 += add(r3,#-0x1); if (cmp.gtu(r3,r5.new)) jump:nt ##pkt_0458 }
pkt_0140:
// offset +0x0140, 4 encoded words
{ r3 = add(r20,r3); p0 = !cmp.gtu(r19,r5); if (!p0.new) r5 = #0x0; if (p0.new) jump:nt pkt_0458 }
pkt_0150:
// offset +0x0150, 1 encoded words
{ loop0(pkt_0154,r4) }
pkt_0154:
// offset +0x0154, 1 encoded words
{ r4 = memub(r3++#0x1) }
pkt_0158:
// offset +0x0158, 1 encoded words
{ r4 = insert(r5,#0x18,#0x8) }
pkt_015c:
// offset +0x015c, 2 encoded words
{ r5 = r4; nop } :endloop0
pkt_0164:
// offset +0x0164, 2 encoded words
{ jump pkt_0440; memw(r2+#0x0) = r4 }
pkt_016c:
// offset +0x016c, 1 encoded words
{ p1 = cmp.eq(r7,#0x0); if (!p1.new) jump:t pkt_01b8 }
pkt_0170:
// offset +0x0170, 3 encoded words
{ if (p0) r21 = #0x0; if (!p0) r4 = add(r26,#0x0); if (p0) jump:nt pkt_01bc }
pkt_017c:
// offset +0x017c, 2 encoded words
{ r0 = #0x1; r5 = add(r3,#-0x1); r7 = #-0x1 }
pkt_0184:
// offset +0x0184, 1 encoded words
{ r3 = lsl(#0x1,r5) }
pkt_0188:
// offset +0x0188, 2 encoded words
{ r4 += add(r3,#-0x1); if (!cmp.gtu(r17,r4.new)) jump:nt pkt_0454 }
pkt_0190:
// offset +0x0190, 1 encoded words
{ r7 += asl(r0,r5) }
pkt_0194:
// offset +0x0194, 2 encoded words
{ if (!p0.new) r4 = #0x0; p0 = cmp.gtu(r7,r4); if (p0.new) jump:nt pkt_0454 }
pkt_019c:
// offset +0x019c, 1 encoded words
{ loop0(pkt_01a0,r3) }
pkt_01a0:
// offset +0x01a0, 2 encoded words
{ r26 = add(r26,#0x1); r21 = memub(r18+r26<<#0x0) }
pkt_01a8:
// offset +0x01a8, 1 encoded words
{ r21 = insert(r4,#0x18,#0x8) }
pkt_01ac:
// offset +0x01ac, 2 encoded words
{ r4 = r21; nop } :endloop0
pkt_01b4:
// offset +0x01b4, 1 encoded words
{ jump pkt_01bc }
pkt_01b8:
// offset +0x01b8, 1 encoded words
{ r21 = memw(r29+#0x4) }
pkt_01bc:
// offset +0x01bc, 3 encoded words
{ r4 = #0x0; r3 = add(r6,#-0xf); if (cmp.gtu(r3.new,#0x5)) jump:nt pkt_0440 }
pkt_01c8:
// offset +0x01c8, 3 encoded words
{ r3 = r27; r5 = memw(r3<<#0x2+##0xb02b75c4) }
pkt_01d4:
// offset +0x01d4, 1 encoded words
{ jumpr r5 }
pkt_01d8:
// offset +0x01d8, 2 encoded words
{ r3 = #0x0; r4 = r27 }
pkt_01e0:
// offset +0x01e0, 1 encoded words
{ r2 = memw(r29+#0x0) }
pkt_01e4:
// offset +0x01e4, 2 encoded words
{ p0 = cmp.eq(r2,r21); if (!p0.new) r3 = add(r4,#0x0) }
pkt_01ec:
// offset +0x01ec, 2 encoded words
{ r26 = add(r3,r26); jump pkt_0440 }
pkt_01f4:
// offset +0x01f4, 2 encoded words
{ p0 = cmp.eq(r7,#0x0); if (!p0.new) r27 = memw(r29+#0x4) }
pkt_01fc:
// offset +0x01fc, 2 encoded words
{ jump pkt_0440; memw(r29+#0x0) += r27 }
pkt_0204:
// offset +0x0204, 3 encoded words
{ p0 = cmp.eq(r7,#0x0); r2 = memw(r29+#0x0); if (!p0.new) r27 = memw(r29+#0x4) }
pkt_0210:
// offset +0x0210, 3 encoded words
{ r2 = mpyi(r2,r27); jump pkt_0440; memw(r29+#0x0) = r2.new }
pkt_021c:
// offset +0x021c, 3 encoded words
{ p0 = cmp.eq(r7,#0x0); if (p0.new) r1 = add(r27,#0x0); if (!p0.new) r1 = memw(r29+#0x4) }
pkt_0228:
// offset +0x0228, 1 encoded words
{ p0 = cmp.eq(r1,#0x0); if (p0.new) jump:nt pkt_0458 }
pkt_022c:
// offset +0x022c, 2 encoded words
{ call -801216; r0 = memw(r29+#0x0) }
pkt_0234:
// offset +0x0234, 2 encoded words
{ jump pkt_0440; memw(r29+#0x0) = r0 }
pkt_023c:
// offset +0x023c, 2 encoded words
{ p0 = cmp.eq(r7,#0x0); if (!p0.new) r27 = memw(r29+#0x4) }
pkt_0244:
// offset +0x0244, 2 encoded words
{ jump pkt_0440; memw(r29+#0x0) &= r27 }
pkt_024c:
// offset +0x024c, 2 encoded words
{ p0 = cmp.eq(r7,#0x0); if (!p0.new) r27 = memw(r29+#0x4) }
pkt_0254:
// offset +0x0254, 2 encoded words
{ jump pkt_0440; memw(r29+#0x0) |= r27 }
pkt_025c:
// offset +0x025c, 3 encoded words
{ p0 = cmp.eq(r7,#0x0); r2 = memw(r29+#0x0); if (!p0.new) r4 = memw(r29+#0x4) }
pkt_0268:
// offset +0x0268, 3 encoded words
{ r4 = asl(r2,r4); r3 = sub(#0x0,r4); p0 = cmp.gt(r4,#0x0) }
pkt_0274:
// offset +0x0274, 1 encoded words
{ r2 = lsr(r2,r3) }
pkt_0278:
// offset +0x0278, 1 encoded words
{ if (!p0) r4 = add(r2,#0x0) }
pkt_027c:
// offset +0x027c, 2 encoded words
{ jump pkt_0440; memw(r29+#0x0) = r4 }
pkt_0284:
// offset +0x0284, 2 encoded words
{ r26 = add(r26,r27); jump pkt_0440 }
pkt_028c:
// offset +0x028c, 3 encoded words
{ p0 = cmp.gtu(r27,#0xf); if (p0.new) jump:t pkt_03bc; if (!p0.new) r3 = memw(r22+r27<<#0x2) }
pkt_0298:
// offset +0x0298, 2 encoded words
{ jump pkt_0440; memw(r2+#0x0) = r3 }
pkt_02a0:
// offset +0x02a0, 2 encoded words
{ r6 = togglebit(r7,#0x0); r5 = add(r29,#0x0); 	r3 = r16 }
pkt_02a8:
// offset +0x02a8, 1 encoded words
{ r5 |= asl(r6,#0x2) }
pkt_02ac:
// offset +0x02ac, 1 encoded words
{ r7 = memw(r5+#0x0) }
pkt_02b0:
// offset +0x02b0, 1 encoded words
{ r6 = add(r7,r4) }
pkt_02b4:
// offset +0x02b4, 1 encoded words
{ r3 &= asr(r6,#0x1f) }
pkt_02b8:
// offset +0x02b8, 1 encoded words
{ r5 = r3 }
pkt_02bc:
// offset +0x02bc, 2 encoded words
{ r5 += add(r7,r4); if (cmp.gtu(r5.new,r24)) jump:nt pkt_0458 }
pkt_02c4:
// offset +0x02c4, 2 encoded words
{ r6 += add(r3,#0x4); p0 = cmp.gtu(r17,r5); if (p0.new) jump:nt pkt_0458 }
pkt_02cc:
// offset +0x02cc, 3 encoded words
{ r3 = add(r18,r5); if (!p0.new) r5 = #0x0; p0 = cmp.gtu(r6,r16); if (p0.new) jump:nt pkt_0458 }
pkt_02d8:
// offset +0x02d8, 1 encoded words
{ loop0(pkt_02dc,#0x4) }
pkt_02dc:
// offset +0x02dc, 1 encoded words
{ r4 = memub(r3++#0x1) }
pkt_02e0:
// offset +0x02e0, 1 encoded words
{ r4 = insert(r5,#0x18,#0x8) }
pkt_02e4:
// offset +0x02e4, 2 encoded words
{ r5 = r4; nop } :endloop0
pkt_02ec:
// offset +0x02ec, 1 encoded words
{ jump pkt_0164 }
pkt_02f0:
// offset +0x02f0, 2 encoded words
{ r6 = togglebit(r7,#0x0); r5 = add(r29,#0x0); 	r3 = r16 }
pkt_02f8:
// offset +0x02f8, 1 encoded words
{ r5 |= asl(r6,#0x2) }
pkt_02fc:
// offset +0x02fc, 1 encoded words
{ r7 = memw(r5+#0x0) }
pkt_0300:
// offset +0x0300, 1 encoded words
{ r6 = add(r7,r4) }
pkt_0304:
// offset +0x0304, 1 encoded words
{ r3 &= asr(r6,#0x1f) }
pkt_0308:
// offset +0x0308, 1 encoded words
{ r5 = r3 }
pkt_030c:
// offset +0x030c, 2 encoded words
{ r5 += add(r7,r4); if (cmp.gtu(r5.new,r24)) jump:nt pkt_0458 }
pkt_0314:
// offset +0x0314, 2 encoded words
{ r6 += add(r3,#0x4); p0 = cmp.gtu(r17,r5); if (p0.new) jump:nt pkt_0458 }
pkt_031c:
// offset +0x031c, 2 encoded words
{ if (!p0.new) r3 = add(r18,r5); p0 = cmp.gtu(r6,r16); if (p0.new) jump:nt pkt_0458 }
pkt_0324:
// offset +0x0324, 2 encoded words
{ loop0(pkt_032c,#0x4); r2 = memw(r2+#0x0) }
pkt_032c:
// offset +0x032c, 3 encoded words
{ r4 = lsr(r2,#0x18); r2 = asl(r2,#0x8); memb(r3++#0x1) = r4.new } :endloop0
pkt_0338:
// offset +0x0338, 1 encoded words
{ jump pkt_0440 }
pkt_033c:
// offset +0x033c, 1 encoded words
{ r2 = memw(r29+#0x0) }
pkt_0340:
// offset +0x0340, 2 encoded words
{ p0 = cmp.gtu(r2,r21); jump pkt_0350 }
pkt_0348:
// offset +0x0348, 1 encoded words
{ r2 = memw(r29+#0x0) }
pkt_034c:
// offset +0x034c, 1 encoded words
{ p0 = cmp.gtu(r21,r2) }
pkt_0350:
// offset +0x0350, 2 encoded words
{ if (!p0) r27 = #0x0; jump pkt_0364 }
pkt_0358:
// offset +0x0358, 1 encoded words
{ r2 = memw(r29+#0x0) }
pkt_035c:
// offset +0x035c, 2 encoded words
{ p0 = bitsclr(r2,r21); if (p0.new) r27 = #0x0 }
pkt_0364:
// offset +0x0364, 2 encoded words
{ r26 = add(r27,r26); jump pkt_0440 }
pkt_036c:
// offset +0x036c, 1 encoded words
{ r4 = r26 }
pkt_0370:
// offset +0x0370, 2 encoded words
{ r4 += add(r21,#-0x1); if (!cmp.gtu(r17,r4.new)) jump:nt pkt_0458 }
pkt_0378:
// offset +0x0378, 2 encoded words
{ r3 = add(r21,#-0x1); if (cmp.gtu(r3.new,r4)) jump:nt pkt_0458 }
pkt_0380:
// offset +0x0380, 2 encoded words
{ r2 = memw(r2+#0x0); if (!cmp.gtu(r19,r2.new)) jump:nt pkt_0458 }
pkt_0388:
// offset +0x0388, 2 encoded words
{ r3 = add(r3,r2); if (cmp.gtu(r2,r3.new)) jump:nt pkt_0458 }
pkt_0390:
// offset +0x0390, 3 encoded words
{ p0 = !cmp.gtu(r19,r3); if (!p0.new) r0 = add(r18,r26); if (p0.new) jump:nt pkt_0458 }
pkt_039c:
// offset +0x039c, 3 encoded words
{ call -794980; r1 = add(r20,r2); r2 = r21 }
pkt_03a8:
// offset +0x03a8, 2 encoded words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) r27 = #0x0 }
pkt_03b0:
// offset +0x03b0, 1 encoded words
{ r27 += add(r26,r21) }
pkt_03b4:
// offset +0x03b4, 2 encoded words
{ r26 = r27; jump pkt_0440 }
pkt_03bc:
// offset +0x03bc, 3 encoded words
{ p0 = cmp.gtu(r27,#0x1f); if (p0.new) jump:t pkt_03d4; if (!p0.new) r2 = memw(r2+#0x0) }
pkt_03c8:
// offset +0x03c8, 1 encoded words
{ r3 = addasl(r22,r27,#0x2) }
pkt_03cc:
// offset +0x03cc, 2 encoded words
{ jump pkt_0440; memw(r3+#-0x40) = r2 }
pkt_03d4:
// offset +0x03d4, 3 encoded words
{ p0 = cmp.gt(r27,#0x21); if (p0.new) r3 = add(r29,#0x0); if (p0.new) jump:t pkt_0400 }
pkt_03e0:
// offset +0x03e0, 3 encoded words
{ p0 = cmp.eq(r27,#0x20); if (p0.new) jump:nt pkt_0420; if (p0.new) r3 = memw(r2+#0x0) }
pkt_03ec:
// offset +0x03ec, 3 encoded words
{ p0 = cmp.eq(r27,#0x21); if (!p0.new) jump:nt pkt_0458; if (p0.new) r3 = memw(r2+#0x0) }
pkt_03f8:
// offset +0x03f8, 2 encoded words
{ r3 = sub(#0x0,r3); jump pkt_0298 }
pkt_0400:
// offset +0x0400, 3 encoded words
{ r4 = togglebit(r7,#0x0); p0 = cmp.eq(r27,#0x22); if (p0.new) jump:nt pkt_0428 }
pkt_040c:
// offset +0x040c, 2 encoded words
{ p0 = cmp.eq(r27,#0x23); if (!p0.new) jump:nt pkt_0458 }
pkt_0414:
// offset +0x0414, 1 encoded words
{ r3 |= asl(r4,#0x2) }
pkt_0418:
// offset +0x0418, 2 encoded words
{ jump pkt_0298; r3 = memw(r3+#0x0) }
pkt_0420:
// offset +0x0420, 2 encoded words
{ r3 = sub(#-0x1,r3); jump pkt_0298 }
pkt_0428:
// offset +0x0428, 2 encoded words
{ r4 = togglebit(r7,#0x0); r3 = add(r29,#0x0); 	r5 = memw(r2+#0x0) }
pkt_0430:
// offset +0x0430, 1 encoded words
{ r3 |= asl(r4,#0x2) }
pkt_0434:
// offset +0x0434, 2 encoded words
{ r4 = memw(r3+#0x0); memw(r2+#0x0) = r4.new }
pkt_043c:
// offset +0x043c, 1 encoded words
{ memw(r3+#0x0) = r5 }
pkt_0440:
// offset +0x0440, 4 encoded words
{ p0 = cmp.eq(r25,#0x0); r25 = add(r25,#-0x1); r27 = r26; r0 = #0x1 }
pkt_0450:
// offset +0x0450, 1 encoded words
{ if (!p0) jump:nt pkt_005c }
pkt_0454:
// offset +0x0454, 1 encoded words
{ jump 1965156 }
pkt_0458:
// offset +0x0458, 2 encoded words
{ r0 = #0x1 ; jump ##1965152 }
pkt_0460:
// offset +0x0460, 2 encoded words
{ r0 = #0x0 ; jump ##1965144 }
