// SPDX-License-Identifier: GPL-3.0-only
// Hexagon TLB reconciliation/locking and restoration.
// LLVM6 assembles all ordinary instructions, extender packets and the loop.
// Seven NOP sentinels mark privileged TLB opcodes handled by typed V66
// operations in the emitter; sentinels are never part of published output.
.text
.globl adsp_tlb_reconcile
adsp_tlb_reconcile:
{
  immext(#0xf000ea80)
  r0 = memw(gp+#0xf000eab4)
  immext(#0xf020a080)
  r1 = memw(gp+#0xf020a0a0)
}
{ p0 = cmp.eq(r1,#0); if (p0.new) jump:nt recon_done }
{
  r0 = lsr(r0,#12)
  r1 = lsr(r1,#12)
  r10 = ##0x3fffff
}
{
  loop0(recon_loop,#127)
  r2 = add(r0,r1)
  r3 = #0
}
recon_loop:
{ nop }
{ p0 = cmp.eq(r5,#0); if(p0.new) jump:nt recon_loop_end }
{ r6 = ct0(r4) }
{ r4 = clrbit(r4,r6) }
{ r4 = asr(r4,#1) }
{ r4 = and(r4,r10) }
{ p0 = cmp.eq(r4,r0); if(p0.new)jump:nt recon_update }
{ p0 = cmp.gtu(r4,r2); if(p0.new)jump:t recon_loop_end }
{ p1 = cmp.gtu(r0,r4); if(p1.new)jump:t recon_loop_end }
recon_update:
{ r7:6 = combine(#0,#0) }
{ nop }
{ nop }
{ nop }
recon_loop_end:
{ r3 = add(r3,#1)
  nop }:endloop0
{
  r4 = ##-0xfff15d8
  immext(#0xf002a600)
  r5 = memw(gp+#0xf002a610)
}
{ r10 = #5
  r8 = #0 }
source_loop:
{ r1:0 = memd(r4++#8) }
{ p0 = cmp.eq(r0,#0); if(p0.new)jump:t recon_done }
{ nop }
{ nop }
{ nop }
{ r3 = add(r3,#-1)
  r8 = add(r8,#1) }
{ p1 = cmp.eq(r3,r5); if(p1.new) jump:nt recon_done }
{
  p0 = cmp.eq(r8,r10)
  if(!p0.new) jump:nt source_loop
}
recon_done:
{ jumpr lr }
