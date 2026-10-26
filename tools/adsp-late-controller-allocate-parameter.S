// SPDX-License-Identifier: GPL-3.0-only
// [0x40d20,0x40ddc): allocate and initialize a parameter/control record.
// Validate optional entry, set initial links and publish in the registry.
.text
.globl adsp_late_controller_allocate_parameter
adsp_late_controller_allocate_parameter:
{ r17:16 = combine(r2,r0); memd(r29+#-16) = r17:16; allocframe(#16) }
{ p0 = cmp.eq(r16,#0); if (!p0.new) jump:t create_record; r19:18 = combine(#0,r1); memd(r29+#0) = r19:18 }
{ call allocate_descriptor; r0 = #204; r19 = #1 }
{ r16 = r0; if (cmp.eq(r16.new,#0)) jump:nt no_memory }
create_record:
{ call clear_descriptor; r0 = add(r16,#12); r1 = #0; r2 = #192 }
{ immext(#4026785216); r2 = ##4026785244; p0 = cmp.eq(r18,#0) }
{ if (p0) r2 = #-1; immext(#5373888); r0 = ##5373951; memw(r16+#0) = r2 }
{ if (p0) jump:nt publish_record; memw(r16+#4) = r0; memw(r16+#8) = #0 }
{ call setup_resource; r3:2 = combine(r16,#80); r18 = #0; r1:0 = combine(#3,#0) }
{ r2 = r0; if (cmp.gt(r2.new,#-1)) jump:t publish_record }
{ p0 = cmp.eq(r19,#0); if (!p0.new) r0 = add(r16,#0) }
{ immext(#4294943232); if (!p0) call free_descriptor }
{ jump return_record }
publish_record:
{ immext(#4029462912); r0 = ##-265504360; memw(r16+#20) = r2; memw(r16+#16) = r17 }
{ call lookup_platform }
{ immext(#4029462720); r3 = ##-265504544 }
lookup_pointer:
{ r2 = r3; r3 = memw(r3+#0) }
{ r3 = add(r3,#12); p0 = cmp.eq(r3,#0); if (!p0.new) jump:t lookup_pointer }
{ call publish_platform; immext(#4029462912); r0 = ##4029462936; memw(r2+#0) = r16 }
{ r18 = r16 }
return_record:
{ r0 = r18; r17:16 = memd(r29+#8); r19:18 = memd(r29+#0) }
{ dealloc_return }
no_memory:
{ r18 = #0; jump return_record }
.set allocate_descriptor, adsp_late_controller_allocate_parameter-0x61b4
.set clear_descriptor, adsp_late_controller_allocate_parameter+0x100cb0
.set setup_resource, adsp_late_controller_allocate_parameter+0xea10c
.set free_descriptor, adsp_late_controller_allocate_parameter-0x5d80
.set lookup_platform, adsp_late_controller_allocate_parameter+0xe9f8c
.set publish_platform, adsp_late_controller_allocate_parameter+0xea048
