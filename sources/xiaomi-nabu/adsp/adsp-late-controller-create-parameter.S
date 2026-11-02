// ADSP controller parameter allocation [0x3fa3c,0x3fa80).
// Allocate optional parameter state, update global count and initialize
// controller status, returning null on allocation failure.
.text
.globl adsp_late_controller_create_parameter
adsp_late_controller_create_parameter:
{ call allocate_parameter_state; memd(r29+#-16) = r17:16; allocframe(#8) }
{ r16 = r0; if (cmp.eq(r16.new,#0)) jump:nt allocation_failed }
{ immext(#4029461760); r2 = ##-265505532; immext(#4029461760); r1:0 = combine(r16,##-265505536) }
{ memw(r2+#0) += #1 }
{ memw(r16+#12) = #-1; memw(r16+#32) = #1 }
{ call initialize_parameter; memw(r16+#4) = #0 }
{ jump return_parameter }
allocation_failed:
{ r16 = #0 }
return_parameter:
{ r0 = r16; r17:16 = memd(r29+#0); dealloc_return }
.set allocate_parameter_state, adsp_late_controller_create_parameter-0xd9c
.set initialize_parameter, adsp_late_controller_create_parameter+0xe6bcc
