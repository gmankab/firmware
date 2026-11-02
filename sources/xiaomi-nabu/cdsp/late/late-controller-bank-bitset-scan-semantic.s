.text
.org 0xbb4
{ r2=setbit(r0,#7) }
{ p0=cmp.eq(r1:0,r23:22)
  if(p0.new) jump:nt bitmap_empty }
{ r3=ct0(r2)
  if(cmp.gtu(r3.new,#6)) jump:nt scan_out_of_range }
{ r3=asl(r3,#1) }
{ r16=lsl(#1,r3) }
.org 0xd4c
bitmap_empty: { nop }
.org 0xe30
scan_out_of_range: { nop }
