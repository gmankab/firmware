// SPDX-License-Identifier: GPL-3.0-only
// Source-reconstructed 416-byte controller mode/argument handler at
// ADSP file [0x3f80c,0x3f9ac). Uses checked packet boundaries and
// symbolic calls into known QuRT/controller routines.
.text
.globl adsp_late_qurt_controller_parameter_handler
adsp_late_qurt_controller_parameter_handler:
{ p0 = cmp.eq(r2,#3); r17:16 = combine(#-1,r1); memd(r29+#-16) = r17:16; allocframe(#40) }
{ if (p0) jump:nt return_status; r18 = r4; memd(r29+#24) = r19:18 }
{ p0 = cmp.eq(r2,#195); if (p0.new) jump:nt handle_secondary_type }
{ p0 = cmp.eq(r2,#53); if (!p0.new) jump:t alternate_type; if (!p0.new) r9 = memw(r29+#52); if (!p0.new) r8 = memw(r29+#56) }
{ call query_controller; r1 = r3; r17 = #0; r0 = memw(r16+#16) }
{ jump return_status }
handle_secondary_type:
{ p0 = cmp.eq(r0,#3); if (!p0.new) jump:t return_status; if (p0.new) r19 = memw(r16+#16) }
{ r2 = memw(r19+#4) }
{ p0 = !tstbit(r2,#2); if (!p0.new) r1:0 = combine(r3,r19); if (!p0.new) r3:2 = combine(r5,r18); if (!p0.new) jump:t external_cleanup }
{ p0 = bitsclr(r2,#3); if (p0.new) jump:nt check_descriptor; if (p0.new) r4 = memw(r19+#24) }
{ r2 = memw(r19+#44); r4 = memw(r19+#48) }
{ r2 = memw(r2+#24) }
{ r2 = memw(r2+#0) }
{ r4 += asl(r2,#12) }
{ r2 = lsr(r4,#12); if (!cmp.eq(r2.new,r3)) jump:t return_status }
{ r2 = memw(r19+#24) }
{ r2 = memw(r2+#4); if (!cmp.eq(r2.new,r18)) jump:t return_status }
{ p0 = cmp.eq(r5,#0); r3 = add(r19,#20) }
{ if (p0) jump:nt setup_new_command; if (p0) memw(r3+#0) = #0 }
{ call platform_submit; r1:0 = combine(r5,#0); r2 = #196 }
{ p0 = cmp.gt(r0,#-1); if (!p0.new) jump:nt return_status }
setup_new_command:
{ r2 = #255; r1:0 = combine(r16,#3); r3 = memw(r19+#12); memw(r19+#4) = r2.new }
{ r2 = #3; r5:4 = combine(#0,#0); memw(r29+#20) = #0; memw(r29+#16) = #0 }
{ r17 = r18; memw(r29+#12) = #0; memw(r29+#8) = #0 }
{ call platform_dispatch; memw(r29+#4) = #0; memw(r29+#0) = #0 }
{ jump return_status }
alternate_type:
{ r1 = r16; r4 = memw(r29+#68); memw(r29+#20) = r4.new }
{ r4 = r18; r7 = memw(r29+#60) }
{ r6 = memw(r29+#64); memw(r29+#16) = r6.new }
{ r12 = memw(r29+#48); memw(r29+#12) = r7 }
{ memw(r29+#8) = r8; memw(r29+#4) = r9 }
{ call platform_dispatch; memw(r29+#0) = r12 }
{ r17:16 = memd(r29+#32); r19:18 = memd(r29+#24) }
{ dealloc_return }
external_cleanup:
{ r17:16 = memd(r29+#32); r19:18 = memd(r29+#24) }
{ jump platform_cleanup; deallocframe }
check_descriptor:
{ r6 = memw(r4+#0); if (!cmp.eq(r6.new,r3)) jump:t return_status }
{ p0 = !tstbit(r2,#16); if (p0.new) jump:nt return_status }
{ r2 = memw(r4+#4); if (cmp.gtu(r2.new,r18)) jump:nt return_status }
{ p0 = cmp.eq(r5,#0); r2 = memw(r19+#32) }
{ if (p0) jump:nt check_not_empty; if (p0) memw(r2+#20) = #0 }
{ call platform_submit; r3 = add(r2,#20); r1:0 = combine(r5,#0); r2 = #196 }
{ p0 = cmp.gt(r0,#-1); if (!p0.new) jump:nt return_status; if (p0.new) r19 = memw(r16+#16) }
check_not_empty:
{ r1:0 = combine(r16,#3); r5:4 = combine(#0,#0); r2 = memw(r19+#24); r3 = memw(r19+#12) }
{ r2 = #3; r17 = memw(r2+#4); memw(r29+#20) = #0 }
{ memw(r29+#16) = #0; memw(r29+#12) = #0 }
{ memw(r29+#8) = #0; memw(r29+#4) = #0 }
{ call platform_dispatch; memw(r29+#0) = #0 }
return_status:
{ r0 = r17; r17:16 = memd(r29+#32); r19:18 = memd(r29+#24) }
{ dealloc_return }
.set query_controller, adsp_late_qurt_controller_parameter_handler+0x1bac
.set platform_submit, adsp_late_qurt_controller_parameter_handler+0xeb620
.set platform_dispatch, adsp_late_qurt_controller_parameter_handler+0xeb0bc
.set platform_cleanup, adsp_late_qurt_controller_parameter_handler+0x13f4
