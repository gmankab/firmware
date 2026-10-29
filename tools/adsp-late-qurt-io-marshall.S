// SPDX-License-Identifier: GPL-3.0-only
// Source-reencoded original Hexagon V66 shared-ISA executable at ADSP file 0x3f654.
.text
.globl adsp_late_qurt_io_marshall
adsp_late_qurt_io_marshall:
// Marshal two register pairs into an allocated frame, dispatch QuRT
// syscall #103, publish three output words, then restore the frame.
{ allocframe(#16) }
{ r23:22 = combine(r1,r0); memd(r29+#0) = r23:22 }
{ r25:24 = combine(r3,r2); memd(r29+#8) = r25:24 }
{ r0 = #5 }
{ trap0(#103) }
{ memw(r22+#0) = r1; memw(r23+#0) = r2 }
{ memw(r24+#0) = r3 }
{ r23:22 = memd(r29+#0); r25:24 = memd(r29+#8) }
{ dealloc_return }
