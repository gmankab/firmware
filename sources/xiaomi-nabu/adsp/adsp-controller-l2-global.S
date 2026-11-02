// SPDX-License-Identifier: GPL-3.0-only
// Hexagon L2 global-cache unlock/clean, ADSP [0x5108,0x5138).
// LLVM6 assembles predicate polling and extended controller memory load;
// v66-only SYSCFG and L2 global privileged operations are typed encoders.
.text
.globl adsp_controller_l2_global
adsp_controller_l2_global:
{ nop } // V66 r0 = SYSCFG
{
 immext(#0xf01eafc0)
 r1 = memw(r23+##-0xfe15004)
}
{ r0 = insert(r1,#3,#16) }
{ nop } // V66 SYSCFG = r0
{ isync }
{ nop } // V66 l2gunlock
{ nop } // V66 l2gclean
poll:
{ nop } // V66 r0 = SYSCFG
{ p0 = tstbit(r0,#31); if (p0.new) jump:t poll }
{ jumpr lr }
