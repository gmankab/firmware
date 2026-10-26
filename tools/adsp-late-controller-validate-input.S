// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 [file 0x3e658,0x3e750): validate command payload,
// bound 64-bit input sizes,
// and write validated response descriptors, with nullable error paths.
.text
.globl adsp_late_validate_controller
adsp_late_validate_controller:
{ r17:16 = combine(r1,r0); memd(r29+#-16) = r17:16; allocframe(#24) }
{ p0 = cmp.eq(r16,#0); if (p0.new) jump:nt null_controller; memd(r29+#8) = r19:18 }
{ p0 = cmp.eq(r17,#0); if (p0.new) jump:nt null_controller; if (!p0.new) r2 = memw(r16+#16) }
{ immext(#2913840512); r3 = ##-1381126739; if (cmp.eq(r3.new,r2)) jump:nt recognized_tag }
{ immext(#3991793088); r3 = ##-303174163; if (!cmp.eq(r3.new,r2)) jump:nt bad_command_type }
recognized_tag:
{ call controller_registry; immext(#4029473984); r0 = ##-265493296 }
{ r0 = #0; immext(#2930683520); r2 = ##-1364283730; memw(r17+#0) = r2.new }
{ call allocate_record; r18 = #0 }
{ r19 = #0; r3:2 = memd(r16+#56) }
{ p0 = cmp.gtu(r3:2,r1:0); if (!p0.new) jump:t short_record }
{ call limit_record; r1:0 = sub(r3:2,r1:0) }
{ r19:18 = combine(r1,r0) }
short_record:
{ r3:2 = combine(#0,#0); r1:0 = memd(r16+#40); memd(r17+#24) = r19:18 }
{ p0 = cmp.eq(r1:0,r3:2); if (!p0.new) r2 = #1; if (p0.new) jump:nt empty_record; if (p0.new) r1:0 = memd(r16+#56) }
{ jump publish_data; memd(r17+#8) = r1:0 }
null_controller:
{ call signal_error; immext(#4028151872); r0 = ##-266815406 }
{ r0 = #4; jump return_code }
bad_command_type:
{ call signal_error; immext(#4028151872); r0 = ##4028151917; memw(r29+#0) = r2 }
{ r0 = #4; jump return_code }
empty_record:
{ r3:2 = memd(r16+#48) }
{ call limit_record; r1:0 = sub(r1:0,r3:2) }
{ call limit_record; r1:0 = memd(r16+#56); memd(r17+#8) = r1:0 }
{ r2 = #0; memd(r17+#16) = r1:0 }
publish_data:
{ immext(#4029473984); r0 = ##-265493296; r2 = memw(r16+#68); memb(r17+#32) = r2 }
{ call publish_results; memw(r17+#36) = r2 }
{ r0 = #0 }
return_code:
{ r17:16 = memd(r29+#16); r19:18 = memd(r29+#8) }
{ dealloc_return }
.set controller_registry, adsp_late_validate_controller+0xe8248
.set allocate_record, adsp_late_validate_controller-0xe2c
.set limit_record, adsp_late_validate_controller+0xed4a8
.set signal_error, adsp_late_validate_controller+0xe87d4
.set publish_results, adsp_late_validate_controller+0xe8388
