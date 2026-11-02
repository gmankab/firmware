// SPDX-License-Identifier: GPL-3.0-only
// Selected-controller-thread early guard, file [0x6220,0x624c).
.text
.globl adsp_agent20_rwe_thread_guard
adsp_agent20_rwe_thread_guard:
{ r19 = extract(r0,#3,#0); p0 = tstbit(r0,#30) }
{ nop } // V66 r1=HTID
{ p1 = cmp.eq(r19,r1) }
{ p2 = and(p0,p1); if (p2.new) jump:nt branch_other_thread }
{ r19 = ##-0xffdd704 }
{ r0 = memw_locked(r19) }
{ p0 = tstbit(r0,r1); if (p0.new) jump:nt branch_locked }
{ nop; r0 = ##-0xffdaed8; r28 = r1 } // V66 P3:0 = R28
.set branch_other_thread,adsp_agent20_rwe_thread_guard+(0x8be03394-0x8be03220)
.set branch_locked,adsp_agent20_rwe_thread_guard+(0x8be034b0-0x8be03220)
