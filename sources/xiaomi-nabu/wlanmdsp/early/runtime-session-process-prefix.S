// SPDX-License-Identifier: GPL-3.0-only
// Session record traversal and header-attribute normalization; initial
// validated prefix of a larger Hexagon function (no fabricated epilogue).
.text
.globl agent13_wlan_session_process_prefix
agent13_wlan_session_process_prefix:
{
    r2 = #80
    r16 = r0
    r22 = memw(r1+#0)
}
{
    r17 = memub(r22+#8)
}
{
    r3 = add(r17,#-1)
    if (!cmp.gtu(r2,r3.new)) jump:nt 0x274
}
{
    r2 = memw(r22+#12)
}
{
    r3 = #11
    if (!cmp.gtu(r3.new,r2)) jump:nt 0x294
}
{
    r18 = #4593
    r25 = memw(r1+#12)
    r23 = memw(r1+#24)
}
{
    r3 = memb(r16+r18<<#0)
    r24 = memub(r16+##4592)
}
{
    p0 = cmp.eq(r3,#0)
    if (p0.new) r3 = add(r17,#0)
    if (p0.new) memb(r16+r18<<#0) = r17
    if (p0.new) r2 = memw(r22+#12)
}:mem_noshuf
{
    r3 = and(r3,#255)
}
{
    r4 = add(r2,r24)
    if (cmp.gtu(r4.new,r3)) jump:nt 0x28c
}
{
    r27 = #0
    p0 = cmp.eq(r2,#0)
    if (p0.new) jump:nt 0x1c0
}
{
    p0 = cmp.eq(r24,#0)
    r19 = #0
    r17 = #0
    r2 = add(r29,#24)
}
{
    r2 = add(r2,#44)
    r0 = add(r16,#4592)
    memw(r29+#16) = r2.new
}
{
    r1 = p0
    memw(r29+#20) = r1.new
}
{
    memw(r29+#12) = r0
}
{
    r2 = memb(r23+r17<<#0)
}
{
    r3 = sxtb(r27)
    if (!cmp.eq(r3.new,r2)) jump:nt 0x194
}
