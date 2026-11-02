// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x108724,0x108a14), runtime [0xb00c8724,0xb00c8a14).
// Controller record dispatch, callback and diagnostic packet formatting
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_session_controller_record_handler
agent14_late_session_controller_record_handler:
pkt_0000:
// +0x0: 2 encoded word(s)
{ call 1953840; allocframe(#0x38) }
pkt_0008:
// +0x8: 3 encoded word(s)
{ r17 = r1; r21 = r0; r23:22 = combine(r4,r5); r20 = memw(r29+#0x40) }
pkt_0014:
// +0x14: 1 encoded word(s)
{ call -558264 }
pkt_0018:
// +0x18: 2 encoded word(s)
{ r16 = r0; r19 = #0x0; r2 = memub(r20+#0x4) }
pkt_0020:
// +0x20: 2 encoded word(s)
{ p0 = tstbit(r2,#0); if (!p0.new) jump:t pkt_00cc; if (p0.new) r2 = memh(r20+#0x8) }
pkt_0028:
// +0x28: 4 encoded word(s)
{ p0 = cmp.eq(r2,##-0x7772); if (p0.new) r0 = add(r16,#0x0); if (!p0.new) jump:t pkt_00cc }
pkt_0038:
// +0x38: 3 encoded word(s)
{ r2 = memw(##0xb0344288); r19 = memw(r21+#0x2a8) }
pkt_0044:
// +0x44: 1 encoded word(s)
{ callr r2 }
pkt_0048:
// +0x48: 2 encoded word(s)
{ r18 = #0x3e40; p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt pkt_0064 }
pkt_0050:
// +0x50: 2 encoded word(s)
{ r16 = ##0xb02b7ae8; 	r19 = #0x0 }
pkt_0058:
// +0x58: 3 encoded word(s)
{ r0 = ##0x6958; jump pkt_00bc }
pkt_0064:
// +0x64: 3 encoded word(s)
{ call 652; r1:0 = combine(r17,r19); r2 = #0x0; 	r19 = #0x0 }
pkt_0070:
// +0x70: 2 encoded word(s)
{ r17 = r0; if (cmp.eq(r17.new,#0x0)) jump:nt pkt_00a8 }
pkt_0078:
// +0x78: 2 encoded word(s)
{ r19 = #0x0; r2 = memw(r17+#0xb8) }
pkt_0080:
// +0x80: 2 encoded word(s)
{ p0 = !tstbit(r2,#0x1); if (p0.new) jump:t pkt_00cc }
pkt_0088:
// +0x88: 3 encoded word(s)
{ r19 = #0x0; r3 = memw(r17+#0x30); if (cmp.eq(r3.new,#0x0)) jump:nt pkt_00d4 }
pkt_0094:
// +0x94: 4 encoded word(s)
{ r16 = ##-0x4fd48500; r0 = ##0x6968 }
pkt_00a4:
// +0xa4: 1 encoded word(s)
{ jump pkt_00b8 }
pkt_00a8:
// +0xa8: 4 encoded word(s)
{ r16 = ##-0x4fd4850c; r0 = ##0x6960 }
pkt_00b8:
// +0xb8: 1 encoded word(s)
{ r18 = #0x3e50 }
pkt_00bc:
// +0xbc: 2 encoded word(s)
{ call 1386964; r1 = r18 }
pkt_00c4:
// +0xc4: 1 encoded word(s)
{ r0 = r16 }
pkt_00c8:
// +0xc8: 1 encoded word(s)
{ call -812244 }
pkt_00cc:
// +0xcc: 2 encoded word(s)
{ r0 = r19 ; jump ##1953856 }
pkt_00d4:
// +0xd4: 4 encoded word(s)
{ p0 = tstbit(r2,#0x2); r4 = add(r29,#0x12); if (!p0.new) r5 = add(r20,#0x0); if (p0.new) jump:t pkt_01e4 }
pkt_00e4:
// +0xe4: 2 encoded word(s)
{ r3 = add(r29,#0x14); r2 = r22; r1 = r23 }
pkt_00ec:
// +0xec: 1 encoded word(s)
{ call -556 }
pkt_00f0:
// +0xf0: 2 encoded word(s)
{ r22 = r0; if (cmp.eq(r22.new,#0x0)) jump:nt pkt_01f8 }
pkt_00f8:
// +0xf8: 4 encoded word(s)
{ r0 = ##0x6980; r2 = memub(r16+#0x3); r3 = memuh(gp+#0x10e) }
pkt_0108:
// +0x108: 4 encoded word(s)
{ r1 = #0x2403; r4 = ##-0x76f07feb; r18 = memuh(r29+#0x12) }
pkt_0118:
// +0x118: 2 encoded word(s)
{ r2 = combine(r2.l,r3.l); memw(r29+#0x8) = r18 }
pkt_0120:
// +0x120: 3 encoded word(s)
{ r23 = ##-0x4fc9fef2; memw(r29+#0x4) = r2; 	memw(r29+#0x0) = r4 }
pkt_012c:
// +0x12c: 1 encoded word(s)
{ call 1386852 }
pkt_0130:
// +0x130: 4 encoded word(s)
{ r0 = ##-0x4fd484dc; r2 = memub(r16+#0x3); r3 = memuh(gp+#0x10e) }
pkt_0140:
// +0x140: 3 encoded word(s)
{ r2 = combine(r2.l,r3.l); r1 = ##0x890f8015; 	r3 = r18 }
pkt_014c:
// +0x14c: 1 encoded word(s)
{ call -811868 }
pkt_0150:
// +0x150: 4 encoded word(s)
{ r0 = r17; r3:2 = combine(r18,r22); r1 = memw(r29+#0x14); memh(r23+#0x0) += #0x1 }
pkt_0160:
// +0x160: 1 encoded word(s)
{ call -1308 }
pkt_0164:
// +0x164: 3 encoded word(s)
{ r2 = #0x0; if (!p0.new) r0 = add(r17,#0x0); p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt pkt_020c }
pkt_0170:
// +0x170: 1 encoded word(s)
{ call -3380 }
pkt_0174:
// +0x174: 4 encoded word(s)
{ call 1386780; r0 = ##0x6988; r1 = #0x3e50 }
pkt_0184:
// +0x184: 3 encoded word(s)
{ call -812432; r0 = ##-0x4fd484d0 }
pkt_0190:
// +0x190: 4 encoded word(s)
{ r0 = ##0x6990; r2 = memub(r16+#0x3); r3 = memuh(gp+#0x10e) }
pkt_01a0:
// +0x1a0: 4 encoded word(s)
{ r1 = #0x2403; r4 = ##-0x76f07fe9; r2 = combine(r2.l,r3.l) }
pkt_01b0:
// +0x1b0: 1 encoded word(s)
{ memw(r29+#0x8) = r18; 	memw(r29+#0x4) = r2 }
pkt_01b4:
// +0x1b4: 2 encoded word(s)
{ call 1386716; memw(r29+#0x0) = r4 }
pkt_01bc:
// +0x1bc: 4 encoded word(s)
{ r0 = ##-0x4fd484c4; r2 = memub(r16+#0x3); r3 = memuh(gp+#0x10e) }
pkt_01cc:
// +0x1cc: 3 encoded word(s)
{ r2 = combine(r2.l,r3.l); r1 = ##0x890f8017; 	r3 = r18 }
pkt_01d8:
// +0x1d8: 1 encoded word(s)
{ call -812008 }
pkt_01dc:
// +0x1dc: 2 encoded word(s)
{ r19 = #0x0 ; jump pkt_00cc; memh(r23+#0x0) += #0x1 }
pkt_01e4:
// +0x1e4: 2 encoded word(s)
{ r16 = ##0xb02b7b0c; 	r19 = #0x1 }
pkt_01ec:
// +0x1ec: 3 encoded word(s)
{ r0 = ##0x6970; jump pkt_00bc }
pkt_01f8:
// +0x1f8: 4 encoded word(s)
{ r16 = ##-0x4fd484e8; r0 = ##0x6978 }
pkt_0208:
// +0x208: 1 encoded word(s)
{ jump pkt_00b8 }
pkt_020c:
// +0x20c: 4 encoded word(s)
{ r1:0 = combine(##0x2000,r21); r19 = #0x0; r3 = memub(r16+#0x250) }
pkt_021c:
// +0x21c: 1 encoded word(s)
{ call 1379584 }
pkt_0220:
// +0x220: 3 encoded word(s)
{ if (p0.new) r0 = add(r17,#0x0); p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt pkt_02d0; memw(r17+#0x30) = r0 }
pkt_022c:
// +0x22c: 3 encoded word(s)
{ call -1736; r1:0 = combine(r22,r17); r19 = add(r20,#0x4) }
pkt_0238:
// +0x238: 4 encoded word(s)
{ call -3116; r1 = add(r19,#0x6); r0 = r17; r3:2 = combine(r18,r22) }
pkt_0248:
// +0x248: 4 encoded word(s)
{ r0 = ##0x69a0; r2 = memub(r16+#0x3); r3 = memuh(gp+#0x10e) }
pkt_0258:
// +0x258: 4 encoded word(s)
{ r1 = #0x2403; r4 = ##-0x76f07fea; r2 = combine(r2.l,r3.l) }
pkt_0268:
// +0x268: 1 encoded word(s)
{ memw(r29+#0x8) = r18; 	memw(r29+#0x4) = r2 }
pkt_026c:
// +0x26c: 2 encoded word(s)
{ call 1386532; memw(r29+#0x0) = r4 }
pkt_0274:
// +0x274: 4 encoded word(s)
{ r0 = ##-0x4fd484ac; r2 = memub(r16+#0x3); r3 = memuh(gp+#0x10e) }
pkt_0284:
// +0x284: 3 encoded word(s)
{ r2 = combine(r2.l,r3.l); r1 = ##0x890f8016; 	r3 = r18 }
pkt_0290:
// +0x290: 2 encoded word(s)
{ call -812192; r16 = add(r17,#0x10) }
pkt_0298:
// +0x298: 3 encoded word(s)
{ call 1388448; r0 = r16; memh(r23+#0x0) += #0x1 }
pkt_02a4:
// +0x2a4: 3 encoded word(s)
{ call 1388312; r4 = #0x0; r3:2 = combine(#0x0,#0x0); r0 = r16 }
pkt_02b0:
// +0x2b0: 4 encoded word(s)
{ call 1386464; r0 = ##0x69a8; r1 = #0x3e50 }
pkt_02c0:
// +0x2c0: 3 encoded word(s)
{ call -812748; r0 = ##0xb02b7b60; 	r19 = #0x1 }
pkt_02cc:
// +0x2cc: 1 encoded word(s)
{ jump pkt_00cc }
pkt_02d0:
// +0x2d0: 1 encoded word(s)
{ call -3732 }
pkt_02d4:
// +0x2d4: 4 encoded word(s)
{ call 1386428; r0 = ##0x6998; r1 = #0x3e50 }
pkt_02e4:
// +0x2e4: 3 encoded word(s)
{ r0 = ##-0x4fd484b8; jump pkt_00c8 }
