// SPDX-License-Identifier: GPL-3.0-only
// Source-only Hexagon V65/V66 semantics, WLAN file [0x47ab0,0x47b58).
// Controller session handler: conditional state selection, diagnostic logging,
// guarded metadata changes, and report/dispatch callbacks.
.text
.globl agent13_session_handler_next_4f8_5a0
agent13_session_handler_next_4f8_5a0:
{
    p0 = r2
    if (!p0.new) r2 = add(r18,#0)
    if (p0.new) r22 = add(r26,#0)
    if (p0.new) jump:nt 0x228
}
{
    call 0x1f72fc
    r1:0 = combine(#43,r19)
}
{
    jump 0x244
}
{
    r0 = ##624
    r1 = #529
}
{
    call 0x2136dc
    memw(r17+##4160) = r18
    memw(r29+#0) = r18
}
{
    r0 = ##-1339396608
    r1 = memw(r17+##4160)
}
{
    call -0x4eb0
}
{
    jump -0x74
}
{
    r2 = #48; jump 0x8
}
{
    r2 = #55
}
{
    r3 = memw(r29+#12)
}
{
    p0 = r3
    if (!p0.new) r3 = add(r2,#0)
    if (p0.new) jump:nt 0x4c
}
{
    r2 = addasl(r26,r16,#2)
    r1 = r3
}
{
    r2 = r18
    r4 = memw(r2+#-4)
}
{
    r0 = memw(r4+#36)
}
{
    call 0x1f7290
}
{
    r2 = addasl(r26,r16,#2)
    r1 = #1
    r5:4 = combine(#1,##3701)
}
{
    r3:2 = combine(##-1339397133,#4)
    r6 = memw(r2+#-4)
}
{
    call 0x204fa4
    r0 = memw(r6+#36)
}
{
    jump 0x29dca8
}
