// SPDX-License-Identifier: GPL-3.0-only
// Agent20-14: file [0x102014,0x102280), runtime [0xb00c2014,0xb00c2280).
// Process interface state, test command flags and relay status events.
// Typed Hexagon V66 packet operations, not opaque OEM instructions.
.text
.globl agent14_late_interface_event_handler
agent14_late_interface_event_handler:
// +0x0: 2 encoded words
{ call 0x1e370c; allocframe(#80) }
// +0x8: 3 encoded words
{ call 0x154dbc; r17:16 = combine(r1,r2); r20 = r0 }
// +0x14: 2 encoded words
{ r19:18 = combine(r1,r0); p0 = cmp.eq(r16,#0); if (p0.new) jump:nt label_1bc }
// +0x1c: 1 encoded words
{ p0 = cmp.eq(r20,#0); if (p0.new) jump:nt label_1bc }
// +0x20: 2 encoded words
{ r24 = memw(r16+#1348); if (cmp.eq(r24.new,#0)) jump:nt label_1bc }
// +0x28: 1 encoded words
{ r2 = memub(r24+#48) }
// +0x2c: 2 encoded words
{ p0 = tstbit(r2,#0); if (!p0.new) jump:t label_1bc; if (p0.new) r3 = memw(r24+#0) }
// +0x34: 4 encoded words
{ r5 = extractu(r2,#1,#1); r0 = ##26024; r1 = #9287 }
// +0x44: 3 encoded words
{ r2 = extractu(r2,#1,#2); r4 = memub(r16+#3); r6 = memub(r24+#49) }
// +0x50: 2 encoded words
{ r7 = memw(r24+#4); memw(r29+#24) = r17 }
// +0x58: 1 encoded words
{ memw(r29+#20) = r7; memw(r29+#16) = r2 }
// +0x5c: 1 encoded words
{ memw(r29+#12) = r5; memw(r29+#8) = r6 }
// +0x60: 2 encoded words
{ call 0x159140; memw(r29+#4) = r4; memw(r29+#0) = r3 }
// +0x68: 2 encoded words
{ r4 = memub(r24+#49); r2 = memw(r24+#0) }
// +0x70: 2 encoded words
{ r5 = memub(r24+#48); r3 = memub(r16+#3) }
// +0x78: 4 encoded words
{ r0 = extractu(r5,#1,#1); r5 = extractu(r5,#1,#2); r6 = memw(r24+#4); memw(r29+#12) = r0.new }
// +0x88: 3 encoded words
{ r1:0 = combine(#7,##-1339330172); memw(r29+#24) = r17; memw(r29+#20) = r6 }
// +0x94: 1 encoded words
{ memw(r29+#16) = r5; memw(r29+#8) = r4 }
// +0x98: 2 encoded words
{ call -785820; memw(r29+#4) = r3; memw(r29+#0) = r2 }
// +0xa0: 2 encoded words
{ p0 = cmp.eq(r17,#0); r2 = memub(r24+#48) }
// +0xa8: 3 encoded words
{ r3 = setbit(r2,#2); if (!p0) jump:nt label_1bc; if (!p0) memb(r24+#48) = r3.new }
// +0xb4: 3 encoded words
{ r2 = and(r2,#251); r3 = memb(r24+#49); memb(r24+#48) = r2.new }
// +0xc0: 3 encoded words
{ r21 = #0; p0 = cmp.eq(r3,#3); if (!p0.new) jump:t label_1bc; if (p0.new) r2 = memw(r24+#4) }
// +0xcc: 2 encoded words
{ r27:26 = memd(r24+#40); r3 = memw(r24+#0) }
// +0xd4: 3 encoded words
{ r23:22 = sub(r19:18,r27:26); r19 = ##-1338753508 }
// +0xe0: 2 encoded words
{ r20 = add(r3,r2); r1:0 = combine(r23,r22) }
// +0xe8: 2 encoded words
{ call -778432; r3:2 = combine(r21,r20) }
// +0xf0: 3 encoded words
{ r21:20 = combine(r1,r0); r0 = ##26032; memw(r29+#12) = r20 }
// +0xfc: 3 encoded words
{ r1 = #9285; memw(r29+#20) = r18; memw(r29+#16) = r26 }
// +0x108: 2 encoded words
{ call 0x159098; memd(r29+#0) = r23:22; memw(r29+#8) = r20 }
// +0x110: 2 encoded words
{ r2 = memw(r24+#4); r1:0 = memd(r24+#40) }
// +0x118: 4 encoded words
{ r1:0 = combine(#5,##-1339330160); r3 = memw(r24+#0); memw(r29+#12) = r0 }
// +0x128: 2 encoded words
{ r2 = add(r3,r2); memw(r29+#16) = r18; memw(r29+#0) = r22 }
// +0x130: 1 encoded words
{ memw(r29+#8) = r2; memw(r29+#4) = r20 }
// +0x134: 1 encoded words
{ call -785976 }
// +0x138: 2 encoded words
{ r1:0 = combine(#5,r16); r2 = memw(r19+#4) }
// +0x140: 1 encoded words
{ callr r2 }
// +0x144: 2 encoded words
{ r17 = add(r24,#8); r2 = memw(r24+#4) }
// +0x14c: 2 encoded words
{ if (p0.new) r18 = sub(r2,r20); p0 = cmp.gtu(r2,r20); if (!p0.new) jump:t label_1c0 }
// +0x154: 3 encoded words
{ call 0x13ebd4; memb(r24+#49) = #1; r0 = memw(r16+#44) }:mem_noshuf
// +0x160: 4 encoded words
{ r5 = add(r0,r18); r1:0 = combine(r16,##-1341383132); r6 = memw(r19+#0) }
// +0x170: 4 encoded words
{ r3:2 = combine(#2,#5); callr r6; r4 = #255; memw(r24+#52) = r5 }
// +0x180: 2 encoded words
{ call 0x154c2c; r0 = r17 }
// +0x188: 2 encoded words
{ call -832; r1:0 = combine(r18,r17) }
// +0x190: 4 encoded words
{ r0 = ##26048; r1 = #9282; r2 = memub(r24+#49) }
// +0x1a0: 2 encoded words
{ call 0x159000; memw(r29+#4) = r2; memw(r29+#0) = r18 }
// +0x1a8: 4 encoded words
{ call 0x1e35c8; r1:0 = combine(r18,##-1339330136); r2 = memub(r24+#49) }
// +0x1b8: 1 encoded words
{ jump -786092 }
label_1bc:
// +0x1bc: 1 encoded words
{ jump 0x1e362c }
label_1c0:
// +0x1c0: 2 encoded words
{ call 0x13eb68; r0 = memw(r16+#44) }
// +0x1c8: 4 encoded words
{ r1:0 = combine(r16,##-1341383132); r5 = memw(r19+#0); memw(r24+#52) = r0 }
// +0x1d8: 2 encoded words
{ r3:2 = combine(#2,#5); r4 = #255 }
// +0x1e0: 1 encoded words
{ callr r5 }
// +0x1e4: 2 encoded words
{ r2 = memub(r24+#48); r3 = memw(r24+#4) }
// +0x1ec: 3 encoded words
{ p0 = tstbit(r2,#1); r18 = sub(r20,r3); if (p0.new) jump:t label_210 }
// +0x1f8: 4 encoded words
{ r2 = setbit(r2,#1); r1 = #82; r5:4 = combine(#0,#0); memb(r24+#48) = r2.new }
// +0x208: 2 encoded words
{ call 0x145df8; r3:2 = combine(#1,#1); r0 = memw(r16+#44) }
label_210:
// +0x210: 3 encoded words
{ call 0x154b9c; r0 = r17; memb(r24+#49) = #2 }
// +0x21c: 2 encoded words
{ r0 = r17; r2 = memw(r24+#0) }
// +0x224: 2 encoded words
{ call -988; r1 = sub(r2,r18) }
// +0x22c: 4 encoded words
{ r0 = ##26040; r1 = #9282; r2 = memw(r24+#0) }
// +0x23c: 3 encoded words
{ r2 = sub(r2,r18); r3 = memub(r24+#49); memw(r29+#4) = r3.new }
// +0x248: 2 encoded words
{ call 0x158f58; memw(r29+#0) = r2 }
// +0x250: 3 encoded words
{ r0 = ##-1339330148; r2 = memw(r24+#0) }
// +0x25c: 3 encoded words
{ call 0x1e3514; r1 = sub(r2,r18); r2 = memub(r24+#49) }
// +0x268: 1 encoded words
{ jump -786268 }
