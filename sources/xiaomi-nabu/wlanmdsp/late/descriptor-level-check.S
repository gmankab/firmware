// SPDX-License-Identifier: GPL-3.0-only
// Agent20-14: file [0x101b78,0x101bd0), runtime [0xb00c1b78,0xb00c1bd0).
// Validate descriptor state and restore saved registers.
// Typed Hexagon V66 packet operations, not opaque OEM instructions.
.text
.globl agent14_late_descriptor_level_check
agent14_late_descriptor_level_check:
// +0x0: 2 encoded words
{ r16 = r0; memd(r29+#-16) = r17:16; allocframe(#8) }
// +0x8: 2 encoded words
{ r2 = memb(r16+#0); if (!cmp.eq(r2.new,#1)) jump:t label_054 }
// +0x10: 2 encoded words
{ r17 = memw(r16+#1348); if (!cmp.eq(r17.new,#0)) jump:t label_038 }
// +0x18: 1 encoded words
{ r2 = memw(gp+#1932) }
// +0x1c: 1 encoded words
{ r2 = memw(r2+#928) }
// +0x20: 2 encoded words
{ call 0x160594; r0 = memw(r2+#0) }
// +0x28: 2 encoded words
{ r17 = r0; memw(r16+#1348) = r17.new }
// +0x30: 2 encoded words
{ if (p0.new) r2 = #54; p0 = cmp.eq(r17,#0); if (p0.new) jump:nt 0x28 }
label_038:
// +0x38: 2 encoded words
{ call -785416; r2 = #56; r1:0 = combine(#0,r17) }
// +0x40: 2 encoded words
{ r0 = add(r17,#8); r2 = r16; r17:16 = memd(r29+#0) }
// +0x48: 2 encoded words
{ r1 = ##-1341383708; deallocframe }
// +0x50: 1 encoded words
{ jump 0x1551d4 }
label_054:
// +0x54: 1 encoded words
{ r17:16 = memd(r29+#0); dealloc_return }
