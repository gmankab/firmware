// SPDX-License-Identifier: GPL-3.0-only
// Agent14 WLAN runtime instruction reconstruction: file [0x104398,0x104488).
// Independent Hexagon V66 mnemonics, control-flow labels and packet boundaries; no OEM opcode arrays.
// Normalize packed little-endian interface header bytes into a status
// descriptor; copy into an allocated response frame, then tail-call the
// external writer. Names are inferred from register dataflow.
.text
.globl agent14_late_interface_status_serializer
agent14_late_interface_status_serializer:
// offset 0x0000; 2 encoded words
{ call 1971168; allocframe(#0x50) }
// offset 0x0008; 2 encoded words
{ r17:16 = combine(r0,r1); r18 = add(r29,#0x0);  r3:2 = combine(#0,#0x0) }
// offset 0x0010; 4 encoded words
{ r0 = add(r18,#0x10); r1 = add(r17,#0x18); r5 = memub(r17+#0x13); memd(r29+#0x38) = r3:2 }
// offset 0x0020; 2 encoded words
{ r6 = memub(r17+#0x12); memd(r29+#0x28) = r3:2 }
// offset 0x0028; 3 encoded words
{ r6 |= asl(r5,#0x8); r7 = memub(r17+#0x11); r9 = memub(r17+#0xb) }
// offset 0x0034; 3 encoded words
{ r2 = #0x10; r4 = memub(r17+#0x10); memd(r29+#0x30) = r3:2 }
// offset 0x0040; 3 encoded words
{ r4 |= asl(r7,#0x8); r7 = memub(r17+#0xa); memb(r18+#0x24) = #0x0 }
// offset 0x004c; 4 encoded words
{ r4 |= asl(r6,#0x10); r7 |= asl(r9,#0x8); r5 = memub(r17+#0x9); r8 = memub(r17+#0x17) }
// offset 0x005c; 2 encoded words
{ r6 = memub(r17+#0x8); memb(r18+#0x23) = #0x0 }
// offset 0x0064; 3 encoded words
{ r6 |= asl(r5,#0x8); r13 = memub(r17+#0xf); r12 = memub(r17+#0x16) }
// offset 0x0070; 4 encoded words
{ r6 |= asl(r7,#0x10); r12 |= asl(r8,#0x8); r14 = memub(r17+#0xe); r11 = memub(r17+#0x15) }
// offset 0x0080; 3 encoded words
{ r14 |= asl(r13,#0x8); r19 = memub(r17+#0xd); r5 = memub(r17+#0x14) }
// offset 0x008c; 3 encoded words
{ r5 |= asl(r11,#0x8); r7 = memub(r17+#0xc); memb(r18+#0x22) = #0x0 }
// offset 0x0098; 2 encoded words
{ r7 |= asl(r19,#0x8); r5 |= asl(r12,#0x10) }
// offset 0x00a0; 2 encoded words
{ r7 |= asl(r14,#0x10); memd(r29+#0x8) = r5:4 }
// offset 0x00a8; 2 encoded words
{ call -795408; memd(r29+#0x0) = r7:6 }
// offset 0x00b0; 4 encoded words
{ r3 = add(r29,#0x28); r0 = add(r29,#0x28); r2 = memub(r17+#0x5); memb(r18+#0x25) = #0x3a }
// offset 0x00c0; 1 encoded words
{ r4 = memub(r17+#0x4);  memw(r29+#0x28) = r16 }
// offset 0x00c4; 2 encoded words
{ r4 |= asl(r2,#0x8); memh(r29+#0x20) = r4.new }
// offset 0x00cc; 3 encoded words
{ r2 = swiz(r4); memw(r29+#0x34) = r18; memw(r3+#0x10) = #0x26 }
// offset 0x00d8; 2 encoded words
{ r2 = lsr(r2,#0x10); memw(r29+#0x2c) = r2.new }
// offset 0x00e0; 3 encoded words
{ call -5620; r2 = add(r3,#0xc); memw(r29+#0x30) = r2.new }
// offset 0x00ec; 1 encoded words
{ jump 1971152 }
