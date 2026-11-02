// ADSP v66 later executable [file 0x3ab6c, 0x3abb4):
// Invoke optional before/after callbacks around a controller probe.
.text
.globl adsp_late_probe_callbacks
adsp_late_probe_callbacks:
{ r16 = r0; memd(r29+#-16) = r17:16; allocframe(#8) }
{ immext(#0xf01f1000); r2 = memw(r17=##4028568888) }
{ r3 = memw(r2+#36); if (cmp.eq(r3.new,#0)) jump:nt no_before }
{ r0 = r2; callr r3 }
{ r2 = memw(r17+#0) }
no_before:
{ call probe_operation; r1:0 = combine(r16,r2) }
{ r16 = r0; r2 = memw(r17+#0) }
{ r3 = memw(r2+#40); if (cmp.eq(r3.new,#0)) jump:nt finish }
{ r0 = r2; callr r3 }
finish:
{ r0 = r16; r17:16 = memd(r29+#0); dealloc_return }
.set probe_operation, adsp_late_probe_callbacks+0x54
