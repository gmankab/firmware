// SPDX-License-Identifier: GPL-3.0-only
// Agent20-14: file [0x1027a8,0x102820), runtime [0xb00c27a8,0xb00c2820).
// Select an optional byte by requested attribute type, initialize the
// bounded stack buffer and forward the selected status descriptor. The source
// explicitly preserves the no-shuffle packet ordering at two load/store
// points to avoid a nonidentical duplex encoding.
.text
.globl agent14_late_attribute_byte_select
agent14_late_attribute_byte_select:
// +0x0000: 3 encoded words
{ p0 = cmp.eq(r1,#0x2); memb(r29+#-0xd) = r1; allocframe(#0x8) }
// +0x000c: 3 encoded words
{ r5 = add(r29,#0x2); r4 = memw(r0+#0x1b4); r6 = memub(r0+#0x3) }
// +0x0018: 1 encoded words
{ memb(r5+#0x5) = #0; 	memb(r5+#0x4) = #0 }
// +0x001c: 1 encoded words
{ memb(r5+#0x3) = #0; 	memb(r5+#0x2) = #0 }
// +0x0020: 2 encoded words
{ if (p0) jump:nt label_0044; memb(r29+#0x2) = r6 }
// +0x0028: 1 encoded words
{ p0 = cmp.eq(r1,#0x1); if (!p0.new) jump:nt label_005c }
// +0x002c: 2 encoded words
{ r5 = memb(r4+#0x2); if (cmp.eq(r5.new,#0x0)) jump:nt label_005c }
// +0x0034: 1 encoded words
{ r5 = add(r29,#0x2) }
// +0x0038: 3 encoded words
{ jump label_0058; memb(r5+#0x2) = #0x1; r4 = memub(r4+#0x2) }:mem_noshuf
label_0044:
// +0x0044: 2 encoded words
{ r5 = memb(r4+#0x1); if (cmp.eq(r5.new,#0x0)) jump:nt label_005c }
// +0x004c: 1 encoded words
{ r5 = add(r29,#0x2) }
// +0x0050: 2 encoded words
{ memb(r5+#0x2) = #0x1; r4 = memub(r4+#0x1) }:mem_noshuf
label_0058:
// +0x0058: 1 encoded words
{ memb(r29+#0x5) = r4 }
label_005c:
// +0x005c: 4 encoded words
{ r1:0 = combine(#0x6,##0xaea); r2 = add(r29,#0x2); memb(r29+#0x6) = r2 }
// +0x006c: 2 encoded words
{ call 1409364; memb(r29+#0x7) = r3 }
// +0x0074: 1 encoded words
{ dealloc_return }
