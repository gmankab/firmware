// SPDX-License-Identifier: GPL-3.0-only
// ADSP post-TLB bootstrap and transfer into controller init.
// File offset 0x4980, RX-relative 0x1980.
.text
.globl adsp_controller_bootstrap
adsp_controller_bootstrap:
{ r17 = lr }
{ sp = ##0xf0023448 }
{ immext(#0xffffff40)
  call tlb_reconcile }
{ immext(#0xf000ea80)
  r0 = memw(gp+#0xf000eaa8) }
{ immext(#0x0)
  call controller_init }
{ lr = r17 }
{ jumpr lr }
.set tlb_reconcile, adsp_controller_bootstrap - 0xb0
controller_init:
