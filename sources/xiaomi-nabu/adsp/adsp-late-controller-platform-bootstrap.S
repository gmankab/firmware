// SPDX-License-Identifier: GPL-3.0-only
// [file 0x3e150,0x3e260): initial platform controller allocation, tagged
// register state and return through an external cleanup trampoline.
.text
.globl adsp_late_controller_platform_bootstrap
adsp_late_controller_platform_bootstrap:
{ r0 = #4096; r17 = #4096; memd(r29+#-16) = r17:16; allocframe(#24) }
{ memd(r29+#8) = r19:18; memd(r29+#0) = r21:20 }
{ call allocate_controller; immext(#4029458112); r18 = memw(##4029458124) }
{ r16 = r0; if (cmp.eq(r16.new,#0)) jump:nt null_controller }
{ immext(#4029460352); r1:0 = combine(#0,##4029460352); immext(#4029473920); r19 = ##-265493372 }
{ call map_controller; r20 = #0; r18 = add(r18,#1) }
{ immext(#16711680); r2 = ##16711680; immext(#4294901504); r0 = ##-65792 }
{ r0 = add(r19,#24); memw(r19+#40) = r2; memw(r19+#44) = r0 }
{ immext(#4028151808); r1 = ##-266815454; r2 = #16; memb(r19+#24) = r20 }
{ memw(r19+#48) = r20; memw(r19+#52) = r20 }
{ call setup_controller; memh(r19+#56) = r20 }
{ immext(#65472); r2 = ##65533; memb(r19+#39) = r20; memw(r19+#48) = r17 }
{ memw(r19+#52) = r16; memh(r19+#42) = r18 }
{ memh(r19+#46) = r2 }
{ r2 = ugp }
{ r0 = r19; immext(#4026773696); r2 = ##-268193584; memb(r19+#16) = r20 }
{ call platform_initialize; memw(r19+#4) = r2 }
{ r1:0 = combine(r19,#3); immext(#4026773696); r2 = ##-268193584 }
{ immext(#4029473984); r3 = ##-265493280; immext(#4029474112); r4 = ##-265493168 }
{ call finish_mapping; memw(r19+#72) = r4 }
{ call init_registry; immext(#4029460352); r0 = ##-265506944 }
{ call unlock_registry; immext(#4028151808); r0 = ##-266815440 }
{ r17:16 = memd(r29+#16); r19:18 = memd(r29+#8) }
{ r21:20 = memd(r29+#0); dealloc_return }
null_controller:
{ call unlock_registry; immext(#4028151808); r0 = ##-266815480 }
{ r1 = #451; immext(#4028151744); r0 = ##4028151794; r17:16 = memd(r29+#16) }
{ r19:18 = memd(r29+#8); r21:20 = memd(r29+#0) }
{ jump report_failure; deallocframe }
.set allocate_controller, adsp_late_controller_platform_bootstrap-0x35e4
.set map_controller, adsp_late_controller_platform_bootstrap+0xeaa34
.set setup_controller, adsp_late_controller_platform_bootstrap+0x3c028
.set platform_initialize, adsp_late_controller_platform_bootstrap+0xeb690
.set finish_mapping, adsp_late_controller_platform_bootstrap+0x4c10
.set init_registry, adsp_late_controller_platform_bootstrap+0xecdd4
.set unlock_registry, adsp_late_controller_platform_bootstrap+0xe8cdc
.set report_failure, adsp_late_controller_platform_bootstrap+0x7a4
