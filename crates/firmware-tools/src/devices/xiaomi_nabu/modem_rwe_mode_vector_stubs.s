// SPDX-License-Identifier: GPL-3.0-only
// Independent Hexagon V60 source for disjoint controller-mode helpers.
// File [0x3a404,0x3a40c) and [0x3a414,0x3a444) are executable; the
// unclassified handler [0x3a40c,0x3a414) is NOT generated or counted.
// Virtual/physical addresses = file offset + 0xc0808000 / 0x8e008000.
.text
.macro AGENT11_RTE marker=3
  .if ((\marker != 1) && (\marker != 3))
    .error "Hexagon RTE requires a packet-continuation or packet-end marker"
  .endif
  // Hexagon architectural return from exception; packet parse bits [15:14].
  .word (0x57e00000 | (\marker << 14))
.endm
.macro AGENT11_MOV_R0_ZERO marker=1
  .if (\marker != 1)
    .error "mode selector r0=#0 must share packet with the following RTE"
  .endif
  // Hexagon V60 immediate move #0 to r0, source-derived instruction class.
  .word (0x78000000 | (\marker << 14))
.endm

// First helper spans 8 bytes. The following 8 bytes belong to a separate
// still-unclassified handler; keeping our code disjoint prevents fake coverage.
.globl agent11_modem_rwe_mode_read_status
agent11_modem_rwe_mode_read_status:
AGENT11_RTE 1
{ r0 = memw(r6+#232) }

// Next two helpers share the contiguous dispatch region with the table below.
.globl agent11_modem_rwe_mode_read_saved_word
agent11_modem_rwe_mode_read_saved_word:
AGENT11_RTE 1
{ r0 = memw(r1+#88) }
.globl agent11_modem_rwe_mode_read_saved_flags
agent11_modem_rwe_mode_read_saved_flags:
AGENT11_RTE 1
{ r0 = memw(r1+#96) }

// The three cases branch back to three *different* handler entrypoints.
// The case-3 target resides in the uncovered 8-byte handler at 0x3a40c;
// this is only a relative source-level symbolic branch, not its contents.
.globl agent11_modem_rwe_mode_dispatch
agent11_modem_rwe_mode_dispatch:
.set agent11_mode_class3_handler,agent11_modem_rwe_mode_dispatch-0x18
.set agent11_mode_class1_handler,agent11_modem_rwe_mode_read_saved_word
.set agent11_mode_class2_handler,agent11_modem_rwe_mode_read_saved_flags
{
  if (!p0) jump:t agent11_mode_dispatch_next
  r1 = memw(r6+#256)
}
{ r1 = memw(r1+#80) }
agent11_mode_dispatch_next:
{ p1 = cmp.eq(r0,#1); if (p1.new) jump:t agent11_mode_class1_handler }
{ p1 = cmp.eq(r0,#2); if (p1.new) jump:t agent11_mode_class2_handler }
{ p1 = cmp.eq(r0,#3); if (p1.new) jump:t agent11_mode_class3_handler }
AGENT11_MOV_R0_ZERO 1
AGENT11_RTE 3
