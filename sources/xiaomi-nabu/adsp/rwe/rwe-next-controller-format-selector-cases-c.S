// SPDX-License-Identifier: GPL-3.0-only
// ADSP codec selector conditional case paths and common return [0x127c8,0x1289c).
.text
.globl adsp_codec_format_cases_c
adsp_codec_format_cases_c:
case_i:
{ call platform_format; immext(#0xf000fac0); r0=##0xf000fad1 }
{ p0=cmpb.eq(r19,#0x62); immext(#0xf000fb00); if(p0.new) r0=##0xf000fb1f; if(p0.new) jump:nt case_k }
{ p0=cmp.eq(r18,#0x61); immext(#0xf000fb00); if(p0.new) r0=##0xf000fb06; if(p0.new) jump:nt case_l }
{ p0=cmp.eq(r18,#0x60); immext(#0xf000fb00); if(!p0.new) r0=##0xf000fb37 }
{ call platform_format; immext(#0xf000fac0); if(p0) r0=##0xf000fae8 }
{ jump common }
case_j:
{ p0=cmp.eq(r18,#0xbf); immext(#0xf000fd80); if(p0.new) r0=##0xf000fd8f; if(!p0.new) jump:t case_m }
{ call platform_format }
{ jump common }
case_j2:
{ immext(#0xf000fdc0); r0=##0xf000fdd9; memw(r29+#4)=r18; memw(r29+#0)=r2 }
{ call platform_format }
{ jump common }
case_m:
{ call platform_format; immext(#0xf000fd80); r0=##0xf000fdb0; memw(r29+#0)=r18 }
{ jump common }
case_n:
{ call platform_format }
{ jump common }
case_k:
{ call platform_format }
{ jump common }
case_l:
{ call platform_format }
common:
{ immext(#0xf000fe00); r0=##0xf000fe09; r2=memw(r17+#0) }
{ r2=and(r2,#7) }
{ r2=add(r16,mpyi(#0xe0,r2)) }
{ r3=memw(r2+#0xb0); memw(sp+#4)=r3.new }
{ r4=memw(r2+#0x84); memw(sp+#8)=r4.new }
{ r5=memw(r2+#0x74); memw(sp+#12)=r5.new }
{ call platform_format; r2=memw(r2+#0x80); memw(sp+#0)=r2.new }
{ r17:16=memd(r29+#24); r19:18=memd(r29+#16) }
{ dealloc_return }
.set platform_format,adsp_codec_format_cases_c+(0x8be14438-0x8be127c8)
