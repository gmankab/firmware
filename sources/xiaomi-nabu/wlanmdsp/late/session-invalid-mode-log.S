// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x107970,0x107998), runtime [0xb00c7970,0xb00c7998).
// Log an invalid session mode and return through the platform assertion handler.
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_session_invalid_mode_log
agent14_late_session_invalid_mode_log:
pkt_0000:
// +0x0: 3 encoded word(s)
{ r1 = #0x3e11; r0 = ##0x68d8; 	allocframe(#0x10) }
pkt_000c:
// +0xc: 1 encoded word(s)
{ r16 = r3; 	memd(r29+#0x8) = r17:16 }
pkt_0010:
// +0x10: 2 encoded word(s)
{ call 1390644; memw(r29+#0x0) = r16 }
pkt_0018:
// +0x18: 3 encoded word(s)
{ r1:0 = combine(r16,##-0x4fd48670); r17:16 = memd(r29+#0x8); 	deallocframe }
pkt_0024:
// +0x24: 1 encoded word(s)
{ jump -806220 }
