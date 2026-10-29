// Hexagon semantic source, relative packet addresses.
.text
.globl wlan_session_case_2a0_2c0
wlan_session_case_2a0_2c0:

{
 r3 = lsr(r18,#16)
 r1 = #2
 r2 = #0
 r0 = memw(r17+#688)
}

{
 call 0x1aebc
 memb(r17+#945) = r18
 memb(r17+#946) = r3
}

{
 jump 0x214
}
