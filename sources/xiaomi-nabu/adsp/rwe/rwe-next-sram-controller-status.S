// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 ADSP SRAM/controller state from file [0x116d8,0x11754).
// Physical PC [0x8be0e6d8,0x8be0e754). Privileged reads are source
// placeholders patched with checked SystemRegister encodings, not OEM words.
.text
.globl adsp_sram_controller_state
adsp_sram_controller_state:
{ immext(#0); r17=##0; immext(#0xf02d1480); r16=##0xf02d1488 }
{ immext(#0xf061e740); r18=##0xf061e744 }
{ nop } // R2=SYSCFG
{ call setup_controller; immext(#0xf0010dc0); memw(gp+##0xf0010dc8)=r2 }
{ call global_controller_startup }
{ call status_source }
{ call initialization_state }
{ immext(#0xf020a280); r2=memw(gp+##0xf020a29c) }
{ nop } // R10=SGP0
{ memw(r10+#0)=r2 }
{ call initialize_channel; r0=#24 }
{ call issue_status; immext(#0xf000e9c0); r0=##0xf000e9c0 }
{ call initialize_channel; r0=#25 }
{ call issue_status; immext(#0xf000e9c0); r0=##0xf000e9d5 }
{ call publish_event }
{ immext(#0xf05d54c0); r2=memw(gp+##0xf05d54fc); if(cmp.gt(r2.new,#1)) jump:t status_fallback }
.set setup_controller,adsp_sram_controller_state+(0x8be1a600-0x8be0e6d8)
.set global_controller_startup,adsp_sram_controller_state+(0x8be0e450-0x8be0e6d8)
.set status_source,adsp_sram_controller_state+(0x8be1e9f0-0x8be0e6d8)
.set initialization_state,adsp_sram_controller_state+(0x8be0dc40-0x8be0e6d8)
.set initialize_channel,adsp_sram_controller_state+(0x8be23268-0x8be0e6d8)
.set issue_status,adsp_sram_controller_state+(0x8be11438-0x8be0e6d8)
.set publish_event,adsp_sram_controller_state+(0x8be15e40-0x8be0e6d8)
.set status_fallback,adsp_sram_controller_state+(0x8be0e77c-0x8be0e6d8)
