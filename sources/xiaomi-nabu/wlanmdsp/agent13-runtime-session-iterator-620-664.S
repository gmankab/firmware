// SPDX-License-Identifier: GPL-3.0-only
// WLAN file [0x47bd8,0x47c1c): report a session record array when populated.
// NOP words preserve compiler/linker packet layout; all are assembled, not copied.
.text
.globl agent13_session_iterator_620_664
agent13_session_iterator_620_664:
{
    r19 = #0
    nop
    r2 = memb(r17+#1)
    if (cmp.eq(r2.new,#0)) jump:nt -0x68
}
{
    nop
    nop
}
{
    nop
    nop
    nop
    nop
}
L_record_iter:
{
    r1 = r18
    r2 = memw(r20++#4)
}
{
    call 0x15f114
    r19 = add(r19,#1); r0 = memw(r2+#36)
}
{
    r2 = memub(r17+#1)
    if (cmp.gtu(r2.new,r19)) jump:t L_record_iter
}
{
    jump -0x190
}
