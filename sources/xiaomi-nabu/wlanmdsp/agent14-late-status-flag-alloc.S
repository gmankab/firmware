// SPDX-License-Identifier: GPL-3.0-only
// File offsets [0x105638,0x105668): allocate status-byte cache.
.text
.globl agent14_late_status_flag_alloc
agent14_late_status_flag_alloc:
{ r2 = memw(r0+#0x1fc) }
{ p0 = cmp.eq(r2,#0x0); if (p0.new) r2 = memub(r0+##0x140); allocframe(#0x0) }
{ r0 = #0x20c; r1 = and(r2,#0xff); r3:2 = combine(##0x100,#0x9) }
{ call 0x15c768 }
{ memw(gp+#0x7d4) = r0 }
{ dealloc_return }
