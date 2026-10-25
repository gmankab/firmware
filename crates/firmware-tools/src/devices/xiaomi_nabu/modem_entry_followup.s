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

// Clear reserved controller ownership bits. Each register update uses a
// barrier so subsequent protected hardware reads observe the written state.
.globl agent11_modem_enable_controller_registers
agent11_modem_enable_controller_registers:
{ r0 = ##67633664 }
{
    r7 = r0
    r5.h = #0
}
{ r5.l = #1 }
{
    r5 = sub(#-1,r5)
    r4 = memw(r0+#0)
}
{ r4 = and(r4,r5) }
{ memw(r0+#0) = r4 }
{ barrier }
{ r0 = ##67633720 }
{
    r7 = r0
    r5.h = #0
}
{ r5.l = #7 }
{
    r5 = sub(#-1,r5)
    r4 = memw(r0+#0)
}
{ r4 = and(r4,r5) }
{ memw(r0+#0) = r4 }
{ barrier }
{ r0 = ##67633664 }
{
    r7 = r0
    r5.h = #0
}
{
    r5.l = #4
    dcinva(r7)
}
{
    r6 = sub(#-1,r5)
    r4 = memw(r0+#0)
}
{ r4 = and(r4,r6) }
{ r4 = or(r4,r5) }
{ memw(r0+#0) = r4 }
{ barrier }
{ r0 = ##67633720 }
{ r1 = memw(r0+#0) }
{ r2 = ##1 }
{ r1 = or(r1,r2) }
{ memw(r0+#0) = r1 }
{ barrier }
// Preserve the callback's signed range mask and link register return PC.
{ r4 = ##67633664 }
{ r2 = ##-2147483648 }
modem_mask_mismatch_retry:
{ r0 = ##192 }
{ r13 = add(pc,#8) }
{ jump agent11_modem_await_counter }

// Await protected status then enable two controller ownership masks. This
// shared callback may retry via the earlier r13/loop shim on mismatch.
.globl agent11_modem_reconcile_protected_status
agent11_modem_reconcile_protected_status:
{ r1 = memw(r4+#0) }
{ r3 = and(r1,r2) }
{
    p0 = cmp.eq(r2,r3)
    if (!p0.new) jump:t modem_mask_mismatch_retry
}
{ r0 = ##67633676 }
{
    r7 = r0
    r5.h = #0
}
{
    r5.l = #2
    dcinva(r7)
}
{
    r6 = sub(#-1,r5)
    r4 = memw(r0+#0)
}
{ r4 = and(r4,r6) }
{ r4 = or(r4,r5) }
{ memw(r0+#0) = r4 }
{ barrier }
{ r0 = ##67633664 }
{
    r7 = r0
    r5.h = #0
}
{
    r5.l = #1
    dcinva(r7)
}
{
    r6 = sub(#-1,r5)
    r4 = memw(r0+#0)
}
{ r4 = and(r4,r6) }
{ r4 = or(r4,r5) }
{ memw(r0+#0) = r4 }
{ barrier }

// Preserve all capability flags except the write-sensitive 11-bit mask;
// fold the control offset from r16 into the new word with a barrier.
{ r0 = ##67633196 }
{ r1 = memw(r0+#0) }
{
    r2 = ##1792
    r3 = ##31
}
{ r2 = or(r2,r3) }
{ r2 = sub(#-1,r2) }
{ r1 = and(r1,r2) }
{
    r3 = asl(r16,#0)
    r2 = ##256
}
{ r2 = or(r2,r3) }
{ r1 = or(r1,r2) }
{ memw(r0+#0) = r1 }
{ barrier }
{ r0 = ##67633192 }
{ r1 = memw(r0+#0) }
{ r2 = ##1 }
{ r1 = or(r1,r2) }
{ memw(r0+#0) = r1 }
{ barrier }

// Poll until all requested capability bits become observable, then clear
// three completion flags and restore the original caller link register.
modem_wait_capability:
{ r1 = memw(r0+#0) }
{ r3 = and(r1,r2) }
{
    p0 = cmp.eq(r3,r2)
    if (p0.new) jump:t modem_wait_capability
}
MODEM_MMIO_STORE_WORD 68288576, 0
MODEM_MMIO_STORE_WORD 68288580, 0
MODEM_MMIO_STORE_WORD 68288584, 0
{ barrier }
{ jumpr lr }
{ brkpt }
{ brkpt }
{ brkpt }

// Query the protected controller's pc-relative global pointer, then return.
.globl agent11_modem_get_gp_base
agent11_modem_get_gp_base:
{ r29 = add(pc,##261488) }
{ jumpr lr }
// Companion no-op invocation slot: preserving original callable layout.
.globl agent11_modem_noop_slot
agent11_modem_noop_slot:
{ jumpr lr }

.macro MODEM_ICKILL marker=3
    // Hexagon V6 Y2_ickill instruction, checked packet marker.
    .if (\marker < 0) || (\marker > 3)
        .error "invalid ickill packet marker"
    .endif
    .word (0x56c01000 | (\marker << 14))
.endm

// Enter the uncached processing window by disabling S18's cache bit,
// synchronizing and invalidating the I-cache, then restoring the flags.
.globl agent11_modem_cache_state_transition
agent11_modem_cache_state_transition:
MODEM_SYS_READ 15,18,3
{ r15 = clrbit(r15,#1) }
MODEM_SYS_WRITE 18,15
{ isync }
MODEM_ICKILL
{ isync }
{ r15 = or(r15,#3) }
MODEM_SYS_WRITE 18,15
{ isync }
{ r31 = sub(r31,r25) }
{ jumpr r31 }

// Trampoline from relocated virtual PC to an absolute controller callback.
.globl agent11_modem_relocate_callback
agent11_modem_relocate_callback:
{
    r30 = ##-1065347776
    r31 = add(r31,r25)
}
{ r30 = add(r30,r25) }
{ jumpr r30 }

// Single-byte cache flag clear and synchronized return.
.globl agent11_modem_clear_controller_bit
agent11_modem_clear_controller_bit:
MODEM_SYS_READ 15,18,3
{ r15 = clrbit(r15,#0) }
MODEM_SYS_WRITE 18,15
{ isync }
{ jumpr lr }

// Walk a sequence of TLB records from protected DDR, write each entry and
// synchronize before returning. The original loop tracks the entry count in
// r0 and terminates when the memory-loaded pair ends the linked table.
.globl agent11_modem_write_tlb_records
agent11_modem_write_tlb_records:
{
    r0 = #0
    r1 = add(r25,##-1064894464)
}
modem_read_tlb_record:
{ r3:2 = memd(r1++#8) }
// Hexagon Y2_tlbw transfers pair r3:2 to translation index r0.
.macro MODEM_TLB_WRITE pairlow, index, marker=3
    .if (\pairlow < 0) || (\pairlow > 30) || ((\pairlow & 1) != 0) || (\index < 0) || (\index > 31)
        .error "invalid Hexagon TLB pair/index"
    .endif
    .word (0x6c000000 | (\pairlow << 16) | (\marker << 14) | (\index << 8))
.endm
MODEM_TLB_WRITE 2,0
{
    p0 = cmp.eq(r2,#0)
    if (!p0.new) jump:t modem_read_tlb_record
    r0 = add(r0,#1)
}
{ isync }
{ jumpr lr }

// Install TLB index 191 for the active register-mapped descriptor. Uses
// two 16-bit source fields packed into a translation descriptor pair.
.globl agent11_modem_install_tlb191
agent11_modem_install_tlb191:
{
    p0 = cmp.eq(r25,#0)
    if (p0.new) jump:nt modem_tlb191_exit
}
{
    r2 = lsr(r24,#11)
    r3 = lsr(r24,#12)
    r4 = #1984
    r5 = #3072
}
{
    r2 = insert(r4,#11,#21)
    r3 = insert(r5,#12,#20)
}
{
    r2 = setbit(r2,#1)
    r0 = #191
}
MODEM_TLB_WRITE 2,0
{ isync }
modem_tlb191_exit:
{ jumpr lr }

// Alternate TLB descriptor uses high-halfword of r26 and index 190.
.globl agent11_modem_install_tlb190
agent11_modem_install_tlb190:
{
    r2 = lsr(r24,#11)
    r3 = lsr(r26,#12)
    r4 = #1984
    r5 = #3072
}
{
    r2 = insert(r4,#11,#21)
    r3 = insert(r5,#12,#20)
}
{
    r2 = setbit(r2,#1)
    r0 = #190
}
MODEM_TLB_WRITE 2,0
{ isync }
{ jumpr lr }

// Invalid TLB map entries intentionally write a zero pair to index zero.
.globl agent11_modem_zero_tlb_pair
agent11_modem_zero_tlb_pair:
{ r3:2 = combine(#0,#0) }
MODEM_TLB_WRITE 2,0
{ isync }
{ jumpr lr }

// Iterate translation slot 191 until the CPU fault predicate is set.
.globl agent11_modem_zero_contiguous_tlb_entries
agent11_modem_zero_contiguous_tlb_entries:
{
    r5 = #191
    r0 = #0
    r3:2 = combine(#0,#0)
}
modem_zero_tlb_next:
{
    p0 = cmp.eq(r0,r5)
    if (p0.new) jumpr:nt lr
}
MODEM_TLB_WRITE 2,0
{
    r0 = add(r0,#1)
    jump modem_zero_tlb_next
}

// Lookup translation entry 191 from a relative protected table and commit it.
.globl agent11_modem_restore_tlb191
agent11_modem_restore_tlb191:
{
    r0 = #191
    r1 = ##-1064894464
}
{ r2 = asl(r0,#3) }
{ r1 = add(r1,r2) }
{
    r0 = #191
    r3:2 = memd(r1++#0)
}
MODEM_TLB_WRITE 2,0
{ jumpr lr }

// Walk protected table A, writing TLB descriptors until the descriptor's
// low 32-bit word is zero. Each memory pair is loaded in source order.
.globl agent11_modem_reload_tlb_table_a
agent11_modem_reload_tlb_table_a:
{
    r1 = add(r25,##-1064885576)
    r0 = #0
}
modem_reload_tlb_a_next:
{ r3:2 = memd(r1++#8) }
{
    p0 = cmp.eq(r2,#0)
    if (p0.new) jump:t modem_reload_tlb_a_complete
}
MODEM_TLB_WRITE 2,0
{ r0 = add(r0,#1) }
{ jump modem_reload_tlb_a_next }
modem_reload_tlb_a_complete:
// Same loop for protected table B. The final linked slot exits via r31.
.globl agent11_modem_reload_tlb_table_b
agent11_modem_reload_tlb_table_b:
{ r1 = add(r25,##-1064885512) }
modem_reload_tlb_b_next:
{ r3:2 = memd(r1++#8) }
{
    p0 = cmp.eq(r2,#0)
    if (p0.new) jump:t modem_reload_tlb_b_complete
}
MODEM_TLB_WRITE 2,0
{ r0 = add(r0,#1) }
{ jump modem_reload_tlb_b_next }
modem_reload_tlb_b_complete:
{ jumpr lr }

// Verified V6 controller copy using two nested hardware loops, L2 fetch and
// paired 64-bit copies. The source cache command buffer is length-checked;
// the two hardware loops both terminate in the same packet.
.globl agent11_modem_copy_cache_buffers
agent11_modem_copy_cache_buffers:
{
    p0 = cmp.eq(r2,#0)
    if (p0.new) jump:nt modem_copy_cache_exit
    r0 = memw(##3249373896)
}
{
    p0 = cmp.gt(r2,#32)
    if (!p0.new) jump:nt modem_copy_cache_exit
}
{
    p0 = cmp.eq(r0,#0)
    if (p0.new) jump:nt modem_copy_cache_exit
    r3 = ##128
}
{
    r6 = lsr(r2,#12)
    r4 = add(r0,#8)
    r5 = add(r1,#8)
}
{
    loop1(modem_cache_outer,r6)
    r7 = ##8421408
}
modem_cache_outer:
{ l2fetch(r1,r7) }
{ loop0(modem_cache_inner,r3) }
modem_cache_inner:
{ dczeroa(r1) }
{
    r7:6 = memd(r0++#16)
    r9:8 = memd(r4++#16)
}
{
    memd(r1++#16) = r7:6
    memd(r5++#16) = r9:8
}
{
    r7:6 = memd(r0++#16)
    r9:8 = memd(r4++#16)
}
{
    nop
    memd(r1++#16) = r7:6
    memd(r5++#16) = r9:8
}:endloop0:endloop1
modem_copy_cache_exit:
{ jumpr lr }
{ brkpt }
{ brkpt }
{ brkpt }

// Copy a protected 64-bit descriptor array from a GP-relative source to a
// mapped destination, bounded by the controller-provided word count.
.globl agent11_modem_copy_protected_descriptors
agent11_modem_copy_protected_descriptors:
{
    r3 = memw(r25+##-1045650720)
    r0 = memw(r25+##-1064885436)
}
{
    p0 = cmp.eq(r3,#0)
    if (p0.new) jump:nt modem_protected_copy_exit
}
{
    r1 = add(r25,##-1045593400)
    p0 = cmp.gt(r3,#32)
    if (!p0.new) jump:nt modem_protected_copy_exit
}
{
    p0 = cmp.eq(r1,#0)
    if (p0.new) jump:nt modem_protected_copy_exit
}
{ r2 = lsr(r3,#3) }
{ loop0(modem_copy_protected_next,r2) }
modem_copy_protected_next:
{ r5:4 = memd(r1++#8) }
{
    nop
    memd(r0++#8) = r5:4
}:endloop0
modem_protected_copy_exit:
{ jumpr lr }

// First protected memory-page scan following the previously rebuilt copies.
// Read up to 191 indexed Hexagon TLB entries, inspect the nonzero low word,
// count trailing zero bits and conditionally update a zero translation pair.
// The three exceptional TLB operations follow the independent Hexagon ISA
// decoder tests: tlbr (Y2_tlbr), tlblock and tlbunlock (rz-hexagon corpus).
// `tlblock`/`tlbunlock` are instruction classes, not original byte slices.
.macro MODEM_TLB_READ pair_low, index, marker=3
    .if (\pair_low < 0) || (\pair_low > 30) || ((\pair_low & 1) != 0) || (\index < 0) || (\index > 31)
        .error "invalid modem Hexagon indexed TLB read operand"
    .endif
    .word (0x6c400000 | (\index << 16) | (\marker << 14) | \pair_low)
.endm
.macro MODEM_TLB_LOCK marker=3
    .if (\marker != 1) && (\marker != 3)
        .error "invalid Hexagon TLBlock parse marker"
    .endif
    .word (0x6c200020 | (\marker << 14))
.endm
.macro MODEM_TLB_UNLOCK marker=3
    .if (\marker != 1) && (\marker != 3)
        .error "invalid Hexagon TLBUnlock parse marker"
    .endif
    .word (0x6c200040 | (\marker << 14))
.endm

.globl agent11_modem_protected_pagewalk
agent11_modem_protected_pagewalk:
{
    r0 = memw(##3230081860)
    r1 = memw(##3249316576)
}
{
    p0 = cmp.eq(r1,#0)
    if (p0.new) jump:nt modem_pagewalk_return
}
{
    r0 = lsr(r0,#12)
    r1 = lsr(r1,#12)
    r10 = ##4194303
}
{
    loop0(modem_pagewalk_scan,#191)
    r2 = add(r0,r1)
    r3 = #0
}
modem_pagewalk_scan:
MODEM_TLB_READ 4,3
{
    p0 = cmp.eq(r5,#0)
    if (p0.new) jump:nt modem_pagewalk_loop_next
}
{ r6 = ct0(r4) }
{ r4 = clrbit(r4,r6) }
{ r4 = asr(r4,#1) }
{ r4 = and(r4,r10) }
{
    p0 = cmp.eq(r4,r0)
    if (p0.new) jump:nt modem_pagewalk_write_zero
}
{
    p0 = cmp.gtu(r4,r2)
    if (p0.new) jump:t modem_pagewalk_loop_next
}
{
    p1 = cmp.gtu(r0,r4)
    if (p1.new) jump:t modem_pagewalk_loop_next
}
modem_pagewalk_write_zero:
{ r7:6 = combine(#0,#0) }
MODEM_TLB_LOCK
MODEM_TLB_WRITE 6,3
MODEM_TLB_UNLOCK
modem_pagewalk_loop_next:
{
    r3 = add(r3,#1)
    nop
}:endloop0
// Reset all translations from the zero-terminated GP table, locking each
// slot during privileged TLB writes, until its original low word is empty.
{ r4 = ##-1064885576; r5 = memw(##3249316816) }
{ r10 = #5; r8 = #0 }
modem_pagewalk_restore_next:
{ r1:0 = memd(r4++#8) }
{
    p0 = cmp.eq(r0,#0)
    if (p0.new) jump:t modem_pagewalk_return
}
MODEM_TLB_LOCK
MODEM_TLB_WRITE 0,3
MODEM_TLB_UNLOCK
{ r3 = add(r3,#-1); r8 = add(r8,#1) }
{
    p1 = cmp.eq(r3,r5)
    if (p1.new) jump:nt modem_pagewalk_return
}
{
    p0 = cmp.eq(r8,r10)
    if (!p0.new) jump:nt modem_pagewalk_restore_next
}
modem_pagewalk_return:
{ jumpr lr }

// Protected word-transfer dispatcher. It preserves the exception link in r17,
// initializes the global memory region and calls an already reconstructed
// pagewalk prior to launching a cache-line transfer helper.
.globl agent11_modem_dispatch_protected_word_transfer
agent11_modem_dispatch_protected_word_transfer:
{ r17 = r31 }
{ r29 = ##-1065079608 }
{ call ##agent11_modem_protected_pagewalk }
{ r0 = memw(##3230081848) }
{ call ##agent11_modem_zero_cache_lines }
{ r31 = r17 }
{ jumpr r31 }

// Bounded single-level loop: zero a whole L2 cache line before copying the
// next two adjacent 64-bit register pairs into the controller destination.
.globl agent11_modem_zero_cache_lines
agent11_modem_zero_cache_lines:
{
    r1 = memw(##3249373896)
    r2 = memw(##3249373900)
}
{
    p0 = cmp.eq(r1,#0)
    if (p0.new) jump:nt modem_zero_cache_return
}
{
    p0 = cmp.eq(r2,#0)
    if (p0.new) jump:nt modem_zero_cache_return
}
{
    p0 = cmp.gt(r2,#32)
    if (!p0.new) jump:nt modem_zero_cache_return
}
{
    r3 = lsr(r2,#5)
    p0 = cmp.eq(r0,#0)
    if (p0.new) jump:nt modem_zero_cache_return
}
{
    loop0(modem_zero_cache_next,r3)
    r4 = add(r0,#8)
    r5 = add(r1,#8)
}
modem_zero_cache_next:
{ dczeroa(r1) }
{
    r7:6 = memd(r0++#16)
    r9:8 = memd(r4++#16)
}
{
    memd(r1++#16) = r7:6
    memd(r5++#16) = r9:8
}
{
    r7:6 = memd(r0++#16)
    r9:8 = memd(r4++#16)
}
{
    memd(r1++#16) = r7:6
    memd(r5++#16) = r9:8
}:endloop0
modem_zero_cache_return:
{ jumpr r31 }

// A four-word physical-indexed copy: extract an eleven-bit selector from a
// source register, read the protected physical word, and write to a buffer.
// MODEM_PHY_WORD_LOAD uses the already checked Hexagon Y2 physical load ISA.
.globl agent11_modem_copy_phys_words
agent11_modem_copy_phys_words:
{
    r1 = memw(##3261442188)
    r3 = memw(##3249316576)
}
{
    p0 = cmp.eq(r1,#0)
    if (p0.new) jump:nt modem_phys_copy_done
}
{
    p0 = cmp.eq(r3,#0)
    if (p0.new) jump:nt modem_phys_copy_done
}
{
    p0 = cmp.gt(r3,#32)
    if (!p0.new) jump:nt modem_phys_copy_done
    r0 = memw(##3230081860)
}
{
    r2 = lsr(r3,#2)
    p0 = cmp.eq(r0,#0)
    if (p0.new) jump:nt modem_phys_copy_done
}
{
    loop0(modem_phys_copy_next,r2)
    r2 = #11
}
modem_phys_copy_next:
{ r4 = lsr(r0,r2) }
MODEM_PHY_WORD_LOAD 5,0,4
{
    r0 = add(r0,#4)
    memw(r1++#4) = r5
}:endloop0
modem_phys_copy_done:
{ jumpr lr }

// Last protected copy before the MMU exception helper group. A bounded L0
// loop compares each hardware physical word against its host-memory partner
// and writes at most four address/value pairs to the source-authored buffer.
.globl agent11_modem_scan_phys_descriptor_pairs
agent11_modem_scan_phys_descriptor_pairs:
{
    r1 = ##0
    r4 = ##-1063448472
}
{
    r0 = ##32768
    r6 = sub(r4,r1)
}
{
    p0 = cmp.gt(r0,r6)
    if (!p0.new) jump:nt modem_phys_pair_scan_done
    r0 = ##0
}
{
    r2 = lsr(r6,#2)
    p0 = cmp.eq(r0,#0)
    if (p0.new) jump:nt modem_phys_pair_scan_done
}
{
    r8 = memw(##3230081712)
    r9 = memw(##3230081708)
}
{
    r5 = sub(r1,r0)
    r9 = sub(r9,r8)
    r3 = ##-1065287680
}
{ r3 = add(r3,r9) }
{ r6 = ##3230035920; r3 = add(r3,r5) }
{
    loop0(modem_phys_pair_scan_next,r2)
    r2 = #11
    r7 = #0
}
modem_phys_pair_scan_next:
{
    r4 = lsr(r3,r2)
    r0 = memw(r1+#0)
}
MODEM_PHY_WORD_LOAD 5,3,4
{
    p0 = cmp.eq(r5,r0)
    if (p0.new) jump:nt modem_phys_pair_scan_increment
}
{
    p0 = cmp.gt(r7,#3)
    if (p0.new) jump:nt modem_phys_pair_scan_done
}
{
    r7 = add(r7,#1)
    memw(r6++#4) = r3
}
{ memw(r6++#4) = r5 }
modem_phys_pair_scan_increment:
{
    nop
    r1 = add(r1,#4)
    r3 = add(r3,#4)
}:endloop0
modem_phys_pair_scan_done:
{ jumpr r31 }

// Preserve the translation-group hardware register S27, derive the low
// physical MMU control halfword and publish it as a protected word in the
// same packet as the final return.
.globl agent11_modem_publish_phys_control
agent11_modem_publish_phys_control:
MODEM_SYS_READ 1,27
{ r1 = asl(r1,#5) }
{ r0 = ##0 }
MODEM_PHY_WORD_LOAD 2,0,1
{
    r2 = asl(r2,#16)
    jumpr lr
    memw(##3230081860) = r2.new
}

// Copy an initialized GP descriptor table if the destination pointer is
// non-null; no private firmware words are copied into source.
.globl agent11_modem_publish_gp_table_a
agent11_modem_publish_gp_table_a:
{ r2 = memw(r25+##-1033525120) }
{ r0 = add(r25,##-1045609280) }
{ r1 = memw(r25+##-1064851284) }
{ p1 = cmp.eq(r1,#0) }
{ if (p1) jump:nt modem_publish_gp_a_done }
{ loop0(modem_publish_gp_a_next,r2) }
modem_publish_gp_a_next:
{
    r3 = memw(r0++#4)
    memw(r1++#4) = r3.new
}:endloop0
modem_publish_gp_a_done:
{ jumpr lr }

// Alternate SRAM table at a fixed 0x2000 virtual address. The 8 KiB flag
// comparison is source-level Hexagon operand logic (not an OEM instruction).
.globl agent11_modem_publish_gp_table_b
agent11_modem_publish_gp_table_b:
{ r2 = memw(r25+##-1033525116) }
{ r0 = add(r25,##-1045609248) }
{ r1 = memw(r25+##-1064851280) }
{ p1 = cmp.eq(r1,##8192) }
{ if (p1) jump:nt modem_publish_gp_b_done }
{ loop0(modem_publish_gp_b_next,r2) }
modem_publish_gp_b_next:
{
    r3 = memw(r0++#4)
    memw(r1++#4) = r3.new
}:endloop0
modem_publish_gp_b_done:
{ jumpr lr }

// The next 17 architecturally executable words are intentional synchronous
// exception traps; count as trap bytes, not independently reconstructed
// application logic or unexplained zero-fill.
.globl agent11_modem_reserved_break_traps
agent11_modem_reserved_break_traps:
.rept 17
{ brkpt }
.endr

// Protected exception-controller initialization. Configures S18/S53 and
// S54, updates the controller's interrupt-mode flags and probes two physical
// page words via the checked Hexagon physical-read operation. This is a
// strict partial routine: successor setup after modem file 0x4ae8 is left
// to the next contiguous checkpoint, with cross-boundary jumps symbolic.
.globl agent11_modem_exception_controller_setup
agent11_modem_exception_controller_setup:
.set modem_exception_resume, agent11_modem_exception_controller_setup + 0x5d4
.set modem_exception_mode_two, agent11_modem_exception_controller_setup + 0x5b0
.set modem_exception_mode_other, agent11_modem_exception_controller_setup + 0x5c0
MODEM_SYS_READ 23,18
MODEM_SYS_WRITE 53,23
{ r0 = #40 }
MODEM_SYS_WRITE 18,0
{ isync }
MODEM_SYS_READ 23,54
{ r0 = add(r23,##-1064891264) }
{
    r25 = add(r0,#4)
    r24 = ##37
}
{ r0 = memw(r25+#0) }
{
    p0 = cmp.eq(r0,##37)
    if (p0.new) jump:nt modem_exception_resume
}
{
    r0 = add(r23,##-1064890044)
    memw(r25+#0) = r24
}
{
    r24 = #1
    memw(r0+#0) = r24.new
}
{ r0 = add(r23,##-1064891264) }
{ r0 = add(r0,#0) }
{ r24 = memw(r0+#0) }
{
    r24 = setbit(r24,#29)
    memw(r0+#0) = r24.new
}
MODEM_SYS_READ 1,42
{
    p0 = tstbit(r1,#2)
    if (!p0.new) jump:t modem_exception_skip_phys_check
}
MODEM_SYS_READ 1,27,1
{ r25 = #0 }
{ r1 = insert(r25,#12,#20) }
{
    r1 = asl(r1,#5)
    r0 = #8
}
MODEM_PHY_WORD_LOAD 1,0,1
{ r1 = insert(r25,#12,#20) }
{
    r1 = asl(r1,#5)
    r0 = #24
}
{ r1 = add(r1,#-256) }
MODEM_PHY_WORD_LOAD 1,0,1
{ r24 = memw(r23+##-1061985012) }
{ r0 = and(r1,r24) }
{
    p0 = cmp.eq(r0,#0)
    if (p0.new) jump:t modem_exception_next_stage
}
modem_exception_skip_phys_check:
{ r1 = #1 }
MODEM_SYS_WRITE 42,1
{ isync }
{ brkpt }
modem_exception_next_stage:
{ r1 = add(r23,##-1065346048) }
MODEM_SYS_WRITE 16,1
{ call agent11_modem_get_gp_base }
{ r25 = memw(r23+##-1064935416) }
{
    p0 = cmp.eq(r25,#2)
    p1 = cmp.eq(r25,#0)
}
{ if (p0) jump:nt modem_exception_mode_two }
{ if (!p1) jump:nt modem_exception_mode_other }
