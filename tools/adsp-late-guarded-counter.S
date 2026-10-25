// ADSP v66 later executable [file 0x3ab2c, 0x3ab6c):
// Check the platform UGP byte cap before notifying a controller callback,
// then atomically increment the shared lifetime counter.
.text
.globl adsp_late_guarded_counter
adsp_late_guarded_counter:
{ p0 = cmp.gt(r2,#1); r16 = r0; memd(r29+#-16) = r17:16; allocframe(#16) }
{ if (!p0) jump:nt update_count }
{ r4 = ugp }
{ r4 = memub(r4+#17); if (cmp.gt(r4.new,r2)) jump:t update_count }
{ r0 = r3; r1 = add(r29,#24); r2 = add(r29,#24); memw(r29+#4) = r2.new }
{ call platform_guarded_notify }
update_count:
{ immext(#0xf01f1000); r2 = ##4029460324; r0 = r16 }
{ memw(r2+#0) += #1 }
{ r17:16 = memd(r29+#8); dealloc_return }
.set platform_guarded_notify, adsp_late_guarded_counter+0xecb20
