// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 window-record-validate-mode: register and buffer-state operations.
// File [0x1073a0,0x1073f4); runtime [0xb00c73a0,0xb00c73f4).
// Encoded-byte equivalence is not a proof of hardware-level behavior.
.text
.globl agent14_late_window_record_validate_mode
agent14_late_window_record_validate_mode:
pkt_0000:
// offset +0x0000, 2 words
{ call 1958872; allocframe(#0x18) }
pkt_0008:
// offset +0x0008, 3 words
{ r17:16 = combine(r2,r0); r19 = memw(r1+#0xc); if (cmp.eq(r19.new,#0x0)) jump:nt pkt_0028 }
pkt_0014:
// offset +0x0014, 1 words
{ r2 = memub(r16+#0x3) }
pkt_0018:
// offset +0x0018, 2 words
{ r3 = memuh(r19+#0x0); if (cmp.eq(r3.new,r2)) jump:nt pkt_002c }
pkt_0020:
// offset +0x0020, 2 words
{ r19 = memw(r19+#0x4); if (!cmp.eq(r19.new,#0x0)) jump:t pkt_0018 }
pkt_0028:
// offset +0x0028, 1 words
{ jump 1959052 }
pkt_002c:
// offset +0x002c, 2 words
{ call 1063316; r0 = #0x1 }
pkt_0034:
// offset +0x0034, 2 words
{ call 1401524; r18 = r0 }
pkt_003c:
// offset +0x003c, 3 words
{ r2 = r0; r5 = #0x0; r1:0 = combine(#0x48,r18); r4 = memw(r17+#0x0) }
pkt_0048:
// offset +0x0048, 2 words
{ call 1404780; memw(r29+#0x4) = r16; 	memw(r29+#0x0) = r19 }
pkt_0050:
// offset +0x0050, 1 words
{ jump 1959012 }
