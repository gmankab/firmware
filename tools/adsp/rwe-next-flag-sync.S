// SPDX-License-Identifier: GPL-3.0-only
// ADSP V66 conditional MMIO flag update; 8 preceding bytes are linker fill.
// File [0x11430,0x11450).
.text
.globl adsp_rwe_flag_sync
adsp_rwe_flag_sync:
{ immext(#0xf0000000); r3=##0xf0000000; immext(#0xf02d1480); r2=memw(GP+##0xf02d1494) }
{ r4=#0; p0=cmp.eq(r2,#0) }
{ r3=insert(r4,#2,#0); jumpr lr; if(!p0) memw(r2+#0)=r3.new }
