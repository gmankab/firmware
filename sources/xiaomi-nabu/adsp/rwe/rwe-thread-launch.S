// SPDX-License-Identifier: GPL-3.0-only
// Reset controller event and restart selected Hexagon thread state, file [0x6474,0x64b4).
.text
.globl adsp_agent20_rwe_thread_launch
adsp_agent20_rwe_thread_launch:
{ immext(#0xf0000040); r1=##-0xfffffbc }
{ immext(#0); memw(r1+#0)=##31 }
{ dccleana(r1) }
{ nop; immext(#0xf0003c80); r0=##-0xfffc380 } // V66 R1=HTID
{ r0+=mpyi(r1,#4) }
{ r1=add(r1,#0); memw(r0+#0)=r1.new } // V66 R1=MODECTL
{ dccleana(r0) }
{ dcinva(r0) }
{ r1=memw(r0+#0) }
{ call service_restore }
loop:
{ jump loop }
.set service_restore,adsp_agent20_rwe_thread_launch+(0x8be237b8-0x8be03474)
