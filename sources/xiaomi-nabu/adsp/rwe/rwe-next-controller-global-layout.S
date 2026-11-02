// SPDX-License-Identifier: GPL-3.0-only
// Source-level ADSP global decoder/controller memory pool pointer layout.
// Original file [0x11594,0x11624), physical [0x8be0e594,0x8be0e624).
.text
.globl adsp_controller_global_layout
adsp_controller_global_layout:
{ r0=add(r2,##0x10000); r3=add(r2,##0x10100) }
{ immext(#0xf000ea80); memw(gp+##0xf000eab8)=r3; immext(#0xf0025080); memd(gp+##0xf00250b8)=r1:0 }
{ r0=add(r2,##0x10200); immext(#0xf00250c0); memw(gp+##0xf00250c8)=r0.new }
{ r1=add(r2,##0x10280); r0=add(r2,##0x10380) }
{ r3=add(r2,##0x10180); immext(#0xf000ea80); memw(gp+##0xf000eabc)=r1 }
{ r1=add(r2,##0x10400); immext(#0xf00250c0); memw(gp+##0xf00250c4)=r0 }
{ r0=add(r2,##0x10480); r2=add(r2,##0x10500) }
{ immext(#0xf0025080); memw(gp+##0xf0025098)=r3; immext(#0xf000eac0); memw(gp+##0xf000eac0)=r1 }
{ immext(#0xf00250c0); memw(gp+##0xf00250c0)=r0; immext(#0xf000eac0); memw(gp+##0xf000eac4)=r2 }
