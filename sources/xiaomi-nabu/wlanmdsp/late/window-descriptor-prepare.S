// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 window-descriptor-prepare: register and buffer-state operations.
// File [0x1073f4,0x107438); runtime [0xb00c73f4,0xb00c7438).
// Encoded-byte equivalence is not a proof of hardware-level behavior.
.text
.globl agent14_late_window_descriptor_prepare
agent14_late_window_descriptor_prepare:
pkt_0000:
// offset +0x0000, 2 words
{ memw(r29+#-0x20) = r1; allocframe(#0x38) }
pkt_0008:
// offset +0x0008, 4 words
{ r3 = add(r29,#0x1c); r4 = add(r29,#0x4); r2 = memw(r0+#0x0); memw(r29+#0x28) = r2.new }
pkt_0018:
// offset +0x0018, 4 words
{ r1 = add(r29,#0x4); r2 = #0x18; memw(r3+#0x8) = #0x0; memw(r3+#0x10) = #0x20 }
pkt_0028:
// offset +0x0028, 2 words
{ memw(r3+#0x14) = #0x0; memw(r3+#0x18) = #0x20 }
pkt_0030:
// offset +0x0030, 4 words
{ call -4144; memw(r29+#0x4) = r3; memw(r4+#0xc) = ##-0x4fcba798 }
pkt_0040:
// offset +0x0040, 1 words
{ dealloc_return }
