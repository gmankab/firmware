// SPDX-License-Identifier: GPL-3.0-only
// Agent20-14: file [0x101b20,0x101b78), runtime [0xb00c1b20,0xb00c1b78).
// Record buffer size/count fields and submit the summary.
// Typed Hexagon V66 packet operations, not opaque OEM instructions.
.text
.globl agent14_late_buffer_counters_store
agent14_late_buffer_counters_store:
// +0x0: 2 encoded words
{ call 0x1e3c58; allocframe(#16) }
// +0x8: 1 encoded words
{ r17 = r0 }
// +0xc: 1 encoded words
{ r16 = memw(r17+#508) }
// +0x10: 3 encoded words
{ p0 = cmp.eq(r16,#0); if (p0.new) r16 = memw(r17+##320) }
// +0x1c: 2 encoded words
{ r18 = memw(r17+#928); if (!cmp.eq(r18.new,#0)) jump:t label_034 }
// +0x24: 2 encoded words
{ call 0x15adec; r1:0 = combine(#4,#3) }
// +0x2c: 2 encoded words
{ r18 = r0; memw(r17+#928) = r18.new }
label_034:
// +0x34: 4 encoded words
{ r1:0 = combine(r16,#56); r3:2 = combine(##256,#9); memb(r18+#3) = #0 }
// +0x44: 1 encoded words
{ memb(r18+#2) = #0; memb(r18+#1) = #0 }
// +0x48: 2 encoded words
{ call 0x16025c; memb(r18+#0) = #0 }
// +0x50: 2 encoded words
{ jump 0x1e3ce4; memw(r18+#0) = r0 }
