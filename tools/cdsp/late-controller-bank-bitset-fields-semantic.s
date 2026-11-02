.text
.org 0xbf0
{ r7:6=lsr(r1:0,#38) }
{ r5=memw(r20+#0)
  memw(fp+#0)=r17 }
{ r8=lsr(r0,#29)
  r9=lsr(r0,#25) }
{ r3=and(r4,r3)
  r18=r6 }
{ r18=or(r3,and(r18,##0x800000)) }
