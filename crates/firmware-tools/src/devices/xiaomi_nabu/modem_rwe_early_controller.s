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
// Combined compare + .new branch is a single 32-bit Hexagon instruction.
.set modem_rwe_nonmatch, agent11_modem_rwe_next_handler + 0x50
{
    p0 = cmp.eq(r0,r2); if (!p0.new) jump:nt modem_rwe_nonmatch
    r3 = memub(r2+#21)
    r9 = memub(r0+#21)
}
{
    r8 = memub(r2+#20)
    memb(r0+#20) = r1
}
.set modem_rwe_branch_ret, agent11_modem_rwe_next_handler + 0xbc
// LLVM6 treats the manually authored system-register word at 0xcb88 as
// outside a packet. Hexagon branches instead use the packet base (0xcb88),
// so compensate by one instruction to request the true `0xcbf4` target.
.set modem_rwe_branch_finish, agent11_modem_rwe_next_handler + 0xb0
.set modem_rwe_skip_tid_update, agent11_modem_rwe_next_handler + 0x48
{
    p0 = cmp.gt(r8,r1)
    if (!p0.new) jump:nt modem_rwe_skip_tid_update
}
MODEM_RWE_SYS_READ 16,2
{
    r16 = insert(r1,#8,#16)
}
.macro MODEM_RWE_SYS_WRITE system, general, packet_mark=3
  .if (\system < 0) || (\system > 63) || (\general < 0) || (\general > 31)
    .error "invalid Hexagon privileged state write operands"
  .endif
  .word (0x67000000 | (\general << 16) | (\packet_mark << 14) | \system)
.endm
MODEM_RWE_SYS_WRITE 2,16,1
{ jump modem_rwe_branch_finish }
{
    r5 = #2
    jump modem_rwe_branch_ret
    r11:10 = combine(r3,r2)
}
.set modem_rwe_next_tag_zero, agent11_modem_rwe_next_handler + 0x78
{
    p0 = cmp.eq(r9,#255)
    if (p0.new) jump:nt modem_rwe_next_tag_zero
    r10 = memub(r0+#20)
}
{
    p3 = r9
    r20 = r0
    r21 = r1
    memb(r0+#20) = r1
}
// Hexagon privilege priority-setting opcode, with checked predicate and
// general-register fields: source operands are p3 and r1, packet end.
.macro MODEM_RWE_SET_PRIORITY pred,reg,packet_mark=3
    .if (\pred < 0) || (\pred > 3) || (\reg < 0) || (\reg > 31)
        .error "invalid privilege priority operands"
    .endif
    .word (0x64800020 | (\reg << 16) | (\packet_mark << 14) | (\pred << 8))
.endm
MODEM_RWE_SET_PRIORITY 3,1
.set modem_rwe_actual_finish, agent11_modem_rwe_next_handler + 0xb0
{
    r1 = r21
    jump modem_rwe_actual_finish
    r0 = r20
}
{
    r17:16 = memd(r0+#8)
}
// Compact 32-bit compare-and-branch class for p0=cmp.eq(r16,#0),
// if(!p0.new) jump:nt. Unlike LLVM6's two independent operations this is
// architecturally a single instruction and the packet uses cbC4 as PC base.
.macro MODEM_RWE_COMPACT_R16_NOT_ZERO delta_bytes,packet_mark=1
    .if (\delta_bytes < 0) || ((\delta_bytes & 1) != 0) || (\delta_bytes > 510)
        .error "invalid compact compare-and-branch target"
    .endif
    .word (0x10400000 | ((16 - 8) << 16) | (\packet_mark << 14) | (\delta_bytes / 2))
.endm
{ p0 = cmp.eq(r17,#0) }
MODEM_RWE_COMPACT_R16_NOT_ZERO (0xcbf4 - 0xcbc4),1
{ r10 = memub(r0+#20) }
{
    r8 = memub(r2+#20)
}
{
    r21:20 = combine(r2,r0)
    r22 = r3
}
.set modem_rwe_resolve_tag, agent11_modem_rwe_next_handler + (0xe8a0-0xcb48)
{ call ##modem_rwe_resolve_tag }
{
    memb(r0+#20) = r1
}
{
    r2 = r22
    jump modem_rwe_branch_ret
    r1:0 = combine(r21,r20)
    r5 = #1
}
modem_rwe_actual_finish:
{
    r5 = #0
    r1 = #0
    memb(r0+#20) = r1
}
modem_rwe_branch_ret:
{
    r27:26 = memd(fp+#-48)
    r25:24 = memd(fp+#-40)
}
{
    r23:22 = memd(fp+#-32)
    r21:20 = memd(fp+#-24)
}
{
    r19:18 = memd(fp+#-16)
    r17:16 = memd(fp+#-8)
}
{
    dealloc_return
}
