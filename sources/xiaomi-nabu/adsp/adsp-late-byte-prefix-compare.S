// ADSP bounded byte comparison / prefix-match helper, original file region
// [0x39590,0x395c0). The loop handles equal bytes, zero terminators,
// and a bounded count. All instructions are Hexagon packet source.
.text
.globl adsp_late_byte_prefix_compare
adsp_late_byte_prefix_compare:
{ r0 = #0; r3 = r0 }
next_byte:
{ p0 = !cmp.gtu(r2,r0); if (p0.new) jump:nt no_match }
{ r4 = memb(r1+#0); if (cmp.eq(r4.new,#0)) jump:nt reached_zero }
{ r0 = add(r0,#1); r1 = add(r1,#1); r5 = memb(r3++#1); if (cmp.eq(r5.new,r4)) jump:t next_byte }
{ jumpr lr }
no_match:
{ r0 = #0; jumpr lr }
reached_zero:
{ r0 = #0; jumpr lr }
