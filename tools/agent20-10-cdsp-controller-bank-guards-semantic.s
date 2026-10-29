.text
.org 0xb2c
controller_guard:
{ r18 = lsr(r2,#12)
  p0 = cmp.eq(r18,#0)
  if (p0.new) jump:nt empty_bank }
{ r3 = memw(gp+##0xc028c0f8) }
{ r16 = lsr(r3,#12)
  p1 = cmp.gtu(r16,r18)
  if(!p1.new) jump:nt exhausted_bank }
{ nop }
.org 0xe70
empty_bank: { nop }
.org 0xe80
exhausted_bank: { nop }
