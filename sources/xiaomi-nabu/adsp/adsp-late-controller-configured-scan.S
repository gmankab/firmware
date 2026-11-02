// SPDX-License-Identifier: GPL-3.0-only
// [file 0x41440,0x4149c): controller configuration registry scanner.
// Walk the registered callback descriptors and select an appropriate state.
.text
.globl adsp_late_controller_configured_scan
adsp_late_controller_configured_scan:
{ r17 = r0; memd(r29+#-16) = r17:16; allocframe(#16) }
{ immext(#4029462912); r0 = ##4029462936; r18 = r1; memd(r29+#0) = r19:18 }
{ call lookup_registry; r16 = #0 }
{ immext(#4029462720); r19 = memw(##4029462752); if (cmp.eq(r19.new,#0)) jump:nt finished }
registry_loop:
{ call inspect_descriptor; r0 = r17; r2 = r18; r1 = memw(r19+#16) }
{ if (p0.new) r16 = add(r19,#0); p0 = cmp.eq(r0,#0); if (p0.new) jump:nt finished }
{ r19 = memw(r19+#12); if (!cmp.eq(r19.new,#0)) jump:t registry_loop }
finished:
{ call finish_scan; immext(#4029462912); r0 = ##-265504360 }
{ r0 = r16; r17:16 = memd(r29+#8); r19:18 = memd(r29+#0) }
{ dealloc_return }
.set lookup_registry, adsp_late_controller_configured_scan+0xe986c
.set inspect_descriptor, adsp_late_controller_configured_scan+0xfe190
.set finish_scan, adsp_late_controller_configured_scan+0xe9928
