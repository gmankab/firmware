// SPDX-License-Identifier: GPL-3.0-only
// Controller debug-permission and physical page-register checks.
// ADSP file offsets [0x4c68,0x4cc0).
.text
.globl adsp_controller_debug_check
adsp_controller_debug_check:
{ nop } // V66 r1 = ISDBEN
{
  p0 = tstbit(r1,#2)
  if (!p0.new) jump:t debug_break
}
{
  nop // V66 r1 = CFGBASE
  r25 = #0
}
{ r1 = insert(r25,#12,#20) }
{
  r1 = asl(r1,#5)
  r0 = #8
}
{ nop } // V66 r1 = memw_phys(r0,r1)
{ r1 = insert(r25,#12,#20) }
{
  r1 = asl(r1,#5)
  r0 = #24
}
{ r1 = add(r1,#-256) }
{ nop } // V66 r1 = memw_phys(r0,r1)
{ r24 = memw(r23+##0xf01eafec) }
{ r0 = and(r1,r24) }
{ p0 = cmp.eq(r0,#0); if (p0.new) jump:t debug_pass }
{ r1 = #1 }
{ nop } // V66 ISDBEN = r1
{ isync }
{ nop } // V66 brkpt
.set debug_pass, adsp_controller_debug_check + 0x58
.set debug_break, adsp_controller_debug_check + 0x48
