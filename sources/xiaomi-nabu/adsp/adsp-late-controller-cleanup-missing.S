// SPDX-License-Identifier: GPL-3.0-only
// ADSP V66 controller resource cleanup: a guarded registered buffer
// is either reported as missing or flushed and released.
// Hexagon packets and branch conditions are source-assembled, never pasted.
.text
.globl adsp_late_controller_cleanup_missing
adsp_late_controller_cleanup_missing:
{ memd(r29+#-16) = r17:16; allocframe(#16) }
{ r16 = r1 }
{ call acquire_platform_resource; r17 = memw(r16+#48) }
{ r2 = memw(r16+#32); if (cmp.eq(r2.new,#0)) jump:nt cleanup_registered }
{ call publish_empty_state; immext(#4028154112); r0 = ##4028154149; memw(r29+#0) = r16 }
{ call platform_log; immext(#4029470912); r0 = ##-265496344 }
{ r0 = #2; r17:16 = memd(r29+#8); dealloc_return }
cleanup_registered:
{ call release_record; r0 = memw(r17+#16); r1 = memw(r16+#24) }
{ r0 = add(r17,#12); r1 = r16; memw(r16+#4) = #0; memw(r16+#32) = #0 }
{ call flush_release }
{ call clear_registry; r0 = r16 }
{ call report_release; immext(#4029470912); r0 = ##-265496344 }
{ r0 = #0; r17:16 = memd(r29+#8); dealloc_return }
.set acquire_platform_resource, adsp_late_controller_cleanup_missing+0xe9b3c
.set publish_empty_state, adsp_late_controller_cleanup_missing+0xe6710
.set platform_log, adsp_late_controller_cleanup_missing+0xe9c84
.set release_record, adsp_late_controller_cleanup_missing+0x1924
.set flush_release, adsp_late_controller_cleanup_missing+0xe7614
.set clear_registry, adsp_late_controller_cleanup_missing-0x1a90
.set report_release, adsp_late_controller_cleanup_missing+0xe9c84
