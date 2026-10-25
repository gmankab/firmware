// SPDX-License-Identifier: GPL-3.0-only
// Agent20-14: file [0x101870,0x101a3c), runtime [0xb00c1870,0xb00c1a3c).
// Configure frame-size and counted completion packets before passing status.
// Typed Hexagon V66 packet operations, not opaque OEM instructions.
.text
.globl agent14_late_buffer_service_post
agent14_late_buffer_service_post:
// +0x0: 2 encoded words
{ call 0x1e3ee4; allocframe(#56) }
// +0x8: 2 encoded words
{ r17:16 = combine(r1,r4); r22 = r3; r19 = r0 }
// +0x10: 4 encoded words
{ r0 = ##25888; r18 = r2; r23 = memw(r17+#1344) }
// +0x20: 2 encoded words
{ r21 = r5; r1 = memub(r17+#3); memw(r29+#20) = r22 }
// +0x28: 2 encoded words
{ r1 = #9236; memw(r29+#4) = r1; memw(r29+#12) = r22 }
// +0x30: 2 encoded words
{ call 0x159914; memw(r29+#8) = r18; memw(r29+#0) = r19 }
// +0x38: 3 encoded words
{ r1:0 = combine(#4,##-1339330460); r2 = memub(r17+#3); memw(r29+#12) = r22 }
// +0x44: 1 encoded words
{ memw(r29+#8) = r18; memw(r29+#4) = r2 }
// +0x48: 2 encoded words
{ call -783784; memw(r29+#0) = r19 }
// +0x50: 2 encoded words
{ r20 = #0; p0 = cmp.eq(r23,#0); if (p0.new) jump:nt label_1c4 }
// +0x58: 2 encoded words
{ r2 = memb(r17+#0); if (!cmp.eq(r2.new,#2)) jump:t label_1c4 }
// +0x60: 3 encoded words
{ r2 = ##30000001; if (cmp.gtu(r2.new,r18)) jump:t label_08c }
// +0x6c: 3 encoded words
{ r1 = #9297; r0 = ##25896; memw(r29+#0) = r18 }
// +0x78: 1 encoded words
{ call 0x1598cc }
// +0x7c: 3 encoded words
{ call -781476; r1:0 = combine(r18,##-1339330448) }
// +0x88: 1 encoded words
{ jump label_1c4 }
label_08c:
// +0x8c: 4 encoded words
{ p0 = cmp.gtu(r22,#100); if (!p0.new) r20 = add(r23,#16); if (p0.new) jump:t label_1c4; if (!p0.new) r2 = memub(r23+#2) }
// +0x9c: 1 encoded words
{ p0 = tstbit(r2,#0); if (!p0.new) jump:t label_134 }
// +0xa0: 4 encoded words
{ call 0x1598a4; r0 = ##25904; r1 = #9232 }
// +0xb0: 3 encoded words
{ call -783880; r0 = ##-1339330436 }
// +0xbc: 3 encoded words
{ r1:0 = combine(#5,r17); r2 = memw(##-1338753504) }
// +0xc8: 1 encoded words
{ callr r2 }
// +0xcc: 2 encoded words
{ call 0x155484; r0 = r20 }
// +0xd4: 2 encoded words
{ r2 = memb(r23+#1); if (!cmp.eq(r2.new,#3)) jump:t label_124 }
// +0xdc: 1 encoded words
{ r2 = memw(r17+#480) }
// +0xe0: 3 encoded words
{ p0 = !tstbit(r2,#20); if (p0.new) jump:t label_108; if (!p0.new) r3 = memw(r17+#40) }
// +0xec: 2 encoded words
{ r2 = clrbit(r2,#20); r4 = memub(r17+#592) }
// +0xf4: 2 encoded words
{ r3 = addasl(r3,r4,#2); memw(r17+#480) = r2 }
// +0xfc: 1 encoded words
{ r2 = memw(r3+#808) }
// +0x100: 2 encoded words
{ call -752; r0 = memw(r2+#36) }
label_108:
// +0x108: 1 encoded words
{ r2 = memub(r23+#2) }
// +0x10c: 3 encoded words
{ p0 = !tstbit(r2,#1); if (!p0.new) r0 = add(r17,#0); if (p0.new) jump:t label_124 }
// +0x118: 3 encoded words
{ call -768; r2 = and(r2,#253); memb(r23+#2) = r2.new }
label_124:
// +0x124: 2 encoded words
{ r2 = memw(r23+#12); if (cmp.eq(r2.new,#0)) jump:nt label_134 }
// +0x12c: 2 encoded words
{ r1:0 = combine(#1,r17); callr r2 }
label_134:
// +0x134: 3 encoded words
{ r3 = add(r23,#8); r2 = #10000; r1 = add(r29,#20); r0 = r17 }
// +0x140: 1 encoded words
{ memb(r23+#0) = r19; memw(r23+#12) = r21 }
// +0x144: 2 encoded words
{ call -800; memw(r23+#4) = r18 }
// +0x14c: 4 encoded words
{ r1:0 = combine(r17,##-1341384632); r3:2 = combine(#2,#5); memb(r23+#2) = setbit(#0) }
// +0x15c: 3 encoded words
{ r4 = #255; r5 = memw(##-1338753508) }
// +0x168: 1 encoded words
{ callr r5 }
// +0x16c: 2 encoded words
{ call 0x1553f0; memb(r23+#1) = #2 }
// +0x174: 2 encoded words
{ r17 = r0; r0 = r20; r2 = memw(r23+#8) }
// +0x17c: 2 encoded words
{ call 0x1553d4; r18 = sub(r2,r17) }
// +0x184: 3 encoded words
{ call 0x1553c0; r1:0 = combine(r18,r20); r2 = #0 }
// +0x190: 4 encoded words
{ r0 = ##25912; r2 = memw(r23+#8); memw(r16+#0) = r2.new }
// +0x1a0: 2 encoded words
{ r1 = #9283; r2 = memw(r23+#8); memw(r29+#8) = r18 }
// +0x1a8: 2 encoded words
{ call 0x15979c; memw(r29+#4) = r17; memw(r29+#0) = r2 }
// +0x1b0: 3 encoded words
{ r3:2 = combine(r18,r17); r0 = ##-1339330424; r1 = memw(r23+#8) }
// +0x1bc: 2 encoded words
{ call -783640; r20 = #1 }
label_1c4:
// +0x1c4: 2 encoded words
{ r0 = r20 ; jump ##0x1e3dfc }
