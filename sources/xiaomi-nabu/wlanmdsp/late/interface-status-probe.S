// SPDX-License-Identifier: GPL-3.0-only
// Agent14 WLAN runtime instruction reconstruction: file [0x104488,0x1044f0).
// Independent Hexagon V66 mnemonics, control-flow labels and packet boundaries; no OEM opcode arrays.
// Temporarily clear two tag bytes, call a status helper, restore them,
// and emit a diagnostic when the returned 16-bit value differs.
.text
.globl agent14_late_interface_status_probe
agent14_late_interface_status_probe:
// offset 0x0000; 2 encoded words
{ call 1970912; allocframe(#0x20) }
// offset 0x0008; 1 encoded words
{ r16 = r1 }
// offset 0x000c; 1 encoded words
{ r19 = memub(r16+#0x2);  memb(r16+#0x2) = #0 }
// offset 0x0010; 2 encoded words
{ r18 = r19; r20 = memb(r16+#0x3);  memb(r16+#0x3) = #0 }
// offset 0x0018; 2 encoded words
{ r18 |= asl(r20,#0x8); call -264 }
// offset 0x0020; 3 encoded words
{ r17 = r0; r0 = #0x1; memb(r16+#0x3) = r20;  memb(r16+#0x2) = r19 }
// offset 0x002c; 2 encoded words
{ r2 = sxth(r17); if (cmp.eq(r2.new,r18)) jump:t label_0064 }
// offset 0x0034; 3 encoded words
{ r1 = #0x3e52; r0 = ##0x66a0;  r2 = zxth(r18) }
// offset 0x0040; 2 encoded words
{ call 1404140; memw(r29+#0x4) = r17;  memw(r29+#0x0) = r2 }
// offset 0x0048; 3 encoded words
{ r0 = ##-0x4fd48b7c; r2 = memub(r16+#0x3);  r1 = memub(r16+#0x2) }
// offset 0x0054; 3 encoded words
{ r1 |= asl(r2,#0x8); call -795068; r2 = r17 }
// offset 0x0060; 1 encoded words
{ r0 = #0x0 }
label_0064:
// offset 0x0064; 1 encoded words
{ jump 1971032 }
