// Hexagon semantic source, relative packet addresses.
.text
.globl wlan_session_case_328_388
wlan_session_case_328_388:

{
 r19 = !cmp.eq(r18,#0)
 r21:20 = combine(r26,#0)
 r2 = memb(r17+#1)
 memw(r29+#16) = #9
}

{
 p0 = cmp.eq(r2,#0); if (p0.new) jump:nt 0x198
}

{
 p0 = cmp.eq(r18,#0)
}

{
 r2 = p0
 memw(r29+#8) = r2.new
}

{
 r1 = add(r29,#16)
 r18 = memw(r21++#4)
 r4 = memw(r29+#8)
}

{
 p0 = r4
 memw(r29+#20) = r19
}

{
 r2 = memw(r18+#20); 	r0 = memw(r18+#36)
}

{
 r3 = setbit(r2,#0)
 r2 = clrbit(r2,#0)
}

{
 if (!p0) r2 = add(r3,#0)
}

{
 call 0x206330
 memw(r18+#20) = r2
}

{
 call -0x978
 r0 = r18; 	r20 = add(r20,#1)
}

{
 r2 = memub(r17+#1)
 if (cmp.gtu(r2.new,r20)) jump:t -0x34
}

{
 jump 0x14c
}
