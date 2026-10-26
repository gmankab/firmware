// SPDX-License-Identifier: GPL-3.0-only
// [file 0x3e368,0x3e3d0): classify a pair of 64-bit status descriptors,
// check its tag with two supported controller formats and report result.
.text
.globl adsp_late_controller_check_status_tag
adsp_late_controller_check_status_tag:
{ r17 = r0; memd(r29+#-16) = r17:16; allocframe(#8) }
{ call platform_registry; immext(#4029473984); r0 = ##-265493296 }
{ r3:2 = combine(#0,#0); r16 = #13; r1:0 = memd(r17+#40) }
{ p0 = cmp.eq(r1:0,r3:2); if (p0.new) jump:nt check_tag; if (p0.new) r2 = memw(r17+#16) }
report:
{ call report_status; immext(#4029473984); r0 = ##-265493296 }
return_result:
{ r0 = r16; r17:16 = memd(r29+#0); dealloc_return }
check_tag:
{ immext(#3991793088); r3 = ##-303174163; if (cmp.eq(r3.new,r2)) jump:nt no_status }
{ immext(#2913840512); p0 = cmp.eq(r2,##-1381126739); if (p0.new) r0 = add(r17,#0); if (!p0.new) jump:nt report }
{ call validate_tag }
{ r16 = r0; jump report }
no_status:
{ r16 = #0; jump report }
.set platform_registry, adsp_late_controller_check_status_tag+0xe8538
.set report_status, adsp_late_controller_check_status_tag+0xe8678
.set validate_tag, adsp_late_controller_check_status_tag-0x694
