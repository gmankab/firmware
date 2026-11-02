// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 doubly-linked descriptor queue primitives.
// Executable file offsets [0x3a650,0x3a72c). The preceding [0x3a648,
// 0x3a650) contains two zero/sentinel words and is NOT source-built code.
// Decode 64-bit descriptor entries, attach/detach links and keep conditional
// memory writes within their valid V60 packets.
.text
.globl agent11_modem_link_detach
agent11_modem_link_detach:
{
 r7 = memw(r0+#0)
 r3:2 = memd(r1+#0)
}
{
 p0 = cmp.eq(r7,r1)
 p1 = cmp.eq(r1,r2)
 memw(r3+#0) = r2
}
{
 if(p1) r2 = #0
 if(!p0) jumpr:nt r31
 memw(r2+#4) = r3
}
{
 jumpr r31
 memw(r0+#0) = r2
}
.globl agent11_modem_link_prepend
agent11_modem_link_prepend:
{
 r5:4 = combine(r1,r1)
 r7 = memw(r0+#0)
}
{
 p0 = cmp.eq(r7,#0)
 if(!p0.new) r5:4 = memd(r7+#0)
 memd(r1+#0) = r5:4
}
{
 if(!p0) r3:2 = combine(r5,r7)
 if(p0) jumpr:nt r31
 memw(r0+#0) = r1
}
{ memd(r1+#0) = r3:2 }
{ memw(r5+#0) = r1 }
{
 jumpr r31
 memw(r7+#4) = r1
}
.globl agent11_modem_link_attach
agent11_modem_link_attach:
{
 r5:4 = combine(r1,r1)
 r7 = memw(r0+#0)
}
{
 p0 = cmp.eq(r7,#0)
 if(!p0.new) r5:4 = memd(r7+#0)
 memd(r1+#0) = r5:4
}
{
 if(!p0) r3:2 = combine(r5,r7)
 if(p0) jumpr:nt r31
 if(p0) memw(r0+#0) = r1
}
{ memd(r1+#0) = r3:2 }
{ memw(r5+#0) = r1 }
{
 jumpr r31
 memw(r7+#4) = r1
}
.globl agent11_modem_link_reparent
agent11_modem_link_reparent:
{
 p2 = or(p0,!p0)
 r5:4 = combine(r2,r2)
 r3 = memw(r1+#0)
 r9:8 = memd(r2+#0)
}
{
 p0 = cmp.eq(r3,#0)
 p3 = cmp.eq(r2,r8)
 r6 = memw(r0+#0)
 if(!p0.new) r5:4 = memd(r3+#0)
}
.set modem_reparent_skip,agent11_modem_link_reparent+0x38
{
 p2 = cmp.eq(r6,r2)
 if(p3) r8 = #0
 if(p3) jump:nt modem_reparent_skip
 if(p0) memw(r1+#0) = r2
}
{ if(!p3) memw(r9+#0) = r8 }
{ if(!p3) memw(r8+#4) = r9 }
modem_reparent_skip:
{
 if(!p0) r5:4 = combine(r5,r3)
 if(p2) memw(r0+#0) = r8
}
{
 if(p0) jumpr:nt r31
 memd(r2+#0) = r5:4
}
{ memw(r5+#0) = r2 }
{
 jumpr r31
 memw(r4+#4) = r2
}
