// ADSP controller parameter unregistration [0x3fa80,0x3fabc).
// Clears flags, calls the shared state release handler and decrements
// the global registration count when the descriptor was active.
.text
.globl adsp_late_controller_release_parameter
adsp_late_controller_release_parameter:
{ r16 = r0; memd(r29+#-16) = r17:16; allocframe(#8) }
{ r2 = memw(r16+#32); if (!cmp.eq(r2.new,#1)) jump:t restore }
{ immext(#4029461760); r1:0 = combine(r16,##-265505536); memw(r16+#32) = #0; memw(r16+#4) = #0 }
{ call flush_parameter; memw(r16+#44) = #0 }
{ call release_parameter; r0 = r16 }
{ immext(#4029461760); r2 = ##-265505532 }
{ memw(r2+#0) -= #1 }
restore:
{ r17:16 = memd(r29+#0); dealloc_return }
.set flush_parameter, adsp_late_controller_release_parameter+0xe82b0
.set release_parameter, adsp_late_controller_release_parameter-0xdd4
