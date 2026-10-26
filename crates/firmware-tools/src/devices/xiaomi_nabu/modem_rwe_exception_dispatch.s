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
