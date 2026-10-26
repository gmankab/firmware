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
