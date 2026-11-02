// SPDX-License-Identifier: GPL-3.0-only
// Hexagon codec format table cases and nested dispatch [0x126dc,0x12764).
.text
.globl adsp_codec_format_cases_a
adsp_codec_format_cases_a:
case_a:
{ call platform_format; immext(#0xf000fa40); r0=##0xf000fa50; memw(r29+#0)=r18 }
{ jump common_handler }
case_b:
{ call platform_format; immext(#0xf000fa40); r0=##0xf000fa70 }
{ jump common_handler }
case_c:
{ call platform_format; immext(#0xf000fa40); r0=##0xf000fa7f }
{ p0=cmpb.eq(r19,#0x71); immext(#0xf000fa80); if(p0.new) r0=##0xf000fa97; if(p0.new) jump:nt alternate_handler }
{ p0=cmp.eq(r18,#0x70); immext(#0xf000fa80); if(!p0.new) r0=##0xf000fab1 }
{ call platform_format; immext(#0xf000fa80); if(p0) r0=##0xf000fa97 }
{ jump common_handler }
case_d:
{ call platform_format; immext(#0xf000fb40); r0=##0xf000fb56 }
{ jump common_handler }
case_e:
{ call platform_format; immext(#0xf000fb40); r0=##0xf000fb71; memw(r29+#0)=r18 }
{ r2=add(r18,#-1); if(cmp.gtu(r2.new,#22)) jump:nt long_case }
{ immext(#0xf000f9c0); r2=memw(r2<<#2+##0xf000f9f4) }
{ jumpr r2 }
.set platform_format,adsp_codec_format_cases_a+(0x8be14438-0x8be126dc)
.set common_handler,adsp_codec_format_cases_a+(0x8be12860-0x8be126dc)
.set alternate_handler,adsp_codec_format_cases_a+(0x8be1284c-0x8be126dc)
.set long_case,adsp_codec_format_cases_a+(0x8be1297c-0x8be126dc)
