// SPDX-License-Identifier: GPL-3.0-only
// First 0xa4 bytes of codec controller bit-pattern matcher, ADSP [0x6818,0x68bc).
// Continue the same function in a separate adjacent source file; no fake return.
.text
.globl adsp_rwe_feature_modes_a
adsp_rwe_feature_modes_a:
{ allocframe(#16) }
{ r9=#0 }
{ r11=r0 }
{ r11=lsr(r11,#14) }
{ r11=and(r11,#3) }
{ r11=asl(r11,#30) }
{ p0=cmp.eq(r11,#0) }
{ if(p0) r9=#1 }
{ p0=cmp.eq(r9,#1); if(p0.new) jump:t matching_mode }
{ r9=#0 }
{ r11=r0 }
{ r11=lsr(r11,#27) }
{ r11=and(r11,#31) }
{ r11=asl(r11,#27) }
{ immext(#0x10000000); p0=cmp.eq(r11,##0x10000000) }
{ if(p0) r9=#1 }
{ p0=cmp.eq(r9,#1); if(!p0.new) jump:nt try_second_match }
{ r9=#0 }
{ r11=r0 }
{ r11=lsr(r11,#25) }
{ r11=and(r11,#3) }
{ r11=asl(r11,#30) }
{ immext(#0xc0000000); p0=cmp.eq(r11,##-1073741824) }
{ if(p0) r9=#1 }
{ p0=cmp.eq(r9,#1); if(p0.new) jump:nt try_second_match }
{ jump later_modes }
try_second_match:
{ r9=#0 }
{ r11=r0 }
{ r11=lsr(r11,#27) }
{ r11=and(r11,#31) }
{ r11=asl(r11,#27) }
{ immext(#0x20000000); p0=cmp.eq(r11,##0x20000000) }
{ if(p0) r9=#1 }
{ p0=cmp.eq(r9,#1); if(!p0.new) jump:nt next_case }
.set matching_mode,adsp_rwe_feature_modes_a+(0x8be03a10-0x8be03818)
.set later_modes,adsp_rwe_feature_modes_a+(0x8be038bc-0x8be03818)
.set next_case,adsp_rwe_feature_modes_a+(0x8be038c8-0x8be03818)
