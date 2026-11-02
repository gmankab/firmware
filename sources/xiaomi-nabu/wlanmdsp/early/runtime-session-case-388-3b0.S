// Hexagon semantic source, relative packet addresses.
.text
.globl wlan_session_case_388_3b0
wlan_session_case_388_3b0:

{
 r2 = memw(r29+#12)
}

{
 p0 = r2
 if (p0.new) jump:nt 0x350
}

{
 r2 = addasl(r26,r16,#2)
 p0 = cmp.eq(r18,#0)
}

{
 r2 = memw(r2+#-4)
}

{
 if (p0) jump:nt 0x3d4
 r0 = memw(r2+#36)
}

{
 call 0x20357c
}

{
 jump 0x1cc
}
