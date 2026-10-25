// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x101524, 0x1015d4),
// runtime [0xb00c1524, 0xb00c15d4).
// For event code 0x13 while context flag 20 is set, look up two resource slots
// from event bytes +4 and +8. Disable the latter slot and enable the former.
// Other events return without switching. Missing session objects enter
// diagnostic forwarding. Resource enable/disable helpers remain external.
// Explicit V66 mnemonic/packet source; no reference bytes are used at build time.
.text
.globl wlan_late_session_resource_switch
wlan_late_session_resource_switch:
{ call 0x1e4254; allocframe(#0x20) }
{ r17:16 = combine(r0,r1) }
{ r18 = memw(r17+#0x540); if (cmp.eq(r18.new,#0x0)) jump:nt report_null_object }
{ r2 = memw(r16+#0x0); if (!cmp.eq(r2.new,#0x13)) jump:t restore_return }
{ r1 = #0x2413; r4 = memub(r18+#0x2); r2 = memub(r17+#0x3) }
{ r0 = extractu(r4,#0x1,#0x1); r3 = memw(r17+#0x1e0); memw(r29+#0x8) = r0.new }
{ r3 = extractu(r3,#0x1,#0x14); r0 = ##0x6540; memw(r29+#0x4) = r3.new }
{ call 0x159c50; memw(r29+#0x0) = r2 }
{ r0 = ##-0x4fd48f6c; r2 = memw(r17+#0x1e0); r3 = memub(r18+#0x2) }
{ r2 = extractu(r2,#0x1,#0x14); r3 = extractu(r3,#0x1,#0x1); r1 = memub(r17+#0x3) }
{ call -0xbf074 }
{ r2 = memub(r17+#0x1e2) }
{ p0 = !tstbit(r2,#0x4); if (!p0.new) jump:t switch_resources; if (!p0.new) r4 = memub(r16+#0x4) }
restore_return:
{ jump 0x1e42b8 }
switch_resources:
{ r2 = memw(r17+#0x28); r3 = memub(r16+#0x8) }
{ r2 = add(r2,#0x328) }
{ r3 = memw(r2+r3<<#0x2) }
{ r16 = memw(r2+r4<<#0x2); r0 = memw(r3+#0x24) }
{ call 0xcc }
{ call 0x1e4234; r1 = #0x1; r0 = memw(r16+#0x24) }
{ jump 0xbdee0 }
report_null_object:
{ call -0x2d0; r0 = ##-0x4fd49009; r1 = #0x153 }
