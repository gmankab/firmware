// ADSP V66 [file 0x3afa0,0x3afe4): invoke registered pre/post shutdown
// callbacks for a controller handle, using platform cleanup in between.
.text
.globl adsp_late_controller_shutdown
adsp_late_controller_shutdown:
{ memd(r29+#-16) = r17:16; allocframe(#8) }
{ r16 = r0; if (cmp.eq(r16.new,#0)) jump:nt return_empty }
{ immext(#0xf01f1500); r0 = memw(r17=##4028568888) }
{ r2 = memw(r0+#36); if (cmp.eq(r2.new,#0)) jump:nt no_pre_callback }
{ callr r2 }
{ r0 = memw(r17+#0) }
no_pre_callback:
{ call platform_cleanup; r1 = r16 }
{ r0 = memw(r17+#0) }
{ r2 = memw(r0+#40); if (cmp.eq(r2.new,#0)) jump:nt return_empty }
{ callr r2 }
{ r17:16 = memd(r29+#0); dealloc_return }
return_empty:
{ r17:16 = memd(r29+#0); dealloc_return }
.set platform_cleanup, adsp_late_controller_shutdown+0x44
