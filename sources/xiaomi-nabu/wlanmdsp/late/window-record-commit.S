// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 window-record-commit: register and buffer-state operations.
// File [0x1072ec,0x1073a0); runtime [0xb00c72ec,0xb00c73a0).
// Encoded-byte equivalence is not a proof of hardware-level behavior.
.text
.globl agent14_late_window_record_commit
agent14_late_window_record_commit:
pkt_0000:
// offset +0x0000, 2 words
{ r16 = r0; memd(r29+#-0x10) = r17:16; 	allocframe(#0x18) }
pkt_0008:
// offset +0x0008, 3 words
{ r1 = #0x3e13; r0 = ##0x6868; 	r17 = memw(r1+#0x0) }
pkt_0014:
// offset +0x0014, 1 words
{ r3 = memw(r17+#0x8); 	r2 = memw(r17+#0x4) }
pkt_0018:
// offset +0x0018, 2 words
{ r4 = memw(r17+#0xc); memw(r29+#0x8) = r4.new }
pkt_0020:
// offset +0x0020, 2 words
{ call 1392296; memw(r29+#0x4) = r3; 	memw(r29+#0x0) = r2 }
pkt_0028:
// offset +0x0028, 3 words
{ r0 = ##-0x4fd48764; r2 = memw(r17+#0x8); 	r1 = memw(r17+#0x4) }
pkt_0034:
// offset +0x0034, 2 words
{ call -806412; r3 = memw(r17+#0xc) }
pkt_003c:
// offset +0x003c, 2 words
{ r2 = memw(r16+#0x20); if (cmp.eq(r2.new,#0x0)) jump:nt pkt_0048 }
pkt_0044:
// offset +0x0044, 1 words
{ r17:16 = memd(r29+#0x10); 	dealloc_return }
pkt_0048:
// offset +0x0048, 1 words
{ r0 = memw(r16+#0xc); 	r2 = memw(r17+#0x4) }
pkt_004c:
// offset +0x004c, 1 words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt pkt_0060 }
pkt_0050:
// offset +0x0050, 2 words
{ r3 = memuh(r0+#0x0); if (cmp.eq(r3.new,r2)) jump:nt pkt_0084 }
pkt_0058:
// offset +0x0058, 2 words
{ jump pkt_004c; r0 = memw(r0+#0x4) }
pkt_0060:
// offset +0x0060, 3 words
{ r1 = #0x3e41; r0 = ##0x6870; 	memw(r29+#0x0) = r2 }
pkt_006c:
// offset +0x006c, 1 words
{ call 1392220 }
pkt_0070:
// offset +0x0070, 3 words
{ r0 = ##-0x4fd48758; r1 = memw(r17+#0x4); 	r17:16 = memd(r29+#0x10) }
pkt_007c:
// offset +0x007c, 2 words
{ jump -804640; deallocframe }
pkt_0084:
// offset +0x0084, 2 words
{ r3:2 = combine(#0x0,#0x0); memw(r0+#0x90) = #0x0 }
pkt_008c:
// offset +0x008c, 2 words
{ memd(r0+#0x88) = r3:2; r2 = memw(r17+#0x8) }:mem_noshuf
pkt_0094:
// offset +0x0094, 2 words
{ memw(r0+#0x88) = r2; r2 = memw(r17+#0xc) }:mem_noshuf
pkt_009c:
// offset +0x009c, 2 words
{ memw(r0+#0x8c) = r2; r17:16 = memd(r29+#0x10) }:mem_noshuf
pkt_00a4:
// offset +0x00a4, 2 words
{ jump -432; deallocframe }
pkt_00ac:
// offset +0x00ac, 2 words
{ r0 = #0x1000; jumpr r31 }
