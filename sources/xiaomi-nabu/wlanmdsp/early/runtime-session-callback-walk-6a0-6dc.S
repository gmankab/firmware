// SPDX-License-Identifier: GPL-3.0-only
// WLAN file [0x47c58,0x47c94): read session rows and dispatch one of two
// controller callbacks, preserving packet-local predicates and duplex loop.
.text
.globl agent13_session_callback_walk_6a0_6dc
agent13_session_callback_walk_6a0_6dc:
{
    p0 = cmp.eq(r18,#0)
    r2 = memb(r17+#1)
    if (cmp.eq(r2.new,#0)) jump:nt -0xe8
}
{
    r19:18 = combine(#0,r26)
}
{
    r2 = p0
    memw(r29+#8) = r2.new
}
{
    r2 = memw(r18+#0)
}
{
    r0 = memw(r2+#36); r2 = memw(r29+#8)
}
{
    p0 = r2
}
{
    call 0x15e288
    r1 = mux(p0,#0,#1)
    r18 = add(r18,#4); r19 = add(r19,#1)
}
{
    r2 = memub(r17+#1)
    if (cmp.gtu(r2.new,r19)) jump:t -0x18
}
{
    jump -0x208
}
