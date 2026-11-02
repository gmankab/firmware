// ADSP Hexagon V66 [file 0x3d048,0x3d060): compare the type-19 descriptor
// against its matching constant word in the platform metadata table.
.text
.globl adsp_late_packet_type_check
adsp_late_packet_type_check:
{ p0 = cmp.eq(r2,#19); if (!p0.new) jump:t next_function }
{ immext(#0xf018b000); r3 = ##4028152120; r2 = memw(r1+#32) }
{ r0 = cmp.eq(r2,r3); jumpr lr }
.set next_function, adsp_late_packet_type_check+0x14
