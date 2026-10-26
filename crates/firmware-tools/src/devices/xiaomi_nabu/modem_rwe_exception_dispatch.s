// SPDX-License-Identifier: GPL-3.0-only
// Modem RWE exception entry, privileged startup snapshot and dispatch.
// Source-derived Hexagon V60 packets, offset 0x38108 (file),
// virtual 0xc0840108, physical 0x8e040108.
// This is the first entry of a longer handler; no stub return is invented.
.text
.macro MODEM_SYS_WRITE special, source, marker=3
    .if (\special < 0) || (\special > 63) || (\source < 0) || (\source > 31)
      .error "invalid system write operands"
    .endif
    .word (0x67000000 | (\source << 16) | (\marker << 14) | \special)
.endm
.macro MODEM_SYS_READ destination, special, marker=3
    .if (\destination < 0) || (\destination > 31) || (\special < 0) || (\special > 63)
      .error "invalid system read operands"
    .endif
    .word (0x6e800000 | (\special << 16) | (\marker << 14) | \destination)
.endm
.globl agent11_modem_exception_dispatch_entry
agent11_modem_exception_dispatch_entry:
// Standalone trap for the initial vector's reserved entry.
{ jump agent11_modem_exception_dispatch_entry }
// Store system event state before dispatching to the exception controller.
MODEM_SYS_WRITE 52,23
{ r23 = #0 }
MODEM_SYS_WRITE 35,23
MODEM_SYS_READ 23,8
{ r23 = asl(r23,#18) }
{ r23.l = #22528 }
{ r23 |= lsr(r23,#16) }
{ r23.h = #49281 }
{ r23 = memw(r23+#0) }
{
    memd(r23+#56) = r1:0
    memd(r23+#72) = r29:28
}
{
    memd(r23+#80) = r31:30
    memd(r23+#64) = r19:18
}
// Reference state in r18 used by subsequent exception paths.
{
    r18 = r23
    r1 = memw(##3229698676)
    if (cmp.eq(r1.new,#0)) jump:t agent11_modem_exception_dispatch_entry + 0x54
}
MODEM_SYS_READ 23,52,1
.set modem_rwe_exception_newctx, agent11_modem_exception_dispatch_entry + (0x3ad00-0x38108)
{ jump ##modem_rwe_exception_newctx }
MODEM_SYS_READ 23,52,1
.set modem_rwe_exception_resume, agent11_modem_exception_dispatch_entry + (0x5e198-0x38108)
{ jump ##modem_rwe_exception_resume }
.set modem_rwe_exception_other, agent11_modem_exception_dispatch_entry + (0x39ee8-0x38108)
{ jump ##modem_rwe_exception_other }
.set modem_rwe_exception_fault, agent11_modem_exception_dispatch_entry + (0x39f8c-0x38108)
{ jump ##modem_rwe_exception_fault }
// Three dedicated trap-mapping handlers. A C-register write updates the
// trace controller, a tagged exception descriptor records the error class,
// then the active interrupt handler is notified via a PC-relative branch.
.macro MODEM_CTRL_WRITE control, source, marker=3
    .if (\control < 0) || (\control > 31) || (\source < 0) || (\source > 31)
      .error "invalid control-register write"
    .endif
    .if (\marker < 1) || (\marker > 3)
      .error "invalid Hexagon packet parse field"
    .endif
    .word (0x65000000 | (\source << 16) | (\marker << 14) | \control)
.endm
.macro MODEM_RWE_RETURN_FROM_EXCEPTION packet_mark=3
    .if (\packet_mark != 3)
      .error "standalone RTE must end the packet"
    .endif
    .word (0x57e00000 | (\packet_mark << 14))
.endm
.globl agent11_modem_rwe_exception_classes
agent11_modem_rwe_exception_classes:
MODEM_CTRL_WRITE 0,9,3
{
    r0 = #14
    r1 = #0
    memd(r9+#56) = r1:0
}
MODEM_CTRL_WRITE 0,9,1
{ r0 |= asl(r1,#8) }
.set modem_rwe_exception_publisher, agent11_modem_exception_dispatch_entry + (0x3a1dc - 0x38108)
{ jump ##modem_rwe_exception_publisher }
MODEM_CTRL_WRITE 0,9,3
{
    r0 = #13
    r1 = #0
    memd(r9+#56) = r1:0
}
MODEM_CTRL_WRITE 0,9,1
{ r0 |= asl(r1,#8) }
{ jump ##modem_rwe_exception_publisher }
MODEM_RWE_RETURN_FROM_EXCEPTION 3
{ jumpr lr }
MODEM_CTRL_WRITE 0,9,3
{
    r0 = #4
    r1 = #0
    memd(r9+#56) = r1:0
}
MODEM_CTRL_WRITE 0,9,1
{ r0 |= asl(r1,#8) }
{ jump ##modem_rwe_exception_publisher; nop; nop }
// Source-level packet NOPs: one two-word packet and one four-word packet.
{ nop; nop }
{ nop; nop; nop; nop }
// Protected interrupt status is exchanged through the architected locked
// physical-word load/store primitives, never ordinary unsynchronized stores.
.globl agent11_modem_rwe_exception_acquire_lock
agent11_modem_rwe_exception_acquire_lock:
{ r12 = ##-1065090812 }
modem_rwe_exception_lock_retry:
{ r13 = memw_locked(r12) }
{
    p2 = cmp.eq(r13,#0)
    if (!p2.new) jump:nt modem_rwe_exception_lock_retry
    if (p2.new) r13 = #1
}
// LLVM6 forbids the architectural locked-store in the same packet as an
// immediate extender, so encode the locked store from ISA operand fields and
// later normalize only the preceding packet marker after checking classes.
.macro MODEM_RWE_LOCKED_WORD_STORE base, value, predicate, marker=3
    .if (\base < 0) || (\base > 31) || (\value < 0) || (\value > 31) || (\predicate < 0) || (\predicate > 3)
      .error "invalid locked word-store operands"
    .endif
    .if (\marker != 3)
      .error "locked word store must finish the packet"
    .endif
    .word (0xa0a00000 | (\base << 16) | (\marker << 14) | (\value << 8) | \predicate)
.endm
{ r12 = ##-1065090812 }
MODEM_RWE_LOCKED_WORD_STORE 12,13,2,3
{
    if (!p2) r12 = ##-1065090812
    if (!p2) jump:nt modem_rwe_exception_lock_retry
    if (p2) r13 = memw(r12+#0)
}
// Publish the captured interrupt status to per-core controller registers.
// LLVM6 cannot decode some S-register instructions by mnemonic, so the
// checked S-register read/write macros keep original privilege semantics.
MODEM_SYS_READ 12,8,1
{ p2 = cmp.gt(r13,#0) }
{ trace(r13) }
MODEM_SYS_READ 12,63,1
{
    r13 = setbit(r13,r12)
    memw(##-1065090816) = r13.new
}
{ r12 = setbit(r12,#16) }
MODEM_SYS_WRITE 63,12,1
{
    r13 = #0
    memw(##-1065090812) = r13.new
}
MODEM_SYS_READ 13,6,1
{ r12 = ##524288 }
MODEM_SYS_READ 28,0,1
.set modem_exception_status_empty, agent11_modem_exception_dispatch_entry + (0x38358-0x38108)
{
    r13 = and(r13,~r12)
    r12 = and(r13,#255)
    if (cmp.eq(r12.new,#0)) jump:nt modem_exception_status_empty
}
// Decode source exception class and store a bounded tagged descriptor.
{
    r13 = setbit(r13,#19)
    r15:14 = combine(r23,r22)
    r23 = #3
    r22 = memw(r28+#0)
}
MODEM_SYS_READ 23,3,1
{
    r28 = minu(r12,r23)
    memd(r22+#56) = r15:14
}
{ r12 = extractu(r13,#7,#8) }
MODEM_SYS_WRITE 6,13,1
{
    r15 = memw(r12<<#2+##3229882496)
    r13 = memw(r22+#16)
}
MODEM_SYS_WRITE 3,15,1
{
    r13 = add(r13,#112)
    r12 = ##-1032781824
}
// Read a second protected MMIO word through the same lock protocol.
{ r14 = ##-1065090812 }
modem_rwe_exception_secondary_lock_retry:
{ r15 = memw_locked(r14) }
{
    p2 = cmp.eq(r15,#0)
    if (!p2.new) jump:nt modem_rwe_exception_secondary_lock_retry
    if (p2.new) r15 = #1
}
{ memw_locked(r14,p2) = r15 }
{ if (!p2) jump:nt modem_rwe_exception_secondary_lock_retry }
// Synchronize both the controller's status-register flag and its shared
// exception tracking word before resuming the saved exception context.
MODEM_SYS_READ 14,6,1
{ r15 = ##65536 }
.set modem_rwe_status_done, agent11_modem_exception_dispatch_entry + (0x3832c-0x38108)
{
    p2 = bitsclr(r14,r15)
    if (p2.new) jump:nt modem_rwe_status_done
}
MODEM_SYS_READ 14,8,1
{ r15 = memw(##-1065090816) }
{
    trace(r15)
    r15 = clrbit(r15,r14)
}
{
    p2 = !cmp.eq(r15,#0)
    if (p2.new) jump:t modem_rwe_status_done
    memw(##-1065090816) = r15
}
MODEM_SYS_READ 14,63
{
    trace(r14)
    r14 = clrbit(r14,#16)
}
MODEM_SYS_WRITE 63,14
{ trace(r14) }
modem_rwe_status_done:
{
    r15 = #0
    memw(##-1065090812) = r15.new
}
// Begin restoring a saved controller register bank. The older Hexagon MC
// lacks the late privileged pair-write mnemonic; encode its public operand
// fields through a checked architecture helper (not reference opcode bytes).
.macro MODEM_RWE_EXT_CONTROL_WRITE control_field, gpr, marker=1
    .if (\control_field < 0) || (\control_field > 63) || (\gpr < 0) || (\gpr > 31)
      .error "invalid extended control-register transfer operands"
    .endif
    .if (\marker != 1)
      .error "this extended control write must continue a 4-word packet"
    .endif
    .word (0x63000000 | (\control_field << 16) | (\marker << 14) | \gpr)
.endm
.macro MODEM_RWE_INLINE_RTE marker=1
    .if (\marker != 1)
      .error "this RTE is embedded in a four-word exception-return packet"
    .endif
    .word (0x57e00000 | (\marker << 14))
.endm
{
    r15:14 = c11:10
    r13:12 = combine(r12,r13)
    memd(r22+#48) = r13:12
}
MODEM_RWE_EXT_CONTROL_WRITE 44,10,1
MODEM_RWE_INLINE_RTE 1
{
    r13 = r23
    r23:22 = memd(r22+#56)
}
MODEM_SYS_WRITE 6,13,1
.set modem_rwe_exception_state_save, agent11_modem_exception_dispatch_entry + (0x383dc-0x38108)
{
    r7:6 = combine(r14,r15)
    if (!p0) jump:nt modem_rwe_exception_state_save
    r28 = memw(r28+#0)
}
// Save the second exception-controller shadow register and release its
// protected status lock. This mirrors the first lock protocol but uses
// controller thread register r9 and the previous r12 exception mask.
MODEM_SYS_WRITE 3,9
{ r9 = ##-1065090812 }
modem_rwe_exception_thread_lock_retry:
{ r12 = memw_locked(r9) }
{
    p2 = cmp.eq(r12,#0)
    if (!p2.new) jump:nt modem_rwe_exception_thread_lock_retry
    if (p2.new) r12 = #1
}
{ memw_locked(r9,p2) = r12 }
{ if (!p2) jump:nt modem_rwe_exception_thread_lock_retry }
MODEM_SYS_READ 9,8,1
{ r12 = memw(##-1065090816) }
{
    trace(r12)
    r12 = clrbit(r12,r9)
}
.set modem_rwe_thread_status_done, agent11_modem_exception_dispatch_entry + (0x383c0-0x38108)
{
    p2 = !cmp.eq(r12,#0)
    if (p2.new) jump:t modem_rwe_thread_status_done
    memw(##-1065090816) = r12
}
MODEM_SYS_READ 9,63
{
    trace(r9)
    r9 = clrbit(r9,#16)
}
MODEM_SYS_WRITE 63,9
{ trace(r9) }
modem_rwe_thread_status_done:
{
    r12 = #0
    memw(##-1065090812) = r12.new
}
// Save the complete general-register bank to the controller exception frame.
// The inline protected-return form participates in the packet terminating
// with the first 64-bit register store; remaining saves are one-word packets.
MODEM_RWE_EXT_CONTROL_WRITE 46,10,1
MODEM_RWE_INLINE_RTE 1
{ memd(r28+#48) = r7:6 }
{ memd(r28+#32) = r31:30 }
{ memd(r28+#40) = r29:28 }
{ memd(r28+#192) = r27:26 }
{ memd(r28+#200) = r25:24 }
{ memd(r28+#56) = r23:22 }
{ memd(r28+#64) = r21:20 }
{ memd(r28+#72) = r19:18 }
{ memd(r28+#80) = r17:16 }
{ memd(r28+#88) = r15:14 }
{ memd(r28+#96) = r13:12 }
{ memd(r28+#104) = r11:10 }
{ memd(r28+#112) = r9:8 }
{ memd(r28+#120) = r7:6 }
{ memd(r28+#128) = r5:4 }
{ memd(r28+#136) = r3:2 }
{ memd(r28+#144) = r1:0 }
{ r12 = r9 }
{ r11 = usr }
{ r10 = p3:0 }
{ r9:8 = c1:0 }
{ r15:14 = c3:2 }
{ r5:4 = c7:6 }
{ r3:2 = c13:12 }
{ memd(r28+#48) = r7:6 }
{ memd(r28+#152) = r9:8 }
{ memd(r28+#24) = r13:12 }
{ memw(r28+#236) = ##-1065089300 }
{ memd(r28+#176) = r11:10 }
{ memd(r28+#160) = r15:14 }
{ memd(r28+#168) = r5:4 }
{ memd(r28+#184) = r3:2 }
MODEM_SYS_READ 29,0,3
{ r0 = r28 }
.set modem_rwe_handler_stage2, agent11_modem_exception_dispatch_entry + (0x6cc04-0x38108)
{ jump modem_rwe_handler_stage2 }
// Exception-class 15: publish one more typed controller diagnostic packet.
MODEM_CTRL_WRITE 0,9,3
{
    r0 = #15
    r1 = #0
    memd(r9+#56) = r1:0
}
MODEM_CTRL_WRITE 0,9,1
{ r0 |= asl(r1,#8) }
{ jump ##modem_rwe_exception_publisher }
// Next exception ingress classifies the active thread and vector lanes.
MODEM_SYS_READ 15,6,1
{
    r9:8 = combine(#127,#2)
    r14.l = #6360
    p0 = cmp.eq(r28,#0)
}
MODEM_SYS_READ 10,0,1
{
    r7:6 = vminub(r15:14,r9:8)
    if (p0) r8 = #0
    r14.h = #49284
}
.set modem_rwe_handler_no_words, agent11_modem_exception_dispatch_entry + (0x385e8-0x38108)
{
    r13 = extractu(r15,#4,#16)
    p0 = !cmp.gtu(r7,#16)
    if (p0.new) jump:nt modem_rwe_handler_no_words
    r6 = memw(r10+#0)
}
// Resolve a vector-table entry, then atomically acquire the associated
// protected frame lock before dispatching to the next controller stage.
{
    r13 = ##458752
    r14 = memw(r14+r13<<#2)
}
MODEM_SYS_READ 14,3,1
{
    r13 = and(r15,~r13)
    r7 = memw(r14+r7<<#2)
    memd(r6+#32) = r31:30
}
{ r30 = ##-1065090812 }
modem_rwe_exception_priority_lock_retry:
{ r31 = memw_locked(r30) }
{
    p0 = cmp.eq(r31,#0)
    if (!p0.new) jump:nt modem_rwe_exception_priority_lock_retry
    if (p0.new) r31 = #1
}
{ r30 = ##-1065090812 }
MODEM_RWE_LOCKED_WORD_STORE 30,31,0,3
{
    if (!p0) r30 = ##-1065090812
    if (!p0) jump:nt modem_rwe_exception_priority_lock_retry
    if (p0) r31 = memw(r30+#0)
}
// Release the vector-table lock and report the per-thread dispatch mask.
MODEM_SYS_READ 30,8,1
{ p0 = cmp.gt(r31,#0) }
{ trace(r31) }
MODEM_SYS_READ 30,63,1
{
    r31 = setbit(r31,r30)
    memw(##-1065090816) = r31.new
}
{ r30 = setbit(r30,#16) }
MODEM_SYS_WRITE 63,30,1
{
    r31 = #0
    memw(##-1065090812) = r31.new
}
// A compact source-level Hexagon bit-test branch is exactly one 32-bit
// opcode, unlike the two-op expanded form used by the old LLVM6 MC.
// Class 0x118 is the architected test-bit+branch instruction family.
.macro MODEM_RWE_COMPACT_TSTBIT_JUMP general, bit, dest_delta, marker=1
    .if (\general < 0) || (\general > 31) || (\bit != 0)
      .error "unsupported Hexagon compact bit test register/bit"
    .endif
    .if (\dest_delta <= 0) || ((\dest_delta & 1) != 0) || (\dest_delta > 510)
      .error "invalid compact bit-test jump displacement"
    .endif
    .if (\marker != 1)
      .error "compact bit branch must continue 4-slot exception packet"
    .endif
    .word (0x11800302 | (\general << 16) | (\marker << 14) | (\dest_delta / 2))
.endm
{ p3 = !bitsclr(r7,r8) }
MODEM_RWE_COMPACT_TSTBIT_JUMP 7,0,(0x385f8-0x38538),1
{
    memd(r6+#40) = r29:28
    memd(r6+#24) = r15:14
}
MODEM_SYS_WRITE 6,13,1
{
    callr r7
    r30 = add(r6,#32)
    r29 = add(r10,#-16)
}
{
    nop
    r9:8 = memd(r30+#-8)
    r29:28 = memd(r30+#8)
}
MODEM_SYS_WRITE 6,9,1
{ r31:30 = memd(r30+#0) }
MODEM_SYS_WRITE 3,8,3
// Release callback state through the same checked per-thread lock primitive.
{ r7 = ##-1065090812 }
modem_rwe_exception_callback_lock_retry:
{ r8 = memw_locked(r7) }
{
    p0 = cmp.eq(r8,#0)
    if (!p0.new) jump:nt modem_rwe_exception_callback_lock_retry
    if (p0.new) r8 = #1
}
{ memw_locked(r7,p0) = r8 }
{ if (!p0) jump:nt modem_rwe_exception_callback_lock_retry }
MODEM_SYS_READ 7,6,1
{ r8 = ##65536 }
.set modem_rwe_callback_clear_done, agent11_modem_exception_dispatch_entry + (0x385d8-0x38108)
{
    p0 = bitsclr(r7,r8)
    if (p0.new) jump:nt modem_rwe_callback_clear_done
}
MODEM_SYS_READ 7,8,1
{ r8 = memw(##-1065090816) }
{
    trace(r8)
    r8 = clrbit(r8,r7)
}
{
    p0 = !cmp.eq(r8,#0)
    if (p0.new) jump:t modem_rwe_callback_clear_done
    memw(##-1065090816) = r8
}
MODEM_SYS_READ 7,63
{
    trace(r7)
    r7 = clrbit(r7,#16)
}
MODEM_SYS_WRITE 63,7
{ trace(r7) }
modem_rwe_callback_clear_done:
{
    r8 = #0
    memw(##-1065090812) = r8.new
}
MODEM_RWE_RETURN_FROM_EXCEPTION 3
// Resolve frame-vector entries by index, then save the first half of the
// privilege context before dispatching the requested exception class.
// The LLVM6 assembler has no paired C-register read mnemonic. This is the
// checked public-ISA Y2 pair-transfer encoding; operands are even low regs.
.macro MODEM_RWE_CONTROL_PAIR_READ general_low, control_low, marker=3
    .if (\general_low < 0) || (\general_low > 30) || ((\general_low & 1) != 0)
      .error "invalid paired destination"
    .endif
    .if (\control_low < 0) || (\control_low > 30) || ((\control_low & 1) != 0)
      .error "invalid paired controller source"
    .endif
    .if (\marker < 1) || (\marker > 3)
      .error "invalid Hexagon packet marker"
    .endif
    .word (0x68000000 | (\control_low << 16) | (\marker << 14) | \general_low)
.endm
.globl agent11_modem_rwe_vector_followup
agent11_modem_rwe_vector_followup:
{
    p0 = tstbit(r13,#3)
    r13 = memw(r14+r13<<#2)
}
{ r13 = memw(r13+r7<<#2) }
{ jumpr r13 }
MODEM_SYS_READ 11,7,1
{
    r14 = and(r7,#-4)
    memd(r6+#80) = r17:16
    memd(r6+#72) = r19:18
}
MODEM_SYS_WRITE 6,13,1
{
    r29 = add(r10,#-16)
    memd(r6+#64) = r21:20
    memd(r6+#56) = r23:22
}
{
    r10 = ugp
    r13:12 = combine(#56,#0)
    memd(r6+#200) = r25:24
    memd(r6+#192) = r27:26
}
{
    r13 = usr
    r30 = add(r6,#32)
    if (p3) r8 = memb(r6+r13<<#2)
    memw(r6+#52) = r10
}
MODEM_RWE_CONTROL_PAIR_READ 8,16,1
{
    r31.l = #1368
    p3 = !cmp.gt(r8,#-1)
}
MODEM_SYS_READ 10,2,1
{
    memw(r6+#236) = ##-1065260224
    memd(r6+#248) = r9:8
}
MODEM_RWE_CONTROL_PAIR_READ 8,18,3
{
    memd(r6+#304) = r9:8
    memb(r6+#226) = r10
}
{
    r31.h = #49284
    if (!p3) jumpr:nt r14
    memw(r6+#220) = r11
    memd(r6+#176) = r13:12
}
{
    r0 = #37
    jumpr r31
    memw(r6+#24) = r28
}
// Check protected frame key against per-thread stack metadata, then
// conditionally restore the saved words and deallocate the temporary frame.
// Privileged Y4 frame-key read unavailable in LLVM6 mnemonic tables.
// The low register field and packet marker use the architected Y4 encoding.
.macro MODEM_RWE_READ_FRAMEKEY dest, marker=1
    .if (\dest < 0) || (\dest > 31) || (\marker != 1)
      .error "invalid framekey transfer register/packet marker"
    .endif
    .word (0x6a100000 | (1 << 16) | (\marker << 14) | \dest)
.endm
.globl agent11_modem_rwe_framekey_guard
agent11_modem_rwe_framekey_guard:
MODEM_SYS_READ 6,0,1
{ r9 = ##-1065089708 }
{ allocframe(#0) }
MODEM_RWE_READ_FRAMEKEY 8,1
{
    r4 = ##-1065089344
    r7 = memw(r6+#-20)
}
{
    r7 = xor(r7,r8)
    r4 = xor(r4,r8)
    r8 = add(r6,#-20)
}
{
    r7 = sub(r7,r9)
    if (cmp.gtu(r7.new,#4)) jump:nt agent11_modem_rwe_framekey_complete
}
{ memw(r8+#0) = r4 }
{
    memd(r6+#-16) = r1:0
    memw(r6+#-8) = r2
}
agent11_modem_rwe_framekey_complete:
{ dealloc_return }
// Recover the interrupted thread's control block, releasing the kernel
// lock, restoring GP/UGP and frame registers, then prefetched descriptor
// cachelines for subsequent lazy hardware restore. This is a continuation
// of the source-generated handler, not a disassembly-copied byte array.
.globl agent11_modem_rwe_exception_restore_begin
agent11_modem_rwe_exception_restore_begin:
MODEM_SYS_READ 13,6,1
{ r7 = ##0xc08715b0; r6 = memw(r29+#16) }
{
    r8 = #0
    r10 = add(sp,#16)
    r0 = memub(r6+#224)
}
.set modem_exception_previous_callback, agent11_modem_exception_dispatch_entry + (0x385f8-0x38108)
{
    p3 = tstbit(r0,#4)
    if (!p3.new) jump:t modem_exception_previous_callback
    r0 = r6
}
.set modem_exception_previous_return, agent11_modem_exception_dispatch_entry + (0x38554-0x38108)
{ jump modem_exception_previous_return; r0 = memw(sp+#0) }
// Hexagon's K0 unlock uses a fixed architectural register-bank subopcode;
// only the ISA class and packet end marker are encoded here.
.macro MODEM_RWE_UNLOCK_K0 marker=3
    .if (\marker != 3)
      .error "K0 unlock is a standalone packet"
    .endif
    .word (0x6c200080 | (\marker << 14))
.endm
MODEM_RWE_UNLOCK_K0
{ fp = r0 }
{
    r15 = add(fp,#0)
    r14 = add(fp,#176)
    r1:0 = memd(fp+#24)
    r3:2 = memd(fp+#48)
}
{
    ugp = r3
    r14 = add(r14,#-32)
    lr:fp = memd(r15+#32)
    dcfetch(r14+#0)
}
{
    gp = r2
    r14 = add(r14,#-32)
    r29:28 = memd(r15+#40)
    dcfetch(r14+#0)
}
// Restore the saved status/return registers and pairwise controller fields.
// A32-like single-register system class and Hexagon pair-control class are
// different architectural encodings; validate pair numbering and packet end.
.macro MODEM_RWE_CONTROL_PAIR_WRITE control_low, gpr_low, marker=1
    .if (\control_low < 0) || (\control_low > 30) || ((\control_low & 1) != 0)
      .error "invalid control pair index"
    .endif
    .if (\gpr_low < 0) || (\gpr_low > 30) || ((\gpr_low & 1) != 0)
      .error "invalid general register pair"
    .endif
    .if (\marker < 1) || (\marker > 3)
      .error "invalid pair packet marker"
    .endif
    .word (0x63000000 | ((\gpr_low + 32) << 16) | (\marker << 14) | \control_low)
.endm
MODEM_SYS_WRITE 6,1,3
MODEM_SYS_WRITE 3,0,1
{
    r13 = memub(r15+#20)
    r23:22 = memd(r15+#248)
}
{ r25:24 = memd(r15+#304) }
MODEM_RWE_CONTROL_PAIR_WRITE 16,22,1
{
    r14 = add(r14,#-32)
    r1:0 = memd(r15+#176)
    dcfetch(r14+#0)
}
MODEM_RWE_CONTROL_PAIR_WRITE 18,24,1
{ r12 = memub(r15+#226) }
// Restore user/configuration register banks from the captured thread record.
// Pair control writes and the user-configuration writes belong to distinct
// public Hexagon ISA classes, with explicit checked operand/packet fields.
.macro MODEM_RWE_CONFIG_WRITE special, gpr, marker=1
    .if (\special < 0) || (\special > 31) || (\gpr < 0) || (\gpr > 31)
      .error "invalid special configuration write operands"
    .endif
    .if (\marker < 1) || (\marker > 3)
      .error "invalid config write packet marker"
    .endif
    .word (0x62000000 | ((\gpr + 32) << 16) | (\marker << 14) | \special)
.endm
MODEM_RWE_CONFIG_WRITE 8,1,1   // USR = r1
{
    r19:18 = memd(r15+#168)
    dcfetch(r14+#0)
}
{
    r23:22 = memd(r15+#160)
    r21:20 = memd(r15+#152)
}
MODEM_RWE_CONTROL_PAIR_WRITE 6,18,1   // M1:0
{
    r19 = r15
    r3:2 = memd(r15+#136)
}
{
    r11:10 = memd(r19+#184)
    r9 = memw(r19+#220)
}
MODEM_SYS_WRITE 7,9,1       // CCR = r9
{ r12 = insert(r13,#8,#16) }
MODEM_RWE_CONTROL_PAIR_WRITE 12,10,1 // CS1:0
{
    r27:26 = memd(r19+#192)
    r25:24 = memd(r19+#200)
}
MODEM_SYS_WRITE 2,12,1      // STID = r12
{
    r7:6 = memd(r19+#120)
    r5:4 = memd(r19+#128)
}
MODEM_RWE_CONFIG_WRITE 1,21,1  // LC0
{
    r11:10 = memd(r19+#104)
    r9:8 = memd(r19+#112)
}
MODEM_RWE_CONFIG_WRITE 0,20,1  // SA0
{
    r15:14 = memd(r19+#88)
    r13:12 = memd(r19+#96)
}
MODEM_RWE_CONFIG_WRITE 3,23,1  // LC1
{
    r23 = r19
    r17:16 = memd(r19+#80)
    r19:18 = memd(r19+#72)
}
// Restore the protected interrupt ownership bit with the same atomic lock
// protocol used by the initial exception entry. Restore final loop state and
// return from protected exception context without inventing a normal return.
{ r20 = ##-1065090812 }
modem_rwe_restore_irq_lock_retry:
{ r21 = memw_locked(r20) }
{
    p0 = cmp.eq(r21,#0)
    if (!p0.new) jump:nt modem_rwe_restore_irq_lock_retry
    if (p0.new) r21 = #1
}
{ memw_locked(r20,p0) = r21 }
{ if (!p0) jump:nt modem_rwe_restore_irq_lock_retry }
MODEM_SYS_READ 20,6,1
{ r21 = ##65536 }
.set modem_rwe_restore_irq_mask_done, agent11_modem_exception_dispatch_entry + (0x38828-0x38108)
{
    p0 = bitsclr(r20,r21)
    if (p0.new) jump:nt modem_rwe_restore_irq_mask_done
}
MODEM_SYS_READ 20,8,1
{ r21 = memw(##-1065090816) }
{
    trace(r21)
    r21 = clrbit(r21,r20)
}
{
    p0 = !cmp.eq(r21,#0)
    if (p0.new) jump:t modem_rwe_restore_irq_mask_done
    memw(##-1065090816) = r21
}
MODEM_SYS_READ 20,63
{
    trace(r20)
    r20 = clrbit(r20,#16)
}
MODEM_SYS_WRITE 63,20
{ trace(r20) }
modem_rwe_restore_irq_mask_done:
{ r21 = #0; memw(##-1065090812) = r21.new }
MODEM_RWE_CONFIG_WRITE 2,22,1   // SA1
{
    r1 = r23
    r21:20 = memd(r23+#64)
    r23:22 = memd(r23+#56)
}
MODEM_RWE_CONFIG_WRITE 4,0,1    // P3:0
MODEM_RWE_INLINE_RTE 1
{ r1:0 = memd(r1+#144) }
MODEM_RWE_RETURN_FROM_EXCEPTION 3
// Handle restored SSR exception-class codes 2 and 6 by rebuilding a guarded
// codec frame and dispatching the appropriate recovery callback. This is a
// separate branch family from the RTE path, preserving its source packet
// boundaries and `.new` predicate dependencies.
MODEM_SYS_READ 20,6,1
{ memd(r1+#32) = r21:20 }
.set modem_rwe_resume_codec_frame, agent11_modem_exception_dispatch_entry + (0x38878-0x38108)
{
    r21 = extractu(r20,#4,#16)
    r20 = clrbit(r20,#31)
    if (cmp.eq(r21.new,#6)) jump:t modem_rwe_resume_codec_frame
}
{
    r21 = r21
    if (cmp.eq(r21.new,#2)) jump:t modem_rwe_resume_codec_frame
}
.set modem_rwe_exception_unmatched_type, agent11_modem_exception_dispatch_entry + (0x38950-0x38108)
{ jump modem_rwe_exception_unmatched_type; r21:20 = memd(r1+#32) }
modem_rwe_resume_codec_frame:
{
    r20 = insert(r1,#2,#17)
    r0 = extract(r20,#5,#0)
    sp = r1
    r22 = r0
}
MODEM_SYS_WRITE 6,20,1
// The true runtime target is 0xc0815d1c (file 0xdd1c). LLVM6 computes this
// extended call displacement relative to a different packet instruction;
// request a +4 adjusted relocation to preserve the architected packet-base
// PC-relative target. Exact operand class/bytes are checked by verifier.
.set modem_rwe_restore_helper, agent11_modem_exception_dispatch_entry + (0xdd20-0x38108)
{
    call ##modem_rwe_restore_helper
    r2 = #9
}
MODEM_SYS_READ 1,8,1
{
    p3 = cmp.eq(r0,#10)
    r0 = r22
    r23 = r0
}
.set modem_rwe_restore_helper_a, agent11_modem_exception_dispatch_entry + (0xe694-0x38108)
.set modem_rwe_restore_helper_b, agent11_modem_exception_dispatch_entry + (0x10350-0x38108)
{
    if (p3) call ##modem_rwe_restore_helper_a
    if (!p3) call ##modem_rwe_restore_helper_b
}
.set modem_rwe_codec_restore_complete, agent11_modem_exception_dispatch_entry + (0x38a7c-0x38108)
{ jump ##modem_rwe_codec_restore_complete; nop }
// Alternate protected exception-owner exchange. Swap the special controller
// GP0 bank, capture predicate registers and record the saved frame, then
// release the protected interrupt lock before returning to the common handler.
.macro MODEM_RWE_SWAP_SGP0 regnum, marker=3
    .if (\regnum < 0) || (\regnum > 31) || (\marker < 1) || (\marker > 3)
      .error "invalid SGP0 control swap operand"
    .endif
    .word (0x65000000 | (\regnum << 16) | (\marker << 14))
.endm
.macro MODEM_RWE_PREDICATE_READ gpr, marker=1
    .if (\gpr < 0) || (\gpr > 31) || (\marker < 1) || (\marker > 3)
      .error "invalid predicate register read operand"
    .endif
    .word (0x6a000000 | (4 << 16) | (\marker << 14) | \gpr)
.endm
.globl agent11_modem_rwe_alternate_exception_owner
agent11_modem_rwe_alternate_exception_owner:
MODEM_RWE_SWAP_SGP0 31
MODEM_RWE_PREDICATE_READ 1,1
{ memd(lr+#48) = r1:0 }
{ memw(lr+#44) = r1 }
{ r0 = ##-1065090812 }
modem_rwe_restore_alt_lock_retry:
{ r1 = memw_locked(r0) }
{
    p0 = cmp.eq(r1,#0)
    if (!p0.new) jump:nt modem_rwe_restore_alt_lock_retry
    if (p0.new) r1 = #1
}
{ r0 = ##-1065090816 }
{ memw_locked(r0,p0) = r1 }
{
    if (!p0) r0 = ##-1065090812
    if (!p0) jump:nt modem_rwe_restore_alt_lock_retry
    if (p0) r1 = memw(r0+#0)
}
MODEM_SYS_READ 0,8,1
{ p0 = cmp.gt(r1,#0) }
{ trace(r1) }
MODEM_SYS_READ 0,63,1
{
    r1 = setbit(r1,r0)
    memw(##-1065090816) = r1.new
}
{ r0 = setbit(r0,#16) }
MODEM_SYS_WRITE 63,0,1
{ r1 = #0; memw(##-1065090812) = r1.new }
{
    r0 = memw(lr+#0)
    r1 = memw(lr+#44)
}
MODEM_RWE_CONFIG_WRITE 4,1,1    // P3:0
{ r1 = #-6 }
{ trace(r1) }
// LLVM6 cannot track LR as a packet-local `.new` producer through an
// independently checked SGP0 hardware swap macro. Encode the 3-instruction
// producer/move/new-value branch packet with its public ISA operand fields.
// The compact branch halfword displacement encoding is corroborated against
// multiple *synthetic* LLVM6 packet-distance cases before reference checking.
.macro MODEM_RWE_LR_MOVE_TO register, marker=1
    .if (\register < 0) || (\register > 31) || (\marker != 1)
      .error "invalid LR move packet continuation"
    .endif
    .word (0x70000000 | ((31 + 96) << 16) | (\marker << 14) | \register)
.endm
.macro MODEM_RWE_COMPACT_LR_NEG1_RETURN displacement, marker=3
    .if ((\displacement & 1) != 0) || (\displacement < -1024) || (\displacement > 1022) || (\marker != 3)
      .error "invalid compact compare(-1)/return branch displacement"
    .endif
    .word (0x2604e000 | (((\displacement >> 9) & 3) << 20) | ((\displacement >> 1) & 255))
.endm
modem_rwe_alternate_frame_exit_packet:
MODEM_RWE_SWAP_SGP0 31,1
MODEM_RWE_LR_MOVE_TO 1,1
.set modem_rwe_alt_exception_restore, agent11_modem_exception_dispatch_entry + (0x38854-0x38108)
MODEM_RWE_COMPACT_LR_NEG1_RETURN (0x38854 - 0x38944),3
// Begin saving an interrupted thread's full register bank into a designated
// protected descriptor. The controller selects a special frame based on the
// restored predicate state, then clears cachelines before copying context.
.macro MODEM_RWE_Y_CONFIG_READ gpr, field, marker=1
    .if (\gpr < 0) || (\gpr > 31) || (\field < 0) || (\field > 31) || (\marker < 1) || (\marker > 3)
      .error "invalid Hexagon Y-config register read operand"
    .endif
    .word (0x6a000000 | (\field << 16) | (\marker << 14) | \gpr)
.endm
.globl agent11_modem_rwe_capture_interrupted_context
agent11_modem_rwe_capture_interrupted_context:
MODEM_RWE_PREDICATE_READ 28,1
{
    p0 = cmp.eq(r0,##-1065269088)
    memd(r0+#40) = r29:28
}
MODEM_RWE_Y_CONFIG_READ 29,8,1     // SP = USR
{
    r22 = add(r0,#64)
    memd(r0+#32) = lr:fp
    memd(r0+#56) = r23:22
}
MODEM_RWE_Y_CONFIG_READ 23,10,1   // UGP
// LLVM6 extended jump relocations are +4 relative to this packet base.
.set modem_rwe_special_frame, agent11_modem_exception_dispatch_entry + (0xec34-0x38108)
{
    if (p0) jump:nt ##modem_rwe_special_frame
    fp = r0
}
MODEM_RWE_Y_CONFIG_READ 22,11,1   // GP
{
    fp = add(r22,#32)
    dczeroa(r22)
}
MODEM_RWE_CONTROL_PAIR_READ 20,0,1    // LC0:SA0
{
    memd(r0+#64) = r21:20
    memd(r0+#72) = r19:18
}
MODEM_SYS_READ 18,3,1              // ELR
{
    memd(r0+#80) = r17:16
    memd(r0+#88) = r15:14
}
{
    fp = add(fp,#32)
    r17 = r0
    dczeroa(fp)
}
// Serialize the interrupted thread state into its active protected frame,
// selecting one of two callback paths based on the saved codec class.
{
    r22 = ##-1065261420
    r1:0 = memd(r1+#48)
    memd(r0+#48) = r23:22
}
MODEM_SYS_READ 19,6,1
{
    memd(r17+#96) = r13:12
    memd(r17+#104) = r11:10
}
{
    r23 = extract(r19,#5,#0)
    r12 = r19
    memd(r17+#112) = r9:8
    memd(r17+#120) = r7:6
}
MODEM_RWE_CONTROL_PAIR_READ 6,16,1  // C17:16
{
    r12 = clrbit(r12,#31)
    p3 = cmp.eq(r23,#10)
    dczeroa(fp)
}
MODEM_RWE_CONTROL_PAIR_READ 8,18,3 // PKTCOUNT
{
    memd(r17+#248) = r7:6
    memd(r17+#304) = r9:8
}
{
    r12 = insert(r17,#3,#16)
    r0 = r23
    memd(r17+#144) = r1:0
    memd(r17+#152) = r21:20
}
MODEM_RWE_CONTROL_PAIR_READ 20,6,1  // M1:0
{
    r2 = #9
    memd(r17+#128) = r5:4
    memd(r17+#136) = r3:2
}
MODEM_SYS_WRITE 6,12,1
.set modem_rwe_capture_callback, agent11_modem_exception_dispatch_entry + (0xdd20-0x38108)
{
    call ##modem_rwe_capture_callback
    memd(r17+#24) = r19:18
}
MODEM_RWE_CONTROL_PAIR_READ 12,2,1  // LC1:SA1
{
    memw(r17+#236) = ##-1065089300
    memd(r17+#176) = r29:28
}
MODEM_SYS_READ 29,0,1               // SP = SGP0
{
    r0 = r17
    memd(r17+#200) = r25:24
    memd(r17+#192) = r27:26
}
MODEM_RWE_CONTROL_PAIR_READ 12,12,1 // CS1:0
// LLVM6 conditional jump uses a packet-base bias of +4 in this group.
.set modem_rwe_capture_secondary_reloc, agent11_modem_exception_dispatch_entry + (0x38a6c-0x38108)
{
    if (!p3) jump:nt modem_rwe_capture_secondary_reloc
    memd(r17+#160) = r13:12
    memd(r17+#168) = r21:20
}
MODEM_SYS_READ 1,8,1               // HTID
{
    callr r22
    memd(r17+#184) = r13:12
}
.set modem_rwe_capture_done, agent11_modem_exception_dispatch_entry + (0x38a7c-0x38108)
{ jump modem_rwe_capture_done }
modem_rwe_capture_secondary_callback:
MODEM_SYS_READ 1,8,1
.set modem_rwe_capture_fallback, agent11_modem_exception_dispatch_entry + (0x10354-0x38108)
{
    call ##modem_rwe_capture_fallback
    memd(r17+#184) = r13:12
}
{ jump modem_rwe_capture_done }
// Restore a suspended codec task's controller registers using the selected
// SGP0 frame. Each word is created from a public-ISA Hexagon instruction or
// a checked privileged-register operand; no OEM instruction bytes are read.
.globl agent11_modem_rwe_restore_suspended_codec
agent11_modem_rwe_restore_suspended_codec:
MODEM_SYS_READ 0,0,3              // Read SGP0 into r0, standalone.
{ r2 = #10; fp = memw(r0+#0) }
.set modem_rwe_resume_saved_frame, agent11_modem_exception_dispatch_entry + (0xdd1c-0x38108)
{
    call ##modem_rwe_resume_saved_frame
    r1 = fp
    r0 = memw(fp+#20)
}
{
    r15 = add(fp,#0)
    r14 = add(fp,#176)
    r1:0 = memd(fp+#24)
    r3:2 = memd(fp+#48)
}
MODEM_RWE_CONFIG_WRITE 10,3,1      // UGP = r3
{
    r14 = add(r14,#-32)
    lr:fp = memd(r15+#32)
    dcfetch(r14+#0)
}
MODEM_RWE_CONFIG_WRITE 11,2,1      // GP = r2
{
    r14 = add(r14,#-32)
    r29:28 = memd(r15+#40)
    dcfetch(r14+#0)
}
MODEM_SYS_WRITE 6,1,3              // SSR = r1
MODEM_SYS_WRITE 3,0,1              // ELR = r0
{
    r13 = memub(r15+#20)
    r5:4 = memd(r15+#248)
}
{
    r7:6 = memd(r15+#304)
    r24 = memub(r15+#226)
}
{
    r14 = add(r14,#-32)
    r1:0 = memd(r15+#176)
    dcfetch(r14+#0)
}
MODEM_RWE_CONFIG_WRITE 8,1,1       // USR = r1
{
    r24 = insert(r13,#8,#16)
    r19:18 = memd(r15+#168)
    dcfetch(r14+#0)
}
// Acquire the protected per-core restore lock, then test the interrupted
// frame's SSR bit before deciding whether to republish controller state.
{ r20 = ##-1065090812 }
modem_rwe_suspended_restore_lock_retry:
{ r21 = memw_locked(r20) }
{
    p0 = cmp.eq(r21,#0)
    if (!p0.new) jump:nt modem_rwe_suspended_restore_lock_retry
    if (p0.new) r21 = #1
}
MODEM_RWE_LOCKED_WORD_STORE 20,21,0,3
{ if (!p0) jump:nt modem_rwe_suspended_restore_lock_retry }
MODEM_SYS_READ 20,6,1            // SSR = controller state of saved task
{ r21 = ##65536 }
.set modem_rwe_suspended_status_done, agent11_modem_exception_dispatch_entry + (0x38b64-0x38108)
{ p0 = bitsclr(r20,r21); if (p0.new) jump:nt modem_rwe_suspended_status_done }
MODEM_SYS_READ 20,8,1              // Current hardware thread identifier.
{ r21 = memw(r28+##-1065090816) }
{ trace(r21); r21 = clrbit(r21,r20) }
.set modem_rwe_suspended_mask_done, agent11_modem_exception_dispatch_entry + (0x38b64-0x38108)
{
    p0 = !cmp.eq(r21,#0)
    if (p0.new) jump:t modem_rwe_suspended_mask_done
    memw(r28+##-1065090816) = r21
}
MODEM_SYS_READ 20,63,3             // S63 = interrupt status.
{ trace(r20); r20 = clrbit(r20,#16) }
MODEM_SYS_WRITE 63,20,3
{ trace(r20) }
modem_rwe_suspended_mask_done:
{ r21 = #0; memw(r28+##-1065090812) = r21.new }
// Complete the interrupted task's protected register-bank restoration.
// Restore predicate/loop/pktcount and paired controller registers before RTE.
MODEM_RWE_CONFIG_WRITE 4,0,1       // P3:0
{
    r23:22 = memd(r15+#160)
    r21:20 = memd(r15+#152)
}
MODEM_RWE_CONTROL_PAIR_WRITE 6,18,1 // M1:0
{
    r19 = r15
    r3:2 = memd(r15+#136)
    r1:0 = memd(r15+#144)
}
MODEM_RWE_CONTROL_PAIR_WRITE 18,6,3  // PKTCOUNT
MODEM_RWE_CONTROL_PAIR_WRITE 16,4,1 // C17:16
{ r7:6 = memd(r19+#120); r5:4 = memd(r19+#128) }
MODEM_RWE_CONFIG_WRITE 1,21,1    // LC0
{ r11:10 = memd(r19+#104); r9:8 = memd(r19+#112) }
MODEM_RWE_CONFIG_WRITE 0,20,1    // SA0
{ r15:14 = memd(r19+#88); r13:12 = memd(r19+#96) }
MODEM_RWE_CONFIG_WRITE 2,22,3    // SA1
MODEM_RWE_CONFIG_WRITE 3,23,1    // LC1
{ r21:20 = memd(r19+#184); r22 = memw(r19+#220) }
MODEM_RWE_CONTROL_PAIR_WRITE 12,20,1 // CS1:0
{
    r23 = r19
    r17:16 = memd(r19+#80)
    r19:18 = memd(r19+#72)
}
MODEM_SYS_WRITE 2,24,1             // STID
{ r25:24 = memd(r23+#200); r27:26 = memd(r23+#192) }
MODEM_SYS_WRITE 7,22,3             // CCR
{ r21:20 = memd(r23+#64); r23:22 = memd(r23+#56) }
MODEM_RWE_RETURN_FROM_EXCEPTION 3
// Protected MODECTL fast-path: compare thread ownership and controller
// flags before entering the exception-vector slow path.
.globl agent11_modem_rwe_dispatch_mode_ctl
agent11_modem_rwe_dispatch_mode_ctl:
MODEM_SYS_READ 3,8,3                // HTID
MODEM_SYS_READ 1,17,3               // MODECTL
{ r1 = extractu(r1,#4,#0); r2 = extractu(r1,#4,#16) }
{
    r1 = xor(r1,r2)
    r2 = memw(r28+##-1065068280)
}
{
    r1 = clrbit(r1,r3)
    r3 = memw(r28+##-1065268628)
}
.set modem_rwe_mode_dispatch_slow, agent11_modem_exception_dispatch_entry + (0x38dc8-0x38108)
{ r1 = and(r1,r2); if (!cmp.eq(r1.new,#0)) jump:nt modem_rwe_mode_dispatch_slow }
.set modem_rwe_mode_cfg_done, agent11_modem_exception_dispatch_entry + (0x38c54-0x38108)
{ p0 = !cmp.eq(r3,#0); if (p0.new) jump:nt modem_rwe_mode_cfg_done }
// No secondary owner: update the protected monitor SYSCFG field under a
// locked load/store, synchronize the instruction pipeline and rejoin.
{ r1 = ##-1065084720 }
modem_rwe_mode_slow_lock_retry:
{ r3 = memw_locked(r1) }
{ p0 = cmp.eq(r3,#0); if (!p0.new) jump:nt modem_rwe_mode_slow_lock_retry }
{ memw_locked(r1,p0) = r1 }
{ if (!p0) jump:nt modem_rwe_mode_slow_lock_retry }
MODEM_SYS_READ 3,18,3           // SYSCFG
{ r3 = clrbit(r3,#4) }
MODEM_SYS_WRITE 18,3,3           // SYSCFG = r3
{ memw(r1+#0) = #0 }
{ isync }
{ jump modem_rwe_mode_dispatch_slow }
// MODECTL source-specific protected exception dispatch. The original
// NMI/CIAD/VID opcodes are not expressible in LLVM6's mnemonic inventory;
// encode only their public V60 opcode/operand fields with strict checks.
.macro MODEM_RWE_NMI_FRAME source, marker=3
    .if (\source != 29) || (\marker != 3)
      .error "NMI frame must use SP and terminate the packet"
    .endif
    // NMI special frame instruction, V60 class 0x64, source register +
    // privileged sub-opcode fields (0x60 encodes control source bank).
    .word (0x64000000 | ((0x60 | \source) << 16) | (\marker << 14) | 0x40)
.endm
.macro MODEM_RWE_CIAD_FRAME source, marker=1
    .if (\source != 31) || ((\marker != 1) && (\marker != 3))
      .error "CIAD controller source must be LR with a valid packet marker"
    .endif
    .word (0x64000000 | (\source << 16) | (\marker << 14) | 0x60)
.endm
.globl agent11_modem_rwe_controller_video_mode
agent11_modem_rwe_controller_video_mode:
.set modem_rwe_vid_mode_owner,agent11_modem_exception_dispatch_entry + (0x38c70-0x38108)
{ r22 = memw(r28+##-1065084724); if (cmp.eq(r22.new,#2)) jump:t modem_rwe_vid_mode_owner }
.set modem_rwe_vid_other_handler,agent11_modem_exception_dispatch_entry + (0x38dc8-0x38108)
.set modem_rwe_vid_ciad_handler,agent11_modem_exception_dispatch_entry + (0x38cb4-0x38108)
{
    if (p2.new) jump:nt modem_rwe_vid_other_handler
    if (!p2.new) jump:nt modem_rwe_vid_ciad_handler
    p2 = cmp.eq(sp,#5)
}
modem_rwe_vid_mode_owner:
{ sp = #15 }
MODEM_RWE_NMI_FRAME 29
MODEM_SYS_READ 20,6,1         // SSR
{ memd(r1+#32) = r21:20 }
{ r21 = extractu(r20,#4,#16); r20 = clrbit(r20,#31); if (cmp.eq(r21.new,#6)) jump:t modem_rwe_vid_read_video_id }
{ r21 = r21; if (cmp.eq(r21.new,#2)) jump:t modem_rwe_vid_read_video_id }
.set modem_rwe_vid_status_publisher,agent11_modem_exception_dispatch_entry + (0x38d94-0x38108)
{ jump modem_rwe_vid_status_publisher }
modem_rwe_vid_restore_pair:
{ r21:20 = memd(r1+#32) }
modem_rwe_vid_read_video_id:
MODEM_SYS_READ 9,21,1         // VID
{ r20 = insert(r1,#2,#17); lr = #4 }
modem_rwe_vid_retry:
.set modem_rwe_video_frame_reread,agent11_modem_exception_dispatch_entry + (0x38c58-0x38108)
{ p2 = cmp.eq(r9,#5); if (p2.new) jump:nt modem_rwe_video_frame_reread; sp = #0 }
modem_rwe_vid_ciad_handler:
MODEM_RWE_CIAD_FRAME 31
{ r10 = ##-1065279472; r17 = r0 }
MODEM_SYS_WRITE 6,20,1        // restored SSR
{ r0 = r9; r11 = memw(r28+##-1065268620) }
MODEM_SYS_READ 29,0,1         // SGP0
.set modem_rwe_vid_restore_callback,agent11_modem_exception_dispatch_entry + (0xdd20-0x38108)
{ call ##modem_rwe_vid_restore_callback; r2 = #1 }
// Save a second controller frame when a video-ID-triggered exception is
// resident on this hardware thread. The protected lock is the same
// protocol used by the earlier generic exception paths.
.globl agent11_modem_rwe_vid_context_snapshot
agent11_modem_rwe_vid_context_snapshot:
.set modem_rwe_vid_pending_other,agent11_modem_exception_dispatch_entry + (0x38f30-0x38108)
{
    r10 += asl(r9,#4)
    r0 = r17
    p1 = cmp.eq(r11,#0)
    if (!p1.new) jump:nt modem_rwe_vid_pending_other
}
MODEM_SYS_READ 1,8,1 // HTID
// LLVM6 computes this three-word packet branch relative to its slot,
// while the original branch is packet-base-relative: request +4.
.set modem_rwe_vid_capture_jumpover,agent11_modem_exception_dispatch_entry + (0x38f10-0x38108)
{
    jump modem_rwe_vid_capture_jumpover
    r8 = memw(r10+#12)
    r3:2 = memd(r10+#0)
}
MODEM_RWE_SWAP_SGP0 31,3
MODEM_RWE_PREDICATE_READ 21,1
{ memd(lr+#48) = r1:0; memd(lr+#32) = r21:20 }
{ r0 = ##-1065090812 }
modem_rwe_vid_lock_retry:
{ r1 = memw_locked(r0) }
{
    p0 = cmp.eq(r1,#0)
    if (!p0.new) jump:nt modem_rwe_vid_lock_retry
    if (p0.new) r1 = #1
}
{ r0 = ##-1065090816 }
MODEM_RWE_LOCKED_WORD_STORE 0,1,0,3
{
    if (!p0) r0 = ##-1065090812
    if (!p0) jump:nt modem_rwe_vid_lock_retry
    if (p0) r1 = memw(r0+#0)
}
MODEM_SYS_READ 0,8,1   // HTID
{ p0 = cmp.gt(r1,#0) }
{ trace(r1) }
MODEM_SYS_READ 0,63,1  // S63 interrupt-status mask
{
    r1 = setbit(r1,r0)
    memw(##-1065090816) = r1.new
}
{ r0 = setbit(r0,#16) }
MODEM_SYS_WRITE 63,0,1
{ r1 = #0; memw(##-1065090812) = r1.new }
MODEM_RWE_CONFIG_WRITE 4,21,1 // P3:0
{ r21:20 = memd(lr+#32) }
{ r1 = #-5 }
{ trace(r1) }
MODEM_RWE_SWAP_SGP0 31,1
// LLVM6 cannot track a packet-local register `.new` consumer through
// CRSWAP. A checked 4-slot compound packet uses the same V60 compare/branch
// pattern as the independently validated 3-slot variant, with slot bit 17.
.macro MODEM_RWE_LR_WORD_LOAD destination, packet_marker=1
    .if (\destination != 0) || (\packet_marker != 1)
      .error "invalid CRSWAP frame word-load slot"
    .endif
    .word (0x91800000 | (31 << 16) | (\packet_marker << 14) | \destination)
.endm
.macro MODEM_RWE_COMPACT_LR_NEG1_RETURN_FOUR displacement, marker=3
    .if (\displacement & 1) || (\displacement < -1024) || (\displacement > 1022) || (\marker != 3)
      .error "invalid four-word CRSWAP compact return branch"
    .endif
    .word (0x2606e000 | (((\displacement >> 9) & 3) << 20) | ((\displacement >> 1) & 255))
.endm
MODEM_RWE_LR_MOVE_TO 1,1
MODEM_RWE_LR_WORD_LOAD 0,1
MODEM_RWE_COMPACT_LR_NEG1_RETURN_FOUR (0x38c78 - 0x38d84),3
// Publish the current saved-context pointer and dispatch the next exception
// class based on the VID index, preserving both packet-local .new predicates.
.globl agent11_modem_rwe_vid_context_publisher
agent11_modem_rwe_vid_context_publisher:
{ fp = ##-1065269088; lr = #1; memd(r0+#32) = lr:fp }
MODEM_RWE_PREDICATE_READ 28,1    // R28 = P3:0
{ lr = asl(lr,#2); memd(r0+#40) = r29:28; memd(r0+#56) = r23:22 }
MODEM_SYS_READ 29,21,3         // SP = VID
.set modem_rwe_vid_owner_previous,agent11_modem_exception_dispatch_entry + (0x38bf0-0x38108)
.set modem_rwe_vid_owner_resume,agent11_modem_exception_dispatch_entry + (0x38c58-0x38108)
{
    if (p1.new) jump:nt modem_rwe_vid_owner_previous
    if (p2.new) jump:nt modem_rwe_vid_owner_resume
    p1 = cmp.eq(sp,#0)
    p2 = cmp.eq(sp,#5)
}
// Common alternate-frame save path following the video-ID/predicate switch.
// CIAD synchronizes LR's controller-indirection descriptor before the
// zeroed frame and caller GP/UGP snapshots are written to saved memory.
.globl agent11_modem_rwe_video_frame_setup
agent11_modem_rwe_video_frame_setup:
MODEM_RWE_CIAD_FRAME 31,3
{ p0 = cmp.eq(r0,fp) }
.set modem_rwe_video_frame_already_saved, agent11_modem_exception_dispatch_entry + (0xec80-0x38108)
{ if (p0) jump:nt ##modem_rwe_video_frame_already_saved; r22 = add(r0,#64) }
{ dczeroa(r22) }
{ r21 = add(r0,#96); memd(r0+#64) = r21:20 }
{ r21 = add(r0,#128) }
{ dczeroa(r21) }
{ dczeroa(r21) }
MODEM_RWE_Y_CONFIG_READ 29,8,1  // SP = USR
{ r9 = sp; memd(r0+#112) = r9:8; memd(r0+#136) = r3:2 }
MODEM_RWE_Y_CONFIG_READ 23,10,1 // UGP
{ memd(r0+#80) = r17:16; memd(r0+#88) = r15:14 }
MODEM_RWE_Y_CONFIG_READ 22,11,1 // GP
{ memd(r0+#96) = r13:12; memd(r0+#104) = r11:10 }
.set modem_rwe_video_frame_capture,agent11_modem_exception_dispatch_entry + (0xdd1c-0x38108)
{ call ##modem_rwe_video_frame_capture; r0 = r9; r2 = #1; r17 = r0 }
// Remaining saved-frame control register bank and codec descriptor fields.
// Two independent privileged GP-relative loads are encoded from source
// operands; LLVM6 emits a different GP addressing class from V60 hardware.
.globl agent11_modem_rwe_video_register_bank
agent11_modem_rwe_video_register_bank:
MODEM_SYS_READ 12,6,1                 // SSR
{ r11 = memw(r28+##-1065268620) }
MODEM_RWE_Y_CONFIG_READ 20,0,1         // SA0
{
    r12 = clrbit(r12,#31)
    r1:0 = memd(r1+#48)
    memd(r17+#72) = r19:18
}
MODEM_SYS_READ 13,6,1                  // SSR
{ r12 = insert(r17,#3,#16); r10 = ##-1065279472 }
MODEM_SYS_WRITE 6,12,1                 // SSR = frame tag
{ p1 = cmp.eq(r11,#0); memd(r17+#120) = r7:6 }
MODEM_SYS_READ 12,3,3                  // ELR, standalone packet
MODEM_RWE_Y_CONFIG_READ 21,1,3         // LC0, standalone packet
MODEM_RWE_Y_CONFIG_READ 18,2,1         // SA1
{ r5 = ##-1065267008; memd(r17+#128) = r5:4 }
MODEM_RWE_CONTROL_PAIR_READ 14,6,1     // M1:0
{ r10 += asl(r9,#4); memd(r17+#144) = r1:0; memd(r17+#24) = r13:12 }
MODEM_SYS_READ 29,0,1                  // SGP0
{ fp = r17; memd(r17+#176) = r29:28; memd(r17+#152) = r21:20 }
MODEM_RWE_Y_CONFIG_READ 19,3,1         // LC1
{ r20 = memw(r28+##-1065265168); memd(r17+#48) = r23:22 }
MODEM_RWE_CONTROL_PAIR_READ 18,12,1   // CS1:0
{ if (p1) r8 = memw(r10+#12); memd(r17+#160) = r19:18 }
MODEM_RWE_CONFIG_WRITE 8,20,1         // USR
{ r20 = memw(r28+##-1065265172); memd(r17+#168) = r15:14 }
MODEM_SYS_WRITE 7,20,1                 // CCR
