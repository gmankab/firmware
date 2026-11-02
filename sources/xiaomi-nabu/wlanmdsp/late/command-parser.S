// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x102368,0x1026bc), runtime [0xb00c2368,0xb00c26bc).
// Parse the codec-facing WLAN status/command message, check class and subtype
// fields, traverse bounded context slots, and invoke dispatch/error hooks.
// The last words form a separate tail call rather than an invented return.
// Explicit far jump extenders preserve the Hexagon V66 packet encoding.
// Calls to inherited routines retain numeric relative targets because their
// implementations are outside this isolated generated source fragment.
.text
.globl agent14_late_command_parser
agent14_late_command_parser:
// +0x0000: 2 encoded words
{ call 1979348; allocframe(#0x48) }
// +0x0008: 1 encoded words
{ r17:16 = combine(r2,r1) }
// +0x000c: 3 encoded words
{ if (p0.new) r21 = ##0xffff; p0 = cmp.eq(r16,#0x0); 	r2 = memw(r17+#0x0) }
// +0x0018: 1 encoded words
{ r2 = memw(r2+#0x10) }
// +0x001c: 2 encoded words
{ r3 = memub(r2+#0x5b); r4 = memub(r2+#0x5a) }
// +0x0024: 4 encoded words
{ r4 |= asl(r3,#0x8); r2 = p0; r5 = memub(r2+#0x59); r20 = memub(r2+#0x58) }
// +0x0034: 2 encoded words
{ r20 |= asl(r5,#0x8); memw(r29+#0x14) = r2 }
// +0x003c: 2 encoded words
{ r20 |= asl(r4,#0x10); if (p0) jump:nt label_00a0 }
// +0x0044: 3 encoded words
{ r1 = #0x3e11; r0 = ##0x65c8; 	r2 = memub(r20+#0x0) }
// +0x0050: 2 encoded words
{ call 1412604; memw(r29+#0x0) = r2 }
// +0x0058: 3 encoded words
{ call -784248; r0 = ##0xb02b71b4; 	r1 = memub(r20+#0x0) }
// +0x0064: 3 encoded words
{ r0 = #0x2; r21 = ##0xffff; r3 = memub(r20+#0x0) }
// +0x0070: 3 encoded words
{ r24 = #0x23; r2 = add(r3,#0x60); r18 = #0x0; 	r22 = #0x0 }
// +0x007c: 1 encoded words
{ r2 = and(r2,#0xf0) }
// +0x0080: 1 encoded words
{ r2 = lsr(r2,#0x4) }
// +0x0084: 2 encoded words
{ r2 = insert(r3,#0x4,#0x4); if (cmp.gt(r2.new,#0x6)) jump:t label_0108 }
// +0x008c: 3 encoded words
{ r24 = #0x3; if (p0.new) r0 = #0x8; p0 = cmp.eq(r2,#0x0); if (!p0.new) jump:t label_0118 }
// +0x0098: 2 encoded words
{ r24 = #0x4; jump label_011c }
label_00a0:
// +0x00a0: 2 encoded words
{ r23 = memw(r0+#0x140); if (cmp.eq(r23.new,#0x0)) jump:nt label_0130 }
// +0x00a8: 4 encoded words
{ r24 = add(r0,#0x54); r25 = #0x0; r2 = #0x0; r18 = add(r20,#0x4) }
// +0x00b8: 1 encoded words
{ r19 = add(r20,#0x10) }
label_00bc:
// +0x00bc: 2 encoded words
{ r22 = memw(r24+r2<<#0x2); if (cmp.eq(r22.new,#0x0)) jump:nt label_00f8 }
// +0x00c4: 1 encoded words
{ r2 = memuh(r22+#0x1c) }
// +0x00c8: 2 encoded words
{ if (p0.new) r1 = add(r22,#0x0); p0 = tstbit(r2,#0); if (!p0.new) jump:t label_00f8 }
// +0x00d0: 2 encoded words
{ r2 = memub(r1++#0x4); if (cmp.eq(r2.new,#0x1)) jump:nt label_00ec }
// +0x00d8: 1 encoded words
{ p0 = cmp.eq(r2,#0x2); if (!p0.new) jump:nt label_00f8 }
// +0x00dc: 2 encoded words
{ call -781000; r2 = #0x6; 	r0 = r19 }
// +0x00e4: 1 encoded words
{ p0 = cmp.eq(r0,#0x0); if (!p0.new) jump:t label_00f8 }
// +0x00e8: 1 encoded words
{ jump label_0288 }
label_00ec:
// +0x00ec: 2 encoded words
{ call -781016; r2 = #0x6; 	r0 = r18 }
// +0x00f4: 1 encoded words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt label_0260 }
label_00f8:
// +0x00f8: 1 encoded words
{ r25 = add(r25,#0x1) }
// +0x00fc: 2 encoded words
{ r2 = and(r25,#0xff); if (cmp.gtu(r23,r2.new)) jump:t label_00bc }
// +0x0104: 1 encoded words
{ r18 = #0x0 ; jump label_0134 }
label_0108:
// +0x0108: 2 encoded words
{ if (!p0.new) r24 = #0x25; p0 = cmp.eq(r2,#0x7); if (p0.new) jump:t label_011c }
// +0x0110: 2 encoded words
{ p0 = cmp.eq(r2,#0x9); if (!p0.new) jump:nt label_0134; jump label_011c }
label_0118:
// +0x0118: 1 encoded words
{ p0 = cmp.eq(r2,#0x2); if (!p0.new) jump:nt label_0134 }
label_011c:
// +0x011c: 1 encoded words
{ call 310664 }
// +0x0120: 2 encoded words
{ if (!p0.new) r21 = add(r24,#0x0); p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt label_0130 }
// +0x0128: 2 encoded words
{ jump label_013c; r19:18 = combine(#0,#0x0); 	r23:22 = combine(#1,#0x0) }
label_0130:
// +0x0130: 1 encoded words
{ r18 = #0x0; 	r22 = #0x0 }
label_0134:
// +0x0134: 1 encoded words
{ r19 = #0x0 }
label_0138:
// +0x0138: 1 encoded words
{ r23 = #0x0 }
label_013c:
// +0x013c: 1 encoded words
{ r2 = memub(r20+#0x0) }
// +0x0140: 3 encoded words
{ p0 = cmp.eq(r2,#0xd0); if (!p0.new) jump:t label_015c; if (p0.new) r3 = memw(r29+#0x14) }
// +0x014c: 2 encoded words
{ p0 = r3; if (!p0.new) jump:nt label_019c }
// +0x0154: 1 encoded words
{ p0 = cmp.eq(r18,#0x0); if (!p0.new) jump:nt label_019c }
// +0x0158: 1 encoded words
{ p0 = cmp.eq(r19,#0x0); if (!p0.new) jump:nt label_019c }
label_015c:
// +0x015c: 2 encoded words
{ r19 = r21; p0 = cmp.eq(r23,#0x0); if (p0.new) jump:nt label_01d4 }
label_0164:
// +0x0164: 1 encoded words
{ r2 = memw(r29+#0x14) }
// +0x0168: 3 encoded words
{ p0 = r2; if (!p0.new) r0 = add(r16,#0x0); if (p0.new) jump:nt label_017c }
// +0x0174: 1 encoded words
{ call 636624 }
// +0x0178: 1 encoded words
{ r22 = memw(r0+#0x18) }
label_017c:
// +0x017c: 2 encoded words
{ r2 = #0xec; p0 = cmp.eq(r22,#0x0); if (p0.new) jump:nt label_02f8 }
// +0x0184: 3 encoded words
{ r1:0 = combine(r19,#0); r3:2 = combine(#0x0,#0x0); r4 = #0x0; r5 = memub(r22+#0x3) }
// +0x0190: 1 encoded words
{ call 310176 }
// +0x0194: 2 encoded words
{ r0 = #0x2 ; jump ##1979164 }
label_019c:
// +0x019c: 1 encoded words
{ r3 = memw(r17+#0x0) }
// +0x01a0: 1 encoded words
{ r3 = memw(r3+#0x10) }
// +0x01a4: 2 encoded words
{ r4 = memub(r3+#0x5d); r3 = memub(r3+#0x5c) }
// +0x01ac: 1 encoded words
{ r3 |= asl(r4,#0x8) }
// +0x01b0: 2 encoded words
{ r3 = add(r3,#-0x4); if (cmp.gtu(r3.new,#0x19)) jump:t label_01dc }
// +0x01b8: 4 encoded words
{ call 1412244; r0 = ##0x65d8; r1 = #0x3e50 }
// +0x01c8: 3 encoded words
{ call -786968; r0 = ##-0x4fd48e34 }
label_01d4:
// +0x01d4: 2 encoded words
{ r0 = #0x1 ; jump ##1979100 }
label_01dc:
// +0x01dc: 4 encoded words
{ r0 = ##0x65e0; r1 = #0x3e53; r18 = memub(r20+#0x18) }
// +0x01ec: 2 encoded words
{ r17 = memub(r20+#0x19); memw(r29+#0x8) = r17.new }
// +0x01f4: 2 encoded words
{ call 1412184; memw(r29+#0x4) = r18; 	memw(r29+#0x0) = r2 }
// +0x01fc: 3 encoded words
{ r3:2 = combine(r17,r18); r0 = ##0xb02b71d8; 	r1 = memub(r20+#0x0) }
// +0x0208: 1 encoded words
{ call -786524 }
// +0x020c: 1 encoded words
{ r2 = memw(r29+#0x14) }
// +0x0210: 3 encoded words
{ p0 = r2; if (!p0.new) r0 = add(r16,#0x0); if (p0.new) jump:nt label_022c }
// +0x021c: 1 encoded words
{ call 636456 }
// +0x0220: 3 encoded words
{ r2 = #0xd5; r22 = memw(r0+#0x18); if (cmp.eq(r22.new,#0x0)) jump:nt label_02f8 }
label_022c:
// +0x022c: 2 encoded words
{ r3 = and(r18,#0x1f); r19 = #0x26; 	r2 = r18 }
// +0x0234: 1 encoded words
{ r2 = and(#0x1c,lsr(r2,#0x3)) }
// +0x0238: 1 encoded words
{ r2 = add(r22,r2) }
// +0x023c: 1 encoded words
{ r2 = memw(r2+#0x5b0) }
// +0x0240: 2 encoded words
{ p0 = !tstbit(r2,r3); if (p0.new) jump:nt label_015c }
// +0x0248: 2 encoded words
{ r2 = addasl(r22,r18,#0x2); p0 = cmp.gtu(r17,#0x1f); if (p0.new) jump:t label_0164 }
// +0x0250: 1 encoded words
{ r2 = memw(r2+#0x808) }
// +0x0254: 2 encoded words
{ p0 = !tstbit(r2,r17); if (!p0.new) jump:t label_015c }
// +0x025c: 1 encoded words
{ jump label_0164 }
label_0260:
// +0x0260: 3 encoded words
{ r19:18 = combine(#0x1,#0x0); r2 = memub(r20+#0x0); if (cmp.eq(r2.new,#0x10)) jump:nt label_02cc }
// +0x026c: 3 encoded words
{ p0 = cmp.eq(r2,#0xb0); if (p0.new) r0 = #0xc; if (!p0.new) jump:nt label_0138 }
// +0x0278: 1 encoded words
{ call 310316 }
// +0x027c: 3 encoded words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt label_0138; jump label_02dc; if (!p0.new) r21 = #0xf }
label_0288:
// +0x0288: 3 encoded words
{ r1 = #0x3e11; r0 = ##0x65d0; 	r2 = memub(r20+#0x0) }
// +0x0294: 2 encoded words
{ call 1412024; memw(r29+#0x0) = r2 }
// +0x029c: 3 encoded words
{ call -784828; r0 = ##0xb02b71c0; 	r1 = memub(r20+#0x0) }
// +0x02a8: 1 encoded words
{ r19:18 = combine(#0,#0x1); 	r2 = memub(r20+#0x0) }
// +0x02ac: 2 encoded words
{ p0 = cmp.eq(r2,#0xb0); if (!p0.new) jump:t label_02bc }
// +0x02b4: 2 encoded words
{ r23 = #0xf ; jump label_02e8; r0 = #0xc }
label_02bc:
// +0x02bc: 2 encoded words
{ p0 = cmp.eq(r2,#0x20); if (!p0.new) jump:t label_02e0 }
// +0x02c4: 2 encoded words
{ r23 = #0x24 ; jump label_02e8; r0 = #0x2 }
label_02cc:
// +0x02cc: 2 encoded words
{ call 310232; r0 = #0x2 }
// +0x02d4: 2 encoded words
{ if (!p0.new) r21 = #0x23; p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt label_0138 }
label_02dc:
// +0x02dc: 1 encoded words
{ r19 = #0x1 ; jump label_02f4 }
label_02e0:
// +0x02e0: 1 encoded words
{ p0 = cmp.eq(r2,#0x0); if (!p0.new) jump:t label_0138 }
// +0x02e4: 1 encoded words
{ r23 = #0x10; 	r0 = #0xd }
label_02e8:
// +0x02e8: 1 encoded words
{ call 310204 }
// +0x02ec: 1 encoded words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt label_0138 }
// +0x02f0: 1 encoded words
{ r18 = #0x1; 	r21 = r23 }
label_02f4:
// +0x02f4: 1 encoded words
{ r23 = #0x1 ; jump label_013c }
label_02f8:
// +0x02f8: 4 encoded words
{ r0 = ##-0x4fd48e1c; r1 = ##-0x4fd48e0c }
// +0x0308: 1 encoded words
{ call 1419604 }
// +0x030c: 3 encoded words
{ r3 = r0; r2 = memw(r1+#0x0); if (cmp.eq(r2.new,#0x1)) jump:nt label_0334 }
// +0x0318: 2 encoded words
{ r1:0 = combine(#0x21,#0x0); p0 = cmp.eq(r2,#0x0); if (!p0.new) jump:nt label_033c }
// +0x0320: 2 encoded words
{ r2 = ##-0x4ff3dc98 }
// +0x0328: 3 encoded words
{ r4 = ##0xffffff; jump -719788 }
label_0334:
// +0x0334: 2 encoded words
{ r0 = #0x21 ; jump ##-719072 }
label_033c:
// +0x033c: 1 encoded words
{ jumpr r31 }
// +0x0340: 4 encoded words
{ r1 = ##-0x4ff3d98c; r2 = #0x0; r0 = memw(gp+#0x78c) }
// +0x0350: 1 encoded words
{ jump 311232 }
