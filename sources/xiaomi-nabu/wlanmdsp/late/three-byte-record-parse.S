// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x100ac8, 0x100b5c),
// runtime [0xb00c0ac8, 0xb00c0b5c).
// Walk records with a three-byte header and payload length from header byte 1.
// Header byte 2 is not incorporated into the length; preserve that behavior.
// Invoke the type dispatcher for each record. On complete success set flag 1.
// Nonpositive signed input lengths take the empty-success path.
// Explicit V66 mnemonic/packet source; no reference bytes are used at build time.
.text
.globl wlan_late_three_byte_record_parse
wlan_late_three_byte_record_parse:
{ call 0x1e4ca0; p0 = cmp.gt(r1,#0x0); allocframe(#0x28) }
{ r17 = r0; r19 = r2; if (!p0) jump:nt complete_records; r16 = #-0x1 }
{ r20 = add(r17,r1) }
record_loop:
{ r2 = add(r17,#0x2); if (!cmp.gtu(r20,r2.new)) jump:nt restore_status }
{ r18 = memub(r17+#0x1) }
{ r21 = r18 }
{ r21 += add(r17,#0x3); if (cmp.gtu(r21.new,r20)) jump:nt restore_status }
{ call 0x60; r1 = add(r17,#0x3); r3:2 = combine(r19,r18); r0 = memub(r17+#0x0) }
// Only a zero dispatcher result advances r17 to the saved record end.
{ if (p0.new) r17 = add(r21,#0x0); p0 = cmp.eq(r0,#0x0); if (!p0.new) jump:nt report_record_failure }
{ p0 = cmp.gtu(r20,r21); if (p0.new) jump:t record_loop }
complete_records:
{ r16 = #0x0; r2 = add(r19,#0x194) }
{ jump restore_status; memw(r2+#0x0) = setbit(#0x1) }
report_record_failure:
{ r1 = #0x2413; r0 = ##0x64c8; r2 = memub(r17+#0x0) }
{ r3 = memub(r17+#0x3); memw(r29+#0x8) = r18 }
{ call 0x15a67c; memw(r29+#0x4) = r3; memw(r29+#0x0) = r2 }
{ r0 = ##0xb02b6f54; r3 = r18; r1 = memub(r17+#0x0) }
{ call -0xbe638; r2 = memub(r17+#0x3) }
restore_status:
{ r0 = r16 ; jump ##0x1e4cf0 }
