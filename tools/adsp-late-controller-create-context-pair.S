// Registered controller context allocation [0x3fabc,0x3faf4).
// Obtain a shared platform object, attach a typed pointer to its state,
// initialize flags and invoke the original metadata-registration callback.
.text
.globl adsp_late_controller_create_context_pair
adsp_late_controller_create_context_pair:
{ r16 = r1; r17 = r0; memd(r29+#-16) = r17:16; allocframe(#16) }
{ call allocate_controller_context; memd(r29+#0) = r19:18 }
{ r18 = r0 }
{ r1:0 = combine(r18,r17); memw(r18+#48) = r16; memw(r18+#16) = #0 }
{ call register_controller_context; memw(r18+#12) = #-1; memw(r18+#4) = #0 }
{ r0 = r18; r17:16 = memd(r29+#8); r19:18 = memd(r29+#0) }
{ dealloc_return }
.set allocate_controller_context, adsp_late_controller_create_context_pair-0xe3c
.set register_controller_context, adsp_late_controller_create_context_pair+0xe6b4c
