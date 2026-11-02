// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x106bc8,0x106cfc); runtime [0xb00c6bc8,0xb00c6cfc).
// Validate event flags, stream window and command counters.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_session_command_validate
agent14_late_session_command_validate:
pkt_0000:
// offset +0x0000, 3 encoded words
{ call 1399336; r16 = r0; memd(r29+#-0x10) = r17:16; 	allocframe(#0x18) }
pkt_000c:
// offset +0x000c, 4 encoded words
{ r0 = ##0x67d8; r2 = memub(r16+#0x16); memd(r29+#0x0) = r1:0 }
pkt_001c:
// offset +0x001c, 4 encoded words
{ call 1394128; r1 = #0x2452; r2 = and(r2,#0x2); memw(r29+#0x8) = r2.new }
pkt_002c:
// offset +0x002c, 1 encoded words
{ call 1399292 }
pkt_0030:
// offset +0x0030, 2 encoded words
{ r5:4 = combine(r1,r0); r2 = memub(r16+#0x16) }
pkt_0038:
// offset +0x0038, 4 encoded words
{ call -805088; r2 = and(r2,#0x2); r1:0 = combine(r4,##-0x4fd4883c) }
pkt_0048:
// offset +0x0048, 1 encoded words
{ r2 = memub(r16+#0x16) }
pkt_004c:
// offset +0x004c, 3 encoded words
{ p0 = !tstbit(r2,#0x1); if (p0.new) r0 = add(r16,#0x0); if (!p0.new) jump:t pkt_00ac }
pkt_0058:
// offset +0x0058, 1 encoded words
{ call 220 }
pkt_005c:
// offset +0x005c, 2 encoded words
{ call 292; r17 = r0; 	r0 = r16 }
pkt_0064:
// offset +0x0064, 1 encoded words
{ r2 = memw(r16+#0x4c) }
pkt_0068:
// offset +0x0068, 1 encoded words
{ r3 = sub(r17,r2) }
pkt_006c:
// offset +0x006c, 2 encoded words
{ r2 = memw(r16+#0x60); if (!cmp.gtu(r2.new,r3)) jump:t pkt_00fc }
pkt_0074:
// offset +0x0074, 1 encoded words
{ r3 = memw(r16+#0x50) }
pkt_0078:
// offset +0x0078, 2 encoded words
{ r3 = sub(r0,r3); if (!cmp.gtu(r2,r3.new)) jump:t pkt_00fc }
pkt_0080:
// offset +0x0080, 2 encoded words
{ call -644; r0 = memub(r16+#0x0) }
pkt_0088:
// offset +0x0088, 4 encoded words
{ r0 = ##0x67e8; r1 = #0x2450; memb(r16+#0x16) = setbit(#0x1) }
pkt_0098:
// offset +0x0098, 1 encoded words
{ call 1394004 }
pkt_009c:
// offset +0x009c, 3 encoded words
{ call -805196; r0 = ##-0x4fd48824 }
pkt_00a8:
// offset +0x00a8, 1 encoded words
{ jump pkt_00fc }
pkt_00ac:
// offset +0x00ac, 2 encoded words
{ r2 = memw(r16+#0x38); if (cmp.eq(r2.new,#0x0)) jump:nt pkt_00fc }
pkt_00b4:
// offset +0x00b4, 1 encoded words
{ r1 = memw(r16+#0x3c) }
pkt_00b8:
// offset +0x00b8, 2 encoded words
{ r3 = memw(r16+#0x5c); if (cmp.gtu(r3.new,r1)) jump:t pkt_00fc }
pkt_00c0:
// offset +0x00c0, 2 encoded words
{ r0 = +mpyi(r2,#0x64); call -806104 }
pkt_00c8:
// offset +0x00c8, 2 encoded words
{ r2 = memw(r16+#0x58); if (!cmp.gtu(r0,r2.new)) jump:t pkt_00fc }
pkt_00d0:
// offset +0x00d0, 4 encoded words
{ call 1393948; r0 = ##0x67e0; r1 = #0x2450 }
pkt_00e0:
// offset +0x00e0, 3 encoded words
{ call -805264; r0 = ##-0x4fd48830 }
pkt_00ec:
// offset +0x00ec, 3 encoded words
{ call -2572; r1 = #0x4; r2 = #0x1; r0 = memub(r16+#0x0) }
pkt_00f8:
// offset +0x00f8, 1 encoded words
{ memb(r16+#0x16) = clrbit(#0x1) }
pkt_00fc:
// offset +0x00fc, 2 encoded words
{ r1:0 = combine(#0x0,#0x0); memw(r16+#0x44) = #0x0 }
pkt_0104:
// offset +0x0104, 2 encoded words
{ call 1399076; memd(r16+#0x38) = r1:0 }
pkt_010c:
// offset +0x010c, 4 encoded words
{ call 40; r0 = r16; memw(r16+#0x40) = r0; memw(r16+#0x48) = #0x0 }
pkt_011c:
// offset +0x011c, 3 encoded words
{ call 100; r0 = r16; memw(r16+#0x4c) = r0 }
pkt_0128:
// offset +0x0128, 2 encoded words
{ memw(r16+#0x50) = r0; memw(r16+#0x54) = #0x1 }
pkt_0130:
// offset +0x0130, 1 encoded words
{ r17:16 = memd(r29+#0x10); 	dealloc_return }
