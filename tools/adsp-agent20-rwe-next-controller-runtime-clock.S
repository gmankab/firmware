// SPDX-License-Identifier: GPL-3.0-only
// ADSP runtime/controller clock state [0x13850,0x138ec), independent function.
.text
.globl adsp_runtime_clock
adsp_runtime_clock:
{ immext(#0xf0010dc0); r17=##0xf0010dd0; memd(r29+#-16)=r17:16; allocframe(#16) }
{ call controller_prepare; memd(sp+#0)=r19:18 }
{ nop } // source-checked privileged R1:0=PCYCLE
{ r3:2=memd(r17+#80) }
{ call controller_clock }
{ r1:0=add(r3:2,r1:0) }
{ r1:0=combine(#4,#8); r18=#0 }
{ r2=#0; r16=memw(r17+#28) }
{ call controller_init }
{ memb(r16+#21)=#-1; immext(#0xf0003000); memw(gp+##0xf0003008)=r18 }
{ r0.h=#0xf000 }
{ r0.l=#0x3008 }
{ dccleaninva(r0) }
{ r1=memw(r0+#0) }
{ r0.h=#0xf002 }
{ r0.l=#0x6c58 }
{ r0=memw(r0+#0) }
{ nop } // checked start(R0)
{ nop } // checked k0lock
{ call controller_wakeup; r0=r16 }
{ nop } // checked k0unlock
{ nop } // source-checked privileged R3:2=PCYCLE
{ r1:0=memd(r17+#80); immext(#0xf00233c0); memd(gp+##0xf00233c0)=r3:2 }
{ r5:4=sub(r3:2,r1:0); r0=#0 }
{ immext(#0xf000d680); memd(gp+##0xf000d6b8)=r5:4; r17:16=memd(sp+#8) }:mem_noshuf
{ jump controller_complete; r19:18=memd(r29+#0); deallocframe }
.set controller_prepare,adsp_runtime_clock+(0x8be138f0-0x8be13850)
.set controller_clock,adsp_runtime_clock+(0x8be248fc-0x8be13850)
.set controller_init,adsp_runtime_clock+(0x8be284f0-0x8be13850)
.set controller_wakeup,adsp_runtime_clock+(0x8be26c70-0x8be13850)
.set controller_complete,adsp_runtime_clock+(0x8be26a54-0x8be13850)
