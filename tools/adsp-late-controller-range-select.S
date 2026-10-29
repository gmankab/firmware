// SPDX-License-Identifier: GPL-3.0-only
// ADSP [file 0x42390, 0x42468): bounded range selection and controller
// callback dispatch, preserving loop0/endloop0 parse markers.
.text
.globl adsp_late_controller_range_select
adsp_late_controller_range_select:
{ r17:16 = combine(r2,r3); memd(r29+#-16) = r17:16; allocframe(#48) }
{ r19:18 = combine(r0,r1); memd(r29+#32) = r19:18; memd(r29+#24) = r21:20 }
{ r21 = add(r19,#168); r22 = add(r19,#172); p0 = cmp.eq(r18,#0); memd(r29+#16) = r23:22 }
{ r0 = p0; jump prepare_all; memw(r29+#4) = r0.new }
entry_lookup:
{ r1 = add(r5,sub(#1,r2)); call query_range; r3 = #1; r0 = r19 }
{ r20 = r0; if (cmp.eq(r20.new,#0)) jump:nt finish }
{ r2 = memw(r20+#16) }
{ if (p0.new) r3 = add(r22,#0); if (!p0.new) r3 = add(r2,#20); p0 = cmp.eq(r2,#0); r1 = memw(r20+#20) }
{ memw(r3+#0) = r1; r2 = memw(r20+#20) }:mem_noshuf
{ if (p0.new) r3 = add(r21,#0); if (!p0.new) r3 = add(r2,#16); p0 = cmp.eq(r2,#0); r0 = memw(r20+#16) }
{ memw(r3+#0) = r0; r2 = memw(r29+#4) }:mem_noshuf
{ p0 = r2; if (!p0.new) r1:0 = combine(r20,r18); if (p0.new) jump:nt remove_current }
{ call registry_update }
{ if (p0.new) r1:0 = combine(r20,r18); p0 = cmp.eq(r0,#0); if (!p0.new) jump:t remove_current }
{ call memory_rebuild }
{ jump prepare_all }
remove_current:
{ call release_old; r0 = r20 }
prepare_all:
{ call begin_check; r0 = r19 }
{ loop0(loop_body,#21); r3 = #0 }
loop_body:
{ r4 = memw(r19+r3<<#3); if (cmp.eq(r4.new,#0)) jump:nt loop_done }
range_check:
{ r2 = memw(r4+#0); r5 = memw(r4+#4) }
{ r5 += add(r2,#-1); r2 = maxu(r2,r17) }
{ r5 = minu(r5,r16); if (!cmp.gtu(r2,r5.new)) jump:nt entry_lookup }
{ r4 = memw(r4+#8); if (!cmp.eq(r4.new,#0)) jump:t range_check }
loop_done:
{ r3 = add(r3,#1); nop }:endloop0
finish:
{ r17:16 = memd(r29+#40); r19:18 = memd(r29+#32) }
{ r21:20 = memd(r29+#24); r23:22 = memd(r29+#16) }
{ dealloc_return }
.set query_range, adsp_late_controller_range_select-0xef0
.set registry_update, adsp_late_controller_range_select-0xd0
.set memory_rebuild, adsp_late_controller_range_select-0x350
.set release_old, adsp_late_controller_range_select-0x372c
.set begin_check, adsp_late_controller_range_select-0x510
