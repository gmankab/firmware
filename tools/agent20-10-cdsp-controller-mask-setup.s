// Independently authored Hexagon V60/V66 semantic fixture
.text
.space 0x734
.global controller_loads
controller_loads:
{ r2 = memw(r19+#0); r3 = memw(r20+#0) }
{ r1 = lsr(r3,#12) }
{ r0 = memw(r2+#0) }
{ call #0x22f8 }
{ r1 = lsr(r20,#12) }
{ r19 = r0; r2 = memw(r19+#0) }
{ call #0x22ec }
{ r0 = memw(r2+#0) }
{ r18 = r0 }
{ r3 = memw(gp+##0xfe0c6680) }
{ r22 = memw(r20+#0) }
{ r17 = #32 }
{ r2 = add(r3,##0x3fffff) }
{ r21 = and(r2,##-0x400000) }
{ r2 = #1 }
{ r22 = insert(r21,#20,#0) }
{ r23 = lsr(r21,#12) }
