.text
.org 0xbd0
{ r3=sub(#0,r16) }
{ r4=and(r1,r3) }
{ r17=extractu(r4,#20,#0)
  if(cmp.gtu(r17.new,r19)) jump:nt invalid_bitmap }
{ r4=r17 }
{ r4+=add(r16,#-1)
  if(cmp.gtu(r18,r4.new)) jump:nt invalid_bitmap }
{ r4=extractu(r0,#23,#1) }
.org 0xd40
invalid_bitmap: { nop }
