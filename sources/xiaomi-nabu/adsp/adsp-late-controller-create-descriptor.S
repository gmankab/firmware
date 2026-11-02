// SPDX-License-Identifier: GPL-3.0-only
// [file 0x3e260,0x3e368): allocate and publish a controller descriptor
// with tag propagation, optional register state, and error unwinding.
.text
.globl adsp_late_controller_create_descriptor
adsp_late_controller_create_descriptor:
{ r17 = r0; memd(r29+#-16) = r17:16; allocframe(#64) }
{ immext(#4029473984); r0 = ##-265493296; memd(r29+#32) = r23:22; memd(r29+#48) = r19:18 }
{ r23:22 = combine(r2,r3); memd(r29+#40) = r21:20; memd(r29+#24) = r25:24 }
{ memd(r29+#16) = r27:26; r19:18 = memd(r1+#8) }:mem_noshuf
{ r26 = memub(r1+#32); r21:20 = memd(r1+#16) }
{ call lookup_registry; r24 = memw(r1+#36); r25 = memw(r1+#40) }
{ call allocate_record; r0 = #72 }
{ p1 = cmp.eq(r25,#0); r16 = r0; if (cmp.eq(r16.new,#0)) jump:nt cleanup }
{ r2 = #255; r5:4 = combine(r21,r20); memw(r16+#28) = r23; memw(r16+#32) = r22 }
{ p0 = bitsclr(r26,r2); r2 = #0; immext(#4026773248); if (p1) r25 = ##-268194036 }
{ r0 = mux(p0,r2,r18); r1 = mux(p0,r2,r19); r3:2 = combine(r19,r18); memw(r16+#24) = r25 }
{ r0 = r16; memw(r29+#8) = r24; memd(r29+#0) = r1:0 }
{ call setup_descriptor }
{ immext(#4027742400); r2 = ##-267224888; r18 = r0; memw(r16+#0) = r2.new }
{ r3:2 = combine(r16,#80); immext(#5373888); r4 = ##5373950; r1:0 = combine(r17,#0) }
{ call initialize_descriptor; memw(r16+#4) = r4; immext(#4026774592); memw(r16+#8) = ##-268192644 }
{ if (!p0.new) r0 = add(r16,#0); p0 = cmp.gt(r0,#-1); if (!p0.new) jump:nt release_allocation }
{ p0 = cmp.eq(r18,#0); if (p0.new) jump:nt alternate_cleanup; memw(r16+#12) = r0 }
{ call notify_descriptor }
{ jump cleanup }
release_allocation:
{ call release_record }
cleanup:
{ call release_registry; immext(#4029473984); r0 = ##4029474000; r16 = #0 }
return_result:
{ r0 = r16; r17:16 = memd(r29+#56); r19:18 = memd(r29+#48) }
{ r21:20 = memd(r29+#40); r23:22 = memd(r29+#32) }
{ r25:24 = memd(r29+#24); r27:26 = memd(r29+#16) }
{ dealloc_return }
alternate_cleanup:
{ call release_registry; immext(#4029473984); r0 = ##-265493296 }
{ jump return_result }
.set lookup_registry, adsp_late_controller_create_descriptor+0xe8640
.set allocate_record, adsp_late_controller_create_descriptor-0x36f4
.set setup_descriptor, adsp_late_controller_create_descriptor-0x790
.set initialize_descriptor, adsp_late_controller_create_descriptor+0xecbcc
.set notify_descriptor, adsp_late_controller_create_descriptor+0xecb80
.set release_record, adsp_late_controller_create_descriptor-0x32c0
.set release_registry, adsp_late_controller_create_descriptor+0xe8780
