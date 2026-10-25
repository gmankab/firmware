// SPDX-License-Identifier: GPL-3.0-only
// Controller boot service routing based on R26 mode 4 or 5, recording phase
// markers, synchronized global mode flag and disabling debug interrupts.
// ADSP executable MBN [0x104b8,0x10534), physical [0x8be0d4b8,0x8be0d534).
.text
.globl adsp_rwe_boot_phase
adsp_rwe_boot_phase:
{ immext(#0xf0024640); memw(sp+#0)=##0xf0024660 }
{ immext(#0xf00250c0); r0=memw(##0xf00250c0) }
{ r1=memw(r0+#0) }
{ syncht }
{ p0=cmp.eq(r26,#4); p1=cmp.eq(r26,#5) }
{ if(p0) jump:nt start_mode4 }
{ if(!p1) jump:nt check_controller }
{ immext(#0xffff4480); call start_mode5 }
check_controller:
{ immext(#0x8080); call controller_synchronize }
{ r1=#1 }
{ p0=cmp.eq(r26,#5); if(p0.new) r1=#3 }
{ jump record_phase }
start_mode4:
{ immext(#0x7f80); call start_mode4_service }
{ r1=#2 }
record_phase:
{ immext(#0xf0003000); memw(r3=##0xf0003008)=r1 }
{ dccleaninva(r3) }
{ immext(#0); r1=##0 }
{ p0=cmp.eq(r1,#0); if(p0.new) jump:t skip_optional }
{ callr r1 }
skip_optional:
{ r0=#0 }
{ nop } // checked V66 ISDBEN=R0
{ isync }
.set start_mode5,adsp_rwe_boot_phase+(0x8be01980-0x8be0d4b8)
.set controller_synchronize,adsp_rwe_boot_phase+(0x8be15568-0x8be0d4b8)
.set start_mode4_service,adsp_rwe_boot_phase+(0x8be15484-0x8be0d4b8)
