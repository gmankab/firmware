// SPDX-License-Identifier: GPL-3.0-only
// V66 controller-cycle snapshot and primitive 64-bit controller counter update.
// ADSP executable file [0x10534,0x10570), physical [0x8be0d534,0x8be0d564).
// The privileged R1:0=PCYCLE and R1=CFGBASE are checked encoder operations.
.text
.globl adsp_rwe_cycle_snapshot
adsp_rwe_cycle_snapshot:
{ nop; immext(#0xf000d680); r5:4=memd(r3=##0xf000d6a0) } // R1:0=PCYCLE
{ r5:4=sub(r1:0,r5:4); memd(r3+#-8)=r1:0 }
{ memd(r3+#8)=r5:4 }
{ memw(r3+#16)+=#1 }
{ dccleaninva(r3) }
{ immext(#0xf000d680); r5:4=memd(r3=##0xf000d688) }
{ nop } // R1=CFGBASE
{ r1=asl(r1,#5); immext(#0); r0=##8 }
{ nop } // R2=memw_phys(R0,R1)
