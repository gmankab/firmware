// SPDX-License-Identifier: GPL-3.0-only
// Restore single-thread SSR/SYSCFG then restart the controller, [0x64d8,0x6568).
// LLVM6 placeholders are constrained to V66 system register operations.
.text
.globl adsp_agent20_rwe_reset_controller
adsp_agent20_rwe_reset_controller:
{ r1=#0 }
{ nop } // V66 R0 = SSR
{ r0=insert(r1,#3,#16) }
{ nop } // V66 SSR = R0
{ call controller_boot }
{ immext(#0xffffe500); call memory_warmup }
{ immext(#0x12040); call codec_manager }
{ r0=#2; immext(#0xf000d300); memw(r1=##0xf000d320)=r0.new }
{ dccleana(r1) }
{ call controller_phase1 }
{ immext(#0xffffe540); call memory_ready }
{ immext(#0x127c0); call low_power_init }
{ immext(#0x12000); call codec_manager }
{ call signal_dispatch }
{ nop } // V66 R0 = SYSCFG
{ r0=clrbit(r0,#4) }
{ nop } // V66 SYSCFG = R0
{ nop; r1=#0 } // V66 R0 = SSR
{ r0=insert(r1,#3,#16) }
{ nop } // V66 SSR = R0
{ immext(#0); r0=#36; immext(#0xf0000040); memw(r1=##0xf0000044)=r0.new }
{ dccleana(r1) }
{ immext(#0x1ecc0); call report_status }
halt:
{ jump halt }
.set controller_boot,adsp_agent20_rwe_reset_controller+(0x8be03c30-0x8be034d8)
.set memory_warmup,adsp_agent20_rwe_reset_controller+(0x8be01a08-0x8be034d8)
.set memory_ready,adsp_agent20_rwe_reset_controller+(0x8be01a54-0x8be034d8)
.set codec_manager,adsp_agent20_rwe_reset_controller+(0x8be15544-0x8be034d8)
.set controller_phase1,adsp_agent20_rwe_reset_controller+(0x8be037a8-0x8be034d8)
.set low_power_init,adsp_agent20_rwe_reset_controller+(0x8be15d00-0x8be034d8)
.set signal_dispatch,adsp_agent20_rwe_reset_controller+(0x8be034b4-0x8be034d8)
.set report_status,adsp_agent20_rwe_reset_controller+(0x8be22220-0x8be034d8)
