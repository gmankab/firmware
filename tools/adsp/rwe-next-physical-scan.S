// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 physical scan and controller-cycle synchronization.
// ADSP executable [0x10570,0x105c4), 21 source instructions.
// Three memw_phys(Ra,Rb) operations and the wait(R0) are privileged V66
// instructions, emitted by checked typed operand encoders at validated slots.
.text
.globl adsp_rwe_physical_scan
adsp_rwe_physical_scan:
{ r0=asl(r2,#5) }
{ r7=add(r0,#66); r6=#4; r8=#0 }
scan_controller:
{ nop } // checked R1=memw_phys(R6,R7)
{ nop } // checked R0=memw_phys(R8,R7)
{ nop } // checked R2=memw_phys(R6,R7)
{ p0=cmp.eq(r2,r1); if(!p0.new) jump:nt scan_controller }
{ memd(r3+#0)=r1:0 }
{ dccleaninva(r3) }
{ immext(#0x15cc0); call process_controller_snapshot; r0=#12 }
{ syncht }
wait_controller:
{ nop } // checked wait(R0)
{ jump wait_controller }
{ immext(#0xf000d680); r3:2=memd(r0=##0xf000d6a8) }
{ jumpr lr; r5:4=memd(r0+#16); r0=memw(r0+#8) }
.set process_controller_snapshot,adsp_rwe_physical_scan+(0x8be23268-0x8be0d570)
