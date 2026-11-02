// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 [file 0x3ea50,0x3ec04): build ADSP service bootstrap;
// initialize three linked metadata
// records and bounded dynamic buffers; fail through platform diagnostics.
.text
.globl adsp_late_controller_service_root
adsp_late_controller_service_root:
{ r17:16 = combine(r1,r0); r0 = #6; memd(r29+#-16) = r17:16; allocframe(#32) }
{ immext(#4030005760); r21 = ##-264961528; memd(r29+#8) = r21:20; memd(r29+#16) = r19:18 }
{ call prepare_service; memd(r29+#0) = r23:22 }
{ r19:18 = combine(r1,r0); r22 = #3; r20 = add(r21,#448) }
{ immext(#4026655552); r19 = ##-268311712 }
linked_entry_loop:
{ call link_entry; r1:0 = combine(r19,r20); r3:2 = combine(#0,#3) }
{ r20 = add(r20,#112); r19 = add(r19,#320); r22 = add(r22,#-1); r2 = memw(r0+#28) }
{ p0 = cmp.eq(r22,#0); r3 = memw(r0+#24); memw(r2+#24) = r3.new }
{ if (!p0) jump:nt linked_entry_loop; memw(r3+#28) = r2 }
{ immext(#4030005760); r1:0 = combine(r18,##-264961528); r18 = #0; r3:2 = combine(#0,#3) }
{ call link_entry }
{ immext(#16711680); r0 = ##16711680; immext(#4294901504); r1 = ##-65792 }
{ immext(#4026776576); r2 = ##-268190720; memw(r21+#68) = r2.new }
{ r0 = add(r21,#112); memd(r21+#152) = r1:0; memw(r21+#72) = r18 }
{ memb(r21+#136) = r18; memh(r21+#168) = r18 }
{ call register_runtime; memw(r21+#160) = r17; memw(r21+#164) = r16 }
{ immext(#4028651840); r2 = memb(##4028651876); if (!cmp.eq(r2.new,#0)) jump:nt live_controller }
{ immext(#4028670208); r2 = memw(##4028670212); if (cmp.eq(r2.new,#0)) jump:t startup }
live_controller:
{ immext(#4028670592); r1 = memw(##4028670608) }
{ p0 = cmp.gt(r1,#-1); if (p0.new) r0 = #16 }
{ immext(#2816); if (p0) call prepare_service }
startup:
{ call initialize_hardware }
{ call connect_services }
{ immext(#4029457600); r2 = memw(##4029457656) }
{ r2 = add(r2,#7) }
{ r16 = and(r2,#-8) }
{ r19 = add(r16,#112); immext(#4029486208); r3 = memw(##4029486208); if (!cmp.eq(r3.new,#1)) jump:t secondary_resource }
{ r18 = add(r16,#192) }
{ call allocate_buffer; r0 = r18 }
{ r17 = r0; if (cmp.eq(r17.new,#0)) jump:nt abort_first }
{ call clear_buffer; r1 = add(r17,r18); r0 = r17 }
{ call allocate_buffer; r0 = #256 }
{ r4 = r0; if (!cmp.eq(r4.new,#0)) jump:t link_allocations }
{ call abort_missing; r1 = #328; immext(#4028152064); r0 = ##-266815201 }
secondary_resource:
{ r18 = add(r16,#448) }
{ call allocate_buffer; r0 = r18 }
{ r17 = r0; if (cmp.eq(r17.new,#0)) jump:nt abort_second }
{ call clear_buffer; r1 = add(r17,r18); r0 = r17 }
{ r4 = r19 }
{ r4 += add(r17,#76) }
link_allocations:
{ r3 = add(r17,r19); r2 = add(r17,r16); r1:0 = combine(r16,r17) }
{ call finalize_service; memw(r3+#72) = r4 }
{ r17:16 = memd(r29+#24); r19:18 = memd(r29+#16) }
{ r21:20 = memd(r29+#8); r23:22 = memd(r29+#0) }
{ jump service_main; deallocframe }
abort_first:
{ call abort_missing; r1 = #323; immext(#4028152064); r0 = ##-266815201 }
abort_second:
{ call abort_missing; r1 = #335; immext(#4028152064); r0 = ##-266815201 }
{ jumpr r31 }
.set prepare_service, adsp_late_controller_service_root+0xbfc
.set link_entry, adsp_late_controller_service_root+0x3ad4
.set register_runtime, adsp_late_controller_service_root+0xead90
.set initialize_hardware, adsp_late_controller_service_root+0xa80
.set connect_services, adsp_late_controller_service_root+0x7b00
.set allocate_buffer, adsp_late_controller_service_root-0x3ee4
.set clear_buffer, adsp_late_controller_service_root-0x1e460
.set abort_missing, adsp_late_controller_service_root-0x15c
.set finalize_service, adsp_late_controller_service_root+0x4e00
.set service_main, adsp_late_controller_service_root+0x7b28
