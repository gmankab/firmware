// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x10099c,0x100a04),
// runtime [0xb00c099c,0xb00c0a04).
// If context flag0 is set, invoke the external teardown request and clear
// that flag even after a nonzero helper result. Always invoke the external
// operation at context+0x288 and write context state=1. Return the request
// helper result, or zero when no request was active.
// Explicit V66 mnemonic/packet source; no reference bytes are used at build time.
.text
.globl wlan_late_request_teardown
wlan_late_request_teardown:
{ r17:16 = combine(#0x0,r0); memd(r29+#-0x10) = r17:16; allocframe(#0x10) }
{ r2 = memw(r16+#0x284) }
{ p0 = tstbit(r2,#0); if (!p0.new) jump:t reset_state }
{ r2 = memw(r16+#0x0); r1 = memw(r16+#0x4) }
// r5 dereferences the old r2 table pointer while r3:r2 receive arguments.
{ r3:2 = combine(#0x0,#0x42); r5 = memw(r2+#0x0); r4 = memw(r16+#0x280) }
{ call -0x976a0; r0 = memw(r5+#0x230) }
{ r17 = r0; if (cmp.eq(r17.new,#0x0)) jump:nt clear_active_flag }
{ r1 = #0x1251; r0 = ##0x64b8; memw(r29+#0x0) = r17 }
{ call 0x15a7dc }
{ call -0xbdd94; r1:0 = combine(r17,##-0x4fd490c4) }
clear_active_flag:
{ r2 = add(r16,#0x284) }
{ memw(r2+#0x0) = clrbit(#0x0) }
reset_state:
{ call 0x15af6c; r0 = add(r16,#0x288); memw(r16+#0x8) = #0x1 }
// r0 reads the saved result in old r17 before the parallel register restore.
{ r0 = r17; r17:16 = memd(r29+#0x8); dealloc_return }
