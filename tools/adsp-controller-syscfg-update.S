// SPDX-License-Identifier: GPL-3.0-only
// Hexagon SYSCFG update and external transition [0x5024,0x505c).
// LLVM6 builds all extended addresses, predicate and far-call packet bits;
// two privileged SYSCFG register ops use checked typed encoders.
.text
.globl adsp_update_syscfg
adsp_update_syscfg:
{
 immext(#0)
 r0 = ##0
 if (cmp.eq(r0.new,#0)) jump:nt continue_controller
}
{ nop } // V66 r0 = SYSCFG
{
 immext(#0xf01eafc0)
 r1 = memw(r23+##-0xfe15004)
}
{ r0 = insert(r1,#3,#16) }
{ nop } // V66 SYSCFG = r0
{ isync }
{
 immext(#0x80)
 r0 = ##0x80
}
{
 immext(#0xfffdf80)
 call far_entry
}
{ jump continue_controller }
.set continue_controller, adsp_update_syscfg + 0x1b0
.set far_entry, adsp_update_syscfg + (0x10003000-0x5024)
