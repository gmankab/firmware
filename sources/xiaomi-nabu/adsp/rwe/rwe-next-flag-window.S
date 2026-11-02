// SPDX-License-Identifier: GPL-3.0-only
// ADSP V66 MMIO controller flag-window assignment. File [0x11400,0x11428).
.text
.globl adsp_rwe_flag_window
adsp_rwe_flag_window:
{ r3=#1; immext(#0xf0026c40); r2=memw(GP+##0xf0026c48) }
{ immext(#0x20000); r4=add(r2,##0x20000); immext(#0xf0025080); memw(GP+##0xf0025090)=r4.new }
{ jump controller_apply; immext(#0x20000); memw(r2+##0x20004)=r3 }
.set controller_apply,adsp_rwe_flag_window+(0x8be1c560-0x8be0e400)
