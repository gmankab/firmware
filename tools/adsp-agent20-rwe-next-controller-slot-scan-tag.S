// SPDX-License-Identifier: GPL-3.0-only
// Controller-slot metadata validation and halfword tag update, file [0x12294,0x12310).
.text
.globl slot_scan_tag
slot_scan_tag:
{ r17=and(r21,#-4) }
{ r3=memuh(r17+#212) }
{ r2=and(r3,#0xfe); immext(#0x200); r4=and(r3,##0x201) }
{ r2=add(r2,#0x1fe); p0=cmp.eq(r4,#1) }
{ r2=and(r2,#0xff) }
{ immext(#0xff00); r3=or(r2,and(r3,##0xff01)); if(!p0) jump:nt check_next; memh(r17+#212)=r3.new }
{ r2=extractu(r2,#7,#1); immext(#0xf020a0c0); r3=memw(GP+##0xf020a0e4) }
{ r3=add(r3,#-1); if(!cmp.eq(r3.new,r2)) jump:t alternate }
{ call platform_process; r1=r17 }
{ jump check_next }
alternate:
{ p0=cmp.eq(r2,#0); if(p0.new) r1=add(r17,#0) }
{ immext(#0x10c40); if(p0) call platform_alternative }
check_next:
{ r2=memub(r17+#212) }
{ r2=and(r2,#0xfe); if(!cmp.eq(r2.new,#0)) jump:t continue_next }
{ r2=#3; memw(r17+#216)=#0 }
{ jump continue_next; memw(r22+#4)&=r2 }
.set platform_process,slot_scan_tag+(0x8be22ec0-0x8be12294)
.set platform_alternative,slot_scan_tag+(0x8be22f30-0x8be12294)
.set continue_next,slot_scan_tag+(0x8be123f0-0x8be12294)
