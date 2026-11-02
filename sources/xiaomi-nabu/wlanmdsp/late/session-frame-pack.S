// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x108288,0x108368), runtime [0xb00c8288,0xb00c8368).
// Copy frame fields, normalize endianness and report the resulting session descriptor.
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_session_frame_pack
agent14_late_session_frame_pack:
pkt_0000:
// +0x0: 2 encoded word(s)
{ call 1955056; allocframe(#0x10) }
pkt_0008:
// +0x8: 2 encoded word(s)
{ r2 = #0x68; r18 = r0; 	r17 = r1 }
pkt_0010:
// +0x10: 1 encoded word(s)
{ r16 = memw(r18+#0x34) }
pkt_0014:
// +0x14: 2 encoded word(s)
{ call -811764; r1:0 = combine(#0x0,r16) }
pkt_001c:
// +0x1c: 2 encoded word(s)
{ r19 = add(r18,#0x118); r2 = memw(r18+#0x8) }
pkt_0024:
// +0x24: 2 encoded word(s)
{ r2 = memub(r2+#0x1e); memw(r16+#0x18) = r2.new }
pkt_002c:
// +0x2c: 4 encoded word(s)
{ p0 = cmp.eq(r2,#0x2); if (!p0.new) r1 = add(r19,#0x0); r3 = memub(r18+#0x138); memw(r16+#0x10) = r3.new }
pkt_003c:
// +0x3c: 2 encoded word(s)
{ memw(r16+#0x14) = #0x1; r2 = memub(r18+#0x139) }:mem_noshuf
pkt_0044:
// +0x44: 3 encoded word(s)
{ if (!p0) jump:nt pkt_008c; memw(r16+#0x54) = r2; r0 = memw(r18+#0x40) }:mem_noshuf
pkt_0050:
// +0x50: 2 encoded word(s)
{ call -811432; r2 = #0x10; 	r1 = r19 }
pkt_0058:
// +0x58: 2 encoded word(s)
{ r1 = add(r19,#0x18); r2 = memw(r18+#0x40) }
pkt_0060:
// +0x60: 3 encoded word(s)
{ call -811448; r0 = add(r2,#0x10); r2 = #0x8 }
pkt_006c:
// +0x6c: 2 encoded word(s)
{ r1 = add(r19,#0x10); r2 = memw(r18+#0x40) }
pkt_0074:
// +0x74: 3 encoded word(s)
{ call -811468; r0 = add(r2,#0x18); r2 = #0x8 }
pkt_0080:
// +0x80: 3 encoded word(s)
{ jump pkt_0094; memw(r16+#0x58) = #0x8; memw(r16+#0x5c) = #0x8 }
pkt_008c:
// +0x8c: 2 encoded word(s)
{ call -811492; r2 = memub(r18+#0x139) }
pkt_0094:
// +0x94: 2 encoded word(s)
{ r2 = memub(r17+#0x40); r3 = memub(r17+#0x3f) }
pkt_009c:
// +0x9c: 3 encoded word(s)
{ r3 |= asl(r2,#0x8); r5 = memub(r17+#0x44); r4 = memub(r17+#0x3e) }
pkt_00a8:
// +0xa8: 2 encoded word(s)
{ r6 = memub(r17+#0x43); r1 = memub(r17+#0x3d) }
pkt_00b0:
// +0xb0: 4 encoded word(s)
{ r1 |= asl(r4,#0x8); r6 |= asl(r5,#0x8); r7 = memub(r17+#0x42); r8 = memub(r17+#0x41) }
pkt_00c0:
// +0xc0: 2 encoded word(s)
{ r1 |= asl(r3,#0x10); r8 |= asl(r7,#0x8) }
pkt_00c8:
// +0xc8: 3 encoded word(s)
{ r8 |= asl(r6,#0x10); r2 = swiz(r1); memw(r16+#0x20) = r2.new }
pkt_00d4:
// +0xd4: 3 encoded word(s)
{ r0 = swiz(r8); jump 1955064; memw(r16+#0x1c) = r0.new }
