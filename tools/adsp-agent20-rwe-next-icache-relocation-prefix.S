// SPDX-License-Identifier: GPL-3.0-only
// Controller code/data relocation helper, first 0x6c bytes source reconstructed.
// Original instruction-cache invalidation and address-plane validation.
// Hexagon ADSP file [0x11170,0x111dc), physical [0x8be0e170,0x8be0e1dc).
.text
.globl adsp_rwe_icache_relocation
adsp_rwe_icache_relocation:
{ memd(r29+#-16)=r17:16; allocframe(#24) }
{ nop } // R2=EVB (checked Hexagon V66 system read)
{ r16=addasl(r2,r1,#2); r3=asl(r1,#2) }
{ icinva(r16) }
{ isync }
{ r7=r0; r5=add(r2,#255) }
{ r7-=add(r2,r3); immext(#0x800000); r3=add(r0,sub(##0x800000,r16)) }
{ r4=extractu(r7,#14,#1); immext(#0x1ff0000); r7=and(##0x1ff0000,asl(r7,#1)) }
{ immext(#0xffffc0); p0=cmp.gtu(r3,##0xfffffc); r3=#0 }
{ immext(#0x5800c000); r7 |= or(r4,##0x5800c000); r4=memw(r16+#0) }
{ r3=insert(r4,#14,#1); if(p0) r7=add(r3,#0); immext(#0x1ff0000); r6=and(r4,##0x1ff0000) }
{ r4=add(r2,#200); r3=add(r3,r16); p0=cmp.eq(r7,#0); if(p0.new) jump:nt no_mapping }
.set no_mapping,adsp_rwe_icache_relocation+(0x8be0e21c-0x8be0e170)
