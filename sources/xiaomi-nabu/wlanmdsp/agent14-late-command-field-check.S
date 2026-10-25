// SPDX-License-Identifier: GPL-3.0-only
// Agent20-14: file [0x101da8,0x101e24), runtime [0xb00c1da8,0xb00c1e24).
// Evaluate command field limits and return a bounded descriptor class.
// Typed Hexagon V66 packet operations, not opaque OEM instructions.
.text
.globl agent14_late_command_field_check
agent14_late_command_field_check:
// +0x0: 2 encoded words
{ call 0x1e39d0; allocframe(#16) }
// +0x8: 1 encoded words
{ r16 = r0 }
// +0xc: 2 encoded words
{ r17 = memw(r16+#1348); if (cmp.eq(r17.new,#0)) jump:nt label_06c }
// +0x14: 2 encoded words
{ r18 = add(r17,#8); r2 = memw(gp+#1932) }
// +0x1c: 2 encoded words
{ r0 = r18; r19 = memw(r2+#928) }
// +0x24: 1 encoded words
{ call 0x154ff4 }
// +0x28: 2 encoded words
{ call 0x154ff4; r0 = r18 }
// +0x30: 3 encoded words
{ r1:0 = combine(#5,r16); r2 = memw(##-1338753504) }
// +0x3c: 1 encoded words
{ callr r2 }
// +0x40: 1 encoded words
{ r2 = memub(r17+#48) }
// +0x44: 2 encoded words
{ p0 = !tstbit(r2,#1); if (p0.new) jump:t label_05c }
// +0x4c: 2 encoded words
{ r2 = and(r2,#253); memb(r17+#48) = r2.new }
// +0x54: 2 encoded words
{ call 0x1c; r0 = memw(r16+#44) }
label_05c:
// +0x5c: 2 encoded words
{ call 0x160240; r1 = r17; r0 = memw(r19+#0) }
// +0x64: 2 encoded words
{ r2 = #0; memw(r16+#1348) = r2.new }
label_06c:
// +0x6c: 1 encoded words
{ jump 0x1e3a40 }
// +0x70: 2 encoded words
{ r1 = #82; r4 = #0; r3:2 = combine(#0,#1) }
// +0x78: 1 encoded words
{ jump 0x146510 }
