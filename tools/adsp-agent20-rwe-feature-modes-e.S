// SPDX-License-Identifier: GPL-3.0-only
// ADSP Hexagon V66 mode-classifier [0x6ab0,0x6b54): compare the alternate
// encoded stream class 0x20 with 0x1f upper byte and required low sign bits.
// Every condition is kept in an original Hexagon `.new` predicate packet.
.text
.globl adsp_rwe_feature_modes_e
adsp_rwe_feature_modes_e:
{ r9=#0 }
{ r11=r0 }
{ r11=lsr(r11,#29) }
{ r11=and(r11,#7) }
{ r11=asl(r11,#29) }
{ immext(#0x20000000); p0=cmp.eq(r11,##0x20000000) }
{ if(p0) r9=#1 }
{ p0=cmp.eq(r9,#1); if(!p0.new) jump:nt fallback_family }
{ r9=#0 }
{ r11=r0 }
{ r11=lsr(r11,#8) }
{ r11=and(r11,#255) }
{ r11=asl(r11,#24) }
{ immext(#0x1f000000); p0=cmp.eq(r11,##0x1f000000) }
{ if(p0) r9=#1 }
{ p0=cmp.eq(r9,#1); if(!p0.new) jump:nt fallback_family }
{ r9=#0 }
{ r11=r0 }
{ r11=lsr(r11,#6) }
{ r11=and(r11,#1) }
{ r11=asl(r11,#31) }
{ immext(#0x80000000); p0=cmp.eq(r11,##-2147483648) }
{ if(p0) r9=#1 }
{ p0=cmp.eq(r9,#1); if(!p0.new) jump:nt fallback_family }
{ r9=#0 }
{ r11=r0 }
{ r11=lsr(r11,#2) }
{ r11=and(r11,#1) }
{ r11=asl(r11,#31) }
{ immext(#0x80000000); p0=cmp.eq(r11,##-2147483648) }
{ if(p0) r9=#1 }
{ p0=cmp.eq(r9,#1); if(!p0.new) jump:t fallback_family }
{ jump matches_required_flags }
.set fallback_family,adsp_rwe_feature_modes_e+(0x8be03b54-0x8be03ab0)
.set matches_required_flags,adsp_rwe_feature_modes_e+(0x8be03bf4-0x8be03ab0)
