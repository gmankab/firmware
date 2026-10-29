// SPDX-License-Identifier: GPL-3.0-only
// WLAN status compact: read codec mode, report pending status and tail-dispatch.
// Semantic Hexagon packets only; no reference instruction words.
.text
.globl agent13_wlan_record_status_compact
agent13_wlan_record_status_compact:
L_start:
{
    r16 = r0
    memd(r29+#-16) = r17:16
    allocframe(#16)
}
{
    r1 = #28690
    r0 = ##472
    r2 = memw(r16+#20)
}
{
    r2 = and(r2,#1)
    r3 = memub(r16+#5)
    memw(r29+#4) = r3.new
}
{
    call 0x2141e0
    memw(r29+#0) = r2
}
{
    r0 = ##2955570472
    r2 = memw(r16+#20)
}
{
    call -0x4cc4
    r1 = and(r2,#1)
    r2 = memub(r16+#5)
}
{
    r2 = memw(r16+#20)
}
{
    p0 = tstbit(r2,#0)
    if (p0.new) jump:t L_yes
}
{
    r2 = memb(r16+#5)
    if (cmp.eq(r2.new,#0)) jump:nt L_no
}
L_yes:
{
    r2 = #1
    jump L_done
}
L_no:
{
    r2 = #0
}
L_done:
{
    r0 = memw(r16+#52)
    r1 = memw(r16+#60)
}
{
    jump 0xc9d44
    r17:16 = memd(r29+#8)
    deallocframe
}
