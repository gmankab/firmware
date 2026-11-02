// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 entry-RX tail: snapshot 128 TLB entry pairs, clean the
// destination D-cache lines, return. File offsets [0x51dc,0x521c).
// LLVM6 does not know the privileged S54 and tlbr mnemonics. Their two
// exact locations are architecture-typed encoded after sentinel validation.
.text
.globl adsp_controller_tlb_snapshot
adsp_controller_tlb_snapshot:
{ nop } // V66 R23 = S54 (typed replacement)
{ r0 = #128; r1 = add(r23,##-0xfff30e0) }
{ r4 = lsr(r0,#2) }
{ loop0(copy_tlb,r0); r0 = #0 }
copy_tlb:
{ nop } // V66 R3:2 = tlbr(R0), typed replacement
{ r0 = add(r0,#1); memd(r1++#8) = r3:2 }:endloop0
{ r1 = add(r23,##-0xfff30e0) }
{ loop0(clean_line,r4) }
clean_line:
{ r1 = add(r1,#32); dccleana(r1) }:endloop0
{ jumpr lr }
