// SPDX-License-Identifier: GPL-3.0-only
// ADSP [file 0x11798,0x1180c); controller init calls and MMIO publication.
.text
.globl adsp_publish_sram_controller
adsp_publish_sram_controller:
{ call init_sram_controller }
{ call setup_common_controller }
{ call configure_service }
{ immext(#0xf000ea00); r2=memw(gp+##0xf000ea1c); immext(#0xf01f10c0); memw(gp+##0xf01f10f0)=r2.new }
{ immext(#0xf01f1100); r0=##0xf01f1100; immext(#0xf000cf00); r1=##0xf000cf20 }
{ r2=#0x400; immext(#0xf000ea00); r3=memw(gp+##0xf000ea20) }
{ immext(#0xf0003000); r3=memw(gp+##0xf0003010); immext(#0xf01f10c0); memw(gp+##0xf01f10f4)=r3 }
{ call setup_hardware; immext(#0xf01f10c0); memw(gp+##0xf01f10f8)=r3 }
{ r0=#26; r2=#1; immext(#0xf000eac0); memw(gp+##0xf000eac8)=r2.new }
{ call initialize_channel }
{ call issue_status; immext(#0xf000e9c0); r0=##0xf000e9f4 }
.set init_sram_controller,adsp_publish_sram_controller+(0x8be0e8c0-0x8be0e798)
.set setup_common_controller,adsp_publish_sram_controller+(0x8be1c550-0x8be0e798)
.set configure_service,adsp_publish_sram_controller+(0x8be17db0-0x8be0e798)
.set setup_hardware,adsp_publish_sram_controller+(0x8be268c0-0x8be0e798)
.set initialize_channel,adsp_publish_sram_controller+(0x8be23268-0x8be0e798)
.set issue_status,adsp_publish_sram_controller+(0x8be11438-0x8be0e798)
