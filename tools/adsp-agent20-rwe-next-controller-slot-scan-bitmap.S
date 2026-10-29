// SPDX-License-Identifier: GPL-3.0-only
// Bitmap walk, lock acquisition and slot callback path [0x12310,0x123a0).
.text
.globl slot_scan_bitmap
slot_scan_bitmap:
{ r2=lsl(#1,r23); p0=!tstbit(r25,r23); r18=add(r26,r23); if(p0.new) jump:nt scan_next }
{ r17=add(r22,#-8); r4=memw(r22+#0); r3=memw(r22+#-8) }
{ p0=tstbit(r3,#0); if(p0.new) jump:t bitmap_class }
{ p0=cmp.eq(r3,#0); if(p0.new) jump:nt scan_next; if(!p0.new) r3=memw(r22+#0) }
{ r3=memw(r3+#256); if(!cmp.eq(r3.new,r20)) jump:t scan_next }
lock_slot:
{ r3=memw_locked(r17) }
{ memw_locked(r17,p0)=r19 }
{ if(!p0) jump:nt lock_slot }
{ p0=cmp.eq(r3,#0); if(p0.new) jump:nt scan_next; if(!p0.new) r3=memw(r22+#0) }
{ r4=memw(r22+#-4) }
{ r4=sub(#-1,r4); r3=add(r3,#212) }
lock_mask:
{ r5=memw_locked(r3) }
{ r5=and(r5,r4) }
{ memw_locked(r3,p0)=r5 }
{ if(!p0) jump:nt lock_mask }
{ immext(#0xf0025080); r3=memw(GP+##0xf0025098) }
{ memw(r3+r27<<#2)=r2; memw(r22+#-4)=r19 }
{ r2=memw(r22+#4) }
{ p0=tstbit(r2,#0); if(p0.new) r1:0=combine(r17,r18) }
{ immext(#0x174c0); if(p0) call callback }
{ jump scan_next; nop; nop }
.set bitmap_class,slot_scan_bitmap+(0x8be123a0-0x8be12310)
.set scan_next,slot_scan_bitmap+(0x8be123f0-0x8be12310)
.set callback,slot_scan_bitmap+(0x8be29854-0x8be12310)
