// SPDX-License-Identifier: GPL-3.0-only
// File offsets [0x1055f8, 0x105638): codec status flag-update helper.
// V66 packet instructions reconstructed from decoded dataflow.
.text
.globl agent14_late_status_flag_update
agent14_late_status_flag_update:
{ call -10552; r0 = ##-0x4fd48ca7; r1 = #0x8d0 }
{ p0 = cmp.eq(r1,#0x0); r3 = memw(r0+#0x1b4) }
{ r4 = memub(r3+#0x3) }
{ r4 = setbit(r4,#0x0); r5 = and(r4,#0xfe) }
{ if (!p0) r5 = add(r4,#0x0); p0 = cmp.eq(r2,#0x0) }
{ r4 = setbit(r5,#0x1); r2 = and(r5,#0xfd) }
{ if (!p0) r2 = add(r4,#0x0) }
{ jumpr r31; memb(r3+#0x3) = r2 }
