// SPDX-License-Identifier: GPL-3.0-only
// Hexagon v66 controller-state completion; file [0x60d4,0x6148),
// physical [0x8be030d4,0x8be03148). Explicit source packet layout and
// v66 privileged operation sentinels preserve the original control flow.
.text
.globl adsp_agent20_rwe_finalize
adsp_agent20_rwe_finalize:
{
 r0 = #2
 immext(#0xf000d300)
 memw(r1=##0xf000d320) = r0.new
}
{ dccleana(r1) }
{ call controller_poll }
{ immext(#0x12440); call controller_runtime }
{ call controller_state }
{ nop } // V66 r0 = SYSCFG
{ r0 = clrbit(r0,#4); r1 = #0 }
{ nop } // V66 SYSCFG = r0
{ nop } // V66 r0 = SSR
{ r0 = insert(r1,#3,#16) }
{ nop } // V66 SSR = r0
{
 immext(#0)
 r0 = #36
 immext(#0xf0000040)
 memw(r1=##0xf0000044) = r0.new
}
{ dccleana(r1) }
{ immext(#0x1f0c0); call idle_dispatch }
{ immext(#0xc540); call external_service }
{ r2 = #-1; r3 = #-1; r0 = #24; r1 = #-1 }
{ trap0(#0) }
self_trap:
{ jump self_trap }
.set controller_poll,adsp_agent20_rwe_finalize + (0x8be037a8-0x8be030d4)
.set controller_runtime,adsp_agent20_rwe_finalize + (0x8be15544-0x8be030d4)
.set controller_state,adsp_agent20_rwe_finalize + (0x8be034b4-0x8be030d4)
.set idle_dispatch,adsp_agent20_rwe_finalize + (0x8be22220-0x8be030d4)
.set external_service,adsp_agent20_rwe_finalize + (0x8be0f6a8-0x8be030d4)
