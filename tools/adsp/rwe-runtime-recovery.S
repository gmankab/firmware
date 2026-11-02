// SPDX-License-Identifier: GPL-3.0-only
// Hexagon ADSP controller recovery/initialization, file [0x63d8,0x6440).
// Source symbolic call addresses are physically mapped, not file offsets.
.text
.globl adsp_agent20_rwe_runtime_recovery
adsp_agent20_rwe_runtime_recovery:
{ call controller_boot }
{ immext(#0xffffe600); call memory_warmup }
{ immext(#0x12140); call codec_manager }
{ r0=#3; immext(#0xf000d300); memw(r1=##0xf000d320)=r0.new }
{ dccleana(r1) }
{ call service_init_a }
{ call service_init_b }
{ call controller_phase1 }
{ immext(#0); r0=#30; immext(#0xf0000040); memw(r1=##0xf0000044)=r0.new }
{ dccleana(r1) }
{ immext(#0xffffe600); call memory_ready }
{ immext(#0x3c0); call region_check }
{ immext(#0x128c0); call low_power_init }
{ immext(#0x12100); call codec_manager }
{ call signal_dispatch }
.set controller_boot,adsp_agent20_rwe_runtime_recovery+(0x8be03c30-0x8be033d8)
.set memory_warmup,adsp_agent20_rwe_runtime_recovery+(0x8be01a08-0x8be033d8)
.set memory_ready,adsp_agent20_rwe_runtime_recovery+(0x8be01a54-0x8be033d8)
.set codec_manager,adsp_agent20_rwe_runtime_recovery+(0x8be15544-0x8be033d8)
.set service_init_a,adsp_agent20_rwe_runtime_recovery+(0x8be035f4-0x8be033d8)
.set service_init_b,adsp_agent20_rwe_runtime_recovery+(0x8be035f8-0x8be033d8)
.set controller_phase1,adsp_agent20_rwe_runtime_recovery+(0x8be037a8-0x8be033d8)
.set region_check,adsp_agent20_rwe_runtime_recovery+(0x8be037e8-0x8be033d8)
.set low_power_init,adsp_agent20_rwe_runtime_recovery+(0x8be15d00-0x8be033d8)
.set signal_dispatch,adsp_agent20_rwe_runtime_recovery+(0x8be034b4-0x8be033d8)
