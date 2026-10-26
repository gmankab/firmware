// SPDX-License-Identifier: GPL-3.0-only
// Follow-on controller-global state publication and conditional startup.
// ADSP file [0x114a8,0x11530); physical [0x8be0e4a8,0x8be0e530).
// Continues the 0x11450 function. Branches to 0x11548 beyond this fragment.
.text
.globl adsp_controller_global_memory
adsp_controller_global_memory:
{ r17=#0; r7=add(r6,#960); immext(#0xf00250c0); memw(gp+##0xf00250cc)=r1 }
{ immext(#0xf020a100); r0=memw(gp+##0xf020a114); immext(#0xf00250c0); memw(gp+##0xf00250d0)=r0.new }
{ immext(#0xf01eb000); r18=memw(gp+##0xf01eb018); immext(#0xf0021f00); if(!p0) memw(##0xf0021f20)=r4 }
{ p0=cmp.eq(r18,#0); immext(#0xf020a100); r1=memw(gp+##0xf020a10c) }
{ immext(#0xf020a100); r0=memw(gp+##0xf020a110); immext(#0xf00250c0); memw(gp+##0xf00250d4)=r1 }
{ immext(#0xf00250c0); memw(gp+##0xf00250d8)=r0 }
{ immext(#0xf00250c0); memw(r2=##0xf00250e0)=r6 }
{ nop; nop; memw(r2+#4)=r3; memw(r2+#8)=r7 }
{ immext(#0xf0010840); memw(gp+##0xf001084c)=r17; immext(#0xf0010840); memw(gp+##0xf0010848)=r17 }
{ immext(#0xf02d1240); if(!p0) r16=##0xf02d1250; if(p0) jump:nt global_after_loop; nop }
{ call global_init_sub; immext(#0xf0010840); r1:0=combine(r16,##0xf001084c); r16=add(r16,#32) }
{ r17=add(r17,#1); if(cmp.gtu(r18,r17.new)) jump:t global_init_loop }
.set global_init_sub,adsp_controller_global_memory+(0x8be23214-0x8be0e4a8)
.set global_init_loop,adsp_controller_global_memory+(0x8be0e530-0x8be0e4a8)

.set global_after_loop,adsp_controller_global_memory+(0x8be0e548-0x8be0e4a8)
