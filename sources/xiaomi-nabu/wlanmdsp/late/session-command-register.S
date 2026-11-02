// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x107b60,0x107c34), runtime [0xb00c7b60,0xb00c7c34).
// Validate controller state and record bounds before transmitting command data.
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_session_command_register
agent14_late_session_command_register:
pkt_0000:
// +0x0: 2 encoded word(s)
{ call 1956888; allocframe(#0x10) }
pkt_0008:
// +0x8: 2 encoded word(s)
{ r16 = r0; r3 = memw(gp+#0x78c) }
pkt_0010:
// +0x10: 1 encoded word(s)
{ r2 = memw(r16+#0x8) }
pkt_0014:
// +0x14: 1 encoded word(s)
{ r2 = memub(r2+#0x3) }
pkt_0018:
// +0x18: 1 encoded word(s)
{ r2 = addasl(r3,r2,#0x2) }
pkt_001c:
// +0x1c: 1 encoded word(s)
{ r2 = memw(r2+#0x54) }
pkt_0020:
// +0x20: 2 encoded word(s)
{ r0 = memw(r2+#0x28); r2 = memub(r2+#0x250) }
pkt_0028:
// +0x28: 1 encoded word(s)
{ r2 = addasl(r0,r2,#0x2) }
pkt_002c:
// +0x2c: 1 encoded word(s)
{ r2 = memw(r2+#0x328) }
pkt_0030:
// +0x30: 1 encoded word(s)
{ r2 = memw(r2+#0x24) }
pkt_0034:
// +0x34: 3 encoded word(s)
{ call 1061324; r0 = memw(r2+##0x1458) }
pkt_0040:
// +0x40: 3 encoded word(s)
{ call 1403200; r1 = #0x3e; r2 = #0x14; r17 = r0 }
pkt_004c:
// +0x4c: 2 encoded word(s)
{ r18 = r0; if (cmp.eq(r18.new,#0x0)) jump:nt pkt_00b4 }
pkt_0054:
// +0x54: 3 encoded word(s)
{ call -809996; r0 = add(r18,#0xc); r1 = #0x0; 	r2 = #0x8 }
pkt_0060:
// +0x60: 4 encoded word(s)
{ r2 = ##0x19002; r1 = ##0x4000c }
pkt_0070:
// +0x70: 3 encoded word(s)
{ r0 = ##0x490008; memw(r18+#0x8) = r0.new }
pkt_007c:
// +0x7c: 2 encoded word(s)
{ r0 = r17; memw(r18+#0x0) = r2; 	memw(r18+#0x4) = r1 }
pkt_0084:
// +0x84: 1 encoded word(s)
{ r2 = memw(r16+#0x8) }
pkt_0088:
// +0x88: 3 encoded word(s)
{ call 1403160; r2 = memub(r2+#0x3); memw(r18+#0x10) = r2.new }
pkt_0094:
// +0x94: 4 encoded word(s)
{ call 1390016; r0 = ##0x6950; r1 = #0x3e50 }
pkt_00a4:
// +0xa4: 3 encoded word(s)
{ call 1956840; r0 = ##-0x4fd48524 }
pkt_00b0:
// +0xb0: 1 encoded word(s)
{ jump -809208 }
pkt_00b4:
// +0xb4: 4 encoded word(s)
{ call 1389984; r0 = ##0x6948; r1 = #0x2450 }
pkt_00c4:
// +0xc4: 3 encoded word(s)
{ call 1956808; r0 = ##-0x4fd48530 }
pkt_00d0:
// +0xd0: 1 encoded word(s)
{ jump -809240 }
