// SPDX-License-Identifier: GPL-3.0-only
// ADSP Hexagon V66 controller record snapshot [file 0x13390, 0x13478).
.text
.globl adsp_record_snapshot
adsp_record_snapshot:
{ r17:16=combine(r1,r0); memd(r29+#-16)=r17:16; allocframe(#16) }
{ memd(sp+#0)=r19:18 }
{ nop } // checked privileged R2=SGP0
{ call controller_lookup; immext(#0xf0010840); r0=##0xf0010844; r18=memw(r2+#0) }
{ immext(#0xf0010840); r2=memw(gp+##0xf0010848); if(cmp.eq(r2.new,#0)) jump:nt no_record }
search_loop:
{ r1:0=memd(r2+#16) }
{ p0=cmp.eq(r1:0,r17:16); if(p0.new) jump:nt found_record }
{ r2=memw(r2+#0); if(!cmp.eq(r2.new,#0)) jump:t search_loop }
no_record:
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844; r17:16=combine(#0,#0) }
{ jump return_record }
found_record:
{ r17:16=combine(#0,#0) }
{ immext(#0xf02d1240); r3=##0xf02d1250; r4=memub(r2+#5) }:mem_noshuf
{ r3=sub(r2,r3); p0=cmp.gt(r4,#1); r2=memw(r2+#8) }
{ r3=asr(r3,#5); if(p0) jump:nt inspect_flags }
{ p0=cmp.eq(r4,#0); if(p0.new) jump:nt check_clear; if(p0.new) r4=memub(r18+#30) }
{ p0=cmp.eq(r4,#1); if(!p0.new) jump:nt done_lookup; if(p0.new) r4=memw(r18+#28) }
{ r4=extractu(r4,#7,#8); if(cmp.eq(r4.new,#0)) jump:nt report_link }
{ r17:16=combine(#0,#0); jump done_lookup }
inspect_flags:
{ if(p0.new) r17:16=combine(r2,r3); p0=cmp.eq(r4,#2); if(p0.new) jump:nt done_lookup }
{ p0=cmp.eq(r4,#3); if(!p0.new) jump:nt done_lookup; immext(#0x100); if(p0.new) r4=memw(r18+#256) }
{ r4=memw(r4+#16); if(cmp.eq(r4.new,#0)) jump:nt found_dead }
{ r17:16=combine(r2,r3); jump done_lookup }
check_clear:
{ p0=!tstbit(r4,#3); if(!p0.new) r17:16=combine(r2,r3); if(!p0.new) jump:t done_lookup }
{ r17:16=combine(#0,#0); jump done_lookup }
report_link:
{ r17:16=combine(r2,r3); jump done_lookup }
found_dead:
{ r17:16=combine(#0,#0) }
done_lookup:
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844 }
return_record:
{ r1:0=combine(r17,r16); r17:16=memd(r29+#8); r19:18=memd(r29+#0) }
{ dealloc_return }
.set controller_lookup,adsp_record_snapshot+(0x8be261bc-0x8be13390)
.set controller_notify,adsp_record_snapshot+(0x8be261e8-0x8be13390)
