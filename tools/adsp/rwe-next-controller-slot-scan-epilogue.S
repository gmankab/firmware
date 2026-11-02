// SPDX-License-Identifier: GPL-3.0-only
// Controller-slot scanning loop/return [0x123f0,0x1242c).
.text
.globl slot_scan_epilogue
slot_scan_epilogue:
{ r22=add(r22,#16); r23=add(r23,#1) }
{ p0=cmp.eq(r23,#32); if(!p0.new) jump:t walk_slots }
{ r26=add(r26,#32); r24=add(r24,#512); r2=memw(sp+#4) }
{ p0=!cmp.eq(r27,r2); r27=add(r27,#1); if(!p0.new) r0=#0; if(p0.new) jump:t advance_bitmap }
{ r17:16=memd(r29+#48); r19:18=memd(r29+#40) }
{ r21:20=memd(r29+#32); r23:22=memd(r29+#24) }
{ r25:24=memd(sp+#16); r27:26=memd(sp+#8) }
{ dealloc_return }
.set walk_slots,slot_scan_epilogue+(0x8be12310-0x8be123f0)
.set advance_bitmap,slot_scan_epilogue+(0x8be12240-0x8be123f0)
