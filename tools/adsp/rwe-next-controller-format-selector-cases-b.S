// SPDX-License-Identifier: GPL-3.0-only
// ADSP codec format selector table-driven validation / return [0x12764,0x127c8).
.text
.globl adsp_codec_format_cases_b
adsp_codec_format_cases_b:
case_f:
{ call platform_format; immext(#0xf000fb80); r0=##0xf000fb81 }
{ jump common_handler }
case_g:
{ call platform_format; immext(#0xf000fd40); r0=##0xf000fd58 }
{ jump common_handler }
case_h:
{ r2=and(r3,#7); immext(#0xbeefbec0); r3=##0xbeefbeef }
{ r4=+mpyi(r2,#0xe0); r2=add(r16,mpyi(#0xe0,r2)) }
{ r4=memw(r16+r4<<#0); if(cmp.eq(r4.new,r3)) jump:t match }
{ immext(#0xf000fd40); r0=##0xf000fd69 }
{ call platform_format; r2=memw(r2+#0x84); memw(sp+#0)=r2.new }:mem_noshuf
match:
{ call platform_format; immext(#0xf000fd40); r0=##0xf000fd7f }
{ r17:16=memd(r29+#24); r19:18=memd(r29+#16) }
{ dealloc_return }
.set platform_format,adsp_codec_format_cases_b+(0x8be14438-0x8be12764)
.set common_handler,adsp_codec_format_cases_b+(0x8be12860-0x8be12764)
