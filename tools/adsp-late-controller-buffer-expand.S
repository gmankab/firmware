// SPDX-License-Identifier: GPL-3.0-only
// [file 0x3eef0,0x3efa8): resize a controller-owned packed descriptor
// allocation, walk its existing blocks, and invoke its completion callback.
.text
.globl adsp_late_controller_buffer_expand
adsp_late_controller_buffer_expand:
{ r16 = r0; memd(r29+#-16) = r17:16; allocframe(#32) }
{ r17 = add(r16,#24); memd(r29+#16) = r19:18; memd(r29+#8) = r21:20 }
{ call prepare_context; r0 = r17; memd(r29+#0) = r23:22 }
{ r2 = memw(r16+#0); if (cmp.eq(r2.new,#0)) jump:nt resize_buffer }
{ call finish_context; r0 = r17 }
{ r0 = r16; r17:16 = memd(r29+#24); r19:18 = memd(r29+#16) }
{ r21:20 = memd(r29+#8); r23:22 = memd(r29+#0) }
{ jump return_context; deallocframe }
resize_buffer:
{ r2 = memw(r16+#40); r3 = memw(r16+#16) }
{ r0 = mpyi(r3,r2); call allocate_storage }
{ r18 = r0; if (cmp.eq(r18.new,#0)) jump:nt allocation_failed }
{ r3 = memw(r16+#40); memw(r16+#44) += #1 }
{ r2 = add(r3,#-1); if (cmp.eq(r2.new,#0)) jump:nt publish_buffer }
{ r21 = sub(#1,r3); r19 = r18; r20 = memw(r16+#16) }
{ r22 = mpyi(r20,r2) }
copy_loop:
{ call append_context; r1:0 = combine(r19,r16); r19 = add(r19,r20) }
{ r21 = add(r21,#1); if (!cmp.eq(r21.new,#0)) jump:t copy_loop }
{ r18 = add(r18,r22) }
publish_buffer:
{ call finish_context; r0 = r17 }
{ r0 = r18; r2 = memw(r16+#4) }
{ callr r2 }
{ r17:16 = memd(r29+#24); r19:18 = memd(r29+#16) }
{ r21:20 = memd(r29+#8); r23:22 = memd(r29+#0) }
{ dealloc_return }
allocation_failed:
{ call finish_context; r0 = r17 }
{ r0 = #0; r17:16 = memd(r29+#24); r19:18 = memd(r29+#16) }
{ r21:20 = memd(r29+#8); r23:22 = memd(r29+#0) }
{ dealloc_return }
.set prepare_context, adsp_late_controller_buffer_expand+0xe79b0
.set finish_context, adsp_late_controller_buffer_expand+0xe7af0
.set return_context, adsp_late_controller_buffer_expand+0x7dc
.set allocate_storage, adsp_late_controller_buffer_expand-0x4384
.set append_context, adsp_late_controller_buffer_expand+0x7c8
