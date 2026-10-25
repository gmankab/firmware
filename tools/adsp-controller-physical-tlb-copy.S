// SPDX-License-Identifier: GPL-3.0-only
// Hexagon physical TLB-buffer copy [0x5138,0x5184).
// LLVM6 compiles the two conditional predicates and Hexagon hardware loop;
// S54 and memw_phys are emitted as checked typed v66 operations.
.text
.globl adsp_controller_physical_tlb_copy
adsp_controller_physical_tlb_copy:
{ nop } // V66 r23 = S54
{
 immext(#0xf02d1480)
 r1 = memw(r23+##-0xfd2eb78)
 immext(#0xf020a080)
 r3 = memw(r23+##-0xfdf5f60)
}
{ p0 = cmp.gt(r3,#0x20); if (!p0.new) jump:nt done }
{
 r1 = add(r1,r23)
 immext(#0xf000ea80)
 r0 = memw(r23+##-0xfff154c)
}
{ r2 = lsr(r3,#2); p0 = cmp.eq(r0,#0); if (p0.new) jump:nt done }
{ loop0(check_loop,r2); r2 = #11 }
check_loop:
{ r4 = lsr(r0,r2) }
{ nop } // V66 r5 = memw_phys(r0,r4)
{ r0 = add(r0,#4); memw(r1++#4) = r5 }:endloop0
done:
{ jumpr lr }
