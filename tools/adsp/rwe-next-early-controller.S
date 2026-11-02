// SPDX-License-Identifier: GPL-3.0-only
// ADSP Hexagon V66 early thread controller startup and SGP0 status getter.
// File-backed source code [0x10ec0,0x10f64); 12 original zero-fill
// bytes [0x10eb4,0x10ec0) are excluded from this executable fragment.
.text
.globl adsp_rwe_early_controller
adsp_rwe_early_controller:
{ jumpr lr; immext(#0xf0026880); r0=memw(##0xf0026894) }
{ memd(r29+#-16)=r17:16; allocframe(#8) }
{ nop } // V66 R2=SGP0, checked encoder
{ immext(#0xf0025100); r0=##0xf0025128; r2=memw(r2+#0) }
{ r2=memub(r2+#21) }
{ call check_thread_status; immext(#0x400c0000); r1=or(r2,##0x400c0000) }
{ call register_controller; r16=r0 }
{ p0=cmp.eq(r16,#0); if(!p0.new) jump:t ready }
{ nop } // V66 R16=HTID
{ call notify_controller_start; r0=r16 }
{ call notify_controller_done; r0=r16 }
{ immext(#0xf01eafc0); r2=memw(##0xf01eafe8); immext(#0xf0025580); memw(##0xf0025594)=r2.new }
{ nop } // V66 R0=SYSCFG
{ r0=clrbit(r0,#4) }
{ nop } // V66 SYSCFG=R0
{ isync }
{ r2=#0; immext(#0xf0021f40); memw(##0xf0021f70)=r2.new }
{ nop } // V66 k0unlock
{ nop } // V66 tlbunlock
ready:
{ call install_thread_work }
{ call reset_controller_state }
{ jump controller_reset ; r17:16=memd(r29+#0); deallocframe }
// A separate nullary-status getter returns a 1-bit status from SGP0.
status_from_sgp0:
{ nop } // V66 R2=SGP0
{ r2=memw(r2+#0) }
{ r2=memw(r2+#28) }
{ r0=extractu(r2,#1,#19); jumpr lr }
.set check_thread_status, adsp_rwe_early_controller+(0x6568-0x10ec0)
.set register_controller, adsp_rwe_early_controller+(0x29840-0x10ec0)
.set notify_controller_start, adsp_rwe_early_controller+(0x25148-0x10ec0)
.set notify_controller_done, adsp_rwe_early_controller+(0x25170-0x10ec0)
.set install_thread_work, adsp_rwe_early_controller+(0x18160-0x10ec0)
.set reset_controller_state, adsp_rwe_early_controller+(0x65f4-0x10ec0)
.set controller_reset, adsp_rwe_early_controller+(0x65f8-0x10ec0)
