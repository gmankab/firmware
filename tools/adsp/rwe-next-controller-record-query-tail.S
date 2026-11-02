// SPDX-License-Identifier: GPL-3.0-only
// ADSP record query's linked-list recovery [0x131e0,0x13250).
// K0-lock privileged V66 operation is supplied by checked typed encoder.
.text
.globl adsp_record_query_tail
adsp_record_query_tail:
{ r2=memw(r16+#12); if(cmp.eq(r2.new,#0)) jump:nt empty_list }
{ r3=memub(r2+#20) }
{ r4=memub(r18+#20); if(cmp.gtu(r3,r4.new)) jump:nt insert_at_head }
find_insert:
{ r3=r2; r2=memw(r2+#0); if(cmp.eq(r2.new,#0)) jump:nt no_next }
{ r5=memub(r2+#20); if(!cmp.gtu(r5.new,r4)) jump:t find_insert }
{ jump relink }
no_next:
{ r2=#0 }
relink:
{ memw(r18+#0)=r2; memw(r3+#0)=r18 }
k0_publish:
{ nop } // checked V66 k0lock at file 0x13214
{ call controller_notify; immext(#0xf0010840); r0=##0xf0010844 }
{ r0=r18; r2=#-0x1112; r1=memub(r18+#21); memw(r18+#0x94)=r16 }
{ call controller_cleanup; memw(r18+#12)=r2 }
empty_list:
{ jump k0_publish; memw(r18+#0)=#0; memw(r16+#12)=r18 }
insert_at_head:
{ jump k0_publish; memw(r18+#0)=r2; memw(r16+#12)=r18 }
.set controller_notify,adsp_record_query_tail+(0x8be261e8-0x8be131e0)
.set controller_cleanup,adsp_record_query_tail+(0x8be268b4-0x8be131e0)
