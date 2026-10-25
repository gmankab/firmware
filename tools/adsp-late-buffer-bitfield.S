// ADSP V66 [file 0x3ba68,0x3bac0): calculate a bounded codec
// bitfield mask from source mode flags, a trailing-zero count and a
// runtime table entry. Includes explicit no-input early return.
.text
.globl adsp_late_buffer_bitfield
adsp_late_buffer_bitfield:
{ r16 = r1; memd(r29+#-16) = r17:16; allocframe(#8) }
{ call platform_map; r1 = asl(r16,#12) }
{ if (!p0.new) r3 = #-1; p0 = cmp.eq(r0,#0); if (p0.new) jump:nt return_zero }
{ r2 = extractu(r0,#22,#0); immext(#0x3fffc0); r4 = add(r0,##4194303) }
{ r2 = ct0(r2); immext(#0x3fffc0); r5 = ##4194303 }
{ r4 &= and(r0,r5); immext(#0xf01f2000); r2 = memw(r2<<#2+##4028149808) }
{ r3 += lsr(r2,#12) }
{ r0 = and(r3,r16); r17:16 = memd(r29+#0) }
{ r0 |= lsr(r4,#1); dealloc_return }
return_zero:
{ r0 = #0; r17:16 = memd(r29+#0); dealloc_return }
.set platform_map, adsp_late_buffer_bitfield+0x408
