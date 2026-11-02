// SPDX-License-Identifier: GPL-3.0-only
// WLAN record occupancy/count probe: input record, per-slot table updates,
// hardware status callbacks; assembler-owned Hexagon V66 encodings only.
.text
.globl wlan_record_flag_probe
wlan_record_flag_probe:
L_6f1c:
{
    r16 = r0
    memd(r29+#-16) = r17:16
    allocframe(#8)
}
L_6f24:
{
    r2 = memb(r16+#1)
    if (cmp.eq(r2.new,#0)) jump:nt L_6f94
}
L_6f2c:
{
    r2 = #0
}
L_6f30:
{
    r3 = and(r2,#255)
    r2 = add(r2,#1)
    r4 = memw(gp+#1932)
}
L_6f3c:
{
    r3 = addasl(r4,r3,#2)
    r0 = and(r2,#255)
}
L_6f44:
{
    r3 = memw(r3+#808)
}
L_6f48:
{
    memb(r3+#1) = #1
}
L_6f4c:
{
    r3 = memub(r16+#1)
    if (cmp.gtu(r3.new,r0)) jump:t L_6f30
}
L_6f54:
{
    r17 = #0
    p0 = cmp.eq(r3,#0)
    if (p0.new) jump:nt L_6f94
}
L_6f5c:
{
    r2 = and(r17,#255)
    r1 = ##-1342151028
    r3 = memw(gp+#1932)
}
L_6f6c:
{
    r2 = addasl(r3,r2,#2)
}
L_6f70:
{
    r3:2 = combine(##2097890,#0)
    r4 = memw(r2+#808)
}
L_6f7c:
{
    call 0x1f6958
    r0 = memw(r4+#36)
}
L_6f84:
{
    p0 = cmp.eq(r0,#0)
    if (!p0.new) jump:nt L_6f9c
}
L_6f88:
{
    r17 = add(r17,#1)
    r2 = memub(r16+#1)
}
L_6f8c:
{
    r3 = and(r17,#255)
    if (cmp.gtu(r2,r3.new)) jump:t L_6f5c
}
L_6f94:
{
    r0 = #0
    r17:16 = memd(r29+#0)
    dealloc_return
}
L_6f9c:
{
    call -0x1454
    r0 = ##-1339397237
    r1 = #1921
}
L_6fac:
{
    jumpr r31
    r0 = memw(gp+#1932)
}
