// SPDX-License-Identifier: GPL-3.0-only
// ADSP Hexagon V66 codec format classifier [0x68bc,0x697c): compare
// carefully isolated codec bit fields against several class encodings.
// The two error paths tail-call the common controller failure handler.
.text
.globl adsp_rwe_feature_modes_b
adsp_rwe_feature_modes_b:
{ immext(#0xffffdfc0); r0=##-0x2002; jump failure_tail }
// Class: upper seven bits must match the 0x5c prefix.
next_case:
{ r9=#0 }
{ r11=r0 }
{ r11=lsr(r11,#25) }
{ r11=and(r11,#127) }
{ r11=asl(r11,#25) }
{ immext(#0x5c000000); p0=cmp.eq(r11,##0x5c000000) }
{ if(p0) r9=#1 }
{ p0=cmp.eq(r9,#1); if(!p0.new) jump:nt test_extended_prefix }
{ jump known_prefix }
// Class: upper six bits 0x50, or an alternative high-bit marker.
test_extended_prefix:
{ r9=#0 }
{ r11=r0 }
{ r11=lsr(r11,#26) }
{ r11=and(r11,#63) }
{ r11=asl(r11,#26) }
{ immext(#0x50000000); p0=cmp.eq(r11,##0x50000000) }
{ if(p0) r9=#1 }
{ p0=cmp.eq(r9,#1); if(!p0.new) jump:nt high_bit_variant }
{ r9=#0 }
{ r11=r0 }
{ r11=lsr(r11,#24) }
{ r11=and(r11,#1) }
{ r11=asl(r11,#31) }
{ immext(#0x80000000); p0=cmp.eq(r11,##-2147483648) }
{ if(p0) r9=#1 }
{ p0=cmp.eq(r9,#1); if(!p0.new) jump:nt high_bit_variant }
{ jump known_prefix }
high_bit_variant:
{ r9=#0 }
{ r11=r0 }
{ r11=lsr(r11,#24) }
{ r11=and(r11,#255) }
{ r11=asl(r11,#24) }
{ immext(#0x61000000); p0=cmp.eq(r11,##0x61000000) }
{ if(p0) r9=#1 }
{ p0=cmp.eq(r9,#1); if(!p0.new) jump:nt next_format }
known_prefix:
{ immext(#0xffffefc0); r0=##-0x1002; jump failure_tail }
.set failure_tail,adsp_rwe_feature_modes_b+(0x8be03c08-0x8be038bc)
.set next_format,adsp_rwe_feature_modes_b+(0x8be0397c-0x8be038bc)
