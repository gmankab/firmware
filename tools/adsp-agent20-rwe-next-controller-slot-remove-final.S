// SPDX-License-Identifier: GPL-3.0-only
// Controller-slot release loop, callbacks and epilogue [0x12514,0x1259c).
.text
.globl slot_remove_final
slot_remove_final:
{ r3=lsl(#1,r22); p0=cmp.eq(r2,#0); if(p0.new) jump:nt next_bit; if(!p0.new) r2=memw(r18+#4) }
{ r2=sub(#-1,r2) }
lock_bitmap:
{ r4=memw_locked(r19) }
{ r4=and(r4,r2) }
{ memw_locked(r19,p0)=r4 }
{ if(!p0) jump:nt lock_bitmap }
{ immext(#0xf0025080); r2=memw(GP+##0xf0025098) }
{ memw(r2+r26<<#2)=r3; memw(r18+#4)=#0 }
{ r2=memw(r18+#12) }
{ p0=tstbit(r2,#0); if(p0.new) r0=add(r25,r22); if(p0.new) r1=add(r18,#0) }
{ immext(#0x17300); if(p0) call slot_callback }
{ r2=memw(r16+#212); if(cmp.eq(r2.new,#0)) jump:nt clear_and_exit }
next_bit:
{ p0=cmp.gtu(r21,r22); if(p0.new) jump:t continue_bit; r18=add(r18,#16); r22=add(r22,#1) }
{ jump next_slot }
no_match:
{ r0=#0; r18=memw(r29+#4) }
{ r18=add(r18,#1); p0=cmp.eq(r18,#0); if(p0.new) jump:t retry_scan }
{ jump exit_function }
clear_and_exit:
{ r0=#0; jump exit_function }
set_zero:
{ r0=#0 }
exit_function:
{ r17:16=memd(r29+#48); r19:18=memd(r29+#40) }
{ r21:20=memd(r29+#32); r23:22=memd(r29+#24) }
{ r25:24=memd(sp+#16); r27:26=memd(sp+#8) }
{ dealloc_return }
.set slot_callback,slot_remove_final+(0x8be29854-0x8be12514)
.set continue_bit,slot_remove_final+(0x8be124e8-0x8be12514)
.set next_slot,slot_remove_final+(0x8be124c4-0x8be12514)
.set retry_scan,slot_remove_final+(0x8be12470-0x8be12514)
