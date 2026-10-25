// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x100e74, 0x100eb0),
// runtime [0xb00c0e74, 0xb00c0eb0).
// Search a pointer array for any eight-byte pattern accepted by 0xb00c0e14.
// Inputs are candidate pointer, pointer-array base, and unsigned item count.
// Return 1 on the first match; return 0 for no match or an empty array.
// The array advances by four bytes after each unsuccessful pattern.
// Explicit V66 mnemonic/packet source; no reference bytes are used at build time.
.text
.globl wlan_late_pattern_pointer_search
wlan_late_pattern_pointer_search:
{ call 0x1e4904; allocframe(#0x10) }
{ r17:16 = combine(r1,r2); r19 = #0x0; r18 = r0 }
{ p0 = cmp.eq(r16,#0x0); if (p0.new) jump:nt no_pattern_matches }
pattern_loop:
{ call -0x74; r0 = r18; r1 = memw(r17+#0x0) }
{ p0 = cmp.eq(r0,#0x0); if (!p0.new) jump:nt pattern_matches }
{ r17 = add(r17,#0x4); r19 = add(r19,#0x1); if (cmp.gtu(r16,r19.new)) jump:t pattern_loop }
no_pattern_matches:
{ r0 = #0x0 ; jump ##0x1e49b4 }
pattern_matches:
{ r0 = #0x1 ; jump ##0x1e49ac }
