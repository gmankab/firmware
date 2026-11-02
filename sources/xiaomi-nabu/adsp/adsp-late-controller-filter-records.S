// SPDX-License-Identifier: GPL-3.0-only
// [file 0x3be9c,0x3bfb8): select and filter typed controller records.
// Bounded record scan, two callback fallbacks, and packed 64-bit mask tests.
.text
.globl adsp_late_controller_filter_records
adsp_late_controller_filter_records:
{ jump filter_entry }
filter_entry:
{ r16 = r1; memd(r29+#-16) = r17:16; allocframe(#72) }
{ r19 = r0; r0 = add(r29,#8); memd(r29+#56) = r19:18; memd(r29+#48) = r21:20 }
{ r20 = r2; memd(r29+#40) = r23:22; memd(r29+#32) = r25:24 }
{ call acquire_filter; memd(r29+#24) = r27:26 }
{ p0 = cmp.eq(r0,#0); if (p0.new) jump:nt scan_controller }
{ call log_filter; r1:0 = combine(#0,#0); immext(#4028149824); r3:2 = combine(##-266817440,#2) }
{ r17 = r0; jump return_status }
scan_controller:
{ p0 = cmp.eq(r16,#0); if (p0.new) jump:nt commit_final_record; if (p0.new) r2 = memw(r29+#12); r23 = memw(r29+#8) }
{ p0 = cmp.gt(r23,#16); r21 = #16; r17 = #0; r18 = memw(r16+#0) }
{ r26 = #0; r22 = r18; if (!p0) jump:nt report_results }
{ immext(#1073741824); r27 = ##1073741824 }
{ r25:24 = combine(#0,#0); r17 = #0; r22 = r18 }
filter_loop:
{ call find_next_record; r1 = add(r29,#8); r0 = r21 }
{ r3:2 = combine(r27,r26); p0 = cmp.eq(r0,#0); if (!p0.new) jump:t process_next; if (p0.new) r1 = memw(r29+#12) }
{ r5:4 = combine(#0,r1); r0 = memw(r29+#8) }
{ r3:2 &= asl(r5:4,#32); r4 = extractu(r1,#7,#20) }
{ p0 = cmp.eq(r3:2,r25:24); if (!p0.new) r4 = #0 }
{ r2 = extractu(r0,#4,#24); p0 = !cmp.eq(r4,r19); if (p0.new) jump:t process_next }
{ p0 = tstbit(r20,r2); if (!p0.new) r17 = add(r17,#1); if (!p0.new) memd(r22++#8) = r1:0 }
process_next:
{ r21 = add(r21,#1); if (!cmp.eq(r21.new,r23)) jump:t filter_loop }
report_results:
{ immext(#4028149888); r0 = ##-266817404; memw(r29+#4) = r18; memw(r29+#0) = r17 }
{ call report_records }
{ call publish_summary; r1:0 = combine(r17,r18); immext(#4026765184); r3:2 = combine(#8,##4026765240) }
{ p0 = cmp.eq(r0,#0); immext(#4028149888); if (!p0.new) r0 = ##-266817371; if (!p0.new) memw(r29+#0) = r18 }
{ immext(#962176); if (!p0) call report_records }
{ jump return_status; memw(r16+#0) = r22 }
commit_final_record:
{ r17 = add(r23,sub(#-16,r2)) }
return_status:
{ r0 = r17; r17:16 = memd(r29+#64); r19:18 = memd(r29+#56) }
{ r21:20 = memd(r29+#48); r23:22 = memd(r29+#40) }
{ r25:24 = memd(r29+#32); r27:26 = memd(r29+#24) }
{ dealloc_return }
.set acquire_filter, adsp_late_controller_filter_records+0xee200
.set log_filter, adsp_late_controller_filter_records-0x1370
.set find_next_record, adsp_late_controller_filter_records+0xee12c
.set report_records, adsp_late_controller_filter_records+0xeaf90
.set publish_summary, adsp_late_controller_filter_records+0x140344
