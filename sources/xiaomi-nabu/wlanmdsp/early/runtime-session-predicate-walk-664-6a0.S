// SPDX-License-Identifier: GPL-3.0-only
// WLAN [0x47c1c,0x47c58): record iteration with a conditional callback
// selected by the record presence predicate and an in-packet duplex increment.
.text
.globl agent13_session_predicate_walk_664_6a0
agent13_session_predicate_walk_664_6a0:
{
    p0 = cmp.eq(r18,#0)
    r2 = memb(r17+#1)
    if (cmp.eq(r2.new,#0)) jump:nt -0xac
}
{
    r19:18 = combine(#0,r26)
}
{
    r2 = p0
    memw(r29+#8) = r2.new
}
{
    r1 = #4; r2 = memw(r18+#0)
}
{
    r0 = memw(r2+#36); r2 = memw(r29+#8)
}
{
    p0 = r2
}
{
    call 0x1f717c
    r2 = mux(p0,#0,#1)
    r18 = add(r18,#4); r19 = add(r19,#1)
}
{
    r2 = memub(r17+#1)
    if (cmp.gtu(r2.new,r19)) jump:t -0x18
}
{
    jump -0x1cc
}
