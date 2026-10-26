// SPDX-License-Identifier: GPL-3.0-only
// ADSP V66 [0x40798,0x40880): unlink a controller resource from its
// sibling chain, notify the platform registry and invoke an optional hook.
.text
.globl adsp_late_controller_unlink_resource
adsp_late_controller_unlink_resource:
{ memd(r29+#-16) = r17:16; allocframe(#16) }
{ call acquire_platform_resource; r16 = r1; memd(r29+#0) = r19:18 }
{ r2 = memw(r16+#44); r3 = memw(r16+#4) }
{ p0 = cmp.eq(r3,#255); r17 = memw(r16+#48); r2 = memw(r2+#24) }
{ r2 = memw(r2+#0) }
{ r17 += asl(r2,#12); if (!p0) jump:nt normal_class }
{ r1 = lsr(r17,#12); r0 = #3; r2 = memw(r16+#24) }
{ call notify_special; r2 = memw(r2+#4) }
normal_class:
{ r17 = lsr(r17,#12); r3 = memw(r16+#24); r18 = memw(r16+#52) }
{ r1:0 = combine(r17,r18) }
{ call update_state; r2 = memw(r3+#0); r3 = memw(r3+#4) }
{ immext(#4029474368); r0 = memw(##4029474384); if (cmp.eq(r0.new,r18)) jump:nt skip_second_update }
{ r1 = r17; r3 = memw(r16+#24) }
{ call update_state; r2 = memw(r3+#0); r3 = memw(r3+#4) }
skip_second_update:
{ r2 = memw(r16+#44) }
{ r3 = add(r2,#32) }
scan_links:
{ r2 = r3; r3 = memw(r3+#0) }
{ p0 = cmp.eq(r3,#0); if (p0.new) jump:nt link_end }
{ p1 = !cmp.eq(r3,r16); r3 = add(r3,#28); if (p1.new) jump:t scan_links }
link_end:
{ immext(#4029470912); if (p0) r0 = ##-265496344; if (p0) jump:nt error_exit }
{ r3 = memw(r16+#28); memw(r2+#0) = r3.new }
{ r0 = memw(r16+#20) }
{ p0 = cmp.eq(r0,#0); if (!p0.new) r1 = memw(r16+#24) }
{ immext(#6144); if (!p0) call cleanup_auxiliary }
{ r2 = memw(r16+#44) }
{ r17 = memw(r2+#16) }
{ call post_unlink; r0 = r17 }
{ p0 = cmp.eq(r0,#1); if (!p0.new) jump:t after_user_hook }
{ r0 = r17; r2 = memw(r17+#8) }
{ callr r2 }
after_user_hook:
{ call release_resource; r0 = r16 }
{ call platform_log; immext(#4029470912); r0 = ##-265496344 }
{ r0 = #0; jump epilogue }
error_exit:
{ call platform_log }
{ r0 = #1 }
epilogue:
{ r17:16 = memd(r29+#8); r19:18 = memd(r29+#0) }
{ dealloc_return }
.set acquire_platform_resource, adsp_late_controller_unlink_resource+0xe9ac0
.set notify_special, adsp_late_controller_unlink_resource-0x1114
.set update_state, adsp_late_controller_unlink_resource-0x4a38
.set cleanup_auxiliary, adsp_late_controller_unlink_resource+0x18a8
.set post_unlink, adsp_late_controller_unlink_resource+0xea2b8
.set release_resource, adsp_late_controller_unlink_resource-0xd18
.set platform_log, adsp_late_controller_unlink_resource+0xe9c08
