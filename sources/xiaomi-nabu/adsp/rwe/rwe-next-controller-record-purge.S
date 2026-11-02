// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 controller record purge [file 0x13610,0x13744),
// including K0 lock/release, active-list unlink and cleanup callbacks.
.text
.globl adsp_record_purge
adsp_record_purge:
{ r17=r0; memd(r29+#-16)=r17:16; allocframe(#40) }
{ memd(r29+#24)=r19:18; memd(r29+#16)=r21:20 }
{ memd(sp+#8)=r23:22; memd(sp+#0)=r25:24 }
{ nop } // validated V66 R2=SGP0
{ call controller_lookup; immext(#0xf0010840); r0=##0xf0010844; r16=memw(r2+#0) }
{ r25=#0; r21:20=combine(#0,#0); immext(#0xf0010840); r24=memw(r19=##0xf0010848) }
{ immext(#0xffffeec0); r23:22=combine(r24,#-0x1112); jump scan_next }
release_lock:
{ nop } // validated V66 k0unlock
{ r25=#1 }
scan_next:
{ r1=r24 }
check_next:
{ p0=cmp.eq(r1,#0); if(p0.new) jump:nt loop_finished }
{ r2=memub(r1+#4); if(!cmp.eq(r2.new,r17)) jump:nt advance_link }
{ immext(#0xf0010840); r0=##0xf001084c; r2=memw(r19+#0); r3=memw(r1+#0) }
{ p0=!cmp.eq(r1,r2); r18=memw(r1+#12); memd(r1+#16)=r21:20 }
{ if(p0) memw(r23+#0)=r3; if(!p0) memw(r19+#0)=r3 }
{ memh(r1+#6)=#0; memw(r1+#12)=#0 }
{ call controller_publish; r24=memw(r1+#0) }
{ r1=r24; p0=cmp.eq(r18,#0); if(p0.new) jump:t check_next }
{ nop } // checked V66 k0lock
linked_loop:
{ r2=memw(r18+#256) }
{ r2=memw(r2+#16); if(!cmp.eq(r2.new,#4)) jump:t deactivate }
{ r2=add(r18,#0xe0) }
{ memh(r2+#0)=setbit(#5) }
{ memw(r18+#12)=r22 }
{ r18=memw(r18+#0); if(!cmp.eq(r18.new,#0)) jump:t linked_loop }
{ jump release_lock; nop; nop }
deactivate:
{ r0=r18; memw(r18+#0x90)=#-5; memd(r18+#8)=r21:20 }
{ call controller_deactivate; r18=memw(r18+#0); immext(#0xf0021100); memw(r18+#0xec)=##0xf002111c }
{ p0=cmp.eq(r18,#0); if(!p0.new) jump:t linked_loop }
{ jump release_lock }
advance_link:
{ r23=r1; jump scan_next; r24=memw(r1+#0) }
loop_finished:
{ p0=cmp.eq(r25,#0); if(p0.new) jump:nt notify_done }
{ nop } // checked V66 k0lock
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844 }
{ call controller_clear; r0=r16; r1=memub(r16+#21); memw(r16+#0x90)=#0 }
notify_done:
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844 }
{ r0=#0; r17:16=memd(r29+#32); r19:18=memd(r29+#24) }
{ r21:20=memd(r29+#16); r23:22=memd(r29+#8) }
{ r25:24=memd(sp+#0); dealloc_return }
.set controller_lookup,adsp_record_purge+(0x8be261bc-0x8be13610)
.set controller_publish,adsp_record_purge+(0x8be26214-0x8be13610)
.set controller_notify,adsp_record_purge+(0x8be261e8-0x8be13610)
.set controller_clear,adsp_record_purge+(0x8be2692c-0x8be13610)
.set controller_deactivate,adsp_record_purge+(0x8be26ac4-0x8be13610)
