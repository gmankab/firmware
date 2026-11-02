// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x1069d0,0x106ab4); runtime [0xb00c69d0,0xb00c6ab4).
// Dispatch packet descriptors by session type and flags.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_session_packet_dispatch
agent14_late_session_packet_dispatch:
pkt_0000:
// offset +0x0000, 2 encoded words
{ call 1961384; allocframe(#0x18) }
pkt_0008:
// offset +0x0008, 1 encoded words
{ r19 = memw(r1+#0x0) }
pkt_000c:
// offset +0x000c, 1 encoded words
{ r2 = memw(r19+#0x4) }
pkt_0010:
// offset +0x0010, 2 encoded words
{ r17 = memw(r0+#0xc); if (cmp.eq(r17.new,#0x0)) jump:nt pkt_002c }
pkt_0018:
// offset +0x0018, 1 encoded words
{ r18 = memub(r19+#0x8); 	r16 = memub(r19+#0xc) }
pkt_001c:
// offset +0x001c, 2 encoded words
{ r3 = memuh(r17+#0x0); if (cmp.eq(r3.new,r2)) jump:nt pkt_004c }
pkt_0024:
// offset +0x0024, 2 encoded words
{ r17 = memw(r17+#0x4); if (!cmp.eq(r17.new,#0x0)) jump:t pkt_001c }
pkt_002c:
// offset +0x002c, 3 encoded words
{ r1 = #0x3e41; r0 = ##0x67b0; 	memw(r29+#0x0) = r2 }
pkt_0038:
// offset +0x0038, 1 encoded words
{ call 1394604 }
pkt_003c:
// offset +0x003c, 3 encoded words
{ call 1961440; r0 = ##0xb02b7788; 	r1 = memw(r19+#0x4) }
pkt_0048:
// offset +0x0048, 1 encoded words
{ jump -802256 }
pkt_004c:
// offset +0x004c, 3 encoded words
{ r1 = #0x2412; r0 = ##0x67b8; 	memw(r29+#0x4) = r16 }
pkt_0058:
// offset +0x0058, 2 encoded words
{ call 1394572; memw(r29+#0x0) = r18 }
pkt_0060:
// offset +0x0060, 4 encoded words
{ call -804624; r1:0 = combine(r18,##-0x4fd4886c); r2 = r16 }
pkt_0070:
// offset +0x0070, 1 encoded words
{ p0 = cmp.eq(r18,#0x4); if (p0.new) jump:nt pkt_0078 }
pkt_0074:
// offset +0x0074, 1 encoded words
{ p0 = cmp.eq(r18,#0x2); if (!p0.new) jump:nt pkt_0088 }
pkt_0078:
// offset +0x0078, 2 encoded words
{ call -948; r1:0 = combine(r18,r17) }
pkt_0080:
// offset +0x0080, 2 encoded words
{ jump pkt_00a4; memb(r17+#0x14) = r18 }
pkt_0088:
// offset +0x0088, 3 encoded words
{ r1 = #0x2451; r0 = ##0x67c0; 	memw(r29+#0x0) = r18 }
pkt_0094:
// offset +0x0094, 1 encoded words
{ call 1394512 }
pkt_0098:
// offset +0x0098, 3 encoded words
{ call -802336; r1:0 = combine(r18,##-0x4fd48860) }
pkt_00a4:
// offset +0x00a4, 2 encoded words
{ r2 = #0x114; p0 = cmp.gtu(r16,#0x8); if (p0.new) jump:nt pkt_00c4 }
pkt_00ac:
// offset +0x00ac, 3 encoded words
{ p0 = tstbit(r2,r16); if (p0.new) r1:0 = combine(r16,r17); if (!p0.new) jump:nt pkt_00c4 }
pkt_00b8:
// offset +0x00b8, 1 encoded words
{ call -876 }
pkt_00bc:
// offset +0x00bc, 2 encoded words
{ jump 1961416; memb(r17+#0x15) = r16 }
pkt_00c4:
// offset +0x00c4, 3 encoded words
{ r1 = #0x2451; r0 = ##0x67c8; 	memw(r29+#0x0) = r16 }
pkt_00d0:
// offset +0x00d0, 1 encoded words
{ call 1394452 }
pkt_00d4:
// offset +0x00d4, 3 encoded words
{ call 1961288; r1:0 = combine(r16,##-0x4fd48854) }
pkt_00e0:
// offset +0x00e0, 1 encoded words
{ jump -802408 }
