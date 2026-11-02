// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x106128,0x1061ec); runtime [0xb00c6128,0xb00c61ec).
// Parser of variable-size script operands, opcode predicates and bounded hardware loops.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_codec_mode_router
agent14_late_codec_mode_router:
pkt_0000:
// offset +0x0000, 1 encoded words
{ memd(r29+#-0x10) = r17:16; 	allocframe(#0x10) }
pkt_0004:
// offset +0x0004, 1 encoded words
{ r4 = memw(gp+#0x78c) }
pkt_0008:
// offset +0x0008, 2 encoded words
{ r16 = memw(r4+#0x3a8); if (cmp.eq(r16.new,#0x0)) jump:nt pkt_0058 }
pkt_0010:
// offset +0x0010, 1 encoded words
{ r1:0 = bitsplit(r1,#0x8) }
pkt_0014:
// offset +0x0014, 3 encoded words
{ p0 = cmp.eq(r1,##0x360); if (!p0.new) jump:t pkt_00c0 }
pkt_0020:
// offset +0x0020, 2 encoded words
{ r4 = add(r0,#-0x1); if (cmp.gtu(r4.new,#0x6)) jump:nt pkt_00c0 }
pkt_0028:
// offset +0x0028, 4 encoded words
{ r4 = ##-0x4ff39ca0; r5 = memw(r4<<#0x2+##0xb02b75f8) }
pkt_0038:
// offset +0x0038, 1 encoded words
{ jumpr r5 }
pkt_003c:
// offset +0x003c, 3 encoded words
{ r1 = #0x3e41; r0 = ##0x6768; 	memw(r29+#0x0) = r16 }
pkt_0048:
// offset +0x0048, 1 encoded words
{ call 1396804 }
pkt_004c:
// offset +0x004c, 3 encoded words
{ r1:0 = combine(r16,##-0x4fd48960); jump pkt_0074 }
pkt_0058:
// offset +0x0058, 4 encoded words
{ r0 = ##0x6760; r1 = #0x3e41; memw(r29+#0x0) = #0x0 }
pkt_0068:
// offset +0x0068, 1 encoded words
{ call 1396772 }
pkt_006c:
// offset +0x006c, 2 encoded words
{ r1:0 = combine(#0x0,##0xb02b7694) }
pkt_0074:
// offset +0x0074, 2 encoded words
{ jump -800084; r17:16 = memd(r29+#0x8); 	deallocframe }
pkt_007c:
// offset +0x007c, 3 encoded words
{ r4 = ##-0x4ff39c0c; jump pkt_00b4 }
pkt_0088:
// offset +0x0088, 3 encoded words
{ r4 = ##-0x4ff399c8; jump pkt_00b4 }
pkt_0094:
// offset +0x0094, 3 encoded words
{ r4 = ##-0x4ff39630; jump pkt_00b4 }
pkt_00a0:
// offset +0x00a0, 3 encoded words
{ r4 = ##-0x4ff39074; jump pkt_00b4 }
pkt_00ac:
// offset +0x00ac, 2 encoded words
{ r4 = ##-0x4ff38fa0 }
pkt_00b4:
// offset +0x00b4, 3 encoded words
{ r1:0 = combine(r2,r16); callr r4; r2 = r3 }
pkt_00c0:
// offset +0x00c0, 1 encoded words
{ r17:16 = memd(r29+#0x8); 	dealloc_return }
