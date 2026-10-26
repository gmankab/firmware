// SPDX-License-Identifier: GPL-3.0-only
// Four independent controller slot helper leaves after file 0x12114 zero fill.
// File [0x12118,0x121bc), physical [0x8be0f118,0x8be0f1bc).
.text
.globl adsp_controller_slot_bounds
adsp_controller_slot_bounds:
{ immext(#0xf001e340); r2=##0xf001e360 }
{ nop } // V66 R3=SGP0 encoded via checked register encoder
{ r4=add(r2,#960); r3=memw(r3+#0) }
{ p0=cmp.gtu(r2,r3); p1=!cmp.gtu(r4,r3) }
{ p0=or(p1,p0) }
{ r0=mux(p0,#1,#0); jumpr r31 }
.globl adsp_controller_slot_set_flag17
adsp_controller_slot_set_flag17:
{ r2=r0; r1:0=combine(#3,#11); immext(#0xf00263c0); r4=memw(##0xf00263c4) }
{ r6=setbit(r2,#17) }
{ memw(r4+#0)=r2; immext(#0xf00263c0); r3=memw(##0xf00263c4) }:mem_noshuf
{ jump publish_registration; memw(r3+#0)=r6 }
.globl adsp_controller_slot_publish_bit
adsp_controller_slot_publish_bit:
{ r2=r0; immext(#0xf000eac0); r3=memw(##0xf000eac0) }
{ r1:0=bitsplit(r2,#5) }
{ r4=lsl(#1,r0); memw(r3+r1<<#2)=r4.new }
{ immext(#0xf00250c0); r3=memw(##0xf00250c4) }
{ r3=memw(r3+#0) }
{ r1:0=combine(#3,#11); immext(#0xf00263c0); r3=memw(##0xf00263c4) }
{ jump publish_registration; memw(r3+#0)=r2 }
.globl adsp_controller_slot_set_flag16
adsp_controller_slot_set_flag16:
{ r2=r0; r1:0=combine(#4,#11); immext(#0xf00263c0); r3=memw(##0xf00263c4) }
{ r4=setbit(r2,#16); jump publish_registration; memw(r3+#0)=r4.new }
.set publish_registration,adsp_controller_slot_bounds+(0x8be254f0-0x8be0f118)
