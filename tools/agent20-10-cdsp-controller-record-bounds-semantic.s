// Independent Hexagon V60/66 semantic fixture. LLVM may reschedule duplexes
// or convert fused new-value guards to explicit predicates.
.text
.global cdsp_record_bounds
cdsp_record_bounds:
{ r4=memw(r2+#16)
  r5=memw(r16+#16) }
{ r3=memw(r5+r23<<#0) }
{ r2=memw(r4+#32)
  p1=cmp.gtu(r3,r2)
  if (p1.new) jump:t skip }
{ r6=r2 }
{ r5=add(r5,r23)
  r4=memw(r4+#36) }
{ r6 += lsr(r4,#12) }
{ r5=memw(r5+#4) }
{ r3 += lsr(r5,#12) }
{ p1=cmp.gtu(r6,r3)
  if(p1.new) jump:t skip }
{ call service }
{ r1=lsr(r4,#12) }
{ r3=#8
  r0=r17 }
skip:
{ nop }
.org 0x7a0c
service: { nop }
