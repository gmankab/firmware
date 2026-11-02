// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x106b74,0x106bc8); runtime [0xb00c6b74,0xb00c6bc8).
// Check command descriptor and update controller event queue.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_session_command_update
agent14_late_session_command_update:
pkt_0000:
// offset +0x0000, 2 encoded words
{ call 1960964; allocframe(#0x10) }
pkt_0008:
// offset +0x0008, 2 encoded words
{ r16 = r1; r2 = memw(gp+#0x78c) }
pkt_0010:
// offset +0x0010, 2 encoded words
{ r17 = add(r16,#0x18); r3 = memuh(r16+#0x0) }
pkt_0018:
// offset +0x0018, 2 encoded words
{ r2 = addasl(r2,r3,#0x2); r0 = r17 }
pkt_0020:
// offset +0x0020, 2 encoded words
{ call 1396168; r18 = memw(r2+#0x54) }
pkt_0028:
// offset +0x0028, 1 encoded words
{ r2 = memuh(r18+#0x1c) }
pkt_002c:
// offset +0x002c, 2 encoded words
{ r2 = and(r2,#0x3); if (!cmp.eq(r2.new,#0x3)) jump:t pkt_0050 }
pkt_0034:
// offset +0x0034, 2 encoded words
{ call 32; r0 = r16 }
pkt_003c:
// offset +0x003c, 4 encoded words
{ call 1961020; r0 = r17; r4 = #0x0; r3:2 = memd(gp+#0xd58) }
pkt_004c:
// offset +0x004c, 1 encoded words
{ jump 1396000 }
pkt_0050:
// offset +0x0050, 1 encoded words
{ jump 1961104 }
