// SPDX-License-Identifier: GPL-3.0-only
// Agent20-14 verified WLAN function: file [0x101724,0x101848)
// runtime [0xb00c1724,0xb00c1848).
// Reconstructed Hexagon packet instructions (not OEM opcode arrays).
.text
.globl agent14_late_status_packet_write
agent14_late_status_packet_write:
// +0x0 (2 words)
{ call 0x1e4030; allocframe(#48) }
// +0x8 (2 words)
{ r19:18 = combine(r1,#0); r17:16 = combine(r2,r0) }
// +0x10 (2 words)
{ r22 = memw(r19+#1344); if (cmp.eq(r22.new,#0)) jump:nt 0x10c }
// +0x18 (1 words)
{ r2 = memub(r22+#2) }
// +0x1c (1 words)
{ p0 = tstbit(r2,#0); if (!p0.new) jump:t 0x100 }
// +0x20 (2 words)
{ call 0x141554; r0 = memw(r19+#44) }
// +0x28 (2 words)
{ call 0x13f5f0; r20 = r0; r0 = memw(r19+#44) }
// +0x30 (2 words)
{ call 0x155678; r20 = sub(r20,r0) }
// +0x38 (1 words)
{ r2 = memw(r22+#8) }
// +0x3c (2 words)
{ r21 = sub(r2,r0); if (!cmp.gtu(r21.new,r20)) jump:t L_0f8 }
// +0x44 (2 words)
{ r19 = sub(r21,r20); r2 = memw(r19+#20) }
// +0x4c (2 words)
{ r2 = memuh(r2+#68); memb(r16+#0) = #40 }
// +0x54 (3 words)
{ r18 = asl(r2,#10); memb(r16+#1) = #6; memb(r16+#3) = #0 }
// +0x60 (2 words)
{ call -784340; r1:0 = combine(r18,r19) }
// +0x68 (3 words)
{ call -771384; r1:0 = combine(r18,r19); memb(r16+#2) = r0 }
// +0x74 (3 words)
{ r2 = lsr(r0,#10); r3 = lsr(r0,#18); memb(r16+#7) = r3.new }
// +0x80 (2 words)
{ memb(r16+#6) = r2; r2 = memw(r22+#4) }:mem_noshuf
// +0x88 (3 words)
{ r0 = lsr(r2,#10); r2 = lsr(r2,#18); memb(r16+#5) = r2.new }
// +0x94 (2 words)
{ memb(r16+#4) = r0; memb(r17+#0) = #8 }
// +0x9c (1 words)
{ r1 = memub(r16+#2); r0 = memub(r16+#5) }
// +0xa0 (1 words)
{ r4 = memub(r16+#4); r5 = memub(r16+#7) }
// +0xa4 (3 words)
{ r4 |= asl(r0,#8); r0 = ##25880; r6 = memub(r16+#6) }
// +0xb0 (3 words)
{ r6 |= asl(r5,#8); r1 = #9220; r7 = memub(r16+#1); memw(r29+#0) = r1 }
// +0xbc (1 words)
{ memw(r29+#12) = r7; memw(r29+#8) = r6 }
// +0xc0 (2 words)
{ call 0x1599d0; memw(r29+#4) = r4 }
// +0xc8 (3 words)
{ r1:0 = combine(#4,##-1339330472); r2 = memub(r16+#2); r3 = memub(r16+#5) }
// +0xd4 (1 words)
{ r4 = memub(r16+#4); r5 = memub(r16+#7) }
// +0xd8 (2 words)
{ r4 |= asl(r3,#8); r6 = memub(r16+#6); r7 = memub(r16+#1) }
// +0xe0 (2 words)
{ r6 |= asl(r5,#8); memw(r29+#12) = r7 }
// +0xe8 (1 words)
{ memw(r29+#8) = r6; memw(r29+#4) = r4 }
// +0xec (2 words)
{ call -783616; r18 = #1; memw(r29+#0) = r2 }
// +0xf4 (1 words)
{ jump 0x28 }
L_0f8:
// +0xf8 (3 words)
{ r1 = #9298; r0 = ##25872; memw(r29+#4) = r20 }
// +0x104 (2 words)
{ call 0x15998c; memw(r29+#0) = r21 }
// +0x10c (4 words)
{ call -783632; r1:0 = combine(r21,##-1339330484); r2 = r20 }
