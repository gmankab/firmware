// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 controller variable-length vector load, first function section.
// Modem file [0x3a778,0x3a84c), virtual +0xc0808000,
// physical +0x8e008000. The function continues beyond this interval.
// Predicate gating avoids reading register pairs beyond the declared vector.
.text
.macro AGENT11_LOAD_VECTOR_PAIR lo,hi,first,second,offlo,offhi
{
 p0 = cmp.gtu(r9,#\first)
 if(p3) jumpr:nt r31
 p1 = cmp.gtu(r9,#\second)
}
{
 p3 = and(p3,and(p1,!p1))
 if(p0) r\lo = memw(r13+#\offlo)
 if(p1) r\hi = memw(r13+#\offhi)
}
.endm
.globl agent11_modem_controller_vector_load
agent11_modem_controller_vector_load:
.set agent11_vector_unaligned,agent11_modem_controller_vector_load+0x138
.set agent11_vector_word_done,agent11_modem_controller_vector_load+0xf8
{
 p0 = !bitsclr(r0,#3)
 r7:6 = combine(r1,r0)
 r8 = r2
 if(p0.new) jump:nt agent11_vector_unaligned
}
{
 r9 = lsr(r8,#2)
 p2 = !bitsclr(r7,#3)
 r13 = and(r7,#-4)
 if(cmp.eq(r9.new,#0)) jump:nt agent11_vector_word_done
}
{
 r7 += asl(r9,#2)
 if(p2) r9 = add(r9,#1)
 p3 = cmp.gt(r0,r0)
}
AGENT11_LOAD_VECTOR_PAIR 16,17,0,1,0,4
AGENT11_LOAD_VECTOR_PAIR 18,19,2,3,8,12
AGENT11_LOAD_VECTOR_PAIR 20,21,4,5,16,20
AGENT11_LOAD_VECTOR_PAIR 22,23,6,7,24,28
AGENT11_LOAD_VECTOR_PAIR 24,25,8,9,32,36
AGENT11_LOAD_VECTOR_PAIR 26,27,10,11,40,44
{
 p0 = cmp.gtu(r9,#12)
 if(p3) jumpr:nt r31
 p1 = cmp.gtu(r9,#13)
}
{
 p3 = and(p3,and(p1,!p1))
 r13 = and(r7,#3)
 if(p0) r28 = memw(r13+#48)
}
.set agent11_vector_max_return,agent11_modem_controller_vector_load+0x170
{
 if(p3) jump:nt agent11_vector_max_return
 if(!p2) jump:nt agent11_vector_word_done
}
{ p1 = r13 }
{ r17:16 = valignb(r19:18,r17:16,p1); r19:18 = valignb(r21:20,r19:18,p1) }
{ r21:20 = valignb(r23:22,r21:20,p1); r23:22 = valignb(r25:24,r23:22,p1) }
{ r25:24 = valignb(r27:26,r25:24,p1); r27:26 = valignb(r29:28,r27:26,p1) }
agent11_vector_word_done:
{
 p0 = !bitsclr(r8,#3)
 p0 = !cmp.gtu(r8,#51)
 r8 = and(r8,#3)
}
{ if(!p0) jump:nt agent11_vector_max_return }
{ r14 = #0 }
{ r9 = #0 }
agent11_vector_byte_loop:
{
 p3 = and(p3,and(p1,!p1))
 r8 = add(r8,#-1)
 r15 = memub(r7++#1)
}
{
 p0 = !cmp.eq(r8,#0)
 if(p3) jumpr:nt r31
}
{
 r14 += asl(r15,r9)
 r9 = add(r9,#8)
 if(p0) jump:nt agent11_vector_byte_loop
}
{ lc1 = r14; jumpr r31 }
agent11_vector_unaligned:
{ r14 = #0 }
{ r9 = #0 }
agent11_vector_unaligned_loop:
{
 p3 = and(p3,and(p1,!p1))
 r6 = add(r6,#1)
 r15 = memub(r7++#1)
}
{
 p0 = !bitsclr(r6,#3)
 if(p3) jumpr:nt r31
 r8 = add(r8,#-1)
 p0 = !cmp.eq(r8,#0)
}
{
 r14 += asl(r15,r9)
 r9 = add(r9,#8)
 if(p0) jump:nt agent11_vector_unaligned_loop
}
{ lc0 = r14; jump agent11_modem_controller_vector_load+0x10 }
agent11_vector_max_return:
{ jumpr r31 }
