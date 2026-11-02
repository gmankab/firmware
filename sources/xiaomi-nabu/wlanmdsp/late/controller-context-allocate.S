// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x108b34,0x108bd4), runtime [0xb00c8b34,0xb00c8bd4).
// Allocate a bounded session context and register the session owner
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_controller_context_allocate
agent14_late_controller_context_allocate:
pkt_0000:
// +0x0: 2 encoded word(s)
{ call 1952820; allocframe(#0x18) }
pkt_0008:
// +0x8: 3 encoded word(s)
{ call -559292; r17:16 = combine(r0,r1); r0 = memw(gp+#0x78c) }
pkt_0014:
// +0x14: 2 encoded word(s)
{ r2 = memub(r0+#0x250); r3 = memw(r0+#0x28) }
pkt_001c:
// +0x1c: 1 encoded word(s)
{ r2 = addasl(r3,r2,#0x2) }
pkt_0020:
// +0x20: 1 encoded word(s)
{ r2 = memw(r2+#0x328) }
pkt_0024:
// +0x24: 1 encoded word(s)
{ r2 = memw(r2+#0x24) }
pkt_0028:
// +0x28: 3 encoded word(s)
{ call 1057284; r0 = memw(r2+##0x1458) }
pkt_0034:
// +0x34: 3 encoded word(s)
{ call 1399160; r2 = #0x9c; r1 = #0x3e; 	r18 = r0 }
pkt_0040:
// +0x40: 2 encoded word(s)
{ r19 = r0; if (cmp.eq(r19.new,#0x0)) jump:nt pkt_0080 }
pkt_0048:
// +0x48: 2 encoded word(s)
{ r20 = add(r19,#0x8); r2 = #0x94 }
pkt_0050:
// +0x50: 2 encoded word(s)
{ call -814044; r1:0 = combine(#0x0,r20) }
pkt_0058:
// +0x58: 3 encoded word(s)
{ r1:0 = combine(r20,r17); r3 = ##0x19001; 	r2 = r16 }
pkt_0064:
// +0x64: 2 encoded word(s)
{ r3 = ##0x40094; 	memw(r19+#0x0) = r3 }
pkt_006c:
// +0x6c: 2 encoded word(s)
{ call -288; memw(r19+#0x4) = r3 }
pkt_0074:
// +0x74: 2 encoded word(s)
{ call 1952816; r0 = r18 }
pkt_007c:
// +0x7c: 1 encoded word(s)
{ jump 1399120 }
pkt_0080:
// +0x80: 4 encoded word(s)
{ call 1385984; r0 = ##0x69b0; r1 = #0x2450 }
pkt_0090:
// +0x90: 3 encoded word(s)
{ call 1952788; r0 = ##-0x4fd48494 }
pkt_009c:
// +0x9c: 1 encoded word(s)
{ jump -813240 }
