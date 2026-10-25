// SPDX-License-Identifier: GPL-3.0-only
// Agent20-14: file [0x101e24,0x101e64), runtime [0xb00c1e24,0xb00c1e64).
// Adjust a packed byte counter using the current interface descriptor.
// Typed Hexagon V66 packet operations, not opaque OEM instructions.
.text
.globl agent14_late_byte_counter_adjust
agent14_late_byte_counter_adjust:
// +0x0: 2 encoded words
{ r2 = r0; memd(r29+#-16) = r17:16; allocframe(#8) }
// +0x8: 3 encoded words
{ call 0x13ef10; r0 = memw(r0+#44); r16 = memw(r2+#1348) }
// +0x14: 2 encoded words
{ p0 = cmp.eq(r16,#0); if (p0.new) jump:nt label_034; r0 = #0; r2 = r0 }
// +0x1c: 1 encoded words
{ r3 = memub(r16+#48) }
// +0x20: 2 encoded words
{ p0 = tstbit(r3,#0); if (!p0.new) jump:t label_034; if (p0.new) r3 = memw(r16+#52) }
// +0x28: 1 encoded words
{ r0 = sub(r3,r2) }
// +0x2c: 2 encoded words
{ p0 = cmp.gt(r0,#0); if (!p0.new) r0 = #1 }
label_034:
// +0x34: 1 encoded words
{ r17:16 = memd(r29+#0); dealloc_return }
// +0x38: 2 encoded words
{ r2 = #0 ; jump ##0x154f58 }
