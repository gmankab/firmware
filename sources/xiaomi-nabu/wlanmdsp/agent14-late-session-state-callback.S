// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x101394, 0x101524),
// runtime [0xb00c1394, 0xb00c1524).
// Process session object states 2 and 3 when object flag 0 is active.
// State 2 enables context flag 20 and object flag 1, issues callbacks, changes
// state to 3 and schedules the embedded callback object. State 3 reverses those
// flags, emits completion callbacks, then sets state 1 and clears flag 0.
// Missing/inactive objects enter diagnostic forwarding. External APIs are unnamed.
// Explicit V66 mnemonic/packet source; no reference bytes are used at build time.
.text
.globl wlan_late_session_state_callback
wlan_late_session_state_callback:
{ call 0x1e43e4; allocframe(#0x20) }
{ r16 = r1 }
{ r18 = memw(r16+#0x540); if (cmp.eq(r18.new,#0x0)) jump:nt report_inactive_object }
{ r2 = memub(r18+#0x2) }
{ if (p0.new) r0 = ##0x64f8; p0 = tstbit(r2,#0); if (!p0.new) jump:nt report_inactive_object }
{ r2 = extractu(r2,#0x1,#0x1); r4 = memw(r16+#0x1e0) }
// The stack diagnostic stores OLD r2 (object flag 1), while r2 becomes
// context flag 20; the following packet stores that new context bit.
{ r2 = extractu(r4,#0x1,#0x14); r1 = #0x2443; r3 = memub(r18+#0x1); memw(r29+#0x8) = r2 }
{ call 0x159de8; memw(r29+#0x4) = r2; memw(r29+#0x0) = r3 }
{ r0 = ##-0x4fd48fd8; r3 = memub(r18+#0x2); r2 = memw(r16+#0x1e0) }
{ r2 = extractu(r2,#0x1,#0x14); r3 = extractu(r3,#0x1,#0x1); r1 = memub(r18+#0x1) }
{ call -0xbeedc }
{ r17 = add(r18,#0x10); r2 = memub(r18+#0x1) }
{ p0 = cmp.eq(r2,#0x3); if (p0.new) jump:nt handle_state_three }
{ p0 = cmp.eq(r2,#0x2); if (!p0.new) jump:nt restore_return }
{ r1 = #0x1; r2 = memw(r16+#0x28); r3 = memub(r16+#0x250) }
{ r2 = addasl(r2,r3,#0x2) }
{ r2 = memw(r2+#0x328) }
{ call 0xbe088; r0 = memw(r2+#0x24) }
{ r2 = add(r16,#0x1e0) }
{ memw(r2+#0x0) = setbit(#0x14) }
{ r5 = memub(r18+#0x2) }
{ p0 = tstbit(r5,#0x1); if (p0.new) jump:t notify_state_two }
{ r3:2 = combine(#0x0,#0x11); r0 = r16; r4 = #0x0; r1 = memw(r18+#0x4) }
{ call -0x7c33c; r5 = setbit(r5,#0x1); memb(r18+#0x2) = r5.new }
notify_state_two:
{ r1:0 = combine(#0x5,r16); r2 = memw(##0xb0343e20) }
{ callr r2 }
{ r2 = memw(r18+#0xc); if (cmp.eq(r2.new,#0x0)) jump:nt enter_state_three }
{ r1:0 = combine(#0x2,r16); callr r2 }
enter_state_three:
{ r0 = ##0x6500; r1 = #0x2411; memb(r18+#0x1) = #0x3 }
{ call 0x159d38; r2 = memw(r18+#0x4); memw(r29+#0x0) = r2.new }
{ call -0xbe840; r0 = ##0xb02b7034; r1 = memw(r18+#0x4) }
{ call 0x15592c; r0 = r17 }
{ call 0x1e4350; r0 = r17; r2 = #0x0; r1 = memw(r18+#0x4) }
{ jump 0x15590c }
handle_state_three:
{ r2 = memw(r16+#0x1e0) }
{ p0 = !tstbit(r2,#0x14); if (p0.new) jump:t clear_object_activity; if (!p0.new) r3 = memw(r16+#0x28) }
{ r2 = clrbit(r2,#0x14); r4 = memub(r16+#0x250) }
{ r3 = addasl(r3,r4,#0x2); memw(r16+#0x1e0) = r2 }
{ r2 = memw(r3+#0x328) }
{ call 0x1b0; r0 = memw(r2+#0x24) }
clear_object_activity:
{ r2 = memub(r18+#0x2) }
{ p0 = !tstbit(r2,#0x1); if (!p0.new) r0 = add(r16,#0x0); if (p0.new) jump:t notify_completion }
{ call 0x1a0; r2 = and(r2,#0xfd); memb(r18+#0x2) = r2.new }
notify_completion:
{ r2 = memw(r18+#0xc); if (cmp.eq(r2.new,#0x0)) jump:nt reset_callback_object }
{ r1:0 = combine(#0x3,r16); callr r2 }
reset_callback_object:
{ call 0x1558bc; r0 = r17 }
{ memb(r18+#0x1) = #0x1 }
{ memb(r18+#0x2) = clrbit(#0x0) }
restore_return:
{ jump 0x1e4340 }
report_inactive_object:
{ call -0x224; r1:0 = combine(#0x65,##0xb02b6fd6) }
