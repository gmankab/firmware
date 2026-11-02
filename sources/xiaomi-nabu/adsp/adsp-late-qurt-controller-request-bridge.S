// SPDX-License-Identifier: GPL-3.0-only
// Original ADSP file [0x3f9ac,0x3fa1c), 112 bytes. Dispatches controller
// request types, using a checked frame and stack-marshalled callback values.
.text
.globl adsp_late_qurt_controller_request_bridge
adsp_late_qurt_controller_request_bridge:
{ p0 = cmp.eq(r2,#53); r7:6 = combine(r0,r1); if (p0.new) jump:nt mode_53; allocframe(#24) }
{ r1:0 = combine(r6,r7); p0 = cmp.eq(r2,#70); if (!p0.new) jump:nt generic_request; if (!p0.new) r8 = memw(r29+#52) }
{ immext(#192); r3:2 = combine(r6,##197); r1:0 = combine(r7,#0); deallocframe }
{ jump dispatch_request }
mode_53:
{ call query_special; r1 = r3; r0 = memw(r6+#16) }
{ r0 = #0; dealloc_return }
generic_request:
{ r12 = memw(r29+#44); r14 = memw(r29+#36) }
{ r13 = memw(r29+#40); memw(r29+#20) = r8 }
{ r9 = memw(r29+#48); memw(r29+#16) = r9.new }
{ r15 = memw(r29+#32); memw(r29+#12) = r12 }
{ memw(r29+#8) = r13; memw(r29+#4) = r14 }
{ call dispatch_marshalled; memw(r29+#0) = r15 }
{ dealloc_return }
.set dispatch_request, adsp_late_qurt_controller_request_bridge+0xeb480
.set query_special, adsp_late_qurt_controller_request_bridge+0x1a0c
.set dispatch_marshalled, adsp_late_qurt_controller_request_bridge+0xeaf1c
