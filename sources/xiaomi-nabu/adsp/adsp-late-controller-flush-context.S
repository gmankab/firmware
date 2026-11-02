// Clear controller context state then tail-publish the restored descriptor.
// Original Hexagon ADSP executable file [0x3faf4,0x3fb10).
.text
.globl adsp_late_controller_flush_context
adsp_late_controller_flush_context:
{ r16 = r1; memd(r29+#-16) = r17:16; allocframe(#8) }
{ call flush_controller_context; memw(r16+#32) = #0; memw(r16+#4) = #0 }
{ r0 = r16; r17:16 = memd(r29+#0); deallocframe }
{ jump publish_restored_context }
.set flush_controller_context, adsp_late_controller_flush_context+0xe823c
.set publish_restored_context, adsp_late_controller_flush_context-0xe68
