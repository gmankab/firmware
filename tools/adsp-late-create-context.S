// ADSP V66 [file 0x3a090,0x3a18c): create a controller buffer context,
// initialize its count/ownership fields, dispatch setup callbacks, and log
// platform errors. Literal-address extenders are reconstructed from operands.
.text
.globl adsp_late_create_context
adsp_late_create_context:
{ immext(#0xf02cc000); r5 = memw(##4029477384); allocframe(#56) }
{ r17 = r0; immext(#0xf02cb000); r1 = memw(##4029474384); memd(r29+#48) = r17:16 }
{ r0 = add(r5,#12); r19 = r3; memd(r29+#40) = r19:18; memd(r29+#32) = r21:20 }
{ call platform_select_runtime; r21:20 = combine(r2,r4) }
{ immext(#0x10000); r2 = ##65536; p0 = cmp.eq(r17,#0); r16 = r0 }
{ memb(r16+#20) = #0; memw(r16+#28) = #0 }
{ memw(r16+#24) = r17; memw(r16+#4) = r2 }
{ if (p0) jump:nt report_invalid_context; immext(#0xf0000000); memw(r16+#8) = ##-268209780 }
{ call platform_initialize_context }
{ r18 = r0; if (cmp.eq(r18.new,#0)) jump:nt report_second_log }
{ r0 = #0; memw(r18+#20) = r21; memb(r18+#36) = #1 }
{ call platform_register_context; memw(r16+#32) = r18; memw(r18+#44) = r16 }
{ r2 = lsr(r20,#12); p0 = or(p0,!p0); r3 = r0; r1:0 = combine(r16,#3) }
{ r2 = p0; memw(r3+#4) = r2; memw(r18+#24) = r3 }
{ call platform_publish_context; memw(r3+#0) = r19 }
{ r0 = add(r29,#0); r2 = memw(r17+#0) }
{ call platform_update_one; r1 = asl(r2,#12) }
{ call platform_update_two; r0 = add(r29,#0) }
{ p0 = cmp.eq(r0,#0); if (p0.new) jump:nt report_diagnostic; if (!p0.new) memw(r0+#44) = r16 }
{ memw(r0+#40) = r18; memw(r16+#40) = r0 }
{ memw(r18+#32) += #1 }
{ r17:16 = memd(r29+#48); r19:18 = memd(r29+#40) }
{ r21:20 = memd(r29+#32); dealloc_return }
report_invalid_context:
{ call platform_log; r1 = #835; immext(#0xf01fc000); r0 = ##-266820987 }
report_second_log:
{ call platform_log; r1 = #838; immext(#0xf01fc000); r0 = ##-266820987 }
report_diagnostic:
{ call platform_diagnostic; immext(#0xf01fc000); r0 = ##-266820808 }
finish:
{ call platform_log; r1 = #858; immext(#0xf01fc000); r0 = ##-266820987 }
.set platform_select_runtime, adsp_late_create_context+0x5a2c
.set platform_initialize_context, adsp_late_create_context+0x59ac
.set platform_register_context, adsp_late_create_context+0x4bb4
.set platform_publish_context, adsp_late_create_context+0x56f8
.set platform_update_one, adsp_late_create_context+0x7dc
.set platform_update_two, adsp_late_create_context+0x9f0
.set platform_log, adsp_late_create_context+0x4864
.set platform_diagnostic, adsp_late_create_context+0xecd9c
