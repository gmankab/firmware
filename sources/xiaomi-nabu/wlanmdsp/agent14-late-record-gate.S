// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x100410,0x1004e8),
// runtime [0xb00c0410,0xb00c04e8).
// Gate processing on record flags, rate-code membership and optional six-byte
// identity comparison. Keep the 0x1b8-byte frame and conditional field reads.
// All old-value register reads within packets are intentional. This gate
// REJECTS a nonzero all-codes-in-set result; the membership test is inverted
// here. The final helper result is also inverted (nonzero permits return 0).
// Explicit V66 mnemonic/packet source; no reference bytes are used at build time.
.text
.globl wlan_late_record_gate
wlan_late_record_gate:
{ call 0x1e5358; allocframe(#0x1b8) }
{ r17:16 = combine(r0,#-0x1); r18 = r1; r1 = add(r29,#0x190); r0 = add(r29,#0x190) }
{ r2 = memw(r17+#0x0); memh(r1+#0x4) = #0x0 }
{ r3 = memw(r17+#0x4); memw(r0+#0x0) = #0 }
{ r2 = memw(r2+#0x0); r19 = memw(r18+#0x194) }
{ r2 = addasl(r2,r3,#0x2); p0 = !tstbit(r19,#0x1); r3 = #0xff }
{ if (p0) jump:nt restore_return; r20 = memw(r2+#0x54) }
{ p0 = tstbit(r19,#0); if (!p0.new) jump:nt restore_return }
{ r2 = memub(r18+#0x198) }
{ p0 = bitsclr(r2,r3); if (!p0.new) r0 = add(r18,##0x198); if (p0.new) jump:nt restore_return }
{ call 0x80 }
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt compare_identity; if (p0.new) r2 = memub(r20+#0x7) }
restore_return:
{ r0 = r16; jump ##0x1e53d0 }
compare_identity:
{ r0 = memw(r18+#0x1e8); r6 = memub(r20+#0x8) }
{ p0 = cmp.eq(r0,#0x0); r3 = memub(r20+#0x6); r4 = memub(r20+#0x5) }
{ r3 |= asl(r2,#0x8); r2 = memub(r20+#0x9); r5 = memub(r20+#0x4) }
{ r5 |= asl(r4,#0x8); if (!p0) r1 = add(r29,##0x190) }
{ r6 |= asl(r2,#0x8); r5 |= asl(r3,#0x10); if (!p0) r2 = #0x6; memh(r29+#0x194) = r6.new }
{ if (p0) jump:nt optional_deep_check; memw(r29+#0x190) = r5 }
{ call -0xbcb3c }
{ p0 = cmp.eq(r0,#0x0); if (!p0.new) jump:t restore_return }
optional_deep_check:
{ p0 = !tstbit(r19,#0x2); if (!p0.new) r1:0 = combine(r18,r29); if (!p0.new) r2 = #0x190; if (!p0.new) jump:t copy_and_check }
{ r16 = #0x0; jump restore_return }
copy_and_check:
{ call -0xba078 }
{ call 0x50; r0 = r17 }
{ jump restore_return; p0 = cmp.eq(r0,#0x0); if (!p0.new) r16 = #0 }
