// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x100840,0x10099c),
// runtime [0xb00c0840,0xb00c099c).
// Build a stack-owned request, invoke the external submission helper, and
// preserve its result. Set context flag0 even when that helper reports nonzero.
// For context states one or two, invoke the external timer-like operations
// using the full unsigned 64-bit product of field+0x10 and 1000.
// Increment context+0x220 on every normal completion. External APIs are unnamed.
// Explicit V66 mnemonic/packet source; no reference bytes are used at build time.
.text
.globl wlan_late_request_submit
wlan_late_request_submit:
{ call 0x1e4f38; allocframe(#0x518) }
{ r19 = #0x0; r18 = add(r29,#0x8); r1 = #0x0; r16 = r0 }
{ r0 = add(r18,#0x14); r2 = memw(r16+#0x0) }
// r3 dereferences the old table pointer in r2 while r2 becomes size 0x4ec.
{ r2 = #0x4ec; r3 = memw(r2+#0x0) }
{ call -0xbe8bc; r17 = memw(r3+#0x230) }
{ r0 = #0x9; r1 = memw(r16+#0x14); memb(r29+#0x387) = r0.new }
// Store the old r1 (field+0x14) while loading field+0x10 into r1.
{ r0 = ##0x140000; r1 = memw(r16+#0x10); memh(r29+#0xc) = r1 }
// The request field stores old r0=0x140000 while r0 becomes 0xa0000.
{ r0 = ##0xa0000; r2 = memw(r16+#0x4); memw(r29+#0x10) = r0 }
// The next request field stores old r0=0xa0000 while r0 becomes 7.
{ r1 = add(r1,#0x3e8); r0 = #0x7; memw(r29+#0x14) = r0; memh(r29+#0x8) = r2 }
// The byte store uses old r0=7 while r0 becomes the copy destination.
{ r2 = #0x8; r0 = add(r18,#0x131); memh(r29+#0x18) = r1; memb(r29+#0x138) = r0 }
{ r1 = ##-0x4fd490f2; r3 = memw(r16+#0xc); memh(r18+#0x16) = #0x1 }
{ memw(r29+#0x24) = r3; memh(r18+#0x18) = #0x0 }
{ memw(r29+#0x134) = r19; memb(r29+#0x38a) = r19 }
{ memh(r18+#0x2) = #0x1e; memh(r18+#0x6) = #0x0 }
{ memh(r18+#0x12) = #0x0; memb(r18+#0x1a) = #0x1 }
{ memb(r29+#0x389) = r19; memb(r29+#0x388) = r19 }
{ call -0xbe7bc; memw(r29+#0x38c) = r19 }
{ r4 = #0x2; r1 = add(r29,#0x8); r3:2 = combine(#0x0,#0x42); r0 = r17 }
{ call -0x97a2c; r5 = add(r16,#0x280); memw(r16+#0x280) = r19 }
{ r17 = r0; if (cmp.eq(r17.new,#0x0)) jump:nt mark_request_active }
{ r1 = #0x1251; r0 = ##0x64a8; memw(r29+#0x0) = r17 }
{ call 0x15a894 }
{ call -0xbdcdc; r1:0 = combine(r17,##-0x4fd490dc) }
mark_request_active:
{ r2 = add(r16,#0x284); r0 = ##0x64b0; r1 = #0x2412 }
{ memw(r2+#0x0) = setbit(#0x0) }
{ r2 = memw(r16+#0x10); r3 = memw(r16+#0x8) }
{ call 0x15a86c; memw(r29+#0x4) = r3; memw(r29+#0x0) = r2 }
{ r0 = ##-0x4fd490d0; r1 = memw(r16+#0x10); r2 = memw(r16+#0x8) }
{ call -0xbe63c }
{ r2 = memw(r16+#0x8) }
{ r2 = add(r2,#-0x1); if (cmp.gtu(r2.new,#0x1)) jump:t count_and_restore }
{ r18 = add(r16,#0x288) }
{ call 0x15afec; r0 = r18 }
{ r3 = #0x3e8; r0 = r18; r4 = #0x0; r2 = memw(r16+#0x10) }
{ r3:2 = mpyu(r2,r3); call 0x15af5c }
count_and_restore:
{ r2 = add(r16,#0x220); r0 = r17 }
{ jump 0x1e4ec0; memw(r2+#0x0) += #0x1 }
