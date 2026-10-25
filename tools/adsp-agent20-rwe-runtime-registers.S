// SPDX-License-Identifier: GPL-3.0-only
// ADSP Hexagon thread register capture, [file 0x6338,0x638c).
// LLVM6 placeholders are checked and replaced by operand-encoded V66 system
// register transfers (HTID, MODECTL, SYSCFG and SSR).
.text
.globl adsp_agent20_rwe_runtime_registers
adsp_agent20_rwe_runtime_registers:
{ nop; immext(#0xf0003c80); r0 = ##-0xfffc380 } // V66 r1=HTID
{ r0 += mpyi(r1,#4) }
{ r1 = add(r1,#0); memw(r0+#0) = r1.new } // V66 r1=MODECTL
{ dccleana(r0) }
{ dcinva(r0) }
{ r1 = memw(r0+#0) }
{ nop } // V66 r0=SYSCFG
{ r0 = clrbit(r0,#4) }
{ nop } // V66 SYSCFG=r0
{ nop; r1=#0 } // V66 r0=SSR
{ r0 = insert(r1,#3,#16) }
{ nop } // V66 SSR=r0
{ immext(#0); r0 = #36; immext(#0xf0000040); memw(r1=##0xf0000044) = r0.new }
{ dccleana(r1) }
