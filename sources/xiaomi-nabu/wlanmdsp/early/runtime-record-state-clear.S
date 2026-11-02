// SPDX-License-Identifier: GPL-3.0-only
// Reset codec metadata status and error flags in a bounded record structure.
.text
.globl agent13_wlan_record_state_clear
agent13_wlan_record_state_clear:
{
    r16 = r0
    r2 = #0
    memd(r29+#-16) = r17:16
    allocframe(#8)
}
{
    r0 = #3840
    memh(r16+##4592) = r2
}
{
    call -0xbdc
    memw(r16+##4588) = r2
}
{
    memw(r16+##4588) = r0
    r17:16 = memd(r29+#0)
}:mem_noshuf
{
    dealloc_return
}
