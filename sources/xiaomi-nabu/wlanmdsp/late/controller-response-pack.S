// SPDX-License-Identifier: GPL-3.0-only
// File [0x1033c4, 0x103560); runtime [0xb00c33c4, 0xb00c3560).
// Construct controller response packet and checksum/field state from a
// fixed input view, serializing multiple packed address pairs and handing
// off through a tagged scheduler. External handler semantics are inferred.
// Original Hexagon packet boundaries and constant extenders are explicit.
// No private opcode-word arrays or OEM payload bytes are included.
.text
.globl agent14_late_controller_response_pack
agent14_late_controller_response_pack:
// +0x0000: 2 encoded words
{ call 1975204; r5:4 = combine(#0,#0x0); 	allocframe(#0x68) }
// +0x0008: 3 encoded words
{ r17:16 = combine(r2,r1); r19:18 = combine(r3,r0); r1 = #0x0; 	r0 = add(r29,#0x8) }
// +0x0014: 2 encoded words
{ r2 = #0x26; memd(r29+#0x40) = r5:4; 	memd(r29+#0x38) = r5:4 }
// +0x001c: 2 encoded words
{ call -791608; r20 = add(r29,#0x8); 	memd(r29+#0x30) = r5:4 }
// +0x0024: 1 encoded words
{ p0 = cmp.eq(r17,#0x0) }
// +0x0028: 3 encoded words
{ r2 = p0; if (p0) jump:nt label_0048; memw(r29+#0x4) = r2.new }
// +0x0034: 4 encoded words
{ call -791240; r1:0 = combine(##-0x4fd48c80,r16); r2 = #0x8 }
// +0x0044: 1 encoded words
{ r16 = add(r16,#0x8) }
label_0048:
// +0x0048: 3 encoded words
{ call -791652; r2 = #0x47; r1 = #0x0; 	r0 = add(r16,#1) }
// +0x0054: 4 encoded words
{ r0 = add(r16,#0x8); r1 = r18; r2 = #0x10; memb(r16+#0x0) = #0x60 }
// +0x0064: 2 encoded words
{ call -791288; r17 = add(r16,#0x18) }
// +0x006c: 3 encoded words
{ call -791296; r1 = add(r18,#0x20); r2 = #0x10; 	r0 = r17 }
// +0x0078: 3 encoded words
{ r2 = #0x10; r1 = r18; r0 = add(r16,#0x30); p0 = cmp.eq(r19,#0x0) }
// +0x0084: 4 encoded words
{ r19 = add(r16,#0x28); r3 = mux(p0,#0x60,#0x20); memb(r16+#0x6) = #0x3a; memb(r16+#0x7) = #-0x1 }
// +0x0094: 2 encoded words
{ memb(r16+#0x5) = #0x20; memb(r16+#0x4) = #0x0 }
// +0x009c: 2 encoded words
{ memb(r16+#0x2c) = r3; memb(r16+#0x28) = #-0x78 }
// +0x00a4: 2 encoded words
{ call -791352; memb(r16+#0x29) = #0x0 }
// +0x00ac: 4 encoded words
{ r2 = #0x2; r0 = add(r16,#0x42); r1 = add(r18,#0x30); memb(r16+#0x40) = r2.new }
// +0x00bc: 4 encoded words
{ call -791376; r2 = #0x6; r3 = #0x1; memb(r16+#0x41) = r3.new }
// +0x00cc: 2 encoded words
{ r2 = memub(r16+#0x13); r6 = memub(r16+#0xb) }
// +0x00d4: 2 encoded words
{ r3 = memub(r16+#0x12); r7 = memub(r16+#0xa) }
// +0x00dc: 4 encoded words
{ r3 |= asl(r2,#0x8); r7 |= asl(r6,#0x8); r2 = #0x10; r5 = memub(r16+#0x11) }
// +0x00ec: 2 encoded words
{ r4 = memub(r16+#0x10); r0 = memub(r16+#0x17) }
// +0x00f4: 2 encoded words
{ r4 |= asl(r5,#0x8); r5 = memub(r16+#0x9); 	r1 = memub(r16+#0xf) }
// +0x00fc: 3 encoded words
{ r4 |= asl(r3,#0x10); r8 = memub(r16+#0x8); r3 = memub(r16+#0x16) }
// +0x0108: 4 encoded words
{ r3 |= asl(r0,#0x8); r8 |= asl(r5,#0x8); r6 = memub(r16+#0x15); r12 = memub(r16+#0xe) }
// +0x0118: 2 encoded words
{ r8 |= asl(r7,#0x10); r5 = memub(r16+#0x14) }
// +0x0120: 3 encoded words
{ r5 |= asl(r6,#0x8); r12 |= asl(r1,#0x8); r1 = r17; 	r0 = memub(r16+#0xd) }
// +0x012c: 2 encoded words
{ r5 |= asl(r3,#0x10); r9 = memub(r16+#0xc) }
// +0x0134: 3 encoded words
{ r9 |= asl(r0,#0x8); r0 = add(r20,#0x10); memd(r29+#0x10) = r5:4 }
// +0x0140: 1 encoded words
{ r9 |= asl(r12,#0x10) }
// +0x0144: 2 encoded words
{ call -791512; memd(r29+#0x8) = r9:8 }
// +0x014c: 3 encoded words
{ r2 = #0x2000; r0 = add(r29,#0x30); memh(r29+#0x28) = r2.new }
// +0x0158: 3 encoded words
{ r2 = add(r29,#0x30); memb(r20+#0x25) = #0x3a; memw(r29+#0x30) = r19 }
// +0x0164: 3 encoded words
{ r3 = add(r2,#0xc); memw(r29+#0x3c) = r20; memw(r2+#0x4) = #0x20 }
// +0x0170: 2 encoded words
{ memw(r29+#0x38) = r3; memw(r2+#0x10) = #0x26 }
// +0x0178: 1 encoded words
{ call -1720 }
// +0x017c: 3 encoded words
{ r3 = lsr(r0,#0x8); memb(r16+#0x2a) = r0; r2 = memw(r29+#0x4) }:mem_noshuf
// +0x0188: 2 encoded words
{ p0 = r2; memb(r16+#0x2b) = r3 }
// +0x0190: 1 encoded words
{ r2 = mux(p0,#0x48,#0x50) }
// +0x0194: 2 encoded words
{ r0 = r2 ; jump ##1975020 }
