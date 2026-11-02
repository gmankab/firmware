// SPDX-License-Identifier: GPL-3.0-only
// ADSP controller record lookup/create [file 0x12e40,0x12f50).
// One privileged SGP0 read at 0x12e50 is a checked typed V66 substitution.
.text
.globl adsp_controller_record_lookup
adsp_controller_record_lookup:
{ r16=r2; memd(r29+#-16)=r17:16; allocframe(#24) }
{ r19:18=combine(r1,r0); memd(r29+#8)=r19:18; memd(r29+#0)=r21:20 }
{ nop } // checked R2=SGP0
{ r4=extractu(r18,#24,#0); immext(#0x5f5340); r6=##0x5f534f; r2=memw(r2+#0) }
{ r7=#0; r5=#0; r20=add(r2,#24); r0=#0 }
{ p0=cmp.eq(r5:4,r7:6); if(!p0.new) jump:t allocation_check }
{ r1=#0; r2=memub(r20+#6) }
{ p0=!tstbit(r2,#3); if(p0.new) jump:t exit }
allocation_check:
{ call controller_lookup; immext(#0xf0010840); r0=##0xf0010844 }
record_loop:
{ immext(#0xf0010840); r2=memw(gp+##0xf0010848); if(cmp.eq(r2.new,#0)) jump:nt allocate_new }
{ r1:0=memd(r2+#16) }
{ p0=cmp.eq(r1:0,r19:18); immext(#0xf0010840); if(p0.new) r0=##0xf0010844; if(p0.new) jump:nt found }
{ r2=memw(r2+#0); if(!cmp.eq(r2.new,#0)) jump:t record_loop_continue }
allocate_new:
{ call controller_alloc; immext(#0xf0010840); r0=##0xf001084c }
{ r3:2=combine(#0,#0); r17=r0; if(cmp.eq(r17.new,#0)) jump:nt allocation_failed }
{ r4=memw(r17+#8); memd(r17+#16)=r19:18 }
{ immext(#0xf0010840); r1:0=combine(r17,##0xf0010848); r4=add(r4,#1); r5=memw(r20+#4) }
{ r5=extractu(r5,#7,#8); p0=cmp.eq(r4,#-1); memd(r17+#24)=r3:2; memb(r17+#5)=r16 }
{ memb(r17+#4)=r5; immext(#0x100); memh(r17+#6)=#256 }
{ r2=mux(p0,#0,r4); memw(r17+#12)=#0 }
{ call controller_publish; memw(r17+#8)=r2 }
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844 }
{ immext(#0xf02d1240); r2=##0xf02d1250; r1=memw(r17+#8) }
{ r2=sub(r17,r2) }
{ r0=asr(r2,#5); jump exit }
found:
{ call controller_notify }
{ r1:0=combine(#0,#0); jump exit }
allocation_failed:
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844 }
{ r1:0=combine(#0,#0) }
exit:
{ r17:16=memd(r29+#16); r19:18=memd(r29+#8) }
{ r21:20=memd(r29+#0); dealloc_return }
// The 'record_loop_continue' target is inside an original packet at the start
// of the list walk; use its original address relative to this function.
.set record_loop_continue,adsp_controller_record_lookup+(0x8be12e98-0x8be12e40)
.set controller_lookup,adsp_controller_record_lookup+(0x8be261bc-0x8be12e40)
.set controller_alloc,adsp_controller_record_lookup+(0x8be261f4-0x8be12e40)
.set controller_publish,adsp_controller_record_lookup+(0x8be26214-0x8be12e40)
.set controller_notify,adsp_controller_record_lookup+(0x8be261e8-0x8be12e40)
