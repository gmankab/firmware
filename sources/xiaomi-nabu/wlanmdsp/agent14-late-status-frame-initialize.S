// SPDX-License-Identifier: GPL-3.0-only
// Agent 14 file [0x102ce4,0x102f0c), runtime [0xb00c2ce4,0xb00c2f0c).
// Initialize a status request, serialize protocol fields, normalize byte
// metadata and calculate record checksum using the controller packet-loop.
// The first 3-word packet is rejoined by the companion fixture because LLVM
// chooses a different valid duplex pairing than the firmware's assembler.
// Only Hexagon mnemonics are authored; no raw firmware opcodes are imported.
.text
.globl agent14_late_status_frame_initialize
agent14_late_status_frame_initialize:
// +0x0000: 2 encoded words
{ call 1976964; r5:4 = combine(#0,#0x0); 	allocframe(#0x68) }
// +0x0008: 3 encoded words
{ r17:16 = combine(r2,r1); r0 = add(r29,#0x8); r1 = #0x0; 	r18 = r0 }
// +0x0014: 2 encoded words
{ r2 = #0x26; memd(r29+#0x40) = r5:4; 	memd(r29+#0x38) = r5:4 }
// +0x001c: 2 encoded words
{ call -789848; r20 = add(r29,#0x8); 	memd(r29+#0x30) = r5:4 }
// +0x0024: 1 encoded words
{ p0 = cmp.eq(r17,#0x0) }
// +0x0028: 3 encoded words
{ r2 = p0; if (p0) jump:nt label_0048; memw(r29+#0x4) = r2.new }
// +0x0034: 4 encoded words
{ call -789480; r1:0 = combine(##-0x4fd48c80,r16); r2 = #0x8 }
// +0x0044: 1 encoded words
{ r16 = add(r16,#0x8) }
label_0048:
// +0x0048: 3 encoded words
{ call -789892; r2 = #0x47; r1 = #0x0; 	r0 = add(r16,#1) }
// +0x0054: 4 encoded words
{ r0 = add(r16,#0x8); r1 = r18; r2 = #0x10; memb(r16+#0x0) = #0x60 }
// +0x0064: 3 encoded words
{ r17 = add(r16,#0x18); memb(r16+#0x5) = #0x20; memb(r16+#0x4) = #0x0 }
// +0x0070: 3 encoded words
{ call -789540; memb(r16+#0x6) = #0x3a; memb(r16+#0x7) = #-0x1 }
// +0x007c: 2 encoded words
{ r19 = add(r18,#0x20); r2 = #0x10 }
// +0x0084: 2 encoded words
{ call -789560; r1:0 = combine(r19,r17) }
// +0x008c: 4 encoded words
{ r1 = r19; r0 = add(r16,#0x30); r2 = #0x10; memb(r16+#0x28) = #-0x79 }
// +0x009c: 3 encoded words
{ r19 = add(r16,#0x28); memb(r16+#0x29) = #0x0; memb(r16+#0x2f) = #0x0 }
// +0x00a8: 2 encoded words
{ memb(r16+#0x2e) = #0x0; memb(r16+#0x2d) = #0x0 }
// +0x00b0: 2 encoded words
{ call -789604; memb(r16+#0x2c) = #0x0 }
// +0x00b8: 3 encoded words
{ r0 = add(r16,#0x42); r1 = add(r18,#0x36); r3 = #0x1; 	r2 = #0x6 }
// +0x00c4: 3 encoded words
{ call -789624; memb(r16+#0x40) = r3; memb(r16+#0x41) = r3 }
// +0x00d0: 2 encoded words
{ r2 = memub(r16+#0x13); r6 = memub(r16+#0xb) }
// +0x00d8: 2 encoded words
{ r3 = memub(r16+#0x12); r7 = memub(r16+#0xa) }
// +0x00e0: 4 encoded words
{ r3 |= asl(r2,#0x8); r7 |= asl(r6,#0x8); r2 = #0x10; r5 = memub(r16+#0x11) }
// +0x00f0: 2 encoded words
{ r4 = memub(r16+#0x10); r0 = memub(r16+#0x17) }
// +0x00f8: 2 encoded words
{ r4 |= asl(r5,#0x8); r5 = memub(r16+#0x9); 	r1 = memub(r16+#0xf) }
// +0x0100: 3 encoded words
{ r4 |= asl(r3,#0x10); r8 = memub(r16+#0x8); r3 = memub(r16+#0x16) }
// +0x010c: 4 encoded words
{ r3 |= asl(r0,#0x8); r8 |= asl(r5,#0x8); r6 = memub(r16+#0x15); r12 = memub(r16+#0xe) }
// +0x011c: 2 encoded words
{ r8 |= asl(r7,#0x10); r5 = memub(r16+#0x14) }
// +0x0124: 3 encoded words
{ r5 |= asl(r6,#0x8); r12 |= asl(r1,#0x8); r1 = r17; 	r0 = memub(r16+#0xd) }
// +0x0130: 2 encoded words
{ r5 |= asl(r3,#0x10); r9 = memub(r16+#0xc) }
// +0x0138: 3 encoded words
{ r9 |= asl(r0,#0x8); r0 = add(r20,#0x10); memd(r29+#0x10) = r5:4 }
// +0x0144: 1 encoded words
{ r9 |= asl(r12,#0x10) }
// +0x0148: 2 encoded words
{ call -789756; memd(r29+#0x8) = r9:8 }
// +0x0150: 3 encoded words
{ r2 = #0x2000; r0 = add(r29,#0x30); memh(r29+#0x28) = r2.new }
// +0x015c: 3 encoded words
{ r2 = add(r29,#0x30); memb(r20+#0x25) = #0x3a; memw(r29+#0x30) = r19 }
// +0x0168: 3 encoded words
{ r3 = add(r2,#0xc); memw(r29+#0x3c) = r20; memw(r2+#0x4) = #0x20 }
// +0x0174: 2 encoded words
{ memw(r29+#0x38) = r3; memw(r2+#0x10) = #0x26 }
// +0x017c: 1 encoded words
{ call label_01a0 }
// +0x0180: 3 encoded words
{ r3 = lsr(r0,#0x8); memb(r16+#0x2a) = r0; r2 = memw(r29+#0x4) }:mem_noshuf
// +0x018c: 2 encoded words
{ p0 = r2; memb(r16+#0x2b) = r3 }
// +0x0194: 1 encoded words
{ r2 = mux(p0,#0x48,#0x50) }
// +0x0198: 2 encoded words
{ r0 = r2 ; jump ##1976776 }
label_01a0:
// +0x01a0: 2 encoded words
{ r3:2 = combine(#0x2,#0x0); p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt label_0218 }
label_01a8:
// +0x01a8: 1 encoded words
{ r5 = memw(r0+#0x4); 	r4 = memw(r0+#0x0) }
// +0x01ac: 2 encoded words
{ r7 = lsr(r5,#0x1); p0 = cmp.gtu(r3,r5); if (p0.new) jump:t label_01e4 }
// +0x01b4: 2 encoded words
{ r6 = add(r5,#-0x2); r5 = r4 }
// +0x01bc: 2 encoded words
{ loop0(label_01c4,r7); r7 = clrbit(r6,#0x0) }
label_01c4:
// +0x01c4: 2 encoded words
{ r8 = memub(r5+#0x1); r9 = memub(r5++#0x2) }
// +0x01cc: 1 encoded words
{ r9 = insert(r8,#0x18,#0x8) }
// +0x01d0: 2 encoded words
{ r2 = add(r9,r2); nop }:endloop0
// +0x01d8: 2 encoded words
{ r7 += add(r4,#0x2); r5 = sub(r6,r7) }
// +0x01e0: 1 encoded words
{ r4 = r7 }
label_01e4:
// +0x01e4: 2 encoded words
{ p0 = cmp.eq(r5,#0x0); if (!p0.new) r4 = memub(r4+#0x0) }
// +0x01ec: 3 encoded words
{ if (!p0) r2 = add(r2,r4); r0 = memw(r0+#0x8); if (!cmp.eq(r0.new,#0x0)) jump:t label_01a8 }
// +0x01f8: 2 encoded words
{ r3 = lsr(r2,#0x10); if (cmp.eq(r3.new,#0x0)) jump:t label_021c }
label_0200:
// +0x0200: 1 encoded words
{ r3 = zxth(r2) }
// +0x0204: 1 encoded words
{ r3 += lsr(r2,#0x10) }
// +0x0208: 1 encoded words
{ r2 = lsr(r3,#0x10) }
// +0x020c: 2 encoded words
{ r2 = r3; p0 = cmp.eq(r2,#0x0); if (!p0.new) jump:t label_0200 }
// +0x0214: 1 encoded words
{ jump label_0220 }
label_0218:
// +0x0218: 1 encoded words
{ r3 = #0x0 ; jump label_0220 }
label_021c:
// +0x021c: 1 encoded words
{ r3 = r2 }
label_0220:
// +0x0220: 1 encoded words
{ r2 = sub(#-0x1,r3) }
// +0x0224: 1 encoded words
{ r0 = zxth(r2); 	jumpr r31 }
