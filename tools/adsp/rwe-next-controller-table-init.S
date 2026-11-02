// SPDX-License-Identifier: GPL-3.0-only
// Initialize packed ADSP controller tables using two hardware loops.
// File [0x11ae0,0x11ba0); physical [0x8be0eae0,0x8be0eba0).
// The preceding diagnostic strings and zero linker fill are not code.
.text
.globl adsp_controller_table_init
adsp_controller_table_init:
{ immext(#0xf0026c40); r4=memw(##0xf0026c5c) }
{ r5=add(r4,#1) }
{ r1:0=bitsplit(r5,#5); r7=lsr(r5,#2); immext(#0xf0028300); r3:2=combine(##0xf0028330,r5) }
{ r2=add(#1,lsr(r2,#5)); p0=cmp.eq(r0,#0); p1=cmp.eq(r7,#0); if(p1.new) jump:nt no_full_words }:mem_noshuf
{ loop0(clear_words,r7); immext(#0xf0028300); r7:6=combine(##0xf0028330,#0) }
clear_words:
{ nop; memw(r7++#4)=r6 }:endloop0
no_full_words:
{ r5=and(r5,#-4); if(p0) r2=add(r1,#0) }
{ if(!p1.new) r4=sub(r4,r5); p1=cmp.gtu(r5,r4); if(p1.new) jump:t no_trailing_bytes }
{ r4=#0; r6=add(r4,#1) }
{ loop0(clear_trailing,r6) }
clear_trailing:
{ r5=add(r5,#1); memb(r3+r5<<#0)=r4 }:endloop0
no_trailing_bytes:
{ immext(#0xf0026c00); r3=##0xf0026c30 }
{ p0=cmp.eq(r2,#0); if(!p0.new) r7=#0 }
{ if(!p0) r8=#-1; if(p0) jump:nt done; immext(#0xf0023780); memw(##0xf00237b4)=r3 }:mem_noshuf
{ immext(#0xf020a180); r5=##0xf020a188; immext(#0xf000ea80); r3=memw(##0xf000eab8) }
{ immext(#0xf000ea80); r4=memw(##0xf000eabc); immext(#0xf000eac0); r6=memw(##0xf000eac0) }
{ loop0(copy_register_table,r2); nop }
copy_register_table:
{ memw(r3++#4)=r7; r2=memw(r5++#4) }:mem_noshuf
{ memw(r4++#4)=r2; memw(r6++#4)=r8 }:endloop0
done:
{ r2=#0; immext(#0xf000f680); memw(##0xf000f6a4)=r2.new }
{ jump platform_finalize }
.set platform_finalize,adsp_controller_table_init+(0x8be26850-0x8be0eae0)
