// Save two controller state registers and delegate to the external
// Hexagon service handler. Original ADSP MBN file [0x390d8,0x390fc).
.text
.globl adsp_late_save_control
adsp_late_save_control:
{ r0 = add(pc,##1814316) }
{ memw(r0+#0) = r31 }
{ r0 = add(pc,##1814308) }
{ memw(r0+#0) = r29 }
{ r0 = #-1 }
{ immext(#0xf1080); jump service_handoff }
.set service_handoff, adsp_late_save_control+0xf10cc
