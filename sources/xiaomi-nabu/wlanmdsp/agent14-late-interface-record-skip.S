// SPDX-License-Identifier: GPL-3.0-only
// Agent14 WLAN runtime instruction reconstruction: file [0x1044f0,0x104524).
// Independent Hexagon V66 mnemonics, control-flow labels and packet boundaries; no OEM opcode arrays.
// Advance across records when the header type and length are valid;
// tail-call a copy helper for type-2 records.
.text
.globl agent14_late_interface_record_skip
agent14_late_interface_record_skip:
// offset 0x0000; 2 encoded words
{ p0 = cmp.gt(r1,#0x7); if (!p0.new) jumpr:nt r31 }
label_0008:
// offset 0x0008; 2 encoded words
{ r3 = memb(r0+#0x0); if (cmp.eq(r3.new,#0x2)) jump:nt label_0028 }
// offset 0x0010; 2 encoded words
{ r3 = memub(r0+#0x1); if (cmp.eq(r3.new,#0x0)) jump:nt label_0024 }
// offset 0x0018; 2 encoded words
{ r1 -= asl(r3,#0x3); r0 = addasl(r0,r3,#0x3) }
// offset 0x0020; 1 encoded words
{ p0 = cmp.gt(r1,#0x7); if (p0.new) jump:t label_0008 }
label_0024:
// offset 0x0024; 1 encoded words
{ jumpr r31 }
label_0028:
// offset 0x0028; 2 encoded words
{ r1 = add(r0,#0x2); r2 = #0x6;  r0 = r2 }
// offset 0x0030; 1 encoded words
{ jump -795632 }
