// SPDX-License-Identifier: GPL-3.0-only
// WLAN runtime file [0x103560,0x1035f8), VMA [0xb00c3560,0xb00c35f8).
// Decode an interface status record and compute a bounded reply
// Typed Hexagon instruction mnemonics, packet layout and local labels;
// external helper names are inferred from observable call sites, not
// independent C-level source or hardware execution evidence.
// No OEM opcode arrays or vendor firmware payload are embedded.
.text
.globl agent14_late_interface_aux_reply
agent14_late_interface_aux_reply:
// +0x0000: 2 encoded words
{ call 1974792; allocframe(#0x18) }
// +0x0008: 3 encoded words
{ r19:18 = combine(r5,r3); r16 = r4; p0 = cmp.eq(r1,#0x2); r2 = memw(r2+#0x10) }
// +0x0014: 2 encoded words
{ r6 = memub(r2+#0x5b); r7 = memub(r2+#0x5a) }
// +0x001c: 3 encoded words
{ r7 |= asl(r6,#0x8); r3 = memub(r2+#0x59); r17 = memub(r2+#0x58) }
// +0x0028: 3 encoded words
{ r17 |= asl(r3,#0x8); r20 = memub(r2+#0x5c); r21 = memub(r2+#0x5d) }
// +0x0034: 2 encoded words
{ r17 |= asl(r7,#0x10); if (p0) jump:nt label_0050 }
// +0x003c: 1 encoded words
{ p0 = cmp.eq(r1,#0x1); if (!p0.new) jump:nt label_0088 }
// +0x0040: 2 encoded words
{ call 488864; r1 = r17; 	r0 = memw(r0+#0x8) }
// +0x0048: 2 encoded words
{ jump label_0070; r2 = zxth(r0); 	memw(r19+#0x0) = #1 }
label_0050:
// +0x0050: 2 encoded words
{ r3 = #0x601; r2 = memuh(r17+#0xc) }
// +0x0058: 1 encoded words
{ r2 = swiz(r2) }
// +0x005c: 1 encoded words
{ r2 = lsr(r2,#0x10) }
// +0x0060: 2 encoded words
{ p0 = cmp.gtu(r3,r2); r2 = #0xe }
// +0x0068: 2 encoded words
{ r3 = mux(p0,#0x1,#0x0); memw(r19+#0x0) = r3.new }
label_0070:
// +0x0070: 3 encoded words
{ r20 |= asl(r21,#0x8); r3 = add(r17,r2); memw(r18+#0x0) = r3.new }
// +0x007c: 3 encoded words
{ r0 = sub(r20,r2); jump 1974888; memw(r16+#0x0) = r0.new }
label_0088:
// +0x0088: 4 encoded words
{ call -2344; r0 = ##-0x4fd48d75; r1 = #0x32c }
