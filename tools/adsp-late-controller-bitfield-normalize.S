// SPDX-License-Identifier: GPL-3.0-only
// [0x40ce8,0x40d20): normalize a packed descriptor bitfield to two
// platform words, choosing zero for an empty source field.
.text
.globl adsp_late_controller_bitfield_normalize
adsp_late_controller_bitfield_normalize:
{ r16 = r1; r0 = and(r0,#127); memd(r29+#-16) = r17:16; allocframe(#8) }
{ r3:2 = bitsplit(r16,#12) }
{ call normalize_device_fields; r1 = r3 }
{ r16 = insert(r0,#20,#12); r4 = extractu(r0,#12,#20); r3 = #0; p0 = cmp.eq(r0,#0) }
{ r2 = mux(p0,r3,r16); if (!p0) r3 = add(r4,#0); r17:16 = memd(r29+#0) }
{ r1:0 = combine(r3,r2); dealloc_return }
.set normalize_device_fields, adsp_late_controller_bitfield_normalize-0x5280
