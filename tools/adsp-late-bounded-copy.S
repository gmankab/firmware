// ADSP overlap-aware small byte copier, original file [0x395c0,0x395fc).
// Uses Hexagon hardware loop0 for safe, strictly bounded direct copies and
// delegates overlapping/ambiguous input ranges to the platform fallback.
.text
.globl adsp_late_bounded_copy
adsp_late_bounded_copy:
{ memd(r29+#-16) = r17:16; allocframe(#8) }
{ r16 = r2; if (cmp.eq(r16.new,#0)) jump:nt finish }
{ r2 = sub(r0,r1); p0 = cmp.gtu(r1,r0); if (p0.new) jump:nt overlap }
{ loop0(copy_bytes,r16); p0 = !cmp.gt(r16,r2); if (p0.new) jump:nt overlap }
copy_bytes:
{ r2 = memub(r1++#1); memb(r0++#1) = r2.new }:endloop0
{ jump finish }
overlap:
{ call platform_overlap_copy; r2 = r16 }
finish:
{ r0 = r16; r17:16 = memd(r29+#0); dealloc_return }
.set platform_overlap_copy, adsp_late_bounded_copy+0x108520
