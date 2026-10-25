// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 controller service sequence in RWE PT_LOAD, file [0x62c8,0x6338).
// All external call PC values are physical addresses, not MBN file offsets.
.text
.globl adsp_agent20_rwe_runtime_calls
adsp_agent20_rwe_runtime_calls:
{ call service_init_a }
{ call service_init_b }
{ call controller_boot }
{ immext(#0xffffe700); call memory_warmup }
{ immext(#0x12240); call codec_manager }
{ r0 = #2; immext(#0xf000d300); memw(r1=##0xf000d320) = r0.new }
{ dccleana(r1) }
{ call controller_phase0 }
{ call controller_phase1 }
{ immext(#0); r0 = #30; immext(#0xf0000040); memw(r1=##0xf0000044) = r0.new }
{ dccleana(r1) }
{ immext(#0xffffe740); call memory_ready }
{ immext(#0x4c0); call region_check }
{ immext(#0x129c0); call low_power_init }
{ immext(#0x12200); call codec_manager }
{ call signal_dispatch }
{ call registration_commit }
.set service_init_a,adsp_agent20_rwe_runtime_calls+(0x8be035f4-0x8be032c8)
.set service_init_b,adsp_agent20_rwe_runtime_calls+(0x8be035f8-0x8be032c8)
.set controller_boot,adsp_agent20_rwe_runtime_calls+(0x8be03c30-0x8be032c8)
.set memory_warmup,adsp_agent20_rwe_runtime_calls+(0x8be01a08-0x8be032c8)
.set memory_ready,adsp_agent20_rwe_runtime_calls+(0x8be01a54-0x8be032c8)
.set codec_manager,adsp_agent20_rwe_runtime_calls+(0x8be15544-0x8be032c8)
.set controller_phase0,adsp_agent20_rwe_runtime_calls+(0x8be0376c-0x8be032c8)
.set controller_phase1,adsp_agent20_rwe_runtime_calls+(0x8be037a8-0x8be032c8)
.set region_check,adsp_agent20_rwe_runtime_calls+(0x8be037e8-0x8be032c8)
.set low_power_init,adsp_agent20_rwe_runtime_calls+(0x8be15d00-0x8be032c8)
.set signal_dispatch,adsp_agent20_rwe_runtime_calls+(0x8be034b4-0x8be032c8)
.set registration_commit,adsp_agent20_rwe_runtime_calls+(0x8be237b8-0x8be032c8)
