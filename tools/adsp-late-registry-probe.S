// Source-authored ADSP late registry health/activation helper.
// File [0x392c8,0x39318), branches to the external error path.
.text
.globl adsp_late_registry_probe
adsp_late_registry_probe:
{ r0 = #-2; memd(r29+#-16) = r17:16; allocframe(#8) }
{ call registry_allocator; r16 = memw(##4028568888) }
{ call registry_allocator; r0 = ##65534; r17 = add(r0,#1) }
{ call platform_query; r2 = add(r0,#1); r1:0 = combine(r17,r16) }
{ call registry_allocator; r0 = #-2 }
{ call registry_allocator; r0 = ##65534; r16 = add(r0,#1) }
{ r1 = add(r0,#1); r0 = r16; r17:16 = memd(r29+#0); deallocframe }
{ jump transfer_control }
.set registry_allocator, adsp_late_registry_probe-0x98
.set platform_query, adsp_late_registry_probe+0x23a8
.set transfer_control, adsp_late_registry_probe+0x4508
