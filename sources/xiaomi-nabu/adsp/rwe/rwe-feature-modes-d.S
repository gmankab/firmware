// SPDX-License-Identifier: GPL-3.0-only
// ADSP V66 codec mode discriminator [0x6a10,0x6ab0). Four dependent field
// checks distinguish a zero high-class, 0x3f middle-byte and two sign flags;
// all match => common -0x19, else proceed to the next class.
.text
.globl adsp_rwe_feature_modes_d
adsp_rwe_feature_modes_d:
{ r9=#0 }
{ r11=r0 }
{ r11=lsr(r11,#29) }
{ r11=and(r11,#7) }
{ r11=asl(r11,#29) }
{ p0=cmp.eq(r11,#0) }
{ if(p0) r9=#1 }
{ p0=cmp.eq(r9,#1); if(!p0.new) jump:nt next_family }
{ r9=#0 }
{ r11=r0 }
{ r11=lsr(r11,#8) }
{ r11=and(r11,#255) }
{ r11=asl(r11,#24) }
{ immext(#0x3f000000); p0=cmp.eq(r11,##0x3f000000) }
{ if(p0) r9=#1 }
{ p0=cmp.eq(r9,#1); if(!p0.new) jump:nt next_family }
{ r9=#0 }
{ r11=r0 }
{ r11=lsr(r11,#6) }
{ r11=and(r11,#1) }
{ r11=asl(r11,#31) }
{ immext(#0x80000000); p0=cmp.eq(r11,##-2147483648) }
{ if(p0) r9=#1 }
{ p0=cmp.eq(r9,#1); if(!p0.new) jump:nt next_family }
{ r9=#0 }
{ r11=r0 }
{ r11=lsr(r11,#2) }
{ r11=and(r11,#1) }
{ r11=asl(r11,#31) }
{ immext(#0x80000000); p0=cmp.eq(r11,##-2147483648) }
{ if(p0) r9=#1 }
{ p0=cmp.eq(r9,#1); if(!p0.new) jump:nt next_family }
{ jump matched_all }
.set next_family,adsp_rwe_feature_modes_d+(0x8be03ab0-0x8be03a10)
.set matched_all,adsp_rwe_feature_modes_d+(0x8be03bf4-0x8be03a10)
