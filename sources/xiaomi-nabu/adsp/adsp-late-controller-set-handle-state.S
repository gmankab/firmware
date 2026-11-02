// SPDX-License-Identifier: GPL-3.0-only
// ADSP [0x3c248,0x3c2f4) -- update controller handle status flags via
// V66-compatible Hexagon LL/SC, checking typed descriptor and counter ranges.
.text
.globl adsp_late_controller_set_handle_state
adsp_late_controller_set_handle_state:
{ call controller_enter }
{ allocframe(#0) }
{ r2 = memw(r0+#12) }
{ r3 = memw(r2+#28) }
{ p0 = tstbit(r3,#16); if (!p0.new) jump:nt alternate_handler; immext(#192); if (p0.new) r4 = memuh(r2+##224) }
{ r5 = lsr(r4,#1) }
{ r5 |= lsr(r3,#19) }
{ p0 = tstbit(r5,#0); if (!p0.new) dealloc_return:t }
{ r3 = and(r4,#64); if (!cmp.eq(r3.new,#0)) jump:nt return_now }
{ immext(#4027745728); r4 = ##-267221564 }
{ r3 = memw(r2+#24); if (cmp.gtu(r4,r3.new)) jump:nt check_flag }
{ immext(#4027745792); r4 = ##-267221456; if (cmp.gtu(r4.new,r3)) jump:t return_now }
check_flag:
{ r3 = memub(r0+#127) }
{ p0 = tstbit(r3,#6); if (!p0.new) r4 = add(r0,#124); if (!p0.new) r3 = add(r0,#-8); if (p0.new) jump:t return_now }
{ immext(#2147483648); r5 = ##-2147483648; memw(r3+#0) = setbit(#0) }
{ memw(r0+#-4) = r1 }
lock_flags:
{ r3 = memw_locked(r4) }
{ r3 = or(r3,r5) }
{ memw_locked(r4,p0) = r3 }
{ if (!p0) jump:nt lock_flags }
{ r3 = #1 }
lock_count:
{ r4 = memw_locked(r1) }
{ r4 = add(r4,r3) }
{ memw_locked(r1,p0) = r4 }
{ if (!p0) jump:nt lock_count }
{ r1:0 = combine(r2,#13); r2 = #1; deallocframe }
{ jump controller_tail }
return_now:
{ dealloc_return }
.set controller_enter, adsp_late_controller_set_handle_state+0xea9a0
.set controller_tail, adsp_late_controller_set_handle_state+0x3404
.set alternate_handler, adsp_late_controller_set_handle_state+0xac
