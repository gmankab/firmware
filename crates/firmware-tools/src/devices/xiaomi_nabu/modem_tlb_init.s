// Hexagon V6 boot translation-cache initialization. 32-bit words are
// generated from named operands and the ISA's official opcode fields.
.text
.macro HEX_SYS_FROM dst, src, parse
    .word (0x6e800000 | (\src << 16) | (\parse << 14) | \dst)
.endm
.macro HEX_SYS_TO dst, src, parse
    .word (0x67000000 | (\src << 16) | (\parse << 14) | \dst)
.endm
.macro HEX_TLBP dst, src
    .word (0x6c800000 | (\src << 16) | (3 << 14) | \dst)
.endm
.macro HEX_TLBR pairlow, src
    .word (0x6c400000 | (\src << 16) | (3 << 14) | \pairlow)
.endm
.macro HEX_TLBW pairlow, index
    .word (0x6c000000 | (\pairlow << 16) | (3 << 14) | (\index << 8))
.endm
.macro HEX_ICKILL
    .word (0x56c01000 | (3 << 14))
.endm
.globl modem_tlb_init
modem_tlb_init:
    // SYSCFG is Hexagon system register S18.
    HEX_SYS_FROM 15,18,1
{
    r14 = ##-0x3f7ffa48
}
{
    r1 = add(pc,##0xac)
    r5 = lsr(r14,#12)
    r15 = and(r15,#-4)
}
{
    lr += sub(r14,r1)
    r4 = lsr(r1,#12)
    if (cmp.eq(lr.new,lr)) jump:nt #0x68
}
    HEX_TLBP 2,5
    HEX_TLBP 3,4
    HEX_TLBR 10,2
{
    r7 = clrbit(r11,#29)
    r12 = cmp.eq(r2,#0)
    p0 = !cmp.gt(r3,#-1)
    p1 = cmp.eq(r2,r3)
}
{
    r7 = insert(r4,#20,#0)
    r6 = setbit(r10,#0)
    if (p1) r3 = add(r12,#0)
    if (p0) r3 = add(r12,#0)
}
    HEX_TLBR 8,3
{
    r6 = insert(r4,#23,#1)
    if (!p1) r14 = add(r14,#-8)
    if (p1) r14 = add(r14,#-20)
    if (!p1) jump:nt #0x24
}
    HEX_TLBW 6,2
{ isync }
{
    r7 = insert(r9,#20,#0)
    r14 = insert(r9,#20,#12)
    r13 = add(r14,#12)
}
    HEX_TLBW 6,3
{ isync }
    HEX_SYS_TO 18,15,1
{ r15 = clrbit(r0,#1) }
{ isync }
    HEX_ICKILL
{ isync }
    HEX_SYS_TO 18,15,3
{ isync }
{ jumpr r14 }
    HEX_TLBW 10,2
{ isync }
{ jumpr r13 }
    HEX_TLBW 8,3
{ isync }
    HEX_SYS_TO 18,0,3
{ isync }
{ jumpr lr }
