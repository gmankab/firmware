// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x106ab4,0x106b74); runtime [0xb00c6ab4,0xb00c6b74).
// Check command format, session flags and callbacks.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_session_command_check
agent14_late_session_command_check:
pkt_0000:
// offset +0x0000, 2 encoded words
{ call 1961156; allocframe(#0x18) }
pkt_0008:
// offset +0x0008, 3 encoded words
{ r16 = r0; r3 = #0x4; r17 = memw(gp+#0x78c) }
pkt_0014:
// offset +0x0014, 1 encoded words
{ r2 = memw(r16+#0x0) }
pkt_0018:
// offset +0x0018, 2 encoded words
{ r4 = add(r2,#-0x3); if (!cmp.gtu(r3,r4.new)) jump:nt pkt_0050 }
pkt_0020:
// offset +0x0020, 2 encoded words
{ r0 = memw(r16+#0x4); if (cmp.eq(r0.new,#0x0)) jump:nt pkt_009c }
pkt_0028:
// offset +0x0028, 1 encoded words
{ r2 = memub(r0+#0x1c) }
pkt_002c:
// offset +0x002c, 4 encoded words
{ p0 = !tstbit(r2,#0x1); if (p0.new) jump:t pkt_009c; if (!p0.new) r1 = memw(r17+##0x3a8) }
pkt_003c:
// offset +0x003c, 2 encoded words
{ p0 = cmp.eq(r1,#0x0); if (!p0.new) r2 = add(r16,#0x0) }
pkt_0044:
// offset +0x0044, 2 encoded words
{ if (!p0) call ##2216 }
pkt_004c:
// offset +0x004c, 1 encoded words
{ jump 1961300 }
pkt_0050:
// offset +0x0050, 2 encoded words
{ r3 = add(r2,#-0x1); if (cmp.gtu(r3.new,#0x1)) jump:nt pkt_00a0 }
pkt_0058:
// offset +0x0058, 3 encoded words
{ r18 = #0x0; r19 = memub(r17+#0x140); if (cmp.eq(r19.new,#0x0)) jump:nt pkt_009c }
pkt_0064:
// offset +0x0064, 2 encoded words
{ call -551064; r1:0 = combine(r18,r17) }
pkt_006c:
// offset +0x006c, 2 encoded words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt pkt_0094; if (!p0.new) r2 = memub(r0+#0x1c) }
pkt_0074:
// offset +0x0074, 4 encoded words
{ p0 = !tstbit(r2,#0x1); if (p0.new) jump:t pkt_0094; if (!p0.new) r1 = memw(r17+##0x3a8) }
pkt_0084:
// offset +0x0084, 2 encoded words
{ p0 = cmp.eq(r1,#0x0); if (!p0.new) r2 = add(r16,#0x0) }
pkt_008c:
// offset +0x008c, 2 encoded words
{ if (!p0) call ##2144 }
pkt_0094:
// offset +0x0094, 2 encoded words
{ r18 = add(r18,#0x1); if (!cmp.eq(r18.new,r19)) jump:t pkt_0064 }
pkt_009c:
// offset +0x009c, 1 encoded words
{ jump 1961220 }
pkt_00a0:
// offset +0x00a0, 3 encoded words
{ r1 = #0x2451; r0 = ##0x67d0; 	memw(r29+#0x0) = r2 }
pkt_00ac:
// offset +0x00ac, 1 encoded words
{ call 1394260 }
pkt_00b0:
// offset +0x00b0, 3 encoded words
{ call 1961096; r0 = ##0xb02b77b8; 	r1 = memw(r16+#0x0) }
pkt_00bc:
// offset +0x00bc, 1 encoded words
{ jump -802600 }
