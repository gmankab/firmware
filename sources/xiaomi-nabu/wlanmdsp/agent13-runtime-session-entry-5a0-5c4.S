// SPDX-License-Identifier: GPL-3.0-only
// WLAN file 0x47b58..0x47b7c. Controller short entry: empty-record
// guard and diagnostic callback, with an alternate class on success.
.text
.globl agent13_session_entry_5a0_5c4
agent13_session_entry_5a0_5c4:
{
    r1:0 = combine(r17,#0)
    r3 = memb(r17+#1)
    if (cmp.eq(r3.new,#0)) jump:nt L_report
}
{
    r3 = r18
}
{
    call 0x2850
    r4 = r20
}
L_report:
{
    call 0x2061dc
    r0 = #1
}
{
    jump 0x29dc84
}
