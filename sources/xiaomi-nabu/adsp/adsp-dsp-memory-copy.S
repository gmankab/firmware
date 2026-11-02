// SPDX-License-Identifier: GPL-3.0-only
// Hexagon DMA-like transfer of n 64-bit words between DSP buffers.
// Structured hardware loop and endloop0 packet built by the LLVM assembler.
.text
.globl adsp_memory_copy
adsp_memory_copy:
{
  r3 = memw(r25+##-0xfdf5f60)
  r0 = memw(r25+##-0xfff154c)
}
{
  p0 = cmp.eq(r3,#0)
  if (p0.new) jump:nt copy_done
}
{
  r1 = add(r25,##-0xfd2eb78)
  p0 = cmp.gt(r3,#32)
  if (!p0.new) jump:nt copy_done
}
{
  p0 = cmp.eq(r1,#0)
  if (p0.new) jump:nt copy_done
}
{ r2 = lsr(r3,#3) }
{ loop0(copy_loop,r2) }
copy_loop:
{ r5:4 = memd(r1++#8) }
{
  nop
  memd(r0++#8) = r5:4
}:endloop0
copy_done:
{ jumpr lr }
