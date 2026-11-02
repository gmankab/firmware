// SPDX-License-Identifier: GPL-3.0-only
// WLAN [0x47cd8,0x47d34): session-record count and callback error/report path.
// Hardware events are reported through the existing platform callback slots.
.text
.globl agent13_session_record_status_720_77c
agent13_session_record_status_720_77c:
{
    r21 = #0
    r2 = memb(r17+#1)
    if (cmp.eq(r2.new,#0)) jump:nt 0x44
}
L_record:
{
    r2 = memw(r22++#4)
}
{
    r21 = add(r21,#1); r19 = memw(r2+#36)
    r2 = r18
}
{
    call 0x1f70cc
    r1:0 = combine(#43,r19)
}
{
    r2 = memub(r17+#1)
    if (cmp.gtu(r2.new,r21)) jump:t L_record
}
{
    r2 = memw(r29+#12)
}
{
    p0 = r2
    if (p0.new) jump:nt 0x18
}
{
    call 0x1f70b0
    r1:0 = combine(#46,r19)
    r2 = r18
}
{
    jump -0x284
}
{
    r2 = memb(r17+#1)
    if (cmp.eq(r2.new,#0)) jump:nt -0x1ac
}
{
    r2 = #46; jump -0x1c0
    r1:0 = combine(r17,#0)
}
{
    call 0x2031c4
}
{
    jump -0x200
}
