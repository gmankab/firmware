// Hexagon semantic source, relative packet addresses.
.text
.globl session_mode9_tail
session_mode9_tail:

{
 r2 = memw(r29+#12)
}

{
 p0 = r2
 if (p0.new) jump:nt 0x514
}

{
 r2 = addasl(r26,r16,#2)
 r1 = #4; 	p0 = cmp.eq(r18,#0)
}

{
 r2 = memw(r2+#-4)
}

{
 r2 = mux(p0,#0,#1)
 jump 0x410
 r0 = memw(r2+#36)
}

{
 r2 = #0 ; jump 0x3e0
}

{
 r2 = #1 ; jump 0x3dc
}

{
 r2 = #2 ; jump 0x3d8
}

{
 r2 = #3 ; jump 0x3d4
}

{
 r2 = #5 ; jump 0x3d0
}

{
 r2 = #11 ; jump 0x3cc
}

{
 r2 = #14 ; jump 0x3c8
}

{
 r2 = #15 ; jump 0x3c4
}

{
 p0 = cmp.eq(r18,#1); if (p0.new) jump:nt 0x8
}

{
 p0 = cmp.eq(r18,#0); if (!p0.new) jump:nt 0x33c
}

{
 call 0x107a00
}

{
 jump 0x334
}
