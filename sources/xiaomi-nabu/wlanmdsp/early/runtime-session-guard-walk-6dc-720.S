// SPDX-License-Identifier: GPL-3.0-only
// WLAN [0x47c94,0x47cd8): validate a session record count and invoke
// alternate successful/failed controller callbacks in a bounded record walk.
.text
.globl agent13_session_guard_walk_6dc_720
agent13_session_guard_walk_6dc_720:
{
    p0 = cmp.eq(r18,#0)
    r2 = memb(r17+#1)
    if (cmp.eq(r2.new,#0)) jump:nt -0x124
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
    if (p0.new) jump:nt 0x10
}
{
    call 0x203220
}
{
    jump 0x8
}
{
    call 0x20322c
}
{
    r18 = add(r18,#4); r19 = add(r19,#1)
}
{
    r2 = memub(r17+#1)
    if (cmp.gtu(r2.new,r19)) jump:t -0x20
}
{
    jump -0x24c
}
