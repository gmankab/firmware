// SPDX-License-Identifier: GPL-3.0-only
// [file 0x3e3d0,0x3e47c): bounded controller status/release validation.
// Check two packet tags and a zero 64-bit input; perform release or platform
// error reporting before restoring the caller frame.
.text
.globl adsp_late_controller_status_release
adsp_late_controller_status_release:
{ memd(r29+#-16) = r17:16; allocframe(#16) }
{ r16 = r0; if (cmp.eq(r16.new,#0)) jump:nt missing_controller }
{ r2 = memw(r16+#16) }
{ immext(#2913840512); r3 = ##-1381126739; if (cmp.eq(r3.new,r2)) jump:nt supported_tag }
{ immext(#3991793088); r3 = ##-303174163; if (!cmp.eq(r3.new,r2)) jump:nt bad_tag }
supported_tag:
{ call query_controller; immext(#4029473984); r0 = ##-265493296 }
{ r3:2 = combine(#0,#0); r1:0 = memd(r16+#40) }
{ p0 = cmp.eq(r1:0,r3:2); if (!p0.new) jump:t destroy_controller; if (p0.new) r2 = memw(r16+#16) }
{ immext(#3991793088); r3 = ##-303174163; if (cmp.eq(r3.new,r2)) jump:t complete_release }
destroy_controller:
{ call release_controller; r0 = r16 }
{ r17 = r0; if (cmp.eq(r17.new,#-1)) jump:nt success_status }
{ p0 = cmp.eq(r17,#0); if (!p0.new) jump:nt report_status }
complete_release:
{ call close_controller; r17 = #0; r0 = memw(r16+#12) }
{ jump report_status }
missing_controller:
{ call report_error; immext(#4028151872); r0 = ##4028151890; r17 = #4 }
{ jump return_code }
bad_tag:
{ call report_error; immext(#4028151872); r0 = ##4028151917; memw(r29+#0) = r2 }
{ r17 = #4; jump return_code }
success_status:
{ r17 = #0 }
report_status:
{ call notify_status; immext(#4029473984); r0 = ##-265493296 }
return_code:
{ r0 = r17; r17:16 = memd(r29+#8); dealloc_return }
.set query_controller, adsp_late_controller_status_release+0xe84d0
.set release_controller, adsp_late_controller_status_release-0x5f0
.set close_controller, adsp_late_controller_status_release+0xeca10
.set report_error, adsp_late_controller_status_release+0xe8a5c
.set notify_status, adsp_late_controller_status_release+0xe8610
