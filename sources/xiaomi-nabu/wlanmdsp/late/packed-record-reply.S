// SPDX-License-Identifier: GPL-3.0-only
// Agent20-14 file [0x1029c4,0x102aa8), runtime [0xb00c29c4,0xb00c2aa8).
// Serialize normalized record bytes into the controller's packed reply buffer,
// preserving the original duplicate status/control writes and alternate
// diagnostic thunks. Two memory packets require :mem_noshuf, and two far
// diagnostic jumps use explicit constant extension (##).
.text
.globl agent14_late_packed_record_reply
agent14_late_packed_record_reply:
// +0x0000: 2 encoded words
{ call 1977780; p0 = cmp.eq(r2,#0x0); 	allocframe(#0x20) }
// +0x0008: 4 encoded words
{ r2 = p0; r18 = r3; r17:16 = combine(r0,r1); memw(r29+#0x4) = r2.new }
// +0x0018: 1 encoded words
{ if (p0) jump:nt label_0030 }
// +0x001c: 4 encoded words
{ call -788656; r1:0 = combine(##-0x4fd48dd0,r16); r2 = #0x8 }
// +0x002c: 1 encoded words
{ r16 = add(r16,#0x8) }
label_0030:
// +0x0030: 4 encoded words
{ call -788676; r1:0 = combine(##-0x4fd48de4,r16); r2 = #0x6 }
// +0x0040: 3 encoded words
{ r0 = add(r16,#0x8); r1 = add(r17,#0x1e); r2 = #0x6; 	p0 = cmp.eq(r18,#0x0) }
// +0x004c: 2 encoded words
{ if (p0) r3 = #0x200; if (!p0) r3 = #0x100 }
// +0x0054: 2 encoded words
{ r3 = lsr(r3,#0x8); memb(r16+#0x6) = r3 }
// +0x005c: 2 encoded words
{ call -788720; memb(r16+#0x7) = r3 }
// +0x0064: 3 encoded words
{ r0 = add(r16,#0x12); r1 = add(r17,#0x18); r3 = memub(r17+#0x8); 	r2 = memub(r17+#0x9) }
// +0x0070: 2 encoded words
{ memb(r16+#0xf) = r2; r2 = memub(r17+#0xa) }:mem_noshuf
// +0x0078: 3 encoded words
{ r2 = #0x6; memb(r16+#0x10) = r2; r4 = memub(r17+#0xb) }:mem_noshuf
// +0x0084: 3 encoded words
{ call -788760; memb(r16+#0x11) = r4; memb(r16+#0xe) = r3 }
// +0x0090: 1 encoded words
{ r2 = memw(r17+#0x4); 	r3 = memw(r29+#0x4) }
// +0x0094: 3 encoded words
{ p0 = r3; r1 = lsr(r2,#0x18); memb(r16+#0x1b) = r1.new }
// +0x00a0: 4 encoded words
{ r1 = lsr(r2,#0x10); r2 = lsr(r2,#0x8); r0 = mux(p0,#0x1c,#0x24); memb(r16+#0x18) = r2 }
// +0x00b0: 3 encoded words
{ jump 1977824; memb(r16+#0x1a) = r1; memb(r16+#0x19) = r2 }
// +0x00bc: 1 encoded words
{ r2 = memw(r0+#0x1b4) }
// +0x00c0: 1 encoded words
{ r2 = add(r2,#0x10) }
// +0x00c4: 3 encoded words
{ r2 = #0xc ; jump ##-793032; r1:0 = combine(r2,r1) }
// +0x00d0: 2 encoded words
{ r1 = #0x0; r2 = memw(r0+#0x1b4) }
// +0x00d8: 3 encoded words
{ r2 = #0xc ; jump ##-793132; r0 = add(r2,#0x10) }
