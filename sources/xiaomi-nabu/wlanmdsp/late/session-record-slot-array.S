// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x109078,0x1090dc), runtime [0xb00c9078,0xb00c90dc).
// Allocate fixed-stride record-array slots with loop0 and controller handoff
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_session_record_slot_array
agent14_late_session_record_slot_array:
pkt_0000:
// +0x0: 2 encoded word(s)
{ call 1951488; allocframe(#0x10) }
pkt_0008:
// +0x8: 3 encoded word(s)
{ call 1390768; r1:0 = combine(#0x10,#0x3); r17 = r1; 	r18 = r0 }
pkt_0014:
// +0x14: 2 encoded word(s)
{ r16 = r0; memw(r18+#0x2a8) = r16.new }
pkt_001c:
// +0x1c: 3 encoded word(s)
{ r18 = +mpyi(r17,##0x218); memw(r16+#0x0) = r18 }
pkt_0028:
// +0x28: 2 encoded word(s)
{ call 1390736; r1:0 = combine(r18,#0x3) }
pkt_0030:
// +0x30: 2 encoded word(s)
{ r2 = add(r18,#-0x218); p0 = cmp.eq(r17,#0x0); if (p0.new) jump:nt pkt_0054 }
pkt_0038:
// +0x38: 2 encoded word(s)
{ loop0(pkt_0040,r17); r3 = #0x0; 	r4 = r0 }
pkt_0040:
// +0x40: 3 encoded word(s)
{ r3 = r4; r4 = add(r4,#0x218); memw(r4+#0xb4) = r3 }  :endloop0
pkt_004c:
// +0x4c: 2 encoded word(s)
{ r2 = add(r0,r2); jump pkt_0058 }
pkt_0054:
// +0x54: 1 encoded word(s)
{ r2 = #0x0 }
pkt_0058:
// +0x58: 3 encoded word(s)
{ r0 = r16 ; jump ##1951620; memw(r16+#0xc) = r2 }
