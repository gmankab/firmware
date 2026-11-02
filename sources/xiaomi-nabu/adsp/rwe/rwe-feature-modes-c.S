// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 ADSP codec format bitmask continuation [0x697c, 0x6a10):
// high codec prefix 0x96 and two sign-bit selectors, with distinct error
// values dispatched to the common controller error epilogue.
.text
.globl adsp_rwe_feature_modes_c
adsp_rwe_feature_modes_c:
{ r9=#0 }
{ r11=r0 }
{ r11=lsr(r11,#21) }
{ immext(#0x7c0); r11=and(r11,##0x7ff) }
{ r11=asl(r11,#21) }
{ immext(#0x96000000); p0=cmp.eq(r11,##-1778384896) }
{ if(p0) r9=#1 }
{ p0=cmp.eq(r9,#1); if(!p0.new) jump:nt next_format }
{ r9=#0 }
{ r11=r0 }
{ r11=lsr(r11,#11) }
{ r11=and(r11,#1) }
{ r11=asl(r11,#31) }
{ immext(#0x80000000); p0=cmp.eq(r11,##-2147483648) }
{ if(p0) r9=#1 }
{ p0=cmp.eq(r9,#1); if(!p0.new) jump:nt test_second_sign_bit }
{ immext(#0xffffefc0); r0=##-0x1021; jump failure_tail }
test_second_sign_bit:
{ r9=#0 }
{ r11=r0 }
{ r11=lsr(r11,#11) }
{ r11=and(r11,#3) }
{ r11=asl(r11,#30) }
{ immext(#0x80000000); p0=cmp.eq(r11,##-2147483648) }
{ if(p0) r9=#1 }
{ p0=cmp.eq(r9,#1); if(!p0.new) jump:nt next_format }
{ immext(#0xffffff40); r0=##-0xa1; jump failure_tail }
.set next_format,adsp_rwe_feature_modes_c+(0x8be03a10-0x8be0397c)
.set failure_tail,adsp_rwe_feature_modes_c+(0x8be03c08-0x8be0397c)
