// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 bounded 32/8-bit vector store and byte-loop cases.
// Source-built modem file interval [0x3a8ec,0x3aa1c) only. The exceptional
// traps/handlers after this block belong to later independent fragments.
.text
.macro AGENT11_STORE_VECTOR_PAIR lo,hi,first,second,offlo,offhi
{
 p0 = cmp.gtu(r9,#\first)
 if(p3) jumpr:nt r31
 p1 = cmp.gtu(r9,#\second)
}
{
 p3 = and(p3,and(p1,!p1))
 if(p0) memw(r13+#\offlo) = r\lo
 if(p1) memw(r13+#\offhi) = r\hi
}
.endm
.globl agent11_modem_controller_vector_store
agent11_modem_controller_vector_store:
.set agent11_store_unaligned,agent11_modem_controller_vector_store+0xf4
.set agent11_store_first_tail,agent11_modem_controller_vector_store+0xc0
.set agent11_store_result,agent11_modem_controller_vector_store+0x120
{
 p0 = !bitsclr(r0,#3)
 r7:6 = combine(r1,r0)
 r8 = r2
 if(p0.new) jump:nt agent11_store_unaligned
}
{
 r9 = lsr(r8,#2)
 r13 = #12
 if(cmp.eq(r9.new,#0)) jump:nt agent11_store_first_tail
}
{ r9 = minu(r9,r13) }
{
 r6 += asl(r9,#2)
 r13 = r6
 p3 = cmp.gt(r0,r0)
}
AGENT11_STORE_VECTOR_PAIR 16,17,0,1,0,4
AGENT11_STORE_VECTOR_PAIR 18,19,2,3,8,12
AGENT11_STORE_VECTOR_PAIR 20,21,4,5,16,20
AGENT11_STORE_VECTOR_PAIR 22,23,6,7,24,28
AGENT11_STORE_VECTOR_PAIR 24,25,8,9,32,36
AGENT11_STORE_VECTOR_PAIR 26,27,10,11,40,44
{ if(p3) jump:nt agent11_store_result }
agent11_store_first_tail:
{
 p0 = !bitsclr(r8,#3)
 p0 = !cmp.gtu(r8,#51)
 r8 = and(r8,#3)
}
{ if(!p0) jump:nt agent11_store_result }
{ r15 = lc1 }
agent11_store_byte_loop:
{
 p3 = and(p3,and(p1,!p1))
 r15 = lsr(r15,#8)
 r8 = add(r8,#-1)
 memb(r6++#1) = r15
}
{ p0 = !cmp.eq(r8,#0); if(p3) jumpr:nt r31 }
{ if(p0) jump:nt agent11_store_byte_loop; jump agent11_store_result }
agent11_store_unaligned:
{ r15 = lc0 }
agent11_store_unaligned_loop:
{
 p3 = and(p3,and(p1,!p1))
 r15 = lsr(r15,#8)
 r8 = add(r8,#-1)
 memb(r6++#1) = r15
}
{
 p0 = !bitsclr(r6,#3)
 if(p3) jumpr:nt r31
 p0 = !cmp.eq(r8,#0)
}
{ if(p0) jump:nt agent11_store_unaligned_loop }
{ lc0 = r14; jump agent11_modem_controller_vector_store+0x10 }
agent11_store_result:
{ r6 = sub(r6,r0) }
// LLVM6 compresses the return with the second ADD instead of forming the
// original two-ADD compact duplex. Express both 16-bit add operations from
// the documented duplex opcode fields; the verifier checks the following
// packet-marker update and the exact source-built compact operands.
.macro AGENT11_DUPLEX_ADD_PAIR src,first,second
 .if (\src<0)||(\src>7)||(\first!=0)||(\second!=1)
  .error "Only the checked add-to-r0/r1 tuple is valid"
 .endif
 .word (((0x3800 | (\src<<4) | \first)<<16) | (0x3800 | (\src<<4) | \second))
.endm
{ r2 = sub(r2,r6); jumpr r31 }
AGENT11_DUPLEX_ADD_PAIR 6,0,1
