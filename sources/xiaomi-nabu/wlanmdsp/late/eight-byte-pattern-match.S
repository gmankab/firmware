// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x100e14, 0x100e74),
// runtime [0xb00c0e14, 0xb00c0e74).
// Call the external helper with (candidate, pattern, 8); return 1 if it returns
// zero. Otherwise require equal first two bytes and a zero six-byte suffix in
// the second argument. The fallback implements a two-byte-prefix wildcard.
// The external helper's stronger comparison contract is not established here.
// Explicit V66 mnemonic/packet source; no reference bytes are used at build time.
.text
.globl wlan_late_eight_byte_pattern_match
wlan_late_eight_byte_pattern_match:
{ r2 = #0x8; r17:16 = combine(r0,r1); memd(r29+#-0x10) = r17:16; allocframe(#0x8) }
{ call -0xbd4a4 }
// The predicate reads the OLD comparison result while r0 becomes 1.
{ r0 = #0x1; p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt restore_result }
{ r2 = memub(r16+#0x0); r4 = memb(r17+#0x1) }
{ r5 = memub(r17+#0x0); r3 = memb(r16+#0x1) }
{ r2 |= asl(r3,#0x8) }
{ r5 |= asl(r4,#0x8); if (!cmp.eq(r5.new,r2)) jump:t no_match }
{ r2 = memub(r16+#0x3); r3 = memub(r16+#0x4) }
{ r3 = asl(r3,#0x8); r2 = aslh(r2); r4 = memub(r16+#0x2); r6 = memub(r16+#0x5) }
{ r2 |= asl(r4,#0x18) }
{ r2 |= or(r3,r6); if (cmp.eq(r2.new,#0x0)) jump:nt check_last_two_pattern_bytes }
no_match:
{ r0 = #0x0 }
restore_result:
{ r17:16 = memd(r29+#0x0); dealloc_return }
check_last_two_pattern_bytes:
{ r2 = memub(r16+#0x7); r3 = memub(r16+#0x6) }
{ r2 = insert(r3,#0x18,#0x8) }
{ jump restore_result; p0 = cmp.eq(r2,#0x0); if (!p0.new) r0 = #0 }
