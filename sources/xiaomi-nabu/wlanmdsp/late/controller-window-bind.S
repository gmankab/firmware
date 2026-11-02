// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x108a80,0x108b34), runtime [0xb00c8a80,0xb00c8b34).
// Initialize controller window handles, class indexes, and conditional pointer copy
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_controller_window_bind
agent14_late_controller_window_bind:
pkt_0000:
// +0x0: 3 encoded word(s)
{ r16 = r1; r17 = r0; memd(r29+#-0x10) = r17:16; 	allocframe(#0x8) }
pkt_000c:
// +0xc: 3 encoded word(s)
{ r3 = ##0x3b0090; memw(r16+#0x0) = r3.new }
pkt_0018:
// +0x18: 4 encoded word(s)
{ r1 = add(r17,#0x205); r0 = add(r16,#0x1a); memw(r16+#0x4) = r2; r2 = memw(r17+#0x0) } :mem_noshuf
pkt_0028:
// +0x28: 2 encoded word(s)
{ r3 = memw(r17+#0x4); memw(r16+#0x14) = r3.new }
pkt_0030:
// +0x30: 2 encoded word(s)
{ memw(r16+#0x10) = r2; r2 = memw(r17+#0xc0) } :mem_noshuf
pkt_0038:
// +0x38: 2 encoded word(s)
{ memw(r16+#0xc) = r2; r2 = memw(r17+#0xb8) } :mem_noshuf
pkt_0040:
// +0x40: 2 encoded word(s)
{ memw(r16+#0x8) = r2; r2 = memub(r17+#0x1e3) } :mem_noshuf
pkt_0048:
// +0x48: 1 encoded word(s)
{ memw(r17+#0xbc) = clrbit(#0x0) }
pkt_004c:
// +0x4c: 3 encoded word(s)
{ r2 = #0x6; memb(r16+#0x18) = r2; r3 = memub(r17+#0x1e4) } :mem_noshuf
pkt_0058:
// +0x58: 2 encoded word(s)
{ call -813480; memb(r16+#0x19) = r3 }
pkt_0060:
// +0x60: 4 encoded word(s)
{ call -813488; r1 = add(r17,#0x1e5); r0 = add(r16,#0x20); r2 = memub(r17+#0x1e4) }
pkt_0070:
// +0x70: 4 encoded word(s)
{ r0 = add(r16,#0x42); r1 = add(r17,#0x13b); r2 = memb(r17+#0x13a); if (cmp.eq(r2.new,#0x0)) jump:nt pkt_00b0 }
pkt_0080:
// +0x80: 2 encoded word(s)
{ r2 = memub(r17+#0x138); memb(r16+#0x40) = r2.new }
pkt_0088:
// +0x88: 4 encoded word(s)
{ call -813528; r2 = #0x6; r3 = memub(r17+#0x13a); memb(r16+#0x41) = r3.new }
pkt_0098:
// +0x98: 4 encoded word(s)
{ r1 = add(r17,#0x118); r0 = add(r16,#0x4a); r2 = memub(r17+#0x13a); r17:16 = memd(r29+#0x0) }
pkt_00a8:
// +0xa8: 2 encoded word(s)
{ jump -813560; deallocframe }
pkt_00b0:
// +0xb0: 1 encoded word(s)
{ r17:16 = memd(r29+#0x0); 	dealloc_return }
