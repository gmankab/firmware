// SPDX-License-Identifier: GPL-3.0-only
// Report selected codec parameter bitfields and branch to the next session
// handling state. This is a partial Hexagon function, not a fabricated return.
.text
.globl agent13_wlan_session_parameter_report
agent13_wlan_session_parameter_report:
{
    r2 = memw(r29+#12)
}
{
    p0 = r2
    if (p0.new) jump:nt 0x618
}
{
    r2 = addasl(r26,r16,#2)
}
{
    r2 = memw(r2+#-4)
}
{
    r2 = and(r18,#1)
    r17 = memw(r2+#36)
}
{
    call 0x1f7844
    r1:0 = combine(#56,r17)
}
{
    r2 = extractu(r18,#1,#1)
    call 0x1f783c
    r1:0 = combine(#57,r17)
}
{
    r2 = extractu(r18,#1,#2)
    call 0x1f7830
    r1:0 = combine(#58,r17)
}
{
    r0 = extractu(r18,#3,#3)
}
{
    r0 |= asl(r0,#16)
    call 0x289a64
}
{
    jump 0x58c
}
{
    r1 = #0
    jump ##0x488
}
{
    r2 = #50
    jump ##0x558
}
