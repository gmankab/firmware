// SPDX-License-Identifier: GPL-3.0-only
// Source-reencoded original Hexagon V66 shared-ISA executable at ADSP file 0x3f6cc.
.text
.globl adsp_late_qurt_atomic_context
adsp_late_qurt_atomic_context:
// Atomically unlink the controller's active context through a guarded
// 64-bit update and retry loop. The predicate exits into a callback.
retry:
{ r4 = r0; r3:2 = memd_locked(r0) }
{ p0 = cmp.eq(r2,#0); if (p0.new) jump:nt return_context; if (!p0.new) r0 = add(r2,#0); if (p0.new) r3 = memw(r0+#8); if (!p0.new) r2 = memw(r2+#0) }
{ memd_locked(r4,p0) = r3:2 }
return_context:
{ if (p0) jumpr:nt r3 }
{ r0 = r4; jump retry }
