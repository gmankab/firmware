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
