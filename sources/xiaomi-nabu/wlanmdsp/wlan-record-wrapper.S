// SPDX-License-Identifier: GPL-3.0-only
// Hexagon wrapper for two WLAN packed-record comparison paths.
// Runtime [0xb0002138,0xb0002184): executes field comparator then array
// comparator and reports status using the shared runtime report trampoline.
.text
.globl wlan_record_compare_wrapper
wlan_record_compare_wrapper:
{ call 0x2a3630; allocframe(#24) }
{ r17:16 = combine(r2,r4); r21:20 = combine(r3,r0); r2 = #24; r19 = r1 }
{ call -28; r1:0 = combine(r17,r16) }
{ call -392; r1:0 = combine(r21,r17) }
{ r18 = #0; if (p0.new) r4 = add(r16,#0); p0 = cmp.eq(r0,#0); if (!p0.new) jump:t compare_done }
{ call -344; r1:0 = combine(r19,r20); r3:2 = combine(r21,r17) }
{ p0 = cmp.eq(r0,#0); if (p0.new) r18 = #1 }
compare_done:
{ r0 = r18; jump ##0x2a36c8 }
