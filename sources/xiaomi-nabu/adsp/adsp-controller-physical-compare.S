// SPDX-License-Identifier: GPL-3.0-only
// Hexagon ADSP controller physical memory reconciliation, [0x4a54,0x4ae4).
// Guard table bounds, compare physical words and record up to four diffs.
.text
.globl adsp_controller_physical_compare
adsp_controller_physical_compare:
{
  immext(#0x0)
  r1 = #0
  immext(#0xf0123600)
  r4 = ##0xf0123608
}
{
  immext(#0x8000)
  r0 = ##0x8000
  r6 = sub(r4,r1)
}
{
  p0 = cmp.gt(r0,r6)
  if (!p0.new) jump:nt compare_done
  immext(#0)
  r0 = ##0
}
{
  r2 = lsr(r6,#2)
  p0 = cmp.eq(r0,#0)
  if (p0.new) jump:nt compare_done
}
{
  immext(#0xf000ea00)
  r8 = memw(gp+#0xf000ea20)
  immext(#0xf000ea00)
  r9 = memw(gp+#0xf000ea1c)
}
{
  r5 = sub(r1,r0)
  r9 = sub(r9,r8)
  immext(#0)
  r3 = ##0
}
{ r3 = add(r3,r9) }
{
  immext(#0xf0003ec0)
  r6 = ##0xf0003ef0
  r3 = add(r3,r5)
}
{
  loop0(compare_loop,r2)
  r2 = #11
  r7 = #0
}
compare_loop:
{
  r4 = lsr(r3,r2)
  r0 = memw(r1+#0)
}
{ nop } // V66 memw_phys(r3,r4) typed replacement
{ p0 = cmp.eq(r5,r0); if (p0.new) jump:nt compare_loop_end }
{ p0 = cmp.gt(r7,#3); if (p0.new) jump:nt compare_done }
{
  r7 = add(r7,#1)
  memw(r6++#4) = r3
}
{ memw(r6++#4) = r5 }
compare_loop_end:
{
  nop
  r1 = add(r1,#4)
  r3 = add(r3,#4)
}:endloop0
compare_done:
{ jumpr lr }
