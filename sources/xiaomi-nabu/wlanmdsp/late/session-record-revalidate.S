// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x107c8c,0x107d30), runtime [0xb00c7c8c,0xb00c7d30).
// Revalidate command inputs and log dispatch failures before returning a status.
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_session_record_revalidate
agent14_late_session_record_revalidate:
pkt_0000:
// +0x0: 2 encoded word(s)
{ call 1956588; allocframe(#0x28) }
pkt_0008:
// +0x8: 3 encoded word(s)
{ r17:16 = combine(r3,r4); r5 = #0xc9; r4 = add(r29,#0x0); 	r7:6 = combine(#0,#0x0) }
pkt_0014:
// +0x14: 3 encoded word(s)
{ p0 = cmp.gtu(r5,r17); r19:18 = combine(r1,r0); memw(r4+#0x10) = #0; 	memd(r29+#0x8) = r7:6 }
pkt_0020:
// +0x20: 2 encoded word(s)
{ if (!p0) jump:nt pkt_0094; memd(r29+#0x0) = r7:6 }
pkt_0028:
// +0x28: 4 encoded word(s)
{ call -809860; r1:0 = combine(r2,##-0x4f958540); r2 = r17 }
pkt_0038:
// +0x38: 2 encoded word(s)
{ if (p0.new) r3:2 = combine(r16,r17); p0 = cmp.eq(r19,#0x3); if (p0.new) jump:nt pkt_0084 }
pkt_0040:
// +0x40: 2 encoded word(s)
{ r1:0 = combine(#0x10,r18); p0 = cmp.eq(r19,#0x2); if (p0.new) jump:nt pkt_0064 }
pkt_0048:
// +0x48: 3 encoded word(s)
{ r3:2 = combine(#0x10,r18); if (p0.new) r4 = add(r16,#0x0); p0 = cmp.eq(r19,#0x1); if (!p0.new) jump:t pkt_0090 }
pkt_0054:
// +0x54: 2 encoded word(s)
{ r0 = ##0xb06a7ac0; 	r1 = zxth(r17) }
pkt_005c:
// +0x5c: 1 encoded word(s)
{ call 517592 }
pkt_0060:
// +0x60: 1 encoded word(s)
{ jump 1956712 }
pkt_0064:
// +0x64: 3 encoded word(s)
{ r3:2 = combine(r17,##-0x4f958540); r4 = add(r29,#0x0) }
pkt_0070:
// +0x70: 1 encoded word(s)
{ call 521764 }
pkt_0074:
// +0x74: 3 encoded word(s)
{ call -809936; r2 = #0x10; r1 = add(r29,#0x0); r0 = r16 }
pkt_0080:
// +0x80: 1 encoded word(s)
{ jump 1956680 }
pkt_0084:
// +0x84: 3 encoded word(s)
{ call 517504; r1:0 = combine(##-0x4f958540,r18) }
pkt_0090:
// +0x90: 1 encoded word(s)
{ jump 1956664 }
pkt_0094:
// +0x94: 4 encoded word(s)
{ call -464; r0 = ##-0x4fd4861b; r1 = #0x128 }
