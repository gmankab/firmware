// SPDX-License-Identifier: GPL-3.0-only
// Source-authored v65/v66-common packetized Hexagon cache-line copier.
// Uses loop1+loop0 nested hardware loops and endloop0/endloop1 parsing.
// Three trailing NOPs model the exact instruction-aligned padding.
.text
.globl adsp_copy_cachelines
adsp_copy_cachelines:
{
  p0 = cmp.eq(r2,#0)
  if (p0.new) jump:nt done_copy
  immext(#0xf02d1480)
  r0 = memw(gp+#0xf02d1488)
}
{
  p0 = cmp.gt(r2,#32)
  if (!p0.new) jump:nt done_copy
}
{
  p0 = cmp.eq(r0,#0)
  if (p0.new) jump:nt done_copy
  r3 = ##0x80
}
{
  r6 = lsr(r2,#12)
  r4 = add(r0,#8)
  r5 = add(r1,#8)
}
{
  loop1(outer_loop,r6)
  r7 = ##0x808020
}
outer_loop:
{ l2fetch(r1,r7) }
{ loop0(inner_loop,r3) }
inner_loop:
{ dczeroa(r1) }
{
  r7:6 = memd(r0++#16)
  r9:8 = memd(r4++#16)
}
{
  memd(r1++#16) = r7:6
  memd(r5++#16) = r9:8
}
{
  r7:6 = memd(r0++#16)
  r9:8 = memd(r4++#16)
}
{
  nop
  memd(r1++#16) = r7:6
  memd(r5++#16) = r9:8
}:endloop0:endloop1
done_copy:
{ jumpr lr }
{ nop }
{ nop }
{ nop }

