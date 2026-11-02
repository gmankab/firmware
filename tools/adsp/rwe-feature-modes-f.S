// SPDX-License-Identifier: GPL-3.0-only
// ADSP Hexagon V66 controller codec classifier epilogue [0x6b54,0x6c10):
// validate upper class 0x40, 0x3f byte and two sign bits, set distinct
// failure codes and return from the earlier `allocframe(#16)`.
.text
.globl adsp_rwe_feature_modes_f
adsp_rwe_feature_modes_f:
{ r9=#0 }
{ r11=r0 }
{ r11=lsr(r11,#29) }
{ r11=and(r11,#7) }
{ r11=asl(r11,#29) }
{ immext(#0x40000000); p0=cmp.eq(r11,##0x40000000) }
{ if(p0) r9=#1 }
{ p0=cmp.eq(r9,#1); if(!p0.new) jump:nt fallback_code }
{ r9=#0 }
{ r11=r0 }
{ r11=lsr(r11,#8) }
{ r11=and(r11,#255) }
{ r11=asl(r11,#24) }
{ immext(#0x3f000000); p0=cmp.eq(r11,##0x3f000000) }
{ if(p0) r9=#1 }
{ p0=cmp.eq(r9,#1); if(!p0.new) jump:nt fallback_code }
{ r9=#0 }
{ r11=r0 }
{ r11=lsr(r11,#6) }
{ r11=and(r11,#1) }
{ r11=asl(r11,#31) }
{ immext(#0x80000000); p0=cmp.eq(r11,##-2147483648) }
{ if(p0) r9=#1 }
{ p0=cmp.eq(r9,#1); if(!p0.new) jump:nt fallback_code }
{ r9=#0 }
{ r11=r0 }
{ r11=lsr(r11,#2) }
{ r11=and(r11,#1) }
{ r11=asl(r11,#31) }
{ immext(#0x80000000); p0=cmp.eq(r11,##-2147483648) }
{ if(p0) r9=#1 }
{ p0=cmp.eq(r9,#1); if(!p0.new) jump:t fallback_code }
{ immext(#0xffffffc0); r0=##-0x19; jump finish_codec_mode }
fallback_code:
{ immext(#0xffffffc0); r0=##-1 }
finish_codec_mode:
{ deallocframe }
{ jumpr lr }
