.text
.global cdsp_select
cdsp_select:
{ p0=!cmp.gt(r21,r19) }
{ p1=cmp.eq(r21,#0)
  if(p1.new) r3=add(r19,#0) }
{ r1:0=combine(#1,#0) }
{ r2=mux(p0,#-1,#0) }
{ if(!p1) r3=add(r21,r2) }
{ r2=add(r20,mpyi(#160,r3)) }
{ call helper }
{ r16=r0
  if(cmp.eq(r16.new,#0)) jump:nt failure }
{ r2=memw(r18+#0)
  if(cmp.eq(r2.new,#0)) jump:nt error }
.org 0x14c
failure:
{ nop }
.org 0x15c
error:
{ nop }
.org 0x7370
helper:
{ nop }
