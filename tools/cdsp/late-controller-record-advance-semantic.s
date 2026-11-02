// Independent Hexagon fixture for the end of the record-scan loop.
// The target labels preserve the original physical distance; LLVM6 may
// change compact/duplex packet scheduling and fused new-value predicates.
.text
.global cdsp_record_advance
retry: { nop }
.org 0xa0
seek: { nop }
.org 0x118
cdsp_record_advance:
{ r22=add(r22,#1)
  r17=memw(r16+#16) }
{ r2=add(r17,r23) }
{ r23=add(r23,#8) }
{ r2=memw(r2+#12)
  p1=cmp.eq(r2,#0)
  if(!p1.new) jump:t seek }
{ if(!p0) r21=add(r21,#1) }
{ p0=cmp.gtu(r21,#30)
  if(p0.new) jump:nt done }
{ r2=mpyi(r21,#160) }
{ r2=memub(r20+r2<<#0) }
{ p1=cmp.eq(r2,#0)
  if(!p1.new) jump:t retry }
done:
{ r17:16=memd(r29+#88)
  r19:18=memd(r29+#80) }
{ r21:20=memd(r29+#72)
  r23:22=memd(r29+#64) }
{ dealloc_return }
