// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x1004e8,0x100528),
// runtime [0xb00c04e8,0xb00c0528).
// Validate each counted byte after masking its high bit. Accepted low-seven
// bit values are {2, 4, 11, 22}; an empty list returns one. The first packet
// reads the incoming r0 into r2 while independently assigning result r0=1.
// The hardware loop consumes exactly the byte count at input[0].
// Explicit V66 mnemonic/packet source; no reference bytes are used at build time.
.text
.globl wlan_late_rate_code_set
wlan_late_rate_code_set:
{ r0 = #0x1; r2 = r0 }
{ r4 = memub(r2+#0x0) }
{ p0 = cmp.eq(r4,#0x0); if (p0.new) jumpr:nt r31 }
{ loop0(rate_loop,r4); r3 = ##0x400814; r2 = add(r2,#1) }
{ r4 = #0x0 }
rate_loop:
{ r5 = memub(r2+r4<<#0x0) }
{ r5 = and(r5,#0x7f); if (cmp.gtu(r5.new,#0x16)) jump:nt reject_rate }
{ p0 = tstbit(r3,r5); if (!p0.new) jump:nt reject_rate }
{ r4 = add(r4,#0x1); nop } :endloop0
{ jumpr r31 }
reject_rate:
{ r0 = #0x0; jumpr r31 }
