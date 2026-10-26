// SPDX-License-Identifier: GPL-3.0-only
// [file 0x422c0,0x4238c): safely insert and link an encoded controller
// descriptor into a range ordered singly linked list.
.text
.globl adsp_late_controller_linked_list_insert
adsp_late_controller_linked_list_insert:
{ r3 = add(r0,#168); allocframe(#16) }
{ r2 = memw(r0+#168); if (cmp.eq(r2.new,#0)) jump:nt no_entries }
{ r4 = memw(r1+#0) }
scan_entries:
{ r5 = memw(r2+#0); if (!cmp.gtu(r4,r5.new)) jump:nt behind_entry }
{ r7 = sub(r4,r5) }
{ r6 = memw(r2+#4); if (cmp.gtu(r6.new,r7)) jump:nt report_size_overflow }
{ r2 = memw(r2+#16); if (!cmp.eq(r2.new,#0)) jump:t scan_entries }
no_entries:
{ r2 = memw(r0+#172); memw(r1+#20) = r2.new }
{ if (!p0.new) r3 = add(r2,#16); p0 = cmp.eq(r2,#0); r2 = #0; memw(r1+#16) = #0 }
{ r0 = r2; memw(r3+#0) = r1; memw(r0+#172) = r1 }
{ dealloc_return }
behind_entry:
{ r7 = sub(r5,r4) }
{ r6 = memw(r1+#4); if (!cmp.gtu(r6.new,r7)) jump:t append_entry }
{ immext(#4028154944); r0 = ##-266812324; r2 = memw(r2+#4); memw(r29+#12) = r2.new }
{ memw(r29+#8) = r5; memw(r29+#4) = r6 }
{ call report_out_of_bounds; memw(r29+#0) = r4 }
{ r2 = #-1 }
{ r0 = r2; dealloc_return }
report_size_overflow:
{ immext(#4028154944); r0 = ##-266812324; r2 = memw(r1+#4); memw(r29+#12) = r6 }
{ memw(r29+#8) = r5; memw(r29+#4) = r2 }
{ call report_out_of_bounds; memw(r29+#0) = r4 }
{ r2 = #-1 }
{ r0 = r2; dealloc_return }
append_entry:
{ r0 = #0; r4 = memw(r2+#20); memw(r1+#20) = r4.new }
{ r0 = r0; memw(r2+#20) = r1; memw(r1+#16) = r2 }
{ r4 = memw(r1+#20) }
{ p0 = cmp.eq(r4,#0); if (!p0.new) r3 = add(r4,#16) }
{ memw(r3+#0) = r1 }
{ dealloc_return }
.set report_out_of_bounds, adsp_late_controller_linked_list_insert+0xe4b6c
