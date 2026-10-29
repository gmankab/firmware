// SPDX-License-Identifier: GPL-3.0-only
// Controller-slot release lookup and lock acquisition [0x12498,0x12514).
.text
.globl slot_remove_search
slot_remove_search:
{ memw(sp+#4)=r18 }
{ r2=memw(r17+#4); if(cmp.eq(r2.new,#0)) jump:nt finish_scan }
{ immext(#0xf0414880); r23=##0xf04148a0; immext(#0xf0026c40); r3=memw(GP+##0xf0026c5c) }
{ r24=lsr(r3,#5); r25=#0; r26=#0; jump lookup }
advance_count:
{ p0=!cmp.gtu(r24,r26); if(p0.new) jump:nt exit_unfound; if(!p0.new) r2=memw(r17+#4) }
{ r26=add(r26,#1); r23=add(r23,#512); r25=add(r25,#32) }
lookup:
{ r27=memw(r2+r26<<#2); if(cmp.eq(r27.new,#0)) jump:nt advance_count }
{ r22=#0; r18=r23 }
next_bit:
{ p0=!tstbit(r27,r22); if(p0.new) jump:nt continue_scan }
{ r2=memw(r18+#0); if(cmp.eq(r2.new,#0)) jump:nt continue_scan }
{ r2=and(r2,#1); if(!cmp.eq(r2.new,#0)) jump:nt continue_scan }
{ r2=memw(r18+#8); if(!cmp.eq(r2.new,r16)) jump:t continue_scan }
lock_retry:
{ r2=memw_locked(r18) }
{ memw_locked(r18,p0)=r20 }
{ if(!p0) jump:nt lock_retry }
.set finish_scan,slot_remove_search+(0x8be12570-0x8be12498)
.set exit_unfound,slot_remove_search+(0x8be12570-0x8be12498)
.set continue_scan,slot_remove_search+(0x8be12564-0x8be12498)
