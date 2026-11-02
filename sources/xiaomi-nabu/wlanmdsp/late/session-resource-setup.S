// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x101304, 0x101394),
// runtime [0xb00c1304, 0xb00c1394).
// For context kind 2, acquire or reuse the object at context+0x540 and clear
// its 0x30-byte contents. Initialize its embedded callback object and register
// the event callback at 0xb00c1524. A null allocation takes diagnostic forwarding.
// All allocator, timer-like, and registration helpers remain external.
// Explicit V66 mnemonic/packet source; no reference bytes are used at build time.
.text
.globl wlan_late_session_resource_setup
wlan_late_session_resource_setup:
{ r16 = r0; memd(r29+#-0x10) = r17:16; allocframe(#0x10) }
{ r2 = memb(r16+#0x0); if (!cmp.eq(r2.new,#0x2)) jump:t restore_return }
{ r17 = memw(r16+#0x540); if (!cmp.eq(r17.new,#0x0)) jump:t initialize_session_object }
{ r2 = memw(gp+#0x78c) }
{ r2 = memw(r2+#0x39c) }
{ call 0x160e08; r0 = memw(r2+#0x0) }
{ r17 = r0; memw(r16+#0x540) = r17.new }
{ p0 = cmp.eq(r17,#0x0); if (p0.new) jump:nt report_null_object }
initialize_session_object:
{ call -0xbf390; r2 = #0x30; r1:0 = combine(#0,r17) }
{ r1 = #0x2411; r0 = ##0x64f0; r2 = memub(r16+#0x3) }
{ call 0x159e68; memw(r29+#0x0) = r2 }
{ call -0xbe70c; r0 = ##0xb02b701c; r1 = memub(r16+#0x3) }
{ r0 = add(r17,#0x10); r1 = ##0xb00c1394; r2 = r16 }
{ call 0x155a30 }
{ r1:0 = combine(##-0x4ff3eadc,r16); r2 = r16; r17:16 = memd(r29+#0x8) }
{ jump -0x88ea8; deallocframe }
restore_return:
{ r17:16 = memd(r29+#0x8); dealloc_return }
report_null_object:
{ call -0x94; r1:0 = combine(#0x36,##0xb02b6fba) }
