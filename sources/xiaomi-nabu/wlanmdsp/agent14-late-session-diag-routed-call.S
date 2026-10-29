// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x10904c,0x109078), runtime [0xb00c904c,0xb00c9078).
// Decode controller session diagnostics or flag references
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_session_diag_routed_call
agent14_late_session_diag_routed_call:
pkt_0000:
// +0x0: 3 encoded word(s)
{ r1 = #0x12; r16 = r0; memd(r29+#-0x10) = r17:16; 	allocframe(#0x8) }
pkt_000c:
// +0xc: 1 encoded word(s)
{ call -797672 }
pkt_0010:
// +0x10: 3 encoded word(s)
{ call -797676; r1:0 = combine(##0x83,r16) }
pkt_001c:
// +0x1c: 3 encoded word(s)
{ r0 = ##-0x4fcba74c; r17:16 = memd(r29+#0x0); 	deallocframe }
pkt_0028:
// +0x28: 1 encoded word(s)
{ jump 1022028 }
