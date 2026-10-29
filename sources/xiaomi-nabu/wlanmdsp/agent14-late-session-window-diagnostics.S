// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x107710,0x1077e8), runtime [0xb00c7710,0xb00c77e8).
// Diagnose bounded four-byte window records, log entries, and handle null session state.
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_session_window_diagnostics
agent14_late_session_window_diagnostics:
pkt_0000:
// +0x0: 2 encoded word(s)
{ call 1957976; allocframe(#0x28) }
pkt_0008:
// +0x8: 2 encoded word(s)
{ call -4512; r17 = r1 }
pkt_0010:
// +0x10: 2 encoded word(s)
{ r16 = r0; if (cmp.eq(r16.new,#0x0)) jump:nt pkt_00b8 }
pkt_0018:
// +0x18: 4 encoded word(s)
{ r0 = ##0x68a0; r1 = #0x3e12; r2 = memw(r16+#0x6c) }
pkt_0028:
// +0x28: 2 encoded word(s)
{ call 1391228; memw(r29+#0x4) = r2; 	memw(r29+#0x0) = r17 }
pkt_0030:
// +0x30: 4 encoded word(s)
{ call -807968; r1:0 = combine(r17,##-0x4fd486c4); r2 = memw(r16+#0x6c) }
pkt_0040:
// +0x40: 2 encoded word(s)
{ r2 = memw(r16+#0x6c); if (cmp.eq(r2.new,#0x0)) jump:nt pkt_00b4 }
pkt_0048:
// +0x48: 2 encoded word(s)
{ r18 = #0x0; r17 = memw(r16+#0x78) }
pkt_0050:
// +0x50: 4 encoded word(s)
{ r3 = add(r18,#0x4); r19 = add(r17,r18); r0 = ##0x68a8 }
pkt_0060:
// +0x60: 4 encoded word(s)
{ r1 = #0x3e14; p0 = cmp.gtu(r2,r3); r3 = memub(r19+#0x1); r4 = memub(r17+r18<<#0x0) }
pkt_0070:
// +0x70: 2 encoded word(s)
{ if (p0) r20 = #0x4; r5 = memub(r19+#0x2); 	r6 = memub(r19+#0x3) }
pkt_0078:
// +0x78: 2 encoded word(s)
{ if (!p0) r20 = sub(r2,r18); memw(r29+#0xc) = r6; 	memw(r29+#0x8) = r5 }
pkt_0080:
// +0x80: 2 encoded word(s)
{ call 1391140; memw(r29+#0x4) = r3; 	memw(r29+#0x0) = r4 }
pkt_0088:
// +0x88: 4 encoded word(s)
{ r1:0 = combine(#0x4,##0xb02b7948); r3 = memub(r19+#0x1); r2 = memub(r17+r18<<#0x0) }
pkt_0098:
// +0x98: 1 encoded word(s)
{ r4 = memub(r19+#0x2); 	r5 = memub(r19+#0x3) }
pkt_009c:
// +0x9c: 1 encoded word(s)
{ memw(r29+#0xc) = r5; 	memw(r29+#0x8) = r4 }
pkt_00a0:
// +0xa0: 3 encoded word(s)
{ call -808096; r18 = add(r20,r18); memw(r29+#0x4) = r3; 	memw(r29+#0x0) = r2 }
pkt_00ac:
// +0xac: 2 encoded word(s)
{ r2 = memw(r16+#0x6c); if (cmp.gtu(r2.new,r18)) jump:t pkt_0050 }
pkt_00b4:
// +0xb4: 1 encoded word(s)
{ jump 1958016 }
pkt_00b8:
// +0xb8: 3 encoded word(s)
{ r1 = #0x3e11; r0 = ##0x6898; 	memw(r29+#0x0) = r17 }
pkt_00c4:
// +0xc4: 1 encoded word(s)
{ call 1391072 }
pkt_00c8:
// +0xc8: 3 encoded word(s)
{ call 1957888; r1:0 = combine(r17,##-0x4fd486d0) }
pkt_00d4:
// +0xd4: 1 encoded word(s)
{ jump -805788 }
