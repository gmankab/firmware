// SPDX-License-Identifier: GPL-3.0-only
// ADSP [file 0x3f1c0,0x3f31c): mode-selecting controller requests,
// stack parameter forwarding and locked-memory linked-node registration.
.text
.globl adsp_late_controller_request_forward
adsp_late_controller_request_forward:
{ r17:16 = combine(r1,r4); p0 = cmp.gt(r2,#2); memd(r29+#-16) = r17:16; allocframe(#48) }
{ r18 = r3; r9 = memw(r29+#76); memd(r29+#32) = r19:18 }
{ r7 = memw(r29+#68); memd(r29+#24) = r21:20 }
{ r6 = memw(r29+#60); r8 = memw(r29+#72) }
{ if (p0) jump:nt high_modes; r4 = memw(r29+#64); r3 = memw(r29+#56) }
{ p0 = cmp.eq(r2,#1); if (p0.new) jump:nt check_descriptor }
{ p0 = cmp.eq(r2,#2); if (!p0.new) jump:nt forward_controller; if (p0.new) memw(r29+#20) = r9; if (p0.new) memw(r29+#16) = r8 }
{ r1 = r17; r3:2 = combine(r18,#2); memw(r29+#8) = r4; memw(r29+#0) = r3 }
{ r4 = r16 }
{ call forward_platform_base; memw(r29+#12) = r7; memw(r29+#4) = r6 }
restore_basic:
{ r17:16 = memd(r29+#40); r19:18 = memd(r29+#32) }
{ r21:20 = memd(r29+#24); dealloc_return }
high_modes:
{ r20 = #-1; p0 = cmp.eq(r2,#3); if (p0.new) jump:nt optional_other_mode }
{ p0 = cmp.eq(r2,#10); if (!p0.new) jump:nt forward_controller; if (p0.new) memw(r29+#20) = r9; if (p0.new) memw(r29+#16) = r8 }
{ r1 = r17; r3:2 = combine(r18,#10); memw(r29+#8) = r4; memw(r29+#0) = r3 }
{ r4 = r16 }
{ call forward_platform_base; memw(r29+#12) = r7; memw(r29+#4) = r6 }
{ r20 = r0; if (!cmp.gt(r20.new,#0)) jump:t report_result }
{ immext(#4028152128); r0 = ##-266815136; memw(r29+#4) = r20; memw(r29+#0) = r17 }
{ call platform_log }
{ call log_optional; immext(#4028152192); r0 = ##-266815064; r1 = #204 }
check_descriptor:
{ r20 = #-1; if (p0.new) r0 = #16; p0 = cmp.eq(r0,#3); if (!p0.new) jump:t report_result }
{ call allocate_record }
{ r19 = r0; if (cmp.eq(r19.new,#0)) jump:nt report_result }
{ call notify_controller; r0 = r18; memw(r19+#4) = r18 }
{ if (!p0.new) r2 = add(r18,r0); p0 = cmp.eq(r0,#0); if (p0.new) jump:nt use_current; memw(r19+#8) = r0 }
{ r2 = memb(r2+#-1) }
{ p0 = cmp.eq(r2,#47); if (!p0.new) r2 = add(r0,#1); jump add_to_registry; if (!p0.new) memw(r19+#8) = r2.new }
optional_other_mode:
{ p0 = cmp.eq(r18,#4); if (p0.new) jump:t report_result }
forward_controller:
{ r1 = r17; r3 = r18; memw(r29+#8) = r4; memw(r29+#0) = r3 }
{ r4 = r16; memw(r29+#20) = r9; memw(r29+#16) = r8 }
{ call forward_secondary; memw(r29+#12) = r7; memw(r29+#4) = r6 }
{ r17:16 = memd(r29+#40); r19:18 = memd(r29+#32) }
{ r21:20 = memd(r29+#24); dealloc_return }
use_current:
{ r2 = add(r0,#1); memw(r19+#8) = r2.new }
add_to_registry:
{ r2 = add(r17,#12); memw(r19+#12) = r16 }
registry_store:
{ r3 = memw(r2+#0); memw(r19+#0) = r3.new }
cas_retry:
{ r4 = memw_locked(r2) }
{ p0 = cmp.eq(r4,r3) }
{ if (!p0) jump:nt cas_finished }
{ memw_locked(r2,p0) = r19 }
{ if (!p0) jump:nt cas_retry }
cas_finished:
{ p0 = cmp.eq(r4,r3); if (!p0.new) jump:t registry_store }
{ r20 = #0 }
report_result:
{ r0 = r20; r17:16 = memd(r29+#40); r19:18 = memd(r29+#32) }
{ r21:20 = memd(r29+#24); dealloc_return }
.set forward_platform_base, adsp_late_controller_request_forward-0x140
.set platform_log, adsp_late_controller_request_forward+0xe7c6c
.set log_optional, adsp_late_controller_request_forward-0x8cc
.set allocate_record, adsp_late_controller_request_forward-0x4654
.set notify_controller, adsp_late_controller_request_forward+0x106580
.set forward_secondary, adsp_late_controller_request_forward+0xeb708
