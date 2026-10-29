// SPDX-License-Identifier: GPL-3.0-only
// Walk record descriptors, copy bounded variable-length attributes into a
// local frame and preserve per-slot counters. A fragment of a larger function.
// All packets are reconstructed as Hexagon instructions, not OEM data bytes.
.text
.globl agent13_wlan_session_record_loop
agent13_wlan_session_record_loop:
L_260:
{
    r20 = add(r25,mpyi(#28,r19))
}
{
    call -0x1bc
    r0 = r20
}
{
    r0 = add(r29,#24)
    r1 = #0
    r2 = #48
    r26 = memw(r20+#4)
}
{
    call -0x5288
    r19 = add(r17,#1)
}
{
    r0 = add(r29,#27)
    r1 = add(r23,r19)
    r2 = memw(r20+#8)
    memw(r29+#64) = r2.new
}
{
    p0 = tstbit(r2,#0)
    if (!p0.new) jump:t L_2f4
}
{
    r3 = memw(r20+#12)
    r18 = memw(r20+#16)
}
{
    r21 = and(r3,#255)
    memb(r29+#24) = r3
    memb(r29+#25) = r18
}
{
    call -0x512c
    r2 = r21
}
{
    r2 = #255
    r19 = add(r21,r19)
}
{
    p0 = bitsclr(r18,r2)
    r1 = add(r23,r19)
    if (p0.new) jump:nt L_2f4
}
{
    r2 = add(r29,#24)
    r21 = and(r18,#255)
}
{
    call -0x514c
    r0 = add(r2,#8)
    r2 = r21
}
{
    r18 = add(r21,#7)
    r3 = add(r29,#24)
    r1 = r23
}
{
    r1 += add(r21,r19)
    r2 = lsr(r18,#3)
    r0 = add(r3,#28)
    r19 = add(r21,r19)
}
{
    call -0x5170
    r19 += lsr(r18,#3)
}
L_2f4:
{
    r2 = memub(r29+#64)
}
{
    p0 = !tstbit(r2,#1)
    if (p0.new) jump:t L_32c
}
{
    r21 = memw(r20+#20)
    if (cmp.eq(r21.new,#0)) jump:nt 0x144
}
{
    r2 = add(r29,#24)
    r1 = add(r23,r19)
}
{
    call -0x5194
    r0 = add(r2,#31)
    r2 = r21
}
{
    r19 += add(r21,#1)
    r2 = add(r21,r19)
}
{
    r2 = memub(r23+r2<<#0)
    memb(r29+#26) = r2.new
}
L_32c:
{
    r20 = memw(r20+#24)
    if (cmp.eq(r20.new,#0)) jump:nt L_344
}
{
    call -0x51b8
    r1 = add(r23,r19)
    r2 = r20
    r0 = memw(r29+#16)
}
{
    r19 = add(r20,r19)
}
L_344:
