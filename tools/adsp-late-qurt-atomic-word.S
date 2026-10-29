// SPDX-License-Identifier: GPL-3.0-only
// Source-reencoded original Hexagon V66 shared-ISA executable at ADSP file 0x3f6b8.
.text
.globl adsp_late_qurt_atomic_word
adsp_late_qurt_atomic_word:
// Atomically update a packed 64-bit word using locked load/store.
retry:
{ r3:2 = memd_locked(r0) }
{ r2 = r1; memw(r1+#0) = r2 }
{ memd_locked(r0,p0) = r3:2 }
{ if (p0) jumpr:nt r31 }
{ jump retry }
