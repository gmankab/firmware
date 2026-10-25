// SPDX-License-Identifier: GPL-3.0-only
// Source-authored Hexagon register/interrupt state save [0x505c,0x5108).
// LLVM6 assembles allocframe/dealloc_return, memory packets and moves;
// v66-only system/control/interrupt mask reads use checked typed encoders.
.text
.globl adsp_controller_save_control
adsp_controller_save_control:
{ nop; r18 = r28; allocframe(#8) } // V66 r19 = P3:0
{ memd(sp+#0) = r19:18 }
{ nop } // r18 = PMUCFG
{ nop } // r19 = MODECTL
{ memd(r0+#0) = r19:18 }
{ nop } // r19:18 = UTIMER
{ memd(r0+#8) = r19:18 }
{ nop } // r18 = EVB
{ nop } // r19 = DIAG
{ memd(r0+#0x10) = r19:18 }
{ nop } // r18 = S20
{ nop } // r19 = S20
{ memd(r0+#0x18) = r19:18 }
{ nop } // r18 = REV
{ nop } // r19 = SGP0
{ r18 = r1; r19 = #0; memd(r0+#0x28) = r19:18 }
{ r1 = #0; memd(r0+#0x40) = r19:18 }
{ nop } // V66 r28 = getimask(r1)
{ r1 = #1; memw(r0+#0x30) = r28 }
{ nop } // V66 r28 = getimask(r1)
{ r1 = #2; memw(r0+#0x34) = r28 }
{ nop } // V66 r28 = getimask(r1)
{ r1 = #3; memw(r0+#0x38) = r28 }
{ nop } // V66 r28 = getimask(r1)
{ r19:18 = memd(sp+#0); memw(r0+#0x3c) = r28 }
{ nop } // r28 = S63
{ memw(r0+#0x44) = r28 }
{ nop } // r28 = S60
{ memw(r0+#0x48) = r28 }
{ nop } // r28 = S35
{ memw(r0+#0x58) = r28 }
{ nop; r28 = r18; dealloc_return } // V66 P3:0 = r19
