// ADSP runtime copy from an indexed callback table into caller descriptor,
// original file offsets [0x39490,0x394a0).
.text
.globl adsp_late_copy_registry_slot
adsp_late_copy_registry_slot:
{ r2 = memw(r3<<#2+##4029459464); memw(r0+#0) = r2.new }
{ jumpr lr }
