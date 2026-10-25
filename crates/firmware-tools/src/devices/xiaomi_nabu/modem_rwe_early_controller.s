// SPDX-License-Identifier: GPL-3.0-only
// Independent source encoding of early modem RWE controller descriptor bootstrap.
// File offset 0xcb00 (VMA 0xc0814b00, PA 0x8e014b00); first RWE PT_LOAD
// begins at file 0xa000; the region does not adjoin first modem executable.
.text
.macro MODEM_RWE_SYS_READ destination, system, packet_mark=3
    .if (\destination < 0) || (\destination > 31) || (\system < 0) || (\system > 63)
        .error "invalid Hexagon S-register operands"
    .endif
    .word (0x6e800000 | (\system << 16) | (\packet_mark << 14) | \destination)
.endm
.macro MODEM_RWE_SYS_PAIR_READ destination, system, packet_mark=3
    .if (\destination < 0) || (\destination > 30) || ((\destination & 1) != 0) || (\system < 0) || (\system > 62) || ((\system & 1) != 0)
        .error "invalid Hexagon S register pair operands"
    .endif
    .word (0x6f000000 | (\system << 16) | (\packet_mark << 14) | \destination)
.endm
.globl agent11_modem_rwe_descriptor_start
agent11_modem_rwe_descriptor_start:
.set modem_rwe_descriptor_next, agent11_modem_rwe_descriptor_start + 0x48
// Emit the standard EXTENDER plus GP-relative 64-bit load. LLVM6 compresses
// the absolute address to a short GP-relative form, so explicitly encode the
// Hexagon immediate-extension fields to preserve the source's chosen packet.
.macro MODEM_RWE_EXT_GPREL_PAIR_READ pairlow,address
    .if ((\pairlow < 0) || (\pairlow > 30) || ((\pairlow & 1) != 0) || ((\address & 0x3f) != 0))
        .error "invalid Hexagon pair or GPREL address"
    .endif
    .word (0x0c000000 | (((\address >> 20) & 0x3ff) << 16) | ((\address >> 6) & 0x3fff) | (1 << 14))
    .word (0x49c00000 | (3 << 14) | \pairlow)
.endm
MODEM_RWE_SYS_READ 12,8,1
MODEM_RWE_EXT_GPREL_PAIR_READ 14,0xc0815840
{
    p0 = tstbit(r15,#25)
    if (!p0.new) jump:nt modem_rwe_descriptor_next
}
{
    r13 = add(r14,#-16)
    r15:14 = memd(r12<<#3+##0xc0815848)
}
{
    p0 = cmp.gtu(r13,r14)
    if (p0.new) r14 = add(r14,#16)
    if (!p0.new) r14 = #0
    r13 = add(r15,r14)
}
MODEM_RWE_SYS_PAIR_READ 14,56,1
{
    memw(r12<<#3+##0xc0815848) = r14
}
{
    r15.h = #0x819
    memd(r13+#8) = r1:0
}
{
    memd(r13+#0) = r15:14
}

.globl agent11_modem_rwe_next_handler
agent11_modem_rwe_next_handler:
{
    allocframe(#48)
}
{
    memd(fp+#-48) = r27:26
    memd(fp+#-40) = r25:24
}
{
    memd(fp+#-32) = r23:22
    memd(fp+#-24) = r21:20
}
{
    memd(fp+#-16) = r19:18
    memd(fp+#-8) = r17:16
}
