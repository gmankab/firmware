// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x107f38,0x107f7c), runtime [0xb00c7f38,0xb00c7f7c).
// Summarize a short session status record and pass it to the controller.
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_session_short_status
agent14_late_session_short_status:
pkt_0000:
// +0x0: 2 encoded word(s)
{ call 1955888; allocframe(#0x28) }
pkt_0008:
// +0x8: 3 encoded word(s)
{ r17:16 = combine(r1,r2); r1 = #0x0; r2 = #0x10; r0 = memw(r29+#0x30) }
pkt_0014:
// +0x14: 3 encoded word(s)
{ call -810916; r19:18 = combine(r5,r3); r20 = r4 }
pkt_0020:
// +0x20: 4 encoded word(s)
{ call -716; r1:0 = combine(r17,r19); r3:2 = combine(r18,r16); r4 = add(r29,#0x0) }
pkt_0030:
// +0x30: 3 encoded word(s)
{ call -804332; r2 = #0x10; r1 = add(r29,#0x0); r0 = r20 }
pkt_003c:
// +0x3c: 2 encoded word(s)
{ r0 = cmp.eq(r0,#0x0); jump 1956048 }
