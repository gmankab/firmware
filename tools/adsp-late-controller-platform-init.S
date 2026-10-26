// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 [file 0x3e910,0x3e9e0): configure ADSP runtime, install
// callback data, probe state and hand off to shared platform lifecycle.
.text
.globl adsp_late_controller_platform_init
adsp_late_controller_platform_init:
{ immext(#4030005760); r16 = ##-264961528; memd(r29+#-16) = r17:16; allocframe(#16) }
{ r1 = add(r16,#1816); r0 = r16; memd(r29+#0) = r19:18 }
{ call add_register_block; immext(#4029460352); r18 = ##-265506928 }
{ call initialize_dispatch }
{ r0 = add(r16,#112); memw(r18+#16) = r0; immext(#4029474368); memw(##4029474384) = r18 }
{ call initialize_controller; immext(#4029477376); memw(##4029477384) = r18 }
{ call init_hardware }
{ call calibrate_controller }
{ call query_controller }
{ call calibrate_controller_repeat }
{ p0 = cmp.eq(r0,#0); if (p0.new) jump:nt retry }
{ call calibrate_controller_repeat }
{ r2 = r0 }
{ call report_capability; r1:0 = combine(r2,#0) }
{ immext(#4028567744); r2 = memw(##4028567788); immext(#4028567744); r1 = memw(##4028567784) }
{ call report_capability; r0 = add(r2,#1) }
{ jump finalize }
retry:
{ call report_capability; r0 = #0; immext(#4028567744); r1 = memw(##4028567784) }
finalize:
{ r2 = add(r16,#784); r3 = #128; r0 = memw(r16+#784); r1 = memw(r16+#788) }
{ call resource_registry; immext(#2147483584); r4 = ##2147483647 }
{ call commit_registry; immext(#4028651840); r0 = memw(##4028651840) }
{ call probe_registry }
{ r2 = add(r16,#188); memw(r16+#184) = r2.new }
{ r17:16 = memd(r29+#8); r19:18 = memd(r29+#0) }
{ jump dispatch_ready; deallocframe }
.set add_register_block, adsp_late_controller_platform_init-0x1e320
.set initialize_dispatch, adsp_late_controller_platform_init-0x4130
.set initialize_controller, adsp_late_controller_platform_init+0xe76fc
.set init_hardware, adsp_late_controller_platform_init+0x3bd0
.set calibrate_controller, adsp_late_controller_platform_init-0x114c
.set query_controller, adsp_late_controller_platform_init+0x40c
.set report_capability, adsp_late_controller_platform_init+0xd0
.set resource_registry, adsp_late_controller_platform_init+0xea524
.set commit_registry, adsp_late_controller_platform_init+0xa0c
.set probe_registry, adsp_late_controller_platform_init+0xb58
.set dispatch_ready, adsp_late_controller_platform_init+0x3ec4

.set calibrate_controller_repeat, adsp_late_controller_platform_init-0x1148
