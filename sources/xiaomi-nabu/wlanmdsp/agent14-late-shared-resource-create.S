// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x1012a0, 0x1012f4),
// runtime [0xb00c12a0, 0xb00c12f4).
// Lazily obtain a four-byte holder at context+0x39c, clear it, and request an
// external resource with constants 0x30, 9, 0x100 and context field +0x140.
// Store the returned handle. A zero handle enters the diagnostic-forwarding
// helper; its non-returning contract is inferred from the missing continuation.
// Explicit V66 mnemonic/packet source; no reference bytes are used at build time.
.text
.globl wlan_late_shared_resource_create
wlan_late_shared_resource_create:
{ r17 = r0; memd(r29+#-0x10) = r17:16; allocframe(#0x8) }
{ r16 = memw(r17+#0x39c); if (!cmp.eq(r16.new,#0x0)) jump:t initialize_holder }
{ call 0x15b680; r1:0 = combine(#0x4,#0x3) }
{ r16 = r0; memw(r17+#0x39c) = r16.new }
initialize_holder:
{ r0 = #0x30; memb(r16+#0x3) = #0; memb(r16+#0x2) = #0 }
{ r3:2 = combine(##0x100,#0x9); memb(r16+#0x1) = #0; memb(r16+#0x0) = #0 }
{ call 0x160af0; r1 = memw(r17+#0x140) }
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt report_null_handle; memw(r16+#0x0) = r0 }
{ r17:16 = memd(r29+#0x0); dealloc_return }
report_null_handle:
{ call 0xc; r1:0 = combine(#0x24,##0xb02b6fa3) }
