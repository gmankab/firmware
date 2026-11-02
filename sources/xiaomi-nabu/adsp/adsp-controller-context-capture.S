// SPDX-License-Identifier: GPL-3.0-only
// Source-authored Hexagon V66 register context capture [0x4e60,0x4f1c).
// All ordinary packets and branch offsets are independently assembled by
// LLVM6; privileged system/control register instructions are encoded from
// explicit architectural register indices after checking their NOP sentinels.
.text
.globl adsp_controller_context_capture
adsp_controller_context_capture:
{ nop } // R23 = HTID
{ r23 = asl(r23,#2) }
{ r23 = asl(r23,#18) }
{ r23.l = #0xdc00 }
{ r23 |= lsr(r23,#16) }
{ r23.h = #0xf000 }
{ nop } // R22 = S54
{ r23 = add(r23,r22); memd(sp+#8) = r3:2 }
{ r3:2 = memd(r23+#0) }
{ r2 = r28; memd(sp+#0) = r3:2; memd(sp+#0x58) = r23:22 }
{ r3 = r1; memd(sp+#0x60) = r25:24; memd(sp+#0x68) = r27:26 }
{ memd(sp+#0x70) = r3:2 }
{ nop; memd(sp+#0x78) = lr:fp; memd(sp+#0x48) = r19:18 } // r2 = p3:0
{ nop; memd(sp+#0x40) = r17:16; memd(sp+#0x38) = r15:14 } // r3 = usr
{ nop; memd(sp+#0x30) = r13:12; memd(sp+#0xa8) = r3:2 } // r2 = gp
{ nop; memd(sp+#0x28) = r11:10; memd(sp+#0x20) = r9:8 } // r3 = ugp
{ nop; memd(sp+#0x18) = r7:6; memd(sp+#0x88) = r3:2 } // r3:2 = LC0:SA0
{ nop; memd(sp+#0x10) = r5:4; memd(sp+#0x90) = r3:2 } // r3:2 = LC1:SA1
{ nop; memd(sp+#0x98) = r3:2 } // r3:2 = M1:0
{ memd(sp+#0xa0) = r3:2 }
{ nop } // r3:2 = CS1:0
{ nop; memd(sp+#0xb8) = r3:2 } // r2 = ccr
{ nop; memw(sp+#0xc0) = r2 } // r3 = ssr
{ nop } // r2 = ELR
{ nop; memd(sp+#0x80) = r3:2 } // r2 = BADVA
{ nop } // r3 = STID
{ memd(sp+#0xb0) = r3:2; memd(sp+#0x50) = r21:20 }
{ nop } // r23 = HTID
{ p0 = cmp.eq(r23,#0); if (p0.new) jump:nt thread0 }
{ jump other_thread }
.set thread0, adsp_controller_context_capture + 0xbc
.set other_thread, adsp_controller_context_capture + 0x374
