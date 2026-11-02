// Hexagon semantic source, relative packet addresses.
.text
.globl wlan_session_case_2e4_328
wlan_session_case_2e4_328:

{
 p0 = r2
 if (!p0.new) jump:t 0x27c
}

{
 r2 = memb(r17+#1)
 if (cmp.eq(r2.new,#0)) jump:nt 0x2cc
}

{
 r2 = r3 ; jump 0x2b8
 r1:0 = combine(r17,#0)
}

{
 r2 = memw(r29+#12)
}

{
 p0 = r2
 if (p0.new) jump:nt 0x3a0
}

{
 r2 = addasl(r26,r16,#2)
 p0 = cmp.eq(r18,#0)
}

{
 r1 = mux(p0,#0,#1)
 r2 = memw(r2+#-4)
}

{
 call 0x15e634
 r0 = memw(r2+#36)
}

{
 jump 0x258
}

{
 r2 = #12 ; jump 0x22c
}
