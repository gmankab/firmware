// SPDX-License-Identifier: GPL-3.0-only
// ADSP v66 runtime controller record allocation, diagnostics, state update.
// First code at file 0x10c40; original file 0x10c30..0x10c40 holds a
// non-executable 16-byte table that is not counted as source-built code.
.text
.globl adsp_rwe_controller_records
adsp_rwe_controller_records:
{ memd(r29+#-16)=r17:16; allocframe(#32) }
{ memd(r29+#16)=r19:18; memd(r29+#8)=r21:20 }
{ memd(sp+#0)=r23:22 }
{ immext(#0xf0026d40); r2=memw(r19=##0xf0026d6c); if(!cmp.gt(r2.new,#0)) jump:nt skip_initial_records }
{ r20=#0; immext(#0xf061f140); r17:16=combine(##0xf061f150,#0) }
{ immext(#0xf0028c40); r18=##0xf0028c70; r23:22=combine(#0,#0) }
next_initial_record:
{ r2=add(r18,#-122); immext(#0xf0014cc0); r1:0=combine(r18,##0xf0014cc0); memw(r18+#-140)=r2.new }
{ memb(r18+#-122)=r20; memw(r18+#-144)=r2 }
{ call init_record; memd(r18+#-136)=r23:22 }
{ call init_record; immext(#0xf0014cc0); r1:0=combine(r17,##0xf0014cc8); r16=add(r16,#1) }
{ r18=add(r18,#320); r17=add(r17,#8); r2=memw(r19+#0); if(cmp.gt(r2.new,r16)) jump:t next_initial_record }
{ jump skip_records_done }
skip_initial_records:
{ r16=#0 }
skip_records_done:
.set init_record,adsp_rwe_controller_records+(0x26214-0x10c40)
{ r3=#2; immext(#0xf00250c0); r4=memw(##0xf00250d8) }
{ r2=sub(r4,r2) }
{ p0=cmp.gtu(r3,r2); if(p0.new) jump:t final_three_records }
{ immext(#0xf061f140); r16=add(##0xf061f150,asl(r16,#3)); r18=#1; r20=#320 }
{ r23:22=combine(#0,#0); nop }
next_controller_record:
{ r3=r20; immext(#0xf020a280); r2=memw(##0xf020a29c) }
{ r3+=add(r2,#22); immext(#0xf0014cc0); r1:0=combine(r20,##0xf0014cc4); r4=add(r2,r20) }
{ immext(#0x80); r1+=add(r2,#144); memw(r2+r20<<#0)=r3; memw(r4+#4)=r3 }
{ call init_record; memb(r4+#22)=#0; memd(r4+#8)=r23:22 }
{ call init_record; immext(#0xf0014cc0); r1:0=combine(r16,##0xf0014cc8) }
{ r18=add(r18,#1); immext(#0xf00250c0); r2=memw(##0xf00250d8); r3=memw(r19+#0) }
{ r16=add(r16,#8); r2=sub(r2,r3); r20=add(r20,#320) }
{ p0=cmp.gtu(r2,r18); if(p0.new) jump:t next_controller_record }
final_three_records:
{ immext(#0xf001e340); r3:2=combine(##0xf001e360,#0); immext(#0xf02d1340); r4=memw(##0xf02d1364) }
{ p0=cmp.eq(r4,#0) }
{ r5=#0; r1:0=combine(#0,#0) }
{ nop } // checked R4=PCYCLELO
{ loop0(three_record_loop,#3) }
three_record_loop:
{ if(p0) r6=add(r2,#0); r5=add(r5,#1); memb(r3+#21)=r5; memb(r3+#22)=#1 }
{ if(!p0) r6=xor(r4,r3); memb(r3+#20)=#0; memd(r3+#8)=r1:0 }
{ r3=add(r3,#320); memw(r3+#252)=r6 }:endloop0
{ r2=#-1; r4=#1; immext(#0xf0011680); r3=memw(##0xf00116ac) }
{ r2+=asl(r4,r3); immext(#0xf0026c40); r5=memw(##0xf0026c58); r17:16=memd(sp+#24) }
{ r2=and(r2,r5); r19:18=memd(r29+#16); r21:20=memd(r29+#8) }
{ r23:22=memd(sp+#0); immext(#0xf0024340); memw(##0xf002434c)=r2 }
{ dealloc_return }
{ r0=#0; jumpr lr }
