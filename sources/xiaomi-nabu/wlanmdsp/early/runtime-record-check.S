// SPDX-License-Identifier: GPL-3.0-only
// Agent13: bounded WLAN record marker validation against the QuRT table.
// Firmware ELF file [0x42708,0x427a0), VMA [0xb0002708,0xb00027a0).
// Hexagon V66 packets authored from decoded control-flow semantics;
// no original machine-code words, reference payload, or OEM signature.
.text
.globl agent13_runtime_record_check
agent13_runtime_record_check:
{ r17 = r0; r16 = r1; memd(r29+#-16) = r17:16; allocframe(#24) }
{ r1 = #0x412; r0 = ##0x40; r2 = memub(r17+#0) }
{ call 0x218a94; memw(r29+#4) = r16; memw(r29+#0) = r2 }
{ r0 = ##0xb02a6258; r2 = r16; r1 = memub(r17+#0) }
{ call -0x414 }
{ loop0(array_entry,#7); r2 = ##0xb02a6224; memw(r29+#8) = #1 }
array_entry:
{ r3 = memw(r2+#-4); if (cmp.eq(r3.new,r16)) jump:nt found_record }
{ r2 = add(r2,#8); nop }:endloop0
{ r2 = #0; jump lookup_done }
found_record:
{ r2 = memw(r2+#0) }
lookup_done:
{ call 0x20b4f4; r1 = add(r29,#8); r0 = memw(r17+#0x24); memw(r29+#0xc) = r2 }
{ if (p0.new) r0 = #0; p0 = cmp.eq(r0,#0); if (p0.new) jump:nt return_status }
{ r1 = #0x452; r0 = ##0x48; r2 = memub(r17+#0) }
{ call 0x218a34; memw(r29+#4) = r16; memw(r29+#0) = r2 }
{ r0 = ##0xb02a6264; r2 = r16; r1 = memub(r17+#0) }
{ call -0x474 }
{ r0 = #-1 }
return_status:
{ r17:16 = memd(r29+#0x10); dealloc_return }
