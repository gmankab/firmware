// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x100b5c, 0x100d78),
// runtime [0xb00c0b5c, 0xb00c0d78).
// Dispatch types 0..18 through the external 19-entry table at 0xb02b6cd0.
// All handler instructions are source encoded here; the table remains external
// data and contributes no recovered bytes to this fragment.
// Handlers validate payload lengths, retain pointers/counts, and parse a bounded
// string with control bytes replaced by underscores. Unknown types return zero.
// Explicit V66 mnemonic/packet source; no reference bytes are used at build time.
.text
.globl wlan_late_short_type_dispatch
wlan_late_short_type_dispatch:
{ p0 = cmp.gtu(r0,#0x12); if (p0.new) jump:nt return_success; memd(r29+#-0x10) = r17:16; allocframe(#0x8) }
// The table load uses OLD r0 (the type) while r0 is reset to success.
{ r0 = #0x0; r4 = memw(r0<<#0x2+##0xb02b6cd0) }
{ jumpr r4 }
type_0000:
{ p0 = cmp.eq(r2,#0x0); if (p0.new) jump:nt return_invalid }
{ jump return_success; memw(r3+#0x1b0) = r1 }
type_0002:
{ r0 = #-0x1; r4 = #0x2; if (cmp.gtu(r4.new,r2)) jump:t restore_status }
{ jump return_success; memw(r3+#0x1a8) = r1 }
type_0003:
{ r0 = #-0x1; r4 = #0x6; if (cmp.gtu(r4.new,r2)) jump:t restore_status }
{ jump return_success; memw(r3+#0x1e8) = r1 }
type_0004:
{ p0 = cmp.eq(r2,#0x0); if (p0.new) jump:nt return_invalid }
{ jump return_success; memw(r3+#0x1ac) = r1 }
type_0005:
{ r0 = #-0x1; r4 = #0x2; if (cmp.gtu(r4.new,r2)) jump:t restore_status }
{ jump return_success; memw(r3+#0x1c4) = r1 }
type_0006:
{ p0 = cmp.eq(r2,#0x0); if (p0.new) jump:nt return_success }
{ p0 = cmp.eq(r2,#0x1); if (p0.new) jump:t return_invalid }
{ jump return_success; memw(r3+#0x1b4) = r1 }
type_0009:
{ r0 = #-0x1; r4 = #0x6; if (cmp.gtu(r4.new,r2)) jump:t restore_status }
{ jump return_success; memw(r3+#0x1c8) = r1 }
type_000b:
{ r0 = #-0x1; r4 = #0x3; if (cmp.gtu(r4.new,r2)) jump:t restore_status }
{ jump return_success; memw(r3+#0x1bc) = r1; memb(r3+#0x1c0) = r2 }
type_000c:
{ r0 = #-0x1; r4 = #0x2; if (cmp.gtu(r4.new,r2)) jump:t restore_status }
{ r2 = add(r2,#-0x2); r6 = ##0x4ec4ec4f }
{ r5:4 = mpyu(r2,r6); r7 = asr(r2,#0x1f) }
{ r9 = r5; r8 = r4 }
{ r9 += mpyi(r6,r7) }
// The pair shifts both read the old product/correction pair.
{ r7:6 = lsr(r9:8,#0x3f); r9:8 = asr(r9:8,#0x22) }
{ r4 = add(r8,r6) }
{ r2 -= mpyi(r4,#0xd); if (!cmp.eq(r2.new,#0x0)) jump:t restore_status }
{ r2 = extractu(r5,#0x10,#0x2); if (cmp.gtu(r2.new,#0x4)) jump:t restore_status }
{ r2 = lsr(r5,#0x2); memw(r3+#0x1e0) = r1 }
{ jump return_success; memb(r3+#0x1e4) = r2 }
type_000d:
{ r0 = #-0x1; r4 = #0x11; if (cmp.gtu(r4.new,r2)) jump:t restore_status }
{ r4 = add(r1,#0x8); memw(r3+#0x1ec) = r1; memw(r3+#0x1f0) = r2 }
// Preserve the encoded store/load slot order.
{ memw(r3+#0x1f4) = r1; r5 = memub(r1+#0x7) }:mem_noshuf
{ r6 = memub(r1+#0x6); memw(r3+#0x1f8) = r4 }
{ r5 |= asl(r6,#0x8); r6 = add(r2,#-0x11); memh(r3+#0x226) = r5.new }
{ r5 = memub(r1+#0x10); memb(r3+#0x1fc) = r5.new }
// The extent check reads the newly calculated count*8 in r4.new.
{ r4 = asl(r5,#0x3); r2 = add(r1,r2); if (cmp.gt(r4.new,r6)) jump:t restore_status }
{ r5 = add(r1,r4); r6 = r2 }
{ r6 -= add(r5,#0x11); if (!cmp.gt(r6.new,#0x3)) jump:t restore_status }
{ r4 += add(r1,#0x11) }
{ r5 = memub(r4+#0x1); r6 = memub(r4+#0x0) }
{ r5 = insert(r6,#0x18,#0x8) }
{ p0 = cmp.eq(r5,##0x1011); if (!p0.new) jump:t restore_status }
{ r2 -= add(r4,#0x4); r16 = memub(r4+#0x3); r5 = memub(r4+#0x2) }
{ r16 = insert(r5,#0x18,#0x8); if (cmp.gt(r16.new,r2)) jump:nt restore_status }
{ r1 = add(r4,#0x4); r17 = add(r3,#0x204); p0 = cmp.gtu(r16,#0x20); if (p0.new) jump:nt restore_status }
{ r2 = r16; r0 = r17 }
{ call -0xbebc0 }
{ r0 = #0x0; p0 = cmp.eq(r16,#0x0); if (p0.new) jump:nt restore_status }
{ loop0(sanitize_string_loop,r16); r0 = #0x0; r2 = #0x0 }
sanitize_string_loop:
{ r3 = memb(r17+r2<<#0x0); if (cmp.eq(r3.new,#0x0)) jump:nt restore_status }
{ r4 = add(r17,r2); r2 = add(r2,#1); r3 = and(r3,#255) }
// Only nonzero control bytes at most 31 are changed; NUL returns early.
{ p0 = cmp.gtu(r3,#0x1f); if (!p0.new) memb(r4+#0x0) = ##0x5f } :endloop0
{ jump restore_status }
type_000e:
{ jump return_success; memw(r3+#0x1cc) = r1; memw(r3+#0x1d0) = r2 }
type_000f:
{ r4 = add(r2,#-0x6); r0 = #-0x1 }
{ p0 = cmph.gtu(r4,#0x20); if (p0.new) jump:t restore_status; if (!p0.new) memw(r3+##0x1d4) = r1 }
{ jump return_success; memw(r3+#0x1d8) = r2 }
type_0011:
{ p0 = cmp.eq(r2,#0x0); if (p0.new) jump:nt return_success }
{ p0 = cmp.eq(r2,#0x1); if (p0.new) jump:t return_invalid }
{ jump return_success; memw(r3+#0x1b8) = r1 }
type_0012:
{ p0 = cmp.eq(r2,#0x0); if (p0.new) jump:nt return_invalid }
{ memw(r3+#0x1dc) = r1 }
return_success:
{ r0 = #0x0 }
restore_status:
{ r17:16 = memd(r29+#0x0); dealloc_return }
return_invalid:
{ r0 = #-0x1; jump restore_status }
