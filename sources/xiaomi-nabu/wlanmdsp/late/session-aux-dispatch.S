// SPDX-License-Identifier: GPL-3.0-only
// Agent20-14: file [0x101e64,0x102014), runtime [0xb00c1e64,0xb00c2014).
// Build auxiliary session metadata and conditionally submit status packets.
// Typed Hexagon V66 packet operations, not opaque OEM instructions.
.text
.globl agent14_late_session_aux_dispatch
agent14_late_session_aux_dispatch:
// +0x0: 2 encoded words
{ call 0x1e38d8; allocframe(#72) }
// +0x8: 3 encoded words
{ r17:16 = combine(r3,r0); r19:18 = combine(r1,r2); r23 = #0; r21 = r4 }
// +0x14: 2 encoded words
{ r24 = memw(r16+#1348); memw(r29+#28) = #0 }
// +0x1c: 2 encoded words
{ p0 = cmp.eq(r24,#0); if (p0.new) jump:nt label_1a8 }
// +0x24: 2 encoded words
{ r2 = memb(r16+#0); if (!cmp.eq(r2.new,#1)) jump:t label_1a8 }
// +0x2c: 2 encoded words
{ r2 = memw(r16+#40); r3 = memub(r16+#592) }
// +0x34: 2 encoded words
{ r2 = addasl(r2,r3,#2); r0 = memw(r16+#44) }
// +0x3c: 1 encoded words
{ r2 = memw(r2+#808) }
// +0x40: 2 encoded words
{ call 0x13ee98; r20 = memw(r2+#36) }
// +0x48: 3 encoded words
{ r1 = #9237; r0 = ##25968; r2 = memub(r16+#3) }
// +0x54: 1 encoded words
{ memw(r29+#16) = r21; memw(r29+#12) = r19 }
// +0x58: 1 encoded words
{ memw(r29+#8) = r18; memw(r29+#4) = r17 }
// +0x5c: 2 encoded words
{ call 0x1592f4; memw(r29+#0) = r2 }
// +0x64: 3 encoded words
{ r1:0 = combine(#5,##-1339330256); r2 = memub(r16+#3); memw(r29+#16) = r21 }
// +0x70: 1 encoded words
{ memw(r29+#12) = r19; memw(r29+#8) = r18 }
// +0x74: 2 encoded words
{ call -785352; memw(r29+#4) = r17; memw(r29+#0) = r2 }
// +0x7c: 3 encoded words
{ r1:0 = combine(#5,r16); r2 = memw(##-1338753504) }
// +0x88: 1 encoded words
{ callr r2 }
// +0x8c: 3 encoded words
{ r22 = add(r24,#8); p0 = cmp.eq(r21,#0); if (p0.new) jump:nt label_0a0; if (!p0.new) r2 = memub(r24+#48) }
// +0x98: 2 encoded words
{ r2 = and(r2,#1); if (cmp.eq(r2.new,#0)) jump:t label_108 }
label_0a0:
// +0xa0: 4 encoded words
{ call 0x1592b0; r0 = ##25976; r1 = #9216 }
// +0xb0: 3 encoded words
{ call -785404; r0 = ##-1339330244 }
// +0xbc: 2 encoded words
{ call 0x154ea0; r0 = r22 }
// +0xc4: 1 encoded words
{ r2 = memub(r24+#48) }
// +0xc8: 2 encoded words
{ p0 = !tstbit(r2,#1); if (p0.new) jump:t label_0e4 }
// +0xd0: 2 encoded words
{ r2 = and(r2,#253); memb(r24+#48) = r2.new }
// +0xd8: 2 encoded words
{ call -292; r0 = memw(r16+#44) }
// +0xe0: 1 encoded words
{ r2 = memub(r24+#48) }
label_0e4:
// +0xe4: 4 encoded words
{ r2 = and(r2,#250); r1 = add(r29,#28); r3 = memw(gp+#1932); memb(r24+#48) = r2.new }
// +0xf4: 3 encoded words
{ r2 = #0; memb(r24+#49) = #0; memw(r29+#28) = #0 }
// +0x100: 2 encoded words
{ call 0x11b5c; r0 = memw(r3+#556) }
label_108:
// +0x108: 2 encoded words
{ p0 = cmp.eq(r21,#1); if (!p0.new) jump:t label_1a8; r23 = #1; r1 = add(r29,#28) }
// +0x110: 2 encoded words
{ r2 = memub(r24+#48); memw(r24+#0) = r19 }
// +0x118: 3 encoded words
{ r2 = and(r2,#250); r3 = memw(gp+#1932); memw(r24+#4) = r18 }
// +0x124: 2 encoded words
{ r2 = setbit(r2,#0); memb(r24+#48) = r2.new }
// +0x12c: 3 encoded words
{ r2 = r20; memb(r24+#49) = #1; r0 = memw(r3+#556) }:mem_noshuf
// +0x138: 4 encoded words
{ call 0x11b24; r3 = ##-1341382636; memw(r29+#28) = r3.new }
// +0x148: 2 encoded words
{ call 0x13ed90; r0 = memw(r16+#44) }
// +0x150: 3 encoded words
{ call 0x154e0c; r18 = sub(r17,r0); r0 = r22 }
// +0x15c: 4 encoded words
{ r1:0 = combine(r16,##-1341383132); r5 = memw(##-1338753508) }
// +0x16c: 4 encoded words
{ r3:2 = combine(#2,#5); callr r5; r4 = #255; memw(r24+#52) = r17 }
// +0x17c: 2 encoded words
{ call -388; r1:0 = combine(r18,r22) }
// +0x184: 3 encoded words
{ r1 = #9234; r0 = ##25984; r2 = memub(r16+#3) }
// +0x190: 2 encoded words
{ call 0x1591c0; memw(r29+#4) = r2; memw(r29+#0) = r18 }
// +0x198: 4 encoded words
{ call -785628; r1:0 = combine(r18,##-1339330232); r2 = memub(r16+#3) }
label_1a8:
// +0x1a8: 2 encoded words
{ r0 = r23 ; jump ##0x1e380c }
