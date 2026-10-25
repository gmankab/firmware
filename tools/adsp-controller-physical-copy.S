// SPDX-License-Identifier: GPL-3.0-only
// Copy physical-addressed controller words into the ADSP view.
// ADSP RX file offsets [0x4a08, 0x4a54).
.text
.globl adsp_controller_physical_copy
adsp_controller_physical_copy:
{
  immext(#0xf061e740)
  r1 = memw(gp+#0xf061e744)
  immext(#0xf020a080)
  r3 = memw(gp+#0xf020a0a0)
}
{ p0 = cmp.eq(r1,#0); if (p0.new) jump:nt physical_done }
{ p0 = cmp.eq(r3,#0); if (p0.new) jump:nt physical_done }
{
  p0 = cmp.gt(r3,#0x20)
  if (!p0.new) jump:nt physical_done
  immext(#0xf000ea80)
  r0 = memw(gp+#0xf000eab4)
}
{
  r2 = lsr(r3,#2)
  p0 = cmp.eq(r0,#0)
  if (p0.new) jump:nt physical_done
}
{
  loop0(physical_loop,r2)
  r2 = #11
}
physical_loop:
{ r4 = lsr(r0,r2) }
{ nop } // V66 memw_phys(r0,r4) typed replacement
{
  r0 = add(r0,#4)
  memw(r1++#4) = r5
}:endloop0
physical_done:
{ jumpr lr }
