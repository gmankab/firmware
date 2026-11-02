// SPDX-License-Identifier: GPL-3.0-only
// [file 0x42468,0x424e0): compare low-level controller bit masks and
// return the normalized mask difference using an early-return fast path.
.text
.globl adsp_late_controller_bitmask_difference
adsp_late_controller_bitmask_difference:
{ r3 = cl0(r1); immext(#2147483584); r5:4 = combine(r2,##2147483647); r6 = sub(#0,r0) }
{ r3 = setbit(r3,#0); r0 = #0 }
{ r5 &= lsr(r4,r3) }
{ p0 = bitsclr(r5,r6); if (p0.new) jumpr:nt r31 }
{ r3 = lsr(r4,r3); r4 = and(r5,r1) }
{ r3 &= and(r2,r6); if (cmp.gtu(r4,r3.new)) jump:t greater_mask }
{ r0 = sub(r3,r4); jumpr r31 }
greater_mask:
{ r6 = r3; r5 = r3; r2 = #-1 }
{ r5 ^= xor(r4,r2); r6 &= xor(r4,r2) }
{ r5 &= asl(r6,#1); r6 = #2730 }
{ r5 |= and(r3,~r4) }
{ p0 = bitsclr(r5,r6); if (p0.new) jumpr:nt r31 }
{ immext(#2688); r5 = and(r5,##2730); r0 = sub(r3,r4) }
{ r3 = cl0(r5) }
{ r0 &= lsr(r2,r3) }
{ jumpr r31 }
