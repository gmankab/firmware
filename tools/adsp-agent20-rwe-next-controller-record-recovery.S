// SPDX-License-Identifier: GPL-3.0-only
// ADSP indexed controller-record error recovery [0x13084,0x130c4).
.text
.globl adsp_record_recovery
adsp_record_recovery:
{ r2=memw(r19+#0); memb(r20+#6)=#0 }
{ memw(r20+#12)=r2 }
{ nop } // checked V66 k0lock, original at 0x1308c
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844 }
{ r1:0=combine(#0,#0); memd(r19+#0x88)=r17:16; memw(r19+#0x90)=#0 }
{ r1:0=combine(r18,r19); memd(r19+#8)=r1:0; immext(#0xf0021100); memw(r19+#0xec)=##0xf002111c }
{ call controller_cleanup; r2=memub(r18+#0x15); memw(r18+#0x90)=#0 }
.set controller_notify,adsp_record_recovery+(0x8be261e8-0x8be13084)
.set controller_cleanup,adsp_record_recovery+(0x8be26828-0x8be13084)
