// SPDX-License-Identifier: GPL-3.0-only
// Controller context initialization and conditional platform allocation.
// Original ADSP file [0x3f788,0x3f80c), 132 executable bytes.
.text
.globl adsp_late_qurt_controller_context_create
adsp_late_qurt_controller_context_create:
{ r16 = r1; memd(r29+#-16) = r17:16; allocframe(#16) }
{ call platform_allocate; r19:18 = combine(r2,r0); r0 = #20; memd(r29+#0) = r19:18 }
{ r17 = r0; if (cmp.eq(r17.new,#0)) jump:nt allocation_failed }
{ p0 = r19; immext(#5373888); r2 = ##5373950; memw(r17+#4) = r2.new }
{ r1:0 = combine(r18,#0); immext(#4026779648); if (p0) r4 = ##-268187636; r3:2 = combine(r17,#80) }
{ memw(r17+#12) = r18; immext(#4026780160); memw(r17+#8) = ##-268187108 }
{ immext(#4026780032); if (!p0) r4 = ##-268187220 }
{ call platform_create; memw(r17+#0) = r4; memw(r17+#16) = r16 }
{ r18 = r0 }
{ p0 = cmp.gt(r18,#-1); if (!p0.new) r0 = add(r17,#0) }
{ immext(#4294948736); if (!p0) call platform_release }
{ jump return_status; memw(r16+#12) = r18; memw(r16+#16) = r17 }
allocation_failed:
{ r18 = #-1 }
return_status:
{ r0 = r18; r17:16 = memd(r29+#8); r19:18 = memd(r29+#0) }
{ dealloc_return }
.set platform_allocate, adsp_late_qurt_controller_context_create-0x4c1c
.set platform_create, adsp_late_qurt_controller_context_create+0xeb6a4
.set platform_release, adsp_late_qurt_controller_context_create-0x47e8
