// SPDX-License-Identifier: GPL-3.0-only
// Register and publish an ADSP global controller slot; file 0x11ba0-0x11c88.
// Physical address 0x8be0eba0; original authentication is not generated.
.text
.globl adsp_controller_slot_register
adsp_controller_slot_register:
{ r16=r0; memd(r29+#-16)=r17:16; allocframe(#24) }
{ r19:18=combine(r1,r2); memd(r29+#8)=r19:18; memd(r29+#0)=r21:20 }
{ nop } // V66 R2=SGP0 encoded at checked position 0x11bb0
{ call get_global_context; r20=memw(r2+#0) }
{ r3=cl0(r18); r2=ct0(r18); p0=cmp.eq(r0,#0); if(!p0.new) jump:t slot_fail }
{ r0=#27; r2=add(r3,r2); if(!cmp.eq(r2.new,#31)) jump:t slot_fail }
{ r2=#0; r17=r16 }
{ immext(#0xf0414880); r17=add(##0xf04148a0,asl(r17,#4)) }
lock_retry:
{ r3=memw_locked(r17) }
{ p0=cmp.eq(r3,r2) }
{ if(!p0) jump:nt lock_acquired }
{ memw_locked(r17,p0)=r19 }
{ if(!p0) jump:nt lock_retry }
lock_acquired:
{ r0=#1; p0=cmp.eq(r3,#0); if(!p0.new) jump:t slot_fail }
{ r2=add(r20,#212); r3=sub(#-1,r18); memw(r17+#4)=r18; memw(r17+#8)=r20 }
bitmap_low_retry:
{ r4=memw_locked(r2) }
{ r4=or(r4,r18) }
{ memw_locked(r2,p0)=r4 }
{ if(!p0) jump:nt bitmap_low_retry }
{ r2=extractu(r3,#31,#0); r3=add(r20,#216) }
bitmap_high_retry:
{ r4=memw_locked(r3) }
{ r4=and(r4,r2) }
{ memw_locked(r3,p0)=r4 }
{ if(!p0) jump:nt bitmap_high_retry }
{ r1:0=bitsplit(r16,#5); immext(#0xf000eac0); r2=memw(GP+##0xf000eac0) }
{ r3=lsl(#1,r0); memw(r2+r1<<#2)=r3.new }
{ immext(#0xf00250c0); r2=memw(GP+##0xf00250c4) }
{ r2=memw(r2+#0) }
{ r1:0=combine(#1,#11); r2=r16; immext(#0xf00263c0); r3=memw(GP+##0xf00263c4) }
{ call publish_registration; memw(r3+#0)=r16 }
{ r2=memw(r17+#12) }
{ p0=tstbit(r2,#0); if(p0.new) r1:0=combine(r17,r16) }
{ immext(#0x17bc0); if(p0) call trigger_slot_callback }
{ r0=#0 }
slot_fail:
{ r17:16=memd(r29+#16); r19:18=memd(r29+#8) }
{ r21:20=memd(r29+#0); dealloc_return }
.set get_global_context,adsp_controller_slot_register+(0x8be25bd0-0x8be0eba0)
.set publish_registration,adsp_controller_slot_register+(0x8be254f0-0x8be0eba0)
.set trigger_slot_callback,adsp_controller_slot_register+(0x8be26854-0x8be0eba0)
