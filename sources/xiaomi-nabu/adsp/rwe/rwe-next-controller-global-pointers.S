// SPDX-License-Identifier: GPL-3.0-only
// Source-assembled MMIO base pointers and controller memory map publication.
// ADSP file [0x11548,0x11594) physical [0x8be0e548,0x8be0e594).
.text
.globl adsp_controller_global_pointers
adsp_controller_global_pointers:
{ call reset_controller_globals }
{ r2=##0xf0208060; immext(#0xf020a040); r3=memw(gp+##0xf020a060) }
{ r2=add(r3,r2); immext(#0xf000eac0); memw(gp+##0xf000eacc)=r2 }
{ immext(#0xf020a280); r2=memw(gp+##0xf020a2b8); immext(#0xf000eac0); memw(gp+##0xf000ead0)=r2 }
{ call setup_global_tables; immext(#0xf000ea00); memw(gp+##0xf000ea18)=r2 }
{ immext(#0xf0026c40); r2=memw(gp+##0xf0026c48); immext(#0xf0026c40); r1=memw(gp+##0xf0026c5c) }
.set reset_controller_globals,adsp_controller_global_pointers+(0x8be15aa0-0x8be0e548)
.set setup_global_tables,adsp_controller_global_pointers+(0x8be14db0-0x8be0e548)
