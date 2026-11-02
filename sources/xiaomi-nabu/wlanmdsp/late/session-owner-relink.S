// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x108a14,0x108a80), runtime [0xb00c8a14,0xb00c8a80).
// Rebind session owner and backpointer after lock-managed lookup
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_session_owner_relink
agent14_late_session_owner_relink:
pkt_0000:
// +0x0: 2 encoded word(s)
{ r17:16 = combine(r2,r0); memd(r29+#-0x10) = r17:16; 	allocframe(#0x8) }
pkt_0008:
// +0x8: 2 encoded word(s)
{ call -559004; r0 = memw(r16+#0x0) }
pkt_0010:
// +0x10: 3 encoded word(s)
{ r3 = r0; r0 = memw(r0+#0xec); if (!cmp.eq(r0.new,#0x0)) jump:t pkt_0068 }
pkt_001c:
// +0x1c: 2 encoded word(s)
{ r0 = #0x0; p0 = cmp.eq(r17,#0x0); if (p0.new) jump:nt pkt_0068 }
pkt_0024:
// +0x24: 2 encoded word(s)
{ r17 = memw(r16+#0xc); if (cmp.eq(r17.new,#0x0)) jump:nt pkt_0068 }
pkt_002c:
// +0x2c: 3 encoded word(s)
{ r0 = add(r17,#0x10); r2 = memw(r17+#0xb4); memw(r16+#0xc) = r2.new }
pkt_0038:
// +0x38: 4 encoded word(s)
{ r1 = ##-0x4ff38668; r2 = r17; memw(r17+#0xb4) = #0x0 }
pkt_0048:
// +0x48: 3 encoded word(s)
{ call 1387680; memw(r3+#0xec) = r17; memw(r17+#0x8) = r3 }
pkt_0054:
// +0x54: 3 encoded word(s)
{ r0 = r17; r2 = add(r17,#0x4c); memw(r17+#0x34) = r2.new }
pkt_0060:
// +0x60: 2 encoded word(s)
{ r2 = add(r17,#0x1c3); memw(r17+#0x40) = r2.new }
pkt_0068:
// +0x68: 1 encoded word(s)
{ r17:16 = memd(r29+#0x0); 	dealloc_return }
