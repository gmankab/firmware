// SPDX-License-Identifier: GPL-3.0-only
// ADSP Hexagon V66 controller K0 locking, config flag and SSR/status getters.
// Four distinct routines file [0x10fe8,0x11088); 8 bytes of following linker
// zero fill must never be classified as source-authored executable code.
.text
.globl adsp_rwe_locked_controller
adsp_rwe_locked_controller:
{ nop } // typed R2 = SGP0
{ r0 = memw(r2+#0) }
{ nop } // checked V66 k0lock
{ immext(#0xf00250c0); r3:2=combine(#0,##0xf00250f0) }
retry_locked:
{ r4 = memw_locked(r2) }
{ p0 = cmp.eq(r4,r3) }
{ if(!p0) jump:nt previous_locked }
{ memw_locked(r2,p0)=r0 }
{ if(!p0) jump:nt retry_locked }
previous_locked:
{ p0=cmp.eq(r4,#0); if(p0.new) jump:nt not_ready; if(p0.new) r1=memub(r0+#21) }
{ nop } // V66 k0unlock
{ immext(#0xc0); r1:0=combine(##0xc0,#12); r2=#0 }
{ jump platform_notify }
not_ready:
{ r2=#-0x1112; memw(r0+#12)=r2.new }
{ jump platform_finish }
// A separate nullary helper updates a global busy-status flag under K0.
set_busy_status:
{ nop } // V66 k0lock
{ r2=!cmp.eq(r0,#0); immext(#0xf0024800); memw(GP+##0xf0024810)=r2.new }
{ nop } // V66 k0unlock
{ jumpr lr }
// SSR getter extracts a 6-bit response code from the current thread.
read_ssr_status:
{ nop } // typed R0 = SSR
{ r2=extractu(r0,#6,#8) }
{ r0=r2; jumpr lr }
// SGP0 controller callback wrapper and tail dispatcher.
forward_controller_context:
{ memd(r29+#-16)=r17:16; allocframe(#8) }
{ nop } // V66 R2=SGP0
{ r16=memw(r2+#0) }
{ r2=r16 }
{ call platform_prepare; r1=memub(r16+#20) }
{ r0=r16; r1=memub(r16+#21); r17:16=memd(sp+#0) }
{ jump platform_finish; deallocframe }
.set platform_notify,adsp_rwe_locked_controller+(0x8be21098-0x8be0dfe8)
.set platform_finish,adsp_rwe_locked_controller+(0x8be238b4-0x8be0dfe8)
.set platform_prepare,adsp_rwe_locked_controller+(0x8be23aa8-0x8be0dfe8)
