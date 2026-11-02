// Reconstructed 80-byte Hexagon page-aligned zero-fill helper,
// original ADSP file offsets [0x3640a8,0x3640f8).
.text
.globl adsp_late_zero_memory
adsp_late_zero_memory:
{ r3 = add(r0,#7); r4 = #0 }
{ r3 = insert(r4,#3,#0); r5 = #0 }
{ p0 = cmp.gt(r3,r1) }
{ if (p0) r3 = add(r1,#0) }
byte_head:
{ p0 = cmp.eq(r0,r3); if (p0.new) jump:nt check_last_bytes }
{ jump byte_head; memb(r0++#1) = r4 }
check_last_bytes:
{ p0 = cmp.eq(r0,r1); if (p0.new) jump:nt finished; r3 = r1 }
{ r3 = insert(r4,#3,#0) }
word_fill:
{ p0 = cmp.eq(r0,r3); if (p0.new) jump:nt tail_bytes }
{ jump word_fill; memd(r0++#8) = r5:4 }
tail_bytes:
{ r3 = r1 }
tail_loop:
{ p0 = cmp.eq(r0,r3); if (p0.new) jump:nt finished }
{ jump tail_loop; memb(r0++#1) = r4 }
finished:
{ jumpr lr }
