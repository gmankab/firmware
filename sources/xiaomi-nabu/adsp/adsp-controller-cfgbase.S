// SPDX-License-Identifier: GPL-3.0-only
// Controller configuration-base read/physical 32-bit access,
// file offsets [0x4ae4,0x4b08).
.text
.globl adsp_controller_cfgbase
adsp_controller_cfgbase:
{ nop } // V66 r1=cfgbase typed replacement
{
  r1 = asl(r1,#5)
  immext(#0)
  r0 = ##0
}
{ nop } // V66 r2 = memw_phys(r0,r1) typed replacement
{
  r2 = asl(r2,#16)
  jumpr lr
  immext(#0xf000ea80)
  memw(gp+#0xf000eab4) = r2.new
}
