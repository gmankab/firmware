.text
.org 0xb78
{ p0=cmp.eq(r0,#0)
  if(!p0.new) jump:nt no_handle }
{ p0=or(p0,!p0) }
{ r0=#3; r1=memw(fp+#28) }
{ call next_controller }
{ r2=p0 }
{ r19=add(r16,#-1) }
{ if(p0.new) r24=#128
  p0=cmp.gt(r0,#-1)
  if(!p0.new) jump:nt missing_bank }
.org 0xe90
no_handle: { nop }
.org 0xea0
missing_bank: { nop }
.org 0x6768
next_controller: { nop }
