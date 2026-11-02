// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x105e70,0x105f48); runtime [0xb00c5e70,0xb00c5f48).
// Parser of variable-size script operands, opcode predicates and bounded hardware loops.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_record_setup
agent14_late_record_setup:
pkt_0000:
// offset +0x0000, 2 encoded words
{ call 1964280; allocframe(#0x18) }
pkt_0008:
// offset +0x0008, 3 encoded words
{ call 5408; r19:18 = combine(r2,r0); r20 = r1 }
pkt_0014:
// offset +0x0014, 3 encoded words
{ r1:0 = combine(##-0x4ff3927c,#0x47); r2 = #0x0; 	r17:16 = combine(r0,#0) }
pkt_0020:
// offset +0x0020, 1 encoded words
{ call 1068816 }
pkt_0024:
// offset +0x0024, 4 encoded words
{ call 1068812; r1:0 = combine(##-0x4ff3982c,#0x48); r2 = #0x0 }
pkt_0034:
// offset +0x0034, 1 encoded words
{ p0 = cmp.eq(r19,#-0x1); if (p0.new) jump:nt pkt_00bc }
pkt_0038:
// offset +0x0038, 2 encoded words
{ r1:0 = combine(#0x24,#0x3); p0 = cmp.eq(r20,#0x0); if (p0.new) jump:nt pkt_00bc }
pkt_0040:
// offset +0x0040, 1 encoded words
{ r3 = #0x2; 	r2 = add(r19,#-0x1) }
pkt_0044:
// offset +0x0044, 4 encoded words
{ r19 = min(r20,r3); call 1403516; p0 = cmp.gtu(r17,r2); if (p0.new) r17 = add(r19,#0x0) }
pkt_0054:
// offset +0x0054, 3 encoded words
{ r16 = r0; r1:0 = combine(r19,##0x98) }
pkt_0060:
// offset +0x0060, 3 encoded words
{ r3:2 = combine(##0x100,#0x1); r5:4 = combine(#3,#0x3); 	memw(r16+#0x18) = r17 }
pkt_006c:
// offset +0x006c, 2 encoded words
{ call 1425252; memw(r16+#0x4) = r18 }
pkt_0074:
// offset +0x0074, 4 encoded words
{ r1:0 = combine(r19,r17); r3:2 = combine(##0x100,#0x1); memw(r16+#0x8) = r0 }
pkt_0084:
// offset +0x0084, 3 encoded words
{ r5:4 = combine(#0x3,#0x3); r6 = add(r16,#0xc); memw(r16+#0x10) = r6.new }
pkt_0090:
// offset +0x0090, 3 encoded words
{ call 1425216; memw(r16+#0xc) = #0x0; memw(r16+#0x0) = #0x4 }
pkt_009c:
// offset +0x009c, 4 encoded words
{ call 1403428; r1:0 = combine(##0x150,#0x1); memw(r16+#0x14) = r0 }
pkt_00ac:
// offset +0x00ac, 3 encoded words
{ if (p0.new) r2 = #0x9d; p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt pkt_00c4; memw(r16+#0x1c) = r0 }
pkt_00b8:
// offset +0x00b8, 1 encoded words
{ memw(r16+#0x20) = #0x0 }
pkt_00bc:
// offset +0x00bc, 2 encoded words
{ r0 = r16 ; jump ##1964312 }
pkt_00c4:
// offset +0x00c4, 4 encoded words
{ r0 = ##-0x4fd489e0; r1 = ##-0x4fd489d2 }
pkt_00d4:
// offset +0x00d4, 1 encoded words
{ call 1405056 }
