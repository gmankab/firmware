// SPDX-License-Identifier: GPL-3.0-only
// V66 cache/tag register diagnostics and controller cleanup [0x661c,0x6734).
// Source-authored predicates, loops and MMIO fields; original OEM codewords
// are never included. Special register reads/writes use checked encoders.
.text
.globl adsp_rwe_cache_diagnostics
adsp_rwe_cache_diagnostics:
{ allocframe(#16) }
{ call controller_boot }
{ jump done }
{ r7=#0; immext(#0xf0003c80); r8=##0xf0003cb0 }
{ immext(#0xf000eac0); r1=memw(gp+##0xf000eadc) }
{ r8=add(r8,#4); p3=cmp.eq(r1,#128) }
{ immext(#0xf000bf00); r14=##0xf000bf10; immext(#0xf0003f00); r15=##0xf0003f10 }
{ nop } // V66 R0=SYSCFG
{ r0=clrbit(r0,#1) }
{ nop } // V66 SYSCFG=R0
{ isync }
{ if(p3) r10=#128; if(!p3) r10=#256 }
{ loop1(outer_loop,r10) }
outer_loop:
{ nop } // V66 R10=LC1 (C3)
{ r12=add(r10,#-1) }
{ nop } // V66 R10=LC0 (C1)
{ r13=add(r10,#-1); r10=add(r10,#-1) }
{ nop } // V66 R1=ictagr(R10)
{ p0=tstbit(r1,#1); if(!p0.new) jump:nt skip_dump }
{ r4=#0 }
{ r4=#0 }
inner_loop:
{ nop } // V66 R1=icdatar(R10)
{ p0=cmp.gt(r7,#2); if(p0.new) jump:t next_tag }
{ r0=r1 }
{ immext(#0x140); call hash_words }
{ r16=and(r0,r1) }
{ r0=r6 }
{ immext(#0x140); call hash_words }
{ r11=and(r0,r6) }
{ p0=cmp.eq(r16,r11); if(p0.new) jump:t next_tag }
{ memw(r8++#4)=r3 }
{ memw(r8++#4)=r2 }
{ memw(r8++#4)=r10 }
{ memw(r8++#4)=r6 }
{ memw(r8++#4)=r1 }
{ r7=add(r7,#1) }
next_tag:
{ r10=add(r10,#4) }
{ r4=add(r4,#1); if(!cmp.eq(r4.new,#8)) jump:t inner_loop }
skip_dump:
{ nop; nop; nop }:endloop0:endloop1
{ nop } // V66 R1=SYSCFG
{ r1=setbit(r1,#1) }
{ nop } // V66 SYSCFG=R1
{ isync }
{ r1=#2 }
{ immext(#0xf0003c80); memw(r0=##0xf0003ca0)=r1 }
{ dccleana(r0) }
{ r0=add(r0,#32) }
{ dccleana(r0) }
{ immext(#0xf0003c80); memw(r0=##0xf0003cb0)=r7 }
{ dccleana(r0) }
done:
{ deallocframe }
{ jumpr lr }
.set controller_boot,adsp_rwe_cache_diagnostics+(0x8be0d6e8-0x8be0361c)
.set hash_words,adsp_rwe_cache_diagnostics+(0x8be03818-0x8be0361c)
