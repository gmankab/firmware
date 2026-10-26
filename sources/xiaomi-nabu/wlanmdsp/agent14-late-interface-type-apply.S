// SPDX-License-Identifier: GPL-3.0-only
// Agent14 WLAN runtime instruction reconstruction: file [0x104524,0x1045e8).
// Independent Hexagon V66 mnemonics, control-flow labels and packet boundaries; no OEM opcode arrays.
// Validate a tagged type record, look up a context, update a bounded
// 16-bit field, and issue the branch to a shared return. Packet order
// near offset 0x88 is required to retain the original duplex choice.
.text
.globl agent14_late_interface_type_apply
agent14_late_interface_type_apply:
// offset 0x0000; 2 encoded words
{ call 1970756; allocframe(#0x18) }
// offset 0x0008; 3 encoded words
{ call -541356; r19:18 = combine(r2,r5); r17 = r4;  r20 = r3 }
// offset 0x0014; 2 encoded words
{ r16 = #0x0; p0 = cmp.eq(r20,#0x0); if (p0.new) jump:nt label_0064 }
// offset 0x001c; 1 encoded words
{ p0 = cmp.eq(r19,#0x0); if (p0.new) jump:nt label_0064 }
// offset 0x0020; 2 encoded words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt label_0064; if (!p0.new) r2 = memub(r20+#0x4) }
// offset 0x0028; 2 encoded words
{ p0 = tstbit(r2,#0); if (!p0.new) jump:t label_003c; if (p0.new) r2 = memh(r20+#0x8) }
// offset 0x0030; 3 encoded words
{ p0 = cmp.eq(r2,##0x806); if (!p0.new) jump:t label_0048 }
label_003c:
// offset 0x003c; 1 encoded words
{ r2 = memub(r20+#0x2a) }
// offset 0x0040; 2 encoded words
{ p0 = tstbit(r2,#0); if (p0.new) jump:t label_006c; if (p0.new) r2 = memub(r20+#0x2b) }
label_0048:
// offset 0x0048; 4 encoded words
{ call 1403976; r0 = ##0x66a8; r1 = #0x3e50 }
// offset 0x0058; 3 encoded words
{ call -795236; r0 = ##-0x4fd48b70 }
label_0064:
// offset 0x0064; 2 encoded words
{ r0 = r16 ; jump ##1970876 }
label_006c:
// offset 0x006c; 2 encoded words
{ r19 = add(r19,r2); if (cmp.eq(r19.new,#0x0)) jump:nt label_0064 }
// offset 0x0074; 4 encoded words
{ call -789532; r1:0 = combine(##-0x4fd48de4,r19); r2 = #0x6 }
// offset 0x0084; 1 encoded words
{ p0 = cmp.eq(r0,#0x0); if (!p0.new) jump:t label_0064 }
// offset 0x0088; 3 encoded words
{ p0 = cmp.eq(r18,#0x0); r1:0 = combine(#0x0,r17); r2 = #0x6; r16 = memub(r19+#0x6) }
// offset 0x0094; 1 encoded words
{ r3 = memub(r19+#0x7) }
// offset 0x0098; 3 encoded words
{ r16 |= asl(r3,#0x8); call -796180; if (!p0) memh(r18+#0x0) = r16.new }
// offset 0x00a4; 4 encoded words
{ p0 = cmp.eq(r16,##0x200); r16 = #0x1; if (!p0.new) jump:t label_0064 }
// offset 0x00b4; 3 encoded words
{ call -795816; r1 = add(r19,#0x8); r2 = #0x6;  r0 = r17 }
// offset 0x00c0; 1 encoded words
{ jump label_0064 }
