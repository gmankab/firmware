// SPDX-License-Identifier: GPL-3.0-only
// Hexagon controller relocation completion, file [0x112d4,0x11394),
// physical [0x8be0e2d4,0x8be0e394). Following bytes are string constants,
// not instruction words.
.text
.globl adsp_relocation_finish
adsp_relocation_finish:
{ r9=sub(r0,r8); immext(#0x5800c000); r1=##0x5800c000; p0=cmp.eq(r6,#0) }
{ r6=extractu(r9,#26,#6); r12=extractu(r9,#14,#6) }
{ r1=insert(r9,#6,#1); r6=asl(r6,#2) }
{ r6=insert(r12,#16,#0) }
{ r0=setbit(r6,#14); if(!p0) jump:nt address_rewrite }
{ p0=!cmp.gtu(r5,r3); if(!p0.new) r3=sub(r3,r4); if(p0.new) jump:nt address_rewrite }
{ r7=lsr(r3,#3); jump publish_data }
address_rewrite:
{ immext(#0x800000); r4=add(r8,sub(##0x800000,r16)); r3=sub(r8,r16) }
{ r5=extractu(r3,#14,#1); immext(#0x1ff0000); r3=and(##0x1ff0000,asl(r3,#1)) }
{ immext(#0xffffc0); p0=cmp.gtu(r4,##0xfffffc); r4=#0 }
{ immext(#0x5800c000); r3 |= or(r5,##0x5800c000) }
{ if(p0) r3=add(r4,#0) }
{ memw(r16+#0)=r3 }
publish_data:
{ r2=addasl(r2,r7,#3) }
{ memd(r2+#200)=r1:0 }
finalize_cache:
{ dccleaninva(r16) }
{ icinva(r16) }
{ syncht }
{ r17:16=memd(r29+#16); dealloc_return }
invalid_slot:
{ call assert_addr_range; immext(#0xf000e380); r0=##0xf000e3b4 }
{ immext(#0xf000e3c0); r0=##0xf000e3c9; r2=#0x3c2; memw(sp+#8)=r2.new }
{ call assert_addr_range; immext(#0xf000e3c0); memw(sp+#4)=##0xf000e3e0; memw(sp+#0)=#14 }
{ call panic_handler; r0=#0xe05 }
hang:
{ jump hang }
.set assert_addr_range,adsp_relocation_finish+(0x8be11438-0x8be0e2d4)
.set panic_handler,adsp_relocation_finish+(0x8be227f4-0x8be0e2d4)
