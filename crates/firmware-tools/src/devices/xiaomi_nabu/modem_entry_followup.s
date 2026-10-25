// SPDX-License-Identifier: GPL-3.0-only
// Agent 11: Hexagon V6 modem snapshot and SRAM controller startup.
// Qualcomm S-register transfer encoding from the Hexagon v6 architecture.
// Source operands drive every emitted word; reference firmware bytes are
// never included in source.
.text
.macro MODEM_SYS_WRITE system, general, marker=3
    .if (\system < 0) || (\system > 63) || (\general < 0) || (\general > 31)
        .error "invalid Hexagon special/general register"
    .endif
    .word (0x67000000 | (\general << 16) | (\marker << 14) | \system)
.endm

.globl agent11_modem_state_snapshot
agent11_modem_state_snapshot:
{
    r2 = add(r25,##-1064879408)
}
{
    r0 = memw(r2++#4)
}
MODEM_SYS_WRITE 33,0
{
    r0 = memw(r2++#8)
    r1 = memw(r2+#4)
}
MODEM_SYS_WRITE 34,0
MODEM_SYS_WRITE 36,1
{
    r0 = memw(r2++#8)
    r1 = memw(r2+#4)
}
MODEM_SYS_WRITE 37,0
MODEM_SYS_WRITE 38,1
{
    r0 = memw(r2+#0)
    r1 = memw(r2+#4)
}
MODEM_SYS_WRITE 39,0
MODEM_SYS_WRITE 43,1
{
    jumpr lr
}

// Read modem physical translation metadata, select either the full map or
// an alternate smaller array, and copy at most a protected count of words.
.macro MODEM_SYS_READ general, system, marker=3
    .if (\system < 0) || (\system > 63) || (\general < 0) || (\general > 31)
        .error "invalid Hexagon S-register or general register"
    .endif
    .word (0x6e800000 | (\system << 16) | (\marker << 14) | \general)
.endm
.macro MODEM_PHY_WORD_LOAD dst, address, offset_reg, marker=3
    .if (\dst < 0) || (\dst > 31) || (\address < 0) || (\address > 31) || (\offset_reg < 0) || (\offset_reg > 31)
        .error "invalid Hexagon physical load operands"
    .endif
    .word (0x92002000 | (\address << 16) | (\marker << 14) | (\offset_reg << 8) | \dst)
.endm

.globl agent11_modem_copy_translation_entries
agent11_modem_copy_translation_entries:
MODEM_SYS_READ 0,27,3
{
    r0 = asl(r0,#5)
    r1 = #16
}
MODEM_PHY_WORD_LOAD 2,1,0
{
    r2 = asl(r2,#16)
    r0 = memw(r25+##-1061984984)
}
{
    p0 = cmp.gtu(r0,#0)
    if (!p0.new) jump:t modem_translation_copy_done
    r3 = add(r25,##-1047412672)
}
{
    loop0(translation_copy_word,r0)
    r1:0 = memd(r3++#8)
}
translation_copy_word:
{ r0.h = #0 }
{
    r1:0 = memd(r3++#8)
    memw(r2+r0<<#0) = r1
}:endloop0
modem_translation_copy_done:
{ jumpr lr }

// Update a packed 64-bit controller format descriptor. The protected word
// is selected by the active S27 register and a high-halfword table index.
.globl agent11_modem_rewrite_packed_descriptor
agent11_modem_rewrite_packed_descriptor:
MODEM_SYS_READ 6,27,1
{
    r3 = add(r25,##-1064894464)
    r1 = memw(r2+#4)
}
{
    p0 = cmpb.eq(r1,#0)
    r1 = lsr(r1,#16)
    if (p0.new) r3 = add(r25,##-1046887928)
}
{
    r6 = asl(r6,#5)
    r5:4 = memd(r3+r1<<#3)
}
{ r1 = extract(r4,#8,#8) }
MODEM_PHY_WORD_LOAD 6,1,6
{ r1 = memw(r2+#4) }
{
    r4 = insert(r6,#16,#5)
    r1 = lsr(r1,#16)
}
{
    jumpr lr
    memd(r3+r1<<#3) = r5:4
}
