// Independently authored Hexagon queue drain from the start of the
// f0036000 ADSP RX region, OEM file offsets [0x39000,0x39054).
.text
.globl adsp_late_queue_drain
adsp_late_queue_drain:
{ allocframe(#8) }
{ memw(r29+#0) = r27 }
{ r0.h = #0xf01e }
{ r0.l = #0xf120 }
{ r1 = memw(r0+#0) }
{ p0 = cmp.eq(r1,#0); if (p0.new) jump:nt drain_callbacks }
{ r3 = #0; r2 = #0; r1 = #0 }
{ r28.h = #0xf014 }
{ r28.l = #0x3f9c }
{ callr r28 }
drain_callbacks:
{ r27.h = #0xf01f }
{ r27.l = #8 }
next_slot:
{ r27 = add(r27,#-4) }
{ r0 = memw(r27+#0) }
{ p0 = cmp.eq(r0,#0); if (p0.new) jump:nt drained }
{ callr r0 }
{ jump next_slot }
drained:
{ r27 = memw(r29+#0); deallocframe }
{ jumpr lr }
