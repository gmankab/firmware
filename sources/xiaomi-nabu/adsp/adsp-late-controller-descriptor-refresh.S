// SPDX-License-Identifier: GPL-3.0-only
// Validate a controller descriptor tag and refresh the cached state before
// tail-jumping to the independently reconstructed shutdown function.
.text
.globl adsp_late_controller_descriptor_refresh
adsp_late_controller_descriptor_refresh:
{ memd(r29+#-16) = r17:16; allocframe(#8) }
{ r2 = memw(r16+#16) }
{ immext(#2913840512); p0 = cmp.eq(r2,##-1381126739); if (!p0.new) jump:t use_cached_tag }
{ call reset_descriptor_state; immext(#4029473984); r0 = ##-265493296 }
{ call get_controller_handle; r0 = r16 }
{ call attach_controller_buffer }
{ immext(#4029473984); r0 = ##-265493296; immext(#3755990976); r2 = ##-538976289 }
{ call publish_descriptor_state; memw(r16+#16) = r2 }
use_cached_tag:
{ r0 = r16; r17:16 = memd(r29+#0); deallocframe }
{ jump shared_shutdown_controller }
.set reset_descriptor_state, adsp_late_controller_descriptor_refresh+0xe8420
.set get_controller_handle, adsp_late_controller_descriptor_refresh-0xc40
.set attach_controller_buffer, adsp_late_controller_descriptor_refresh-0x860
.set publish_descriptor_state, adsp_late_controller_descriptor_refresh+0xe8560
.set shared_shutdown_controller, adsp_late_controller_descriptor_refresh-0x34e0
