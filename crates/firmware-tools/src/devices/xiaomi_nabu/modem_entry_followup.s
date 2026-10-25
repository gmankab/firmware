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

// Save the exception return state and protected mode register pairs before
// a controller startup probe; this prefix is part of a larger path.
.macro MODEM_SYS_PAIR_WRITE system, pairlow, marker=3
    .if (\system < 0) || (\system > 63) || (\pairlow < 0) || (\pairlow > 30) || ((\pairlow & 1) != 0)
        .error "invalid Hexagon paired-S-register operand"
    .endif
    .word (0x6d000000 | (\pairlow << 16) | (\marker << 14) | \system)
.endm

.globl agent11_modem_protected_state_entry
agent11_modem_protected_state_entry:
MODEM_SYS_PAIR_WRITE 50,30
MODEM_SYS_WRITE 47,29
MODEM_SYS_PAIR_WRITE 48,0
MODEM_SYS_PAIR_WRITE 44,24
{ r25 = p3:0 }
MODEM_SYS_WRITE 52,25
MODEM_SYS_READ 0,16,3

// Branch to the existing reset-vector exception at modem file 0x3020.
.set modem_reset_recovery, agent11_modem_protected_state_entry-0x10fc
{
    r0 = add(r0,#0)
    if (cmp.eq(r0.new,#0)) jump:nt modem_reset_recovery
}
{ r24 = pc }
{ r23 = ##-1065348796 }
{ r25 = sub(r24,r23) }
MODEM_SYS_WRITE 54,25
MODEM_SYS_READ 23,27,1
{ r24 = #0 }
{ r23 = insert(r24,#12,#20) }
{
    r23 = asl(r23,#5)
    r22 = #8
}
MODEM_PHY_WORD_LOAD 23,22,23
{ r23 = insert(r24,#12,#20) }
{
    r23 = asl(r23,#5)
    r22 = #68
}
{ r23 = add(r23,#-256) }
MODEM_PHY_WORD_LOAD 23,22,23

// Both error bits transfer control to the protected response at modem file
// 0x4a00, well outside this source-owned forward interval.
.set modem_protected_response, agent11_modem_protected_state_entry+0x8e4
{
    p0 = tstbit(r23,#2)
    if (p0.new) jump:t ##modem_protected_response
}
{
    p0 = tstbit(r23,#0)
    if (p0.new) jump:t ##modem_protected_response
}
{ jump modem_reset_recovery }
{ brkpt }

// Reset three controller status flags, then transfer control to register
// configuration. These MMIO data constants are authored as device fields.
.globl agent11_modem_reset_controller_flags
agent11_modem_reset_controller_flags:
{ r0 = ##68288576 }
{ memw(r0+#0) = ##7 }
{ r0 = ##68288580 }
{ memw(r0+#0) = ##7 }
{ r0 = ##68288584 }
{ memw(r0+#0) = ##7 }
{ barrier }
.set modem_initialize_mmio, agent11_modem_protected_state_entry + 0xf4
{ jump modem_initialize_mmio }

// Three-parameter return shim polls a mutable controller status counter
// before transferring its operands to the next bootstrap using r13.
.globl agent11_modem_await_counter
agent11_modem_await_counter:
{
    r8 = r1
    r9 = r2
    r10 = r3
}
{ r2 = r0 }
{ r0 = ##68296704 }
{ r1 = memw(r0+#0) }
modem_wait_for_change:
{ r3 = memw(r0+#0) }
{ r3 = sub(r3,r1) }
{
    p0 = cmp.gtu(r3,r2)
    if (!p0.new) jump:t modem_wait_for_change
}
{
    r1 = r8
    r2 = r9
    r3 = r10
}
{ jumpr r13 }

// Initialize the controller's DRAM window, write an indexed dispatch marker,
// reset its state-count limits, and clear the three MMIO completion flags.
// Absolute device-register addresses are named assembler operands, not
// imported instruction words; `##` retains the architecture extenders.
.macro MODEM_MMIO_STORE_WORD absolute, value
    .if ((\absolute & 3) != 0)
        .error "modem MMIO register address must be word aligned"
    .endif
    { r0 = ##\absolute }
    { memw(r0+#0) = ##\value }
.endm

.globl agent11_modem_initialize_mmio
agent11_modem_initialize_mmio:
MODEM_MMIO_STORE_WORD 67633688, 541611673
MODEM_MMIO_STORE_WORD 67633692, 8807
MODEM_MMIO_STORE_WORD 67633696, 36
MODEM_MMIO_STORE_WORD 67633676, 0
MODEM_MMIO_STORE_WORD 67633680, 2053
MODEM_MMIO_STORE_WORD 67633684, 208
{ barrier }
MODEM_MMIO_STORE_WORD 67633700, 0
MODEM_MMIO_STORE_WORD 67633704, 0
MODEM_MMIO_STORE_WORD 67633708, 32
{ barrier }

// Check platform capability status and choose the bootstrap sizes that will
// be written to its controller dimension registers. Preserve the original
// 256-bit mask comparison and `.new` predicate forwarding.
.globl agent11_modem_set_controller_dimensions
agent11_modem_set_controller_dimensions:
{ r0 = ##33325056 }
{ r1 = memw(r0+#0) }
{ r2 = and(r1,##256) }
{
    p0 = cmp.eq(r2,##256)
    if (p0.new) jump:t modem_initialize_channel_dimensions
}
{ jump modem_initialize_channel_dimensions }
modem_initialize_channel_dimensions:
{ r14 = ##61 }
{ r15 = ##68 }
{ r16 = ##1 }
{ jump modem_controller_dimension_writes }
modem_controller_dimension_writes:
{
    r0 = ##67633668
    r1 = r14
}
{ memw(r0+#0) = r1 }
{ barrier }
{ r0 = ##67633672 }
{ r1 = r15 }
{ memw(r0+#0) = r1 }
{ barrier }
