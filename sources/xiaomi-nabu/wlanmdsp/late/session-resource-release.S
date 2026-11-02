// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x1015d4, 0x101680),
// runtime [0xb00c15d4, 0xb00c1680).
// If context+0x540 is nonzero, stop/reset its embedded callback object and
// unregister the event callback. Clear context bit 20 and object bit 1, invoke
// the corresponding disable helpers, release the object, and zero its pointer.
// A missing object returns immediately without those side effects.
// Explicit V66 mnemonic/packet source; no reference bytes are used at build time.
.text
.globl wlan_late_session_resource_release
wlan_late_session_resource_release:
{ call 0x1e41a4; allocframe(#0x10) }
{ r16 = r0 }
{ r17 = memw(r16+#0x540); if (cmp.eq(r17.new,#0x0)) jump:nt restore_return }
{ r18 = add(r17,#0x10); r2 = memw(gp+#0x78c) }
{ r0 = r18; r19 = memw(r2+#0x39c) }
{ call 0x1557c8 }
{ call 0x1557c8; r0 = r18 }
{ r1:0 = combine(#0x5,r16); r2 = memw(##0xb0343e20) }
{ callr r2 }
{ call -0x8884c; r1:0 = combine(##-0x4ff3eadc,r16); r2 = r16 }
{ r2 = memw(r16+#0x1e0) }
{ p0 = !tstbit(r2,#0x14); if (p0.new) jump:t clear_object_activity; if (!p0.new) r3 = memw(r16+#0x28) }
{ r2 = clrbit(r2,#0x14); r4 = memub(r16+#0x250) }
{ r3 = addasl(r3,r4,#0x2); memw(r16+#0x1e0) = r2 }
{ r2 = memw(r3+#0x328) }
{ call 0x38; r0 = memw(r2+#0x24) }
clear_object_activity:
{ r2 = memub(r17+#0x2) }
{ p0 = !tstbit(r2,#0x1); if (!p0.new) r0 = add(r16,#0x0); if (p0.new) jump:t release_object }
{ call 0x28; r2 = and(r2,#0xfd); memb(r17+#0x2) = r2.new }
release_object:
{ call 0x1609d8; r1 = r17; r0 = memw(r19+#0x0) }
{ r2 = #0x0; memw(r16+#0x540) = r2.new }
restore_return:
{ jump 0x1e41d8 }
