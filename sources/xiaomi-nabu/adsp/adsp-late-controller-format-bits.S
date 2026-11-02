// SPDX-License-Identifier: GPL-3.0-only
// [file 0x3bb4c,0x3bbe4): callback-backed codec format bitfield encoder.
// Queries a dynamic parameter, folds a trailing-zero lookup into a 12-bit
// alignment mask, and optionally publishes the computed four-byte result.
.text
.globl adsp_late_controller_format_bits
adsp_late_controller_format_bits:
{ r17:16 = combine(r4,r0); r2 = #44; memd(r29+#-16) = r17:16; allocframe(#24) }
{ r18 = r3; r1:0 = combine(r16,#0); r3 = add(r29,#4); memd(r29+#8) = r19:18 }
{ call read_codec_bits; memw(r29+#4) = #0 }
{ call query_parameter; r1 = asl(r18,#12); r0 = memw(r29+#4) }
{ r2 = #-1; p0 = cmp.eq(r0,#0); if (p0.new) jump:t send_result; if (p0.new) memw(r29+#4) = #0 }
{ r2 = extractu(r0,#22,#0); r3 = #-1; immext(#4194240); r4 = add(r0,##4194303) }
{ r2 = ct0(r2); immext(#4194240); r5 = ##4194303 }
{ r4 &= and(r0,r5); immext(#4028149760); r2 = memw(r2<<#2+##4028149808) }
{ r3 += lsr(r2,#12); r2 = #-1 }
{ r3 = and(r3,r18) }
{ r3 |= lsr(r4,#1); memw(r29+#4) = r3.new }
{ p0 = cmp.eq(r3,#0); if (p0.new) jump:nt send_result }
{ r3:2 = combine(r17,#83); r5 = #4; r4 = add(r29,#4); r1:0 = combine(r16,#0) }
{ call read_codec_bits }
{ r2 = r0 }
send_result:
{ r0 = r2; r17:16 = memd(r29+#16); r19:18 = memd(r29+#8) }
{ dealloc_return }
.set read_codec_bits, adsp_late_controller_format_bits+0xef2e0
.set query_parameter, adsp_late_controller_format_bits+0x324
