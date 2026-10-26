// SPDX-License-Identifier: GPL-3.0-only
// Agent14 WLAN runtime instruction reconstruction: file [0x1050d8,0x105140).
// Named Hexagon V66 mnemonics and packet boundaries only; no OEM opcode words.
.text
.globl agent14_late_record_ack_thunk
agent14_late_record_ack_thunk:
// offset 0x0000; 4 encoded words
{ call -9240; r0 = ##-0x4fd48ce3; r1 = #0x71f }
// offset 0x0010; 3 encoded words
{ r16 = r0; r0 = r1; memd(r29+#-0x10) = r17:16;  allocframe(#0x10) }
// offset 0x001c; 1 encoded words
{ call 625336 }
// offset 0x0020; 1 encoded words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt 72 }
// offset 0x0024; 3 encoded words
{ call 53976; r0 = memw(r0+#0x18); memb(r29+#0x2) = #0x6 }
// offset 0x0030; 4 encoded words
{ r1:0 = combine(#0x6,##0xc80); r3 = memw(r16+#0x4); memb(r29+#0x3) = r0 }
// offset 0x0040; 3 encoded words
{ r2 = lsr(r3,#0x18); r4 = lsr(r3,#0x10); memb(r29+#0x7) = r2.new }
// offset 0x004c; 4 encoded words
{ r3 = lsr(r3,#0x8); r2 = add(r29,#0x2); memb(r29+#0x4) = r3; memb(r29+#0x6) = r4 }
// offset 0x005c; 2 encoded words
{ call 1398836; memb(r29+#0x5) = r3 }
// offset 0x0064; 1 encoded words
{ r17:16 = memd(r29+#0x8);  dealloc_return }
