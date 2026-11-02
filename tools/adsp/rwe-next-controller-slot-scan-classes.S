// SPDX-License-Identifier: GPL-3.0-only
// Controller-slot active/inactive class-bit handling [0x123a0,0x123f0).
.text
.globl slot_scan_classes
slot_scan_classes:
{ p0=tstbit(r4,#0); if(p0.new) jump:t class_high; immext(#0xffffffc0); if(!p0.new) r3=memw(r22+##-4) }
{ r3=extractu(r3,#7,#8); if(!cmp.eq(r3.new,r16)) jump:t scan_next }
{ r4=#0; immext(#0xf0025080); r3=memw(GP+##0xf0025098); memw(r22+#-8)=r4.new }
{ r19=#0 }
{ memw(r3+r27<<#2)=r2; memw(r22+#-4)=r4 }
{ r2=memw(r22+#4) }
{ if(p0.new) r1:0=combine(r17,r18); p0=tstbit(r2,#0); if(!p0.new) jump:t scan_next }
{ call slot_callback }
{ jump scan_next }
class_high:
{ r0=and(r3,#-4) }
{ r2=memw(r0+#256); if(cmp.eq(r2.new,r20)) jump:nt claim_slot }
.set scan_next,slot_scan_classes+(0x8be123f0-0x8be123a0)
.set claim_slot,slot_scan_classes+(0x8be12254-0x8be123a0)
.set slot_callback,slot_scan_classes+(0x8be29854-0x8be123a0)
