// SPDX-License-Identifier: GPL-3.0-only
// Agent20-14: file [0x102754,0x1027a8), runtime [0xb00c2754,0xb00c27a8).
// Normalize optional one-byte descriptor fields into a four-byte temporary
// status record, preserving predicate-qualified memory operations, and call
// the device's external status-registration helper.
.text
.globl agent14_late_attribute_byte_snapshot
agent14_late_attribute_byte_snapshot:
// +0x0000: 1 encoded words
{ allocframe(#0x8) }
// +0x0004: 3 encoded words
{ r2 = add(r29,#0x4); r3 = memw(r0+#0x1b4); r4 = memub(r0+#0x3) }
// +0x0010: 3 encoded words
{ r1:0 = combine(#0x4,##0xae9); memb(r2+#0x3) = #0; 	memb(r2+#0x2) = #0 }
// +0x001c: 1 encoded words
{ r5 = memb(r3+#0x2); 	memb(r2+#0x1) = #0 }
// +0x0020: 3 encoded words
{ p0 = cmp.eq(r5,#0x0); memb(r29+#0x4) = r4; if (!p0.new) memb(r2+#0x1) = #0x1 }
// +0x002c: 3 encoded words
{ r6 = mux(p0,#0x2,#0x3); r2 = memb(r3+#0x1); memb(r29+#0x6) = r5 }
// +0x0038: 4 encoded words
{ p1 = cmp.eq(r2,#0x0); r2 = add(r29,#0x4); if (!p1.new) memb(r29+#0x5) = r6; r3 = memub(r3+#0x1) }:mem_noshuf
// +0x0048: 2 encoded words
{ call 1409484; memb(r29+#0x7) = r3 }
// +0x0050: 1 encoded words
{ dealloc_return }
