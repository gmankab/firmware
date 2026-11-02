// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x100a04, 0x100ac8),
// runtime [0xb00c0a04, 0xb00c0ac8).
// Parse four-byte-header attributes into a temporary 0x190-byte structure.
// On success, copy an optional bounded string (at most 32 bytes), a big-endian
// halfword, and an optional byte into the destination extension fields.
// The string copy adds no terminator; absent or oversized strings leave their
// destination string/length fields unchanged and do not turn success into error.
// Copy the temporary base structure and set destination flag bit 2.
// The copy-like helpers remain external dependencies.
// Explicit V66 mnemonic/packet source; no reference bytes are used at build time.
.text
.globl wlan_late_attribute_record_apply
wlan_late_attribute_record_apply:
{ call 0x1e4d74; allocframe(#0x1a8) }
// The call receives the original input/length and new stack destination;
// r17:r16 simultaneously preserve the incoming length/destination.
{ call 0x36c; r17:16 = combine(r1,r2); r2 = add(r29,#0x8); r18 = r0 }
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt apply_optional_string }
{ r1 = #0x2412; r0 = ##0x64d0; r2 = memub(r18+#0x0) }
{ call 0x15a78c; memw(r29+#0x4) = r17; memw(r29+#0x0) = r2 }
{ r0 = ##0xb02b6f60; r2 = r17; r1 = memub(r18+#0x0) }
{ call -0xbe71c; r17 = #-0x1 }
{ jump restore_status }
apply_optional_string:
{ r1 = memw(r29+#0xd8); if (cmp.eq(r1.new,#0x0)) jump:nt apply_optional_halfword }
{ r17 = memw(r29+#0xdc) }
{ p0 = cmp.gtu(r17,#0x20); if (!p0.new) r0 = add(r16,##0x204); if (p0.new) jump:t apply_optional_halfword }
{ call -0xbe934; r2 = r17 }
{ memw(r16+#0x200) = r17 }
apply_optional_halfword:
{ r2 = memw(r29+#0x30); if (cmp.eq(r2.new,#0x0)) jump:nt apply_optional_byte }
{ r3 = memub(r2+#0x1); r0 = memub(r2+#0x0) }
{ r3 |= asl(r0,#0x8); memh(r16+#0x228) = r3.new }
apply_optional_byte:
{ r3 = #0x0; r0 = r16; r1 = add(r29,#0x8); r2 = memw(r29+#0x9c) }
{ p0 = cmp.eq(r2,#0x0); r17 = #0x0; if (!p0.new) r2 = memub(r2+#0x0) }
// The optional byte store reads OLD r2 while r2 becomes copy size 0x190.
{ if (!p0) r3 = #0x1; r2 = #0x190; if (!p0) memb(r16+##0x230) = r2 }
{ call -0xba654; memw(r16+#0x22c) = r3 }
{ r2 = add(r16,#0x194) }
{ memw(r2+#0x0) = setbit(#0x2) }
restore_status:
{ r0 = r17 ; jump ##0x1e4d94 }
