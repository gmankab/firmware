// SPDX-License-Identifier: GPL-3.0-only
// [file 0x40f80,0x41008): remove one controller registry node,
// revise the linked list and submit completion to the platform logger.
.text
.globl adsp_late_controller_remove_registry
adsp_late_controller_remove_registry:
{ immext(#4029462720); r17:16 = combine(##-265504544,r0); memd(r29+#-16) = r17:16; allocframe(#8) }
{ call find_global; immext(#4029462912); r0 = ##-265504360 }
{ r2 = memw(r17+#0); if (cmp.eq(r2.new,#0)) jump:nt registry_removed }
walk_registered:
{ p0 = !cmp.eq(r2,r16); if (p0.new) r17 = add(r2,#12); if (!p0.new) jump:nt replace_head; if (p0.new) r3 = memw(r2+#12) }
{ r2 = r3; p0 = cmp.eq(r3,#0); if (!p0.new) jump:t walk_registered }
{ jump registry_removed }
replace_head:
{ r2 = memw(r16+#12); memw(r17+#0) = r2.new }
registry_removed:
{ call update_global_registry; immext(#4029462912); r0 = ##-265504360 }
{ call notify_parameter; r2 = #81; r1:0 = combine(#3,#0); r3 = memw(r16+#20) }
{ call clear_parameter_state; immext(#4029470912); r0 = ##-265496344 }
{ call release_parameter; r0 = add(r16,#24); r1 = #0; r3:2 = combine(#0,#-1) }
{ immext(#4029470912); r0 = ##-265496344; r17:16 = memd(r29+#0); deallocframe }
{ jump controller_return }
.set find_global, adsp_late_controller_remove_registry+0xe9d2c
.set update_global_registry, adsp_late_controller_remove_registry+0xe9de8
.set notify_parameter, adsp_late_controller_remove_registry+0xe9eac
.set clear_parameter_state, adsp_late_controller_remove_registry+0xe92d8
.set release_parameter, adsp_late_controller_remove_registry+0xa90
.set controller_return, adsp_late_controller_remove_registry+0xe9420
