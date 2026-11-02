// Hexagon semantic source, relative packet addresses.
.text
.globl wlan_session_case_1a0_1e4
wlan_session_case_1a0_1e4:

{
 p0 = cmp.eq(r18,#0)
 r2 = memb(r17+#1)
 if (cmp.eq(r2.new,#0)) jump:nt ##0x330
}

{
 r19:18 = combine(#0,r26)
}

{
 r2 = p0
 memw(r29+#8) = r2.new
}

{
 r2 = memw(r18+#0)
}

{
 r0 = memw(r2+#52); 	r1 = memw(r2+#60)
}

{
 r2 = memw(r29+#8)
}

{
 p0 = r2
}

{
 call 0xc95c8
 r2 = mux(p0,#0,#1)
 r18 = add(r18,#4); 	r19 = add(r19,#1)
}

{
 r2 = memub(r17+#1)
 if (cmp.gtu(r2.new,r19)) jump:t -0x1c
}

{
 jump 0x2f0
}
