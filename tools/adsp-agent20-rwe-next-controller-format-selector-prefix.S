// SPDX-License-Identifier: GPL-3.0-only
// ADSP controller format selector entry [file 0x126a8,0x126dc).
// The mode-case table is an external original memory table at 0xf000f9bc;
// it is a runtime lookup, not inline instructions included in this fragment.
.text
.globl adsp_controller_format_selector
adsp_controller_format_selector:
{ memd(r29+#-16)=r17:16; allocframe(#32) }
{ memd(sp+#16)=r19:18; immext(#0xf0025100); r3=memw(r17=##0xf0025128) }:mem_noshuf
{ r2=extractu(r3,#8,#16); r19=lsr(r3,#8); r16=add(r17,#104) }
{ r18=and(r19,#0xff); r4=add(r2,#-1); if(cmp.gtu(r4.new,#13)) jump:nt invalid_mode }
{ immext(#0xf000f980); r4=memw(r4<<#2+##0xf000f9bc) }
{ jumpr r4 }
.set invalid_mode,adsp_controller_format_selector+(0x8be12828-0x8be126a8)
