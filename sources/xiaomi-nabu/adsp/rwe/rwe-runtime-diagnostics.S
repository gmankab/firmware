// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 runtime register/status capture [0x638c,0x63d8).
// Four named V66 privileged register transfers use checked typed encoders;
// other words are assembled by LLVM6 from original independent mnemonics.
.text
.globl adsp_agent20_rwe_runtime_diagnostics
adsp_agent20_rwe_runtime_diagnostics:
{ immext(#0x1ee80); call report_status }
{ nop } // V66 R1 = SSR
{ nop; immext(#0xf0003c80); memw(r2=##0xf0003c90) = r1 } // V66 R0 = ELR
{ nop; immext(#0xf0003c80); memw(gp+##0xf0003c94) = r0 } // V66 R1 = DIAG
{ nop; immext(#0xf00228c0); memw(r3=##0xf00228f8) = r1 } // V66 R0 = SYSCFG
{ immext(#0xf000d300); r1=memw(gp+##0xf000d320); immext(#0xf0003c80); memw(gp+##0xf0003c98)=r0 }
{ dccleana(r2) }
{ dccleana(r3) }
{ p0=cmp.eq(r1,#1); if (p0.new) jump:nt ready }
.set report_status,adsp_agent20_rwe_runtime_diagnostics+(0x8be22220-0x8be0338c)
.set ready,adsp_agent20_rwe_runtime_diagnostics+(0x8be03440-0x8be0338c)
