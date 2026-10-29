// SPDX-License-Identifier: GPL-3.0-only
// Controller-slot scanning, first atomic update path, file [0x12254,0x12294).
.text
.globl slot_scan_atomic
slot_scan_atomic:
{ call platform_lock }
retry_lock:
{ r21=memw_locked(r17) }
{ memw_locked(r17,p0)=r19 }
{ if(!p0) jump:nt retry_lock }
{ r3=setbit(r18,#16); p0=cmp.eq(r21,#0); if(p0.new) jump:nt skip_locked }
{ immext(#0xf00263c0); r2=memw(GP+##0xf00263c4) }
{ memw(r2+#0)=r3; memw(r22+#-4)=r19 }
{ r2=memw(r22+#4); memw(r22+#0)=#0 }
{ if(p0.new) r16=add(r0,#0); if(p0.new) r1:0=combine(r17,r18); p0=tstbit(r2,#0); if(!p0.new) jump:t skip_notify }
{ call platform_notify }
{ r0=r16; r16=memw(r29+#0) }
.set platform_lock,slot_scan_atomic+(0x8be293d0-0x8be12254)
.set skip_locked,slot_scan_atomic+(0x8be123f0-0x8be12254)
.set skip_notify,slot_scan_atomic+(0x8be12294-0x8be12254)
.set platform_notify,slot_scan_atomic+(0x8be29854-0x8be12254)
