// SPDX-License-Identifier: GPL-3.0-only
// Status/record callback thunk; source-only Hexagon packets.
.text
.globl agent13_wlan_record_callback_thunk
agent13_wlan_record_callback_thunk:
{
    call -0x5c
    r16 = r1
    memd(r29+#-16) = r17:16
    allocframe(#8)
}
{
    r0 = r16
    r17:16 = memd(r29+#0)
    deallocframe
}
{
    jump -0x70
}
