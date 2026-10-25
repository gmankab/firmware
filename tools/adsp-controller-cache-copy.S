// SPDX-License-Identifier: GPL-3.0-only
// Reconstructed controller copy routine at file [0x49ac, 0x4a04).
// Fetches controller source/destination/size from GP-relative registers,
// skips unavailable work and copies 32 bytes per Hexagon hardware-loop turn.
.text
.globl adsp_controller_cache_copy
adsp_controller_cache_copy:
{
  immext(#0xf02d1480)
  r1 = memw(gp+#0xf02d1488)
  immext(#0xf02d1480)
  r2 = memw(gp+#0xf02d148c)
}
{ p0 = cmp.eq(r1,#0); if (p0.new) jump:nt cache_done }
{ p0 = cmp.eq(r2,#0); if (p0.new) jump:nt cache_done }
{
  p0 = cmp.gt(r2,#0x20)
  if (!p0.new) jump:nt cache_done
}
{
  r3 = lsr(r2,#5)
  p0 = cmp.eq(r0,#0)
  if (p0.new) jump:nt cache_done
}
{
  loop0(cache_loop,r3)
  r4 = add(r0,#8)
  r5 = add(r1,#8)
}
cache_loop:
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
  memd(r1++#16) = r7:6
  memd(r5++#16) = r9:8
}:endloop0
cache_done:
{ jumpr lr }
