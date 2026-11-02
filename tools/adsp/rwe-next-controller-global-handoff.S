// SPDX-License-Identifier: GPL-3.0-only
// Hexagon global-controller physical pointer and conditional startup handoff.
// ADSP file [0x11624,0x1167c), physical [0x8be0e624,0x8be0e67c).
.text
.globl adsp_controller_global_handoff
adsp_controller_global_handoff:
{ call initialize_controller_memory }
{ r0=#1; immext(#0xf0026c40); r2=memw(gp+##0xf0026c48) }
{ r3=add(r2,##0x20000); immext(#0xf0025080); memw(gp+##0xf0025090)=r3.new }
{ call prepare_physical_controller; memw(r2+##0x20004)=r0 }
{ immext(#0xf02d1200); r2=memw(gp+##0xf02d1230); if(cmp.eq(r2.new,#0)) jump:nt irq_ready }
{ call handoff_system_registers; r1:0=combine(#1,#1); r3:2=combine(#0,#63) }
{ r1:0=combine(#1,#1); r3:2=combine(#1,#63); r17:16=memd(r29+#8); r19:18=memd(r29+#0) }
{ jump handoff_system_registers; deallocframe }
.set initialize_controller_memory,adsp_controller_global_handoff+(0x8be0eae0-0x8be0e624)
.set prepare_physical_controller,adsp_controller_global_handoff+(0x8be1c560-0x8be0e624)
.set handoff_system_registers,adsp_controller_global_handoff+(0x8be176f8-0x8be0e624)
.set irq_ready,adsp_controller_global_handoff+(0x8be0e67c-0x8be0e624)
