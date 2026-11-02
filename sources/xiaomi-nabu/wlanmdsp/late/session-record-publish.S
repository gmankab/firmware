// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x105f48,0x106044); runtime [0xb00c5f48,0xb00c6044).
// Parser of variable-size script operands, opcode predicates and bounded hardware loops.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_session_record_publish
agent14_late_session_record_publish:
pkt_0000:
// offset +0x0000, 2 encoded words
{ call 1964080; allocframe(#0x18) }
pkt_0008:
// offset +0x0008, 2 encoded words
{ call 1576; r16 = r1; 	r18 = r0 }
pkt_0010:
// offset +0x0010, 2 encoded words
{ r2 = #0xad; p0 = cmp.eq(r0,#0x0); if (!p0.new) jump:nt pkt_00e8 }
pkt_0018:
// offset +0x0018, 2 encoded words
{ call 1425868; r0 = memw(r18+#0x8) }
pkt_0020:
// offset +0x0020, 2 encoded words
{ r17 = r0; if (cmp.eq(r17.new,#0x0)) jump:nt pkt_00a4 }
pkt_0028:
// offset +0x0028, 4 encoded words
{ call -802760; r0 = add(r17,#0x2); r1 = #0x0; r2 = #0x96 }
pkt_0038:
// offset +0x0038, 4 encoded words
{ r3:2 = combine(##0x249f0,r17); r1:0 = memd(gp+#0xf10); memw(r17+#0x8) = r18 }
pkt_0048:
// offset +0x0048, 4 encoded words
{ r0 = add(r17,#0x18); r1 = ##-0x4ff3948c; memd(r17+#0x58) = r1:0 }
pkt_0058:
// offset +0x0058, 2 encoded words
{ memw(r17+#0x60) = r3; memh(r17+#0x0) = r16 }
pkt_0060:
// offset +0x0060, 2 encoded words
{ call 1398612; memw(r17+#0x80) = #0x1 }
pkt_0068:
// offset +0x0068, 3 encoded words
{ r0 = add(r17,#0x64); r1 = #0x0; r2 = #0x1c; memw(r17+#0x4) = #0 }
pkt_0074:
// offset +0x0074, 1 encoded words
{ r3 = memw(r18+#0x10) }
pkt_0078:
// offset +0x0078, 2 encoded words
{ r3 = add(r17,#0x4); memw(r3+#0x0) = r17 }
pkt_0080:
// offset +0x0080, 2 encoded words
{ call -802848; memw(r18+#0x10) = r3 }
pkt_0088:
// offset +0x0088, 2 encoded words
{ call 1425756; r0 = memw(r18+#0x14) }
pkt_0090:
// offset +0x0090, 2 encoded words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt pkt_00c8; memw(r17+#0x78) = r0 }
pkt_0098:
// offset +0x0098, 3 encoded words
{ jump pkt_00c0; r2 = memw(r18+#0x18); memw(r17+#0x7c) = r2.new }
pkt_00a4:
// offset +0x00a4, 3 encoded words
{ r1 = #0x3e41; r0 = ##0x6748; 	memw(r29+#0x0) = r16 }
pkt_00b0:
// offset +0x00b0, 1 encoded words
{ call 1397180 }
pkt_00b4:
// offset +0x00b4, 3 encoded words
{ call -799668; r1:0 = combine(r16,##-0x4fd48990) }
pkt_00c0:
// offset +0x00c0, 2 encoded words
{ r0 = r17 ; jump ##1964108 }
pkt_00c8:
// offset +0x00c8, 3 encoded words
{ r1 = #0x3e41; r0 = ##0x6740; 	memw(r29+#0x0) = r16 }
pkt_00d4:
// offset +0x00d4, 1 encoded words
{ call 1397144 }
pkt_00d8:
// offset +0x00d8, 3 encoded words
{ call -799704; r1:0 = combine(r16,##-0x4fd4899c) }
pkt_00e4:
// offset +0x00e4, 1 encoded words
{ r2 = #0xc6 }
pkt_00e8:
// offset +0x00e8, 4 encoded words
{ r0 = ##-0x4fd489e0; r1 = ##-0x4fd489b7 }
pkt_00f8:
// offset +0x00f8, 1 encoded words
{ call 1404804 }
