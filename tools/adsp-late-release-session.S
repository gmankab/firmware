// ADSP v66 late R-E executable [file 0x3a18c,0x3a214): release the
// selected session record, optionally invoke its callback, unlink it, and
// return a zero result. The 3 padding NOPs that follow are NOT included.
.text
.globl adsp_late_release_session
adsp_late_release_session:
{ r16 = r1; memd(r29+#-16) = r17:16; allocframe(#16) }
{ memd(r29+#0) = r19:18; r2 = memw(r16+#24) }:mem_noshuf
{ r17 = memw(r16+#32) }
{ r18 = memw(r2+#0); r19 = memw(r2+#4) }
{ call platform_prepare_release; r1:0 = combine(r18,#3); r2 = r19 }
{ r1 = r18; r2 = memw(r17+#24); immext(#0xf02cb000); r0 = memw(##4029474384) }
{ r3 = r19; r2 = memw(r2+#0) }
{ call platform_link_release }
{ call platform_commit_release; r0 = memw(r16+#40) }
{ call platform_close_release; r0 = memw(r16+#24) }
{ r0 = memw(r17+#20) }
{ p0 = cmp.eq(r0,#0); if (!p0.new) r1 = memw(r17+#24) }
{ immext(#0x7e40); if (!p0) call optional_release }
{ r1 = r16; immext(#0xf02cbc00); r2 = memw(##4029477384) }
{ call platform_verify_release; r0 = add(r2,#12) }
{ call platform_finalize_release; r0 = r17 }
{ r0 = #0; r17:16 = memd(r29+#8); r19:18 = memd(r29+#0) }
{ dealloc_return }
.set platform_prepare_release, adsp_late_release_session+0x54f8
.set platform_link_release, adsp_late_release_session+0x1bd4
.set platform_commit_release, adsp_late_release_session+0x974
.set platform_close_release, adsp_late_release_session+0x4ad8
.set optional_release, adsp_late_release_session+0x7eb4
.set platform_verify_release, adsp_late_release_session+0x5968
.set platform_finalize_release, adsp_late_release_session+0x58f4
