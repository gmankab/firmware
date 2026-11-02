// SPDX-License-Identifier: GPL-3.0-only
// ADSP file [0x1034c,0x103a4), physical [0x8be0d34c,0x8be0d3a4).
// Read per-thread cycle count and update controller SRAM accumulator.
// V66 HTID / PCYCLE bank instructions are typed checked encoders replacing
// exactly one LLVM-assembled NOP each, not copied reference opcodes.
.text
.globl adsp_rwe_thread_cycles
adsp_rwe_thread_cycles:
{ r26=r2 }
{ r15=#-1 }
{ trace(r15) }
{ immext(#0xf0024800); r1=memw(##0xf0024810) }
{ p3=cmp.eq(r1,#0); if(p3.new) jump:t store_thread_cycle }
{ nop } // r6=HTID
{ nop } // r5:4=PCYCLE
{ r1=add(r0,#0x110) }
{ r1 += mpyi(r6,#8); r15:14=memd(r0+#0x108) }
{ r15:14=sub(r5:4,r15:14); r5:4=memd(r1+#0) }
{ r5:4=add(r15:14,r5:4) }
{ memd(r1+#0)=r5:4 }
store_thread_cycle:
{ nop; immext(#0xf00247c0); r1=##0xf00247e0 } // r12=HTID
{ nop } // checked R5:4=PCYCLE continuation, packed with next instruction
{ r1 += mpyi(r12,#16) } // one-word source form becomes packet end after checked pairing
{ memd(r1+#0)=r5:4 }
