// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x107998,0x107b60), runtime [0xb00c7998,0xb00c7b60).
// Validate codec session resources, notify controller callbacks, and allocate a new descriptor.
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_session_controller_create
agent14_late_session_controller_create:
pkt_0000:
// +0x0: 2 encoded word(s)
{ call 1957328; allocframe(#0x28) }
pkt_0008:
// +0x8: 3 encoded word(s)
{ r1 = #0x2411; r0 = ##0x68e0; 	r16 = r1 }
pkt_0014:
// +0x14: 2 encoded word(s)
{ r3 = memw(r16+#0xc0); memw(r29+#0x0) = r3.new }
pkt_001c:
// +0x1c: 3 encoded word(s)
{ r3 = memw(##0xb02b79a0); memw(r29+#0xc) = r3.new }
pkt_0028:
// +0x28: 4 encoded word(s)
{ call 1390580; r2 = memw(##0xb02b799c); memw(r29+#0x8) = r2.new }
pkt_0038:
// +0x38: 4 encoded word(s)
{ call -806280; r0 = ##-0x4fd4865c; r1 = memw(r16+#0xc0) }
pkt_0048:
// +0x48: 1 encoded word(s)
{ r17 = memw(r16+#0x8) }
pkt_004c:
// +0x4c: 2 encoded word(s)
{ r20 = memw(r17+#0xc); if (cmp.eq(r20.new,#0x0)) jump:nt pkt_01a8 }
pkt_0054:
// +0x54: 1 encoded word(s)
{ call 1395716 }
pkt_0058:
// +0x58: 3 encoded word(s)
{ r19:18 = combine(r1,r0); r3 = memw(r16+#0x30); if (cmp.eq(r3.new,#0x0)) jump:nt pkt_00b8 }
pkt_0064:
// +0x64: 3 encoded word(s)
{ r1:0 = combine(r20,r17); r4 = #-0x1; r5 = add(r29,#0x8); r2 = memw(r20+#0xc) }
pkt_0070:
// +0x70: 3 encoded word(s)
{ r4 ^= asl(r2,#0x3); memw(r29+#0x0) = #0x2; memw(r29+#0x8) = r16 }
pkt_007c:
// +0x7c: 3 encoded word(s)
{ call -758920; r2 = and(r4,#0x10); r4 = #-0x1 }
pkt_0088:
// +0x88: 2 encoded word(s)
{ r19 = r0; if (cmp.eq(r19.new,#0x0)) jump:nt pkt_00ec }
pkt_0090:
// +0x90: 2 encoded word(s)
{ call 312; r0 = r16 }
pkt_0098:
// +0x98: 3 encoded word(s)
{ r1 = #0x2451; r0 = ##0x68f0; 	memw(r29+#0x0) = r19 }
pkt_00a4:
// +0xa4: 1 encoded word(s)
{ call 1390456 }
pkt_00a8:
// +0xa8: 3 encoded word(s)
{ call -806392; r1:0 = combine(r19,##-0x4fd485bc) }
pkt_00b4:
// +0xb4: 1 encoded word(s)
{ jump pkt_00d4 }
pkt_00b8:
// +0xb8: 4 encoded word(s)
{ call 1390436; r0 = ##0x68e8; r1 = #0x2450 }
pkt_00c8:
// +0xc8: 3 encoded word(s)
{ call -808776; r0 = ##-0x4fd485c8 }
pkt_00d4:
// +0xd4: 1 encoded word(s)
{ r18 = #0x4 }
pkt_00d8:
// +0xd8: 4 encoded word(s)
{ call 452; r1:0 = combine(r18,r16); r3:2 = combine(#-0x1,#-0x1); memw(r16+#0x30) = #0x0 }
pkt_00e8:
// +0xe8: 1 encoded word(s)
{ jump 1957316 }
pkt_00ec:
// +0xec: 2 encoded word(s)
{ r2 = memb(r16+#0x139); if (cmp.eq(r2.new,#0x0)) jump:nt pkt_014c }
pkt_00f4:
// +0xf4: 4 encoded word(s)
{ call 1390376; r0 = ##0x68f8; r1 = #0x3e10 }
pkt_0104:
// +0x104: 3 encoded word(s)
{ call -808836; r0 = ##-0x4fd485b0 }
pkt_0110:
// +0x110: 3 encoded word(s)
{ call -291380; r2 = add(r16,#0x34); r1:0 = combine(r20,r17) }
pkt_011c:
// +0x11c: 2 encoded word(s)
{ r2 = memb(r16+#0x1e4); if (cmp.eq(r2.new,#0x0)) jump:nt pkt_0144 }
pkt_0124:
// +0x124: 4 encoded word(s)
{ r2 = and(r2,#0xff); r5 = memw(##0xb03441f4); r0 = memub(r17+#0x3) }
pkt_0134:
// +0x134: 4 encoded word(s)
{ r3 = add(r16,#0x205); callr r5; r4 = add(r16,#0x1e5); r1 = memub(r16+#0x1e3) }
pkt_0144:
// +0x144: 2 encoded word(s)
{ r2 = #0x0; memb(r16+#0x139) = r2.new }
pkt_014c:
// +0x14c: 2 encoded word(s)
{ r2 = memw(r16+#0x20c); memw(r16+#0xc0) += #0x1 }
pkt_0154:
// +0x154: 2 encoded word(s)
{ if (!p0.new) r2 = sub(r18,r2); p0 = cmp.eq(r2,#0x0); if (p0.new) jump:nt pkt_0198 }
pkt_015c:
// +0x15c: 2 encoded word(s)
{ r3 = #0x1f5; if (!cmp.gtu(r3.new,r2)) jump:nt pkt_0198 }
pkt_0164:
// +0x164: 3 encoded word(s)
{ r3 = #0xa; r18 = #0x0; r2 = memub(r16+#0x210) }
pkt_0170:
// +0x170: 2 encoded word(s)
{ r2 = add(r2,#0x1); memb(r16+#0x210) = r2.new }
pkt_0178:
// +0x178: 2 encoded word(s)
{ r4 = and(r2,#0xff); if (cmp.gtu(r3,r4.new)) jump:t pkt_00d8 }
pkt_0180:
// +0x180: 3 encoded word(s)
{ r2 = memw(##0xb0346060); r0 = memw(r17+#0xc) }
pkt_018c:
// +0x18c: 1 encoded word(s)
{ callr r2 }
pkt_0190:
// +0x190: 2 encoded word(s)
{ jump pkt_00d8; memw(r16+#0xb8) = setbit(#0x2) }
pkt_0198:
// +0x198: 2 encoded word(s)
{ r2 = #0x1; memw(r16+#0x20c) = r18 }
pkt_01a0:
// +0x1a0: 2 encoded word(s)
{ r18 = #0x0 ; jump pkt_00d8; memb(r16+#0x210) = r2 }
pkt_01a8:
// +0x1a8: 4 encoded word(s)
{ call pkt_01b8; r0 = ##-0x4fd4863d; r1 = #0xbd }
pkt_01b8:
// +0x1b8: 1 encoded word(s)
{ r3:2 = combine(r0,r1) }
pkt_01bc:
// +0x1bc: 3 encoded word(s)
{ r1:0 = combine(r3,##-0x4fd48650); jump 1397872 }
