// SPDX-License-Identifier: GPL-3.0-only
// Hexagon v60 auxiliary executable guard: read mapped controller address,
// return when disabled, otherwise calculate PC-relative handoff.
// file [0x446260,0x446278), VMA [0x1e400260,0x1e400278).
.text
.globl modem_late_aux_guard
modem_late_aux_guard:
.set aux_guard_handoff, modem_late_aux_guard + (0x9cc0 - 0x260)
{ r0 = ##507550912 }
{ p0 = cmp.eq(r0,#0) }
{ if (p0) jumpr:nt r31 }
{ r0 = pc }
{ jump aux_guard_handoff }
