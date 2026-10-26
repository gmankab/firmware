// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 window-record-resolve: register and buffer-state operations.
// File [0x1071e0,0x1072ec); runtime [0xb00c71e0,0xb00c72ec).
// Encoded-byte equivalence is not a proof of hardware-level behavior.
.text
.globl agent14_late_window_record_resolve
agent14_late_window_record_resolve:
pkt_0000:
// offset +0x0000, 2 words
{ call 1959320; allocframe(#0x10) }
pkt_0008:
// offset +0x0008, 1 words
{ r16 = r0 }
pkt_000c:
// offset +0x000c, 2 words
{ r2 = memw(r16+#0x8); r4 = memw(r16+#0x88) }
pkt_0014:
// offset +0x0014, 2 words
{ r5 = memw(r2+#0x18); if (!cmp.gtu(r5.new,r4)) jump:t pkt_0024 }
pkt_001c:
// offset +0x001c, 2 words
{ r3 = memw(r16+#0x8c); if (!cmp.gtu(r3.new,r5)) jump:t pkt_0030 }
pkt_0024:
// offset +0x0024, 3 words
{ jump 1959504; memw(r2+#0x20) = #0x0; memw(r16+#0x90) = #0x0 }
pkt_0030:
// offset +0x0030, 2 words
{ r4 = add(r3,r4); if (cmp.gtu(r4.new,r5)) jump:nt pkt_0024 }
pkt_0038:
// offset +0x0038, 1 words
{ p0 = cmp.gtu(r3,r4); if (p0.new) jump:nt pkt_0024 }
pkt_003c:
// offset +0x003c, 3 words
{ p0 = cmp.gtu(r3,#0x12c); r1 = #0x12c; r18 = memw(r2+#0x1c) }
pkt_0048:
// offset +0x0048, 3 words
{ r17 = minu(r3,r1); r2 = mux(p0,#0x1,#0x0); memw(r18+#0x1c) = r2.new }
pkt_0054:
// offset +0x0054, 2 words
{ r2 = memw(r16+#0x88); memw(r16+#0x8c) -= r17 }
pkt_005c:
// offset +0x005c, 3 words
{ r3 = add(r2,r17); r0 = memuh(r16+#0x0); memw(r16+#0x88) = r3.new }
pkt_0068:
// offset +0x0068, 2 words
{ r0 = add(r18,#0x24); memw(r18+#0x10) = r0; 	memw(r18+#0x14) = r2 }
pkt_0070:
// offset +0x0070, 2 words
{ memw(r18+#0x18) = r17; r3 = memw(r16+#0x78) }:mem_noshuf
pkt_0078:
// offset +0x0078, 3 words
{ call -807208; r1 = add(r3,r2); r2 = r17 }
pkt_0084:
// offset +0x0084, 2 words
{ call 1029868; r1:0 = combine(#0x0,#0x0) }
pkt_008c:
// offset +0x008c, 3 words
{ r2 = add(r17,#0x1b); if (!p0.new) r4 = add(r18,#0xc); p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt pkt_00e8 }
pkt_0098:
// offset +0x0098, 3 words
{ r3 = ##0x2cf0010; memw(r18+#0xc) = r3.new }
pkt_00a4:
// offset +0x00a4, 2 words
{ r2 = and(r2,##0x3fc) }
pkt_00ac:
// offset +0x00ac, 2 words
{ r1 = add(r2,##0xffe8) }
pkt_00b4:
// offset +0x00b4, 3 words
{ r3 = or(r1,##0x110000); memw(r18+#0x20) = r3.new }
pkt_00c0:
// offset +0x00c0, 2 words
{ r1 = ##0x36003; 	r3 = memw(r16+#0x8) }
pkt_00c8:
// offset +0x00c8, 3 words
{ r3 = #0x0; memw(r3+#0x20) = #0x1; memw(r16+#0x90) = #0x1 }
pkt_00d4:
// offset +0x00d4, 1 words
{ memw(r0+#0x14) = r16; 	memw(r0+#0x4) = r4 }
pkt_00d8:
// offset +0x00d8, 3 words
{ call 1959220; memw(r0+#0x10) = ##-0x4ff38e58 }
pkt_00e4:
// offset +0x00e4, 1 words
{ jump 1030300 }
pkt_00e8:
// offset +0x00e8, 4 words
{ call 1392364; r0 = ##0x6860; r1 = #0x3e50 }
pkt_00f8:
// offset +0x00f8, 3 words
{ call -806848; r0 = ##-0x4fd48770 }
pkt_0104:
// offset +0x0104, 2 words
{ jump pkt_0024; r2 = memw(r16+#0x8) }
