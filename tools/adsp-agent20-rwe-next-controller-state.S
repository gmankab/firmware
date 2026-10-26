// SPDX-License-Identifier: GPL-3.0-only
// ADSP V66 controller status validation, runtime callback and recovery
// [file 0x10f64,0x10fe8); physical [0x8be0df64,0x8be0dfe8).
.text
.globl adsp_rwe_controller_state
adsp_rwe_controller_state:
{ call controller_status; r16=r0; memd(r29+#-16)=r17:16; allocframe(#8) }
{ p0=cmp.eq(r0,#0); if(p0.new) jump:nt bad_state }
{ call query_runtime_controller }
{ immext(#0xf0025580); r2=memw(GP+##0xf00255b0); if(cmp.eq(r2.new,#0)) jump:nt after_monitor }
{ immext(#0xf0026f00); r2=memw(GP+##0xf0026f20); if(!cmp.eq(r2.new,#0)) jump:nt after_monitor }
{ immext(#0xf0025540); r2=memw(GP+##0xf0025560); if(cmp.eq(r2.new,#0)) jump:nt after_monitor }
{ call disable_watchdog }
{ immext(#0xf0025540); r2=memw(GP+##0xf0025560) }
{ memw(r2+#4)=clrbit(#21) }
after_monitor:
{ p0=cmp.eq(r16,#0); if(p0.new) jump:nt panic_state; immext(#0xf0025100); if(!p0.new) r2=memub(##0xf002512b) }
{ p0=tstbit(r2,#5) }
{ immext(#0xffffff00); if(!p0) call controller_recovery }
{ jump start_controller; r17:16=memd(r29+#0); deallocframe }
bad_state:
{ r1=#194; r2=#0; r0=#36; r17:16=memd(r29+#0) }
{ jump notify_error; deallocframe }
panic_state:
{ jump controller_recovery; r17:16=memd(r29+#0); deallocframe }
.set controller_status,adsp_rwe_controller_state+(0x8be0df50-0x8be0df64)
.set query_runtime_controller,adsp_rwe_controller_state+(0x8be22134-0x8be0df64)
.set disable_watchdog,adsp_rwe_controller_state+(0x8be17588-0x8be0df64)
.set controller_recovery,adsp_rwe_controller_state+(0x8be0decc-0x8be0df64)
.set start_controller,adsp_rwe_controller_state+(0x8be034d8-0x8be0df64)
.set notify_error,adsp_rwe_controller_state+(0x8be21098-0x8be0df64)
