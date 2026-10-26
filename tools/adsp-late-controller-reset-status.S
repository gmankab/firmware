// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 [file 0x3e750,0x3e798): reset controller status safely,
// retain the returned diagnostic value and release the saved-register frame.
.text
.globl adsp_late_reset_status
adsp_late_reset_status:
{ memd(r29+#-16) = r17:16; allocframe(#8) }
{ r16 = r0; if (cmp.eq(r16.new,#0)) jump:nt null_context }
{ call read_platform; immext(#0xf02cb000); r0 = ##-265493296 }
{ call prepare_registry; r0 = #0; r17 = #0 }
{ call finalize_platform; immext(#0xf02cb000); r0 = ##-265493296; memd(r16+#0) = r1:0 }
{ jump end }
null_context:
{ call report_bad_context; immext(#4028151936); r0 = ##4028151962; r17 = #4 }
end:
{ r0 = r17; r17:16 = memd(r29+#0); dealloc_return }
.set read_platform, adsp_late_reset_status+0xe8150
.set prepare_registry, adsp_late_reset_status-0xf24
.set finalize_platform, adsp_late_reset_status+0xe8290
.set report_bad_context, adsp_late_reset_status+0xe86dc
