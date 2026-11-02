// SPDX-License-Identifier: GPL-3.0-only
// Controller mode clock/performance snapshot [0x13a2c,0x13a7c).
.text
.globl adsp_controller_mode_clock
adsp_controller_mode_clock:
{ nop } // checked V66 paired R1:0=PCYCLE
{ call controller_cycle_publish; immext(#0xf000d680); memd(gp+##0xf000d6a0)=r1:0 }
{ immext(#0xf0024300); r2=memw(gp+##0xf0024328); immext(#0xf000d680); memd(gp+##0xf000d680)=r1:0 }
{ p0=cmp.eq(r2,#0); if(p0.new) jump:nt restore_baseline; immext(#0xf0010d80); if(!p0.new) memw(sp+#4)=##0xf0010d8c }
{ immext(#0xf000e3c0); r0=##0xf000e3c9; r2=#0x92; memw(sp+#8)=r2.new }
{ call assert_addr_range; memw(sp+#0)=#0xf }
{ call terminate_startup; r0=#0xf05 }
trap_mode:
{ jump trap_mode }
.set controller_cycle_publish,adsp_controller_mode_clock+(0x8be26378-0x8be13a2c)
.set restore_baseline,adsp_controller_mode_clock+(0x13b24-0x13a2c)
.set assert_addr_range,adsp_controller_mode_clock+(0x8be14438-0x8be13a2c)
.set terminate_startup,adsp_controller_mode_clock+(0x8be257f4-0x8be13a2c)
