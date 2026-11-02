// Hexagon semantic source, relative packet addresses.
.text
.globl session_state_100_144
session_state_100_144:

{
 r1 = #64
 jump 0x3c8
}

{
 r22 = #1
 r25 = ##-1339396680
}

{
 immext(#576)
 r0 = ##576
 jump 0x2b0
}

{
 r3 = #8
}

{
 immext(#65280)
 r2 = and(r18,##65280)
}

{
 p0 = cmp.eq(r2,#256)
 if (p0.new) r0 = zxtb(r18)
 if (!p0.new) jump:t 0x1b8
 if (!p0.new) r2 = memw(r29+#12)
}

{
 call 0x42ca8
}

{
 jump 0x390
}
