// SPDX-License-Identifier: GPL-3.0-only
// Hexagon TLB index clearing, remapping and saved-entry restoration.
// Five LLVM6 NOP sentinels occupy V66-only tlbw(R3:2,R0) instructions.
// The final emitter rejects wrong sentinel locations and replaces each
// with a separately checked fixed-operand architectural TLB encoding.
.text
.globl adsp_tlb_clear_one
adsp_tlb_clear_one:
{ r3:2 = combine(#0,#0) }
{ nop }
{ isync }
{ jumpr lr }
.globl adsp_tlb_clear_all
adsp_tlb_clear_all:
{
 r5 = #0x7f
 r0 = #0; r3:2 = combine(#0,#0)
}
clear_loop:
{
 p0 = cmp.eq(r0,r5)
 if (p0.new) jumpr:nt lr
}
{ nop }
{
 r0 = add(r0,#1)
 jump clear_loop
}
.globl adsp_tlb_restore_entry
adsp_tlb_restore_entry:
{
 r0 = #0x7f
 r1 = ##-0xfff30e0
}
{ r2 = asl(r0,#3) }
{ r1 = add(r1,r2) }
{
 r0 = #0x7f
 r3:2 = memd(r1++#0)
}
{ nop }
{ jumpr lr }
.globl adsp_tlb_restore_lists
adsp_tlb_restore_lists:
{
 r1 = add(r25,##-0xfff15d8)
 r0 = #0
}
walk_table1:
{ r3:2 = memd(r1++#8) }
{ p0 = cmp.eq(r2,#0); if (p0.new) jump:t walk_table2_setup }
{ nop }
{ r0 = add(r0,#1) }
{ jump walk_table1 }
walk_table2_setup:
{ r1 = add(r25,##-0xfff1598) }
walk_table2:
{ r3:2 = memd(r1++#8) }
{ p0 = cmp.eq(r2,#0); if (p0.new) jump:t walk_done }
{ nop }
{ r0 = add(r0,#1) }
{ jump walk_table2 }
walk_done:
{ jumpr lr }
