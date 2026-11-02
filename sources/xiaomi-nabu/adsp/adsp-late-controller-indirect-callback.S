// ADSP indirect controller callback wrapper, file [0x3fa1c,0x3fa3c).
// Load a callback from the nested controller table, call it, then restore
// the frame and tail-jump to the common controller publisher.
.text
.globl adsp_late_controller_indirect_callback
adsp_late_controller_indirect_callback:
{ r16 = r0; memd(r29+#-16) = r17:16; allocframe(#8) }
{ r1 = memw(r16+#16); r0 = memw(r16+#12) }
{ r2 = memw(r1+#8) }
{ callr r2 }
{ r0 = r16; r17:16 = memd(r29+#0); deallocframe }
{ jump platform_publish_controller }
.set platform_publish_controller, adsp_late_controller_indirect_callback-0x4a7c
