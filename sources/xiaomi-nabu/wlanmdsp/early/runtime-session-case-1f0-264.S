// Hexagon semantic source, relative packet addresses.
.text
.globl wlan_session_case_1f0_264
wlan_session_case_1f0_264:

{
 immext(#2954931968)
 r27 = ##-1340035324
}

{
 r2 = memw(r29+#12)
}

{
 p0 = r2
 if (!p0.new) r1 = add(r23,#0)
 if (p0.new) jump:nt 0x24
}

{
 r2 = addasl(r26,r16,#2)
}

{
 r2 = r18
 r3 = memw(r2+#-4)
}

{
 callr r27
 r0 = memw(r3+#36)
}

{
 jump 0x35c
}

{
 r19 = #0
 r2 = memb(r17+#1)
 if (cmp.eq(r2.new,#0)) jump:nt ##0x398
}

{
 r20 = r26
 nop
 nop
 nop
}

{
 nop
 nop
}

{
 r1 = r23
 r2 = memw(r20++#4)
}

{
 callr r27
 r2 = r18
 r19 = add(r19,#1)
 r0 = memw(r2+#36)
}

{
 r2 = memub(r17+#1)
 if (cmp.gtu(r2.new,r19)) jump:t -0x14
}
