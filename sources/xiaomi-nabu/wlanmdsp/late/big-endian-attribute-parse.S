// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x100d78, 0x100e14),
// runtime [0xb00c0d78, 0xb00c0e14).
// Clear a 0x190-byte output and walk big-endian 16-bit type/length headers.
// Require each four-byte header and its declared payload before dispatch.
// Negative dispatcher results enter diagnostics and retain failure status.
// Nonpositive signed total lengths take the empty-success path.
// Explicit V66 mnemonic/packet source; no reference bytes are used at build time.
.text
.globl wlan_late_big_endian_attribute_parse
wlan_late_big_endian_attribute_parse:
{ call 0x1e49f0; allocframe(#0x28) }
{ r17:16 = combine(r2,r1); r20 = r0; r2 = #0x190 }
{ call -0xbede4; r1:0 = combine(#0x0,r17) }
// The signed length test reads OLD r16; positive input installs -1 as
// default status, while a nonpositive input is completed successfully.
{ r21 = add(r20,r16); if (p0.new) r16 = #-0x1; p0 = cmp.gt(r16,#0x0); if (!p0.new) jump:nt complete_attributes }
attribute_loop:
{ r2 = sub(r21,r20); if (!cmp.gt(r2.new,#0x3)) jump:nt restore_status }
{ r19 = r20; r18 = memub(r20+#0x3); r3 = memub(r20+#0x2) }
// r19 post-increments by 4; other loads in this packet use the old input.
{ r18 |= asl(r3,#0x8); r20 = memub(r20+#0x1); r2 = memub(r19++#0x4) }
{ r20 |= asl(r2,#0x8); r3 = sub(r21,r19); if (cmp.gt(r18,r3.new)) jump:nt restore_status }
{ r3:2 = combine(r18,r19); r1:0 = combine(r20,r17) }
{ call 0xe0 }
{ p0 = cmp.gt(r0,#-0x1); if (!p0.new) jump:nt report_attribute_failure }
{ r20 = add(r19,r18); if (cmp.gtu(r21,r20.new)) jump:t attribute_loop }
complete_attributes:
{ r16 = #0x0 ; jump restore_status }
report_attribute_failure:
{ r1 = #0x2413; r0 = ##0x64d8; r2 = memub(r19+#0x0) }
{ memw(r29+#0x8) = r18; memw(r29+#0x4) = r2 }
{ call 0x15a3c0; memw(r29+#0x0) = r20 }
{ r1:0 = combine(r20,##-0x4fd49094); r3 = r18; r2 = memub(r19+#0x0) }
{ call -0xbe8f4 }
restore_status:
{ r0 = r16 ; jump ##0x1e4a38 }
