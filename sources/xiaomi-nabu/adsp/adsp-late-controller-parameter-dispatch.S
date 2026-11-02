// SPDX-License-Identifier: GPL-3.0-only
// [file 0x412b4,0x413b8): mode-indexed controller request dispatcher.
// The external handler table lives in the separate GP-relative data segment;
// its contents are NOT imported as reference machine-code words.
.text
.globl adsp_late_controller_parameter_dispatch
adsp_late_controller_parameter_dispatch:
{ r17:16 = combine(r5,r4); r6 = add(r2,#-54); memd(r29+#-16) = r17:16; allocframe(#40) }
{ r18 = r3; p0 = cmp.gtu(r6,#12); r5 = memw(r29+#52); memd(r29+#24) = r19:18 }
{ if (p0) jump:nt other_modes; r4 = r0; r3 = memw(r29+#48) }
{ immext(#4028152448); r6 = memw(r6<<#2+##4028152448) }
{ jumpr r6 }
mode_bytes:
{ call mode_bytes_convert; r5 = and(r5,#255); r1:0 = combine(r18,r4); r3:2 = combine(r17,r16) }
{ r16 = r0; jump return_code }
other_modes:
{ r0 = r4; r3 = r18; r6 = memw(r29+#68); memw(r29+#0) = r3 }
{ r5:4 = combine(r17,r16); r8 = memw(r29+#60); memw(r29+#4) = r5 }
{ r7 = memw(r29+#64); memw(r29+#20) = r6 }:mem_noshuf
{ r9 = memw(r29+#56); memw(r29+#16) = r7 }:mem_noshuf
{ call handle_extended; memw(r29+#12) = r8; memw(r29+#8) = r9 }
{ r17:16 = memd(r29+#32); r19:18 = memd(r29+#24) }
{ dealloc_return }
mode_lookup:
{ call handle_lookup; r0 = r4 }
{ call verify_parameter; immext(#4029470912); r0 = ##4029470952; r19 = r0 }
{ call handle_context; r3:2 = combine(r17,r16); r1 = r18; r0 = memw(r19+#12) }
{ call update_context; immext(#4029470912); r0 = ##4029470952; r16 = r0 }
{ jump return_code }
mode_aux_1:
{ call aux_1; r1:0 = combine(r16,r18); r3 = r4; r2 = and(r17,#255) }
{ r16 = r0; jump return_code }
mode_aux_2:
{ call aux_2; r1:0 = combine(r16,r18); r2 = r17 }
{ r16 = r0; jump return_code }
mode_aux_3:
{ call aux_3; r1:0 = combine(r4,r18) }
{ r16 = r0; jump return_code }
mode_aux_4:
{ call aux_4; r1:0 = combine(r16,r18); r2 = r4 }
{ r16 = r0; jump return_code }
mode_aux_5:
{ call aux_5; r1:0 = combine(r16,r18) }
{ r16 = r0 }
return_code:
{ r0 = sub(#0,r16); p0 = cmp.gt(r16,#-1); r19:18 = memd(r29+#24) }
{ if (!p0) r0 = add(r16,#0); r17:16 = memd(r29+#32); dealloc_return }
.set mode_bytes_convert, adsp_late_controller_parameter_dispatch-0xa34
.set handle_extended, adsp_late_controller_parameter_dispatch+0xe9614
.set handle_lookup, adsp_late_controller_parameter_dispatch+0x2154
.set verify_parameter, adsp_late_controller_parameter_dispatch+0xe8fa4
.set handle_context, adsp_late_controller_parameter_dispatch-0x914
.set update_context, adsp_late_controller_parameter_dispatch+0xe90ec
.set aux_1, adsp_late_controller_parameter_dispatch+0x329c
.set aux_2, adsp_late_controller_parameter_dispatch+0x2df0
.set aux_3, adsp_late_controller_parameter_dispatch+0x378c
.set aux_4, adsp_late_controller_parameter_dispatch+0x30ec
.set aux_5, adsp_late_controller_parameter_dispatch+0x30c4
