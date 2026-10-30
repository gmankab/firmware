// SPDX-License-Identifier: GPL-3.0-only
// WLAN [0x47d34,0x47d48): report an empty session or select controller
// state 2 for the next event-dispatch path.
.text
.globl agent13_session_setup_77c_790
agent13_session_setup_77c_790:
{
    r1:0 = combine(r17,#0)
    r2 = memb(r17+#1)
    if (cmp.eq(r2.new,#0)) jump:nt -0x1c4
}
{
    r3:2 = combine(#2,#18)
    jump -0x1d8
}
