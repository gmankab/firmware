// SPDX-License-Identifier: GPL-3.0-only
// WLAN file 0x47b7c..0x47bd8. Walk the recorded event table and report
// three bitfields and a per-event status through platform callbacks.
.text
.globl agent13_session_report_5c4_620
agent13_session_report_5c4_620:
{
    r19 = extractu(r18,#1,#1)
    r20 = extractu(r18,#3,#3)
    r2 = memb(r17+#1)
    if (cmp.eq(r2.new,#0)) jump:nt -0xc
}
{
    r20 |= asl(r20,#16)
    r24 = r26
    r23 = #0; r21 = and(r18,#1)
}
{
    r18 = extractu(r18,#1,#2)
}
L_report_loop:
{
    r2 = memw(r24++#4)
}
{
    r2 = r21; r22 = memw(r2+#36)
}
{
    call 0x1f7218
    r1:0 = combine(#56,r22)
}
{
    call 0x1f7210
    r1:0 = combine(#57,r22)
    r2 = r19
}
{
    call 0x1f7204
    r1:0 = combine(#58,r22)
    r2 = r18
}
{
    call 0x28943c
    r0 = r20; r23 = add(r23,#1)
}
{
    r2 = memub(r17+#1)
    if (cmp.gtu(r2.new,r23)) jump:t L_report_loop
}
{
    jump -0x14c
}
