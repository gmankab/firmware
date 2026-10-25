// SPDX-License-Identifier: GPL-3.0-only
// Agent20-14: file [0x101be4,0x101da8), runtime [0xb00c1be4,0xb00c1da8).
// Inspect interface metadata, emit diagnostic fields and dispatch processing callbacks.
// Typed Hexagon V66 packet operations, not opaque OEM instructions.
.text
.globl agent14_late_interface_query_report
agent14_late_interface_query_report:
// +0x0: 2 encoded words
{ call 0x1e3b94; allocframe(#40) }
// +0x8: 1 encoded words
{ r16 = r1 }
// +0xc: 2 encoded words
{ r18 = memw(r16+#1348); if (cmp.eq(r18.new,#0)) jump:nt label_1ac }
// +0x14: 1 encoded words
{ r2 = memub(r18+#48) }
// +0x18: 1 encoded words
{ p0 = tstbit(r2,#0); if (!p0.new) jump:nt label_1ac }
// +0x1c: 3 encoded words
{ r1 = #9286; r0 = ##25960; r3 = memw(r18+#0) }
// +0x28: 4 encoded words
{ r5 = extractu(r2,#1,#1); r2 = extractu(r2,#1,#2); r4 = memub(r16+#3); r6 = memub(r18+#49) }
// +0x38: 2 encoded words
{ r7 = memw(r18+#4); memw(r29+#20) = r7.new }
// +0x40: 1 encoded words
{ memw(r29+#16) = r2; memw(r29+#12) = r5 }
// +0x44: 1 encoded words
{ memw(r29+#8) = r6; memw(r29+#4) = r4 }
// +0x48: 2 encoded words
{ call 0x159588; memw(r29+#0) = r3 }
// +0x50: 2 encoded words
{ r4 = memub(r18+#49); r2 = memw(r18+#0) }
// +0x58: 2 encoded words
{ r5 = memub(r18+#48); r3 = memub(r16+#3) }
// +0x60: 4 encoded words
{ r0 = extractu(r5,#1,#1); r5 = extractu(r5,#1,#2); r6 = memw(r18+#4); memw(r29+#12) = r0.new }
// +0x70: 3 encoded words
{ r1:0 = combine(#6,##-1339330268); memw(r29+#20) = r6; memw(r29+#16) = r5 }
// +0x7c: 1 encoded words
{ memw(r29+#8) = r4; memw(r29+#4) = r3 }
// +0x80: 2 encoded words
{ call -784724; memw(r29+#0) = r2 }
// +0x88: 3 encoded words
{ r1:0 = combine(#5,r16); r17 = add(r18,#8); r2 = memub(r18+#49) }
// +0x94: 1 encoded words
{ p0 = cmp.eq(r2,#2); if (p0.new) jump:nt label_0d4 }
// +0x98: 2 encoded words
{ p0 = cmp.eq(r2,#1); if (!p0.new) jump:nt label_19c; if (p0.new) r2 = memub(r18+#48) }
// +0xa0: 2 encoded words
{ p0 = tstbit(r2,#1); if (p0.new) jump:t label_0c0 }
// +0xa8: 4 encoded words
{ r2 = setbit(r2,#1); r1 = #82; r5:4 = combine(#0,#0); memb(r18+#48) = r2.new }
// +0xb8: 2 encoded words
{ call 0x146378; r3:2 = combine(#1,#1); r0 = memw(r16+#44) }
label_0c0:
// +0xc0: 3 encoded words
{ call 0x15511c; r0 = r17; memb(r18+#49) = #2 }
// +0xcc: 2 encoded words
{ jump label_150; r1 = memw(r18+#0) }
label_0d4:
// +0xd4: 2 encoded words
{ r2 = memw(##-1338753504) }
// +0xdc: 1 encoded words
{ callr r2 }
// +0xe0: 1 encoded words
{ r2 = memub(r18+#48) }
// +0xe4: 3 encoded words
{ p0 = tstbit(r2,#2); if (p0.new) jump:t label_15c; if (p0.new) r3 = memw(r18+#4) }
// +0xf0: 2 encoded words
{ p0 = !tstbit(r2,#1); if (p0.new) jump:t label_108 }
// +0xf8: 2 encoded words
{ r2 = and(r2,#253); memb(r18+#48) = r2.new }
// +0x100: 2 encoded words
{ call 0x134; r0 = memw(r16+#44) }
label_108:
// +0x108: 2 encoded words
{ call 0x1550d4; r0 = r17 }
// +0x110: 2 encoded words
{ r2 = memw(r18+#4); if (cmp.eq(r2.new,#0)) jump:nt label_1a0 }
// +0x118: 3 encoded words
{ call 0x13f040; memb(r18+#49) = #1; r0 = memw(r16+#44) }:mem_noshuf
// +0x124: 4 encoded words
{ r4 = #255; r2 = memw(r18+#4); r6 = memw(##-1338753508) }
// +0x134: 4 encoded words
{ r5 = add(r2,r0); r1:0 = combine(r16,##-1341383132); r3:2 = combine(#2,#5) }
// +0x144: 2 encoded words
{ callr r6; memw(r18+#52) = r5 }
// +0x14c: 1 encoded words
{ r1 = memw(r18+#4) }
label_150:
// +0x150: 2 encoded words
{ call 0x1e3ab8; r0 = r17 }
// +0x158: 1 encoded words
{ jump 0x120 }
label_15c:
// +0x15c: 1 encoded words
{ r2 = and(r2,#254) }
// +0x160: 3 encoded words
{ p0 = cmp.eq(r3,#0); if (p0.new) jump:t label_19c; if (p0.new) memb(r18+#49) = #0; if (p0.new) memb(r18+#48) = r2 }
// +0x16c: 2 encoded words
{ call 0x155070; r0 = r17 }
// +0x174: 1 encoded words
{ call 0x155080 }
// +0x178: 2 encoded words
{ r2 = memub(r18+#48); memd(r18+#40) = r1:0 }
// +0x180: 2 encoded words
{ p0 = !tstbit(r2,#1); if (p0.new) jump:t label_198 }
// +0x188: 2 encoded words
{ r2 = and(r2,#253); memb(r18+#48) = r2.new }
// +0x190: 2 encoded words
{ call 0xa4; r0 = memw(r16+#44) }
label_198:
// +0x198: 1 encoded words
{ memb(r18+#49) = #3 }
label_19c:
// +0x19c: 1 encoded words
{ jump 0x1e3ad4 }
label_1a0:
// +0x1a0: 1 encoded words
{ memb(r18+#49) = #0 }
// +0x1a4: 2 encoded words
{ jump 0x1e3acc; memb(r18+#48) = clrbit(#0) }
label_1ac:
// +0x1ac: 4 encoded words
{ r0 = ##-1339330352; r1 = ##-1339330303 }
// +0x1bc: 2 encoded words
{ call 0x15b224; r2 = #125 }
