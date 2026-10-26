// SPDX-License-Identifier: GPL-3.0-only
// Independently assembled modem RWE protected exception-lock helpers.
// Disjoint from the prior [file 0x38108,0x390dc) verified region.
// This executable region begins after non-executable fill and opaque data;
// never count the gap as independently authored instruction bytes.
.text
.globl agent11_modem_rwe_bit31_lock_update
agent11_modem_rwe_bit31_lock_update:
{ r18 = memw_locked(r0) }
agent11_modem_rwe_bit31_lock_retry:
{
    p0 = tstbit(r18,#31)
    if (p0.new) jump:nt agent11_modem_rwe_bit31_lock_update
}
{ r18 = setbit(r18,#31) }
{ memw_locked(r0,p0) = r18 }
{
    p1 = tstbit(r18,#30)
    if (!p0) jump:nt agent11_modem_rwe_bit31_lock_update
}
.set agent11_modem_rwe_lock_failed,agent11_modem_rwe_bit31_lock_update + (0x3994c-0x39920)
{ r18 = clrbit(r18,#31); if (!p1) jump:nt agent11_modem_rwe_lock_failed }
{ jumpr lr; r0 = #1; memw(r0+#0) = r18 }
agent11_modem_rwe_lock_failed:
{ jumpr lr; r0 = #0; memw(r0+#0) = r1 }
