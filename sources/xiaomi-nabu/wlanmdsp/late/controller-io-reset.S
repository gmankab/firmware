// SPDX-License-Identifier: GPL-3.0-only
// WLAN runtime file [0x1035f8,0x103660), VMA [0xb00c35f8,0xb00c3660).
// Check and clear controller state-byte pairs
// Typed Hexagon instruction mnemonics, packet layout and local labels;
// external helper names are inferred from observable call sites, not
// independent C-level source or hardware execution evidence.
// No OEM opcode arrays or vendor firmware payload are embedded.
.text
.globl agent14_late_controller_io_reset
agent14_late_controller_io_reset:
// +0x0000: 2 encoded words
{ r2 = r0; memd(r29+#-0x10) = r17:16; 	allocframe(#0x8) }
// +0x0008: 4 encoded words
{ r16 = add(r2,#0x786); r3 = memb(r2+##0x786); if (cmp.eq(r3.new,#0x0)) jump:nt label_0034 }
// +0x0018: 3 encoded words
{ r0 = ##0x6618; r1 = #0x2410 }
// +0x0024: 1 encoded words
{ call 1407896 }
// +0x0028: 3 encoded words
{ r0 = ##-0x4fd48c48; jump label_005c }
label_0034:
// +0x0034: 4 encoded words
{ r16 = add(r2,#0x787); r3 = memb(r2+##0x787); if (cmp.eq(r3.new,#0x0)) jump:nt label_0064 }
// +0x0044: 3 encoded words
{ r0 = ##0x6620; r1 = #0x2410 }
// +0x0050: 1 encoded words
{ call 1407852 }
// +0x0054: 2 encoded words
{ r0 = ##-0x4fd48c3c }
label_005c:
// +0x005c: 1 encoded words
{ call -791356 }
// +0x0060: 1 encoded words
{ memb(r16+#0x0) = #0x0 }
label_0064:
// +0x0064: 1 encoded words
{ r17:16 = memd(r29+#0x0); 	dealloc_return }
