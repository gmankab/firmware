// SPDX-License-Identifier: GPL-3.0-only
// Advance per-session record cursor, update bitmap/header flags, and
// conditionally append variable-length attributes to the bounded queue.
// Partial Hexagon function; continuation remains in following source.
.text
.globl agent13_wlan_session_record_update
agent13_wlan_session_record_update:
L_344:
{
    r17 = add(r26,r17)
    r21 = #0
    r2 = memw(r29+#20)
}
{
    p0 = r2
    if (p0.new) jump:nt L_384
    if (!p0.new) r20 = memw(r16+##4588)
}
L_35c:
{
    r0 = add(r29,#24)
    r1 = r20
    r2 = #48
    r26 = memuh(r20+#38)
}
{
    call -0x39a4
    memh(r29+#62) = r26
}
{
    p0 = cmp.eq(r0,#0)
    if (p0.new) jump:nt L_388
}
{
    r20 = add(r20,#48)
    r21 = add(r21,#1)
    if (cmp.gtu(r24,r21.new)) jump:t L_35c
}
L_384:
{
    r2 = #0
    jump L_394
}
L_388:
{
    r2 = #1
    r3 = memw(r22+#4)
}
{
    r3 = setbit(r26,r3)
    memh(r20+#38) = r3.new
}
L_394:
{
    p0 = !cmp.gtu(r19,r17)
    if (!p0.new) jump:nt 0x88
    if (!p0.new) memw(r29+#0) = r19
}
{
    r1 = add(r29,#24)
    p0 = cmp.eq(r2,#0)
    if (!p0.new) jump:t L_3d8
}
{
    r2 = memub(r16+##4592)
    r4 = memw(r16+##4588)
}
{
    r3 = memw(r22+#4)
}
{
    r0 = add(r4,mpyi(#48,r2))
    r3 = lsl(#1,r3)
    r2 = #48
    memh(r29+#62) = r3.new
}
{
    call -0x5250
}
{
    r2 = memw(r29+#12)
}
{
    memb(r2+#0) += #1
}
L_3d8:
{
    r27 = add(r27,#1)
    r2 = memw(r22+#12)
}
{
    r19 = and(r27,#255)
    if (cmp.gtu(r2,r19.new)) jump:t -0x18c
}
