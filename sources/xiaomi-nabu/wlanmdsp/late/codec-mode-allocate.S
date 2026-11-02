// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x1061ec,0x1062a8); runtime [0xb00c61ec,0xb00c62a8).
// Parser of variable-size script operands, opcode predicates and bounded hardware loops.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_codec_mode_allocate
agent14_late_codec_mode_allocate:
pkt_0000:
// offset +0x0000, 2 encoded words
{ call 1963388; allocframe(#0x20) }
pkt_0008:
// offset +0x0008, 3 encoded words
{ r16 = r3; r19:18 = combine(r1,r2); r21 = memw(gp+#0x78c) }
pkt_0014:
// offset +0x0014, 2 encoded words
{ r17 = memw(r21+#0x3a8); if (cmp.eq(r17.new,#0x0)) jump:nt pkt_0074 }
pkt_001c:
// offset +0x001c, 2 encoded words
{ call 1067864; r0 = #0x1 }
pkt_0024:
// offset +0x0024, 2 encoded words
{ call 1406072; r20 = r0 }
pkt_002c:
// offset +0x002c, 4 encoded words
{ r2 = r0; r1:0 = combine(#0x43,r20); r5:4 = combine(#0x0,#0x0); memw(r29+#0x4) = #0x0 }
pkt_003c:
// offset +0x003c, 2 encoded words
{ call 1409324; memw(r29+#0x0) = #0x0 }
pkt_0044:
// offset +0x0044, 2 encoded words
{ call -737128; r0 = add(r21,#0x1020) }
pkt_004c:
// offset +0x004c, 4 encoded words
{ p0 = cmp.eq(r19,##0x36008); if (p0.new) r1:0 = combine(r18,r17); if (p0.new) r2 = add(r16,#0x0) }
pkt_005c:
// offset +0x005c, 2 encoded words
{ if (p0) call ##4260 }
pkt_0064:
// offset +0x0064, 3 encoded words
{ call 1963400; r0 = add(r21,#0x1030); r1 = #0x1 }
pkt_0070:
// offset +0x0070, 1 encoded words
{ jump -737180 }
pkt_0074:
// offset +0x0074, 4 encoded words
{ r0 = ##0x6770; r1 = #0x3e41; memw(r29+#0x0) = #0x0 }
pkt_0084:
// offset +0x0084, 1 encoded words
{ call 1396548 }
pkt_0088:
// offset +0x0088, 3 encoded words
{ call 1963364; r1:0 = combine(#0x0,##0xb02b76ac) }
pkt_0094:
// offset +0x0094, 1 encoded words
{ jump -800312 }
pkt_0098:
// offset +0x0098, 3 encoded words
{ call -785940; r1 = #0x5b; allocframe(#0x0) }
pkt_00a4:
// offset +0x00a4, 3 encoded words
{ call 1033776; r0 = ##-0x4fcba7cc }
pkt_00b0:
// offset +0x00b0, 2 encoded words
{ r0 = ##0xb034584c; 	deallocframe }
pkt_00b8:
// offset +0x00b8, 1 encoded words
{ jump 1033756 }
