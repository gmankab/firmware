// Hexagon semantic source, relative packet addresses.
.text
.globl wlan_session_case_2c0_2e4
wlan_session_case_2c0_2e4:

{
 r2 = #52 ; jump 0x290
}

{
 r1 = #62 ; jump 0x204
}

{
 p0 = cmp.eq(r18,#1); if (!p0.new) jump:t 0x208
 if (p0.new) r2 = memw(r29+#12)
}

{
 p0 = r2
 if (!p0.new) r2 = #2
 if (p0.new) jump:nt 0x4ac
}

{
 r1:0 = combine(#18,r19)
 jump 0x298
}
