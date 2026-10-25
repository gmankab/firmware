// SPDX-License-Identifier: GPL-3.0-only
// ADSP Hexagon TLB handoff and two map helpers.
// The three `nop` instructions are constrained assembler sentinels:
// tools/emit-adsp-entry.py replaces those output words with the checked,
// fixed-operand V66 tlbw(R3:2,R0) encoding before exposing any artifact.
// No embedded OEM instruction words or binary opcode tables are used.

.text
.globl adsp_tlb_copy
adsp_tlb_copy:
{
  r0 = #0
  r1 = add(r25,##-0xfff30e0)
}
tlb_loop:
{ r3:2 = memd(r1++#8) }
{ r4 = extractu(r3,#7,#20) }
{
  r4 = sub(#125,r4)
  if (!cmp.eq(r4.new,#0)) jump:t tlb_skip_insert
}
{ r3 = insert(r4,#7,#20) }
tlb_skip_insert:
{ nop }
{
  p0 = cmp.eq(r2,#0)
  if (!p0.new) jump:t tlb_loop
  r0 = add(r0,#1)
}
{ isync }
{ jumpr lr }

.globl adsp_tlb_map_7f
adsp_tlb_map_7f:
{
  p0 = cmp.eq(r25,#0)
  if (p0.new) jump:nt return_7f
}
{
  r2 = lsr(r24,#11)
  r3 = lsr(r24,#12)
  r4 = #0x7c0
  r5 = #0xc00
}
{
  r2 = insert(r4,#11,#21)
  r3 = insert(r5,#12,#20)
}
{
  r2 = setbit(r2,#1)
  r0 = #0x7f
}
{ nop }
{ isync }
return_7f:
{ jumpr lr }

.globl adsp_tlb_map_7e
adsp_tlb_map_7e:
{
  r2 = lsr(r24,#11)
  r3 = lsr(r26,#12)
  r4 = #0x7c0
  r5 = #0xc00
}
{
  r2 = insert(r4,#11,#21)
  r3 = insert(r5,#12,#20)
}
{
  r2 = setbit(r2,#1)
  r0 = #0x7e
}
{ nop }
{ isync }
{ jumpr lr }
