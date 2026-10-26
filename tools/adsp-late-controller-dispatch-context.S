// SPDX-License-Identifier: GPL-3.0-only
// File [0x3f080,0x3f1c0): dispatch codec callback with bounded input metadata,
// explicit packet-boundary scheduling and frame-sized stack records.
.text
.globl adsp_late_controller_dispatch_context
adsp_late_controller_dispatch_context:
{ r16 = r3; memd(r29+#-16) = r17:16; allocframe(#216) }
{ r19:18 = combine(r0,r1); immext(#4027742400); r6 = ##4027742408; memd(r29+#200) = r19:18 }
{ r3 = add(r29,#164); memd(r29+#192) = r21:20; memd(r29+#184) = r23:22 }
{ r23 = r2; r1:0 = combine(r19,#0); memd(r29+#176) = r25:24; memd(r29+#168) = r27:26 }
{ r2 = #115; r21 = r5; r20 = memw(r29+#244); r26 = memw(r29+#236) }
{ r24 = memw(r29+#228); memw(r29+#28) = r4 }
{ r27 = memw(r29+#240); memw(r29+#164) = r6 }
{ call check_callback_handler; r25 = memw(r29+#232); r17 = memw(r29+#224) }
{ if (!p0.new) r22 = add(r29,#32); p0 = cmp.eq(r19,#3); if (!p0.new) jump:t no_preparation }
{ r22 = r16; p0 = cmp.eq(r16,#0); if (!p0.new) jump:t dispatch_ready }
{ jump callback_finished }
no_preparation:
{ immext(#128); r3:2 = combine(##129,r16); r1 = add(r29,#32); r0 = r19 }
{ call reserve_callback_state }
{ p0 = cmp.gt(r0,#-1); if (!p0.new) jump:nt callback_finished }
dispatch_ready:
{ r20 = r26; r26 = r25; r25:24 = combine(r24,r23); memw(r29+#24) = r20 }
{ nop; nop; r23 = memw(r18+#12); if (cmp.eq(r23.new,#0)) jump:nt no_descriptors }
iterate_callback:
{ call callback_indirect; r0 = r22; r1 = memw(r23+#4); r2 = memw(r23+#8) }
{ p0 = cmp.eq(r0,#0); if (p0.new) jump:nt found_callback }
{ r23 = memw(r23+#0); if (!cmp.eq(r23.new,#0)) jump:t iterate_callback }
{ r23 = r24; r25:24 = combine(r26,r25); r26 = r20; r20 = memw(r29+#24) }
{ jump callback_finished }
no_descriptors:
{ r23 = r24; r25:24 = combine(r26,r25); r26 = r20; r20 = memw(r29+#24) }
{ jump callback_finished }
found_callback:
{ r23 = r24; r25:24 = combine(r26,r25); r26 = r20; r18 = memw(r23+#12) }
{ r20 = memw(r29+#24) }
{ r2 = memw(r18+#0); memw(r29+#164) = r2.new }
callback_finished:
{ r1:0 = combine(r18,r19); r3:2 = combine(r16,r23); r6 = memw(r29+#164); r4 = memw(r29+#28) }
{ r5 = r21; memw(r29+#20) = r20; memw(r29+#16) = r27 }
{ memw(r29+#12) = r26; memw(r29+#8) = r25 }
{ callr r6; memw(r29+#4) = r24; memw(r29+#0) = r17 }
{ r17:16 = memd(r29+#208); r19:18 = memd(r29+#200) }
{ r21:20 = memd(r29+#192); r23:22 = memd(r29+#184) }
{ r25:24 = memd(r29+#176); r27:26 = memd(r29+#168) }
{ dealloc_return }
.set check_callback_handler, adsp_late_controller_dispatch_context+0xebdac
.set reserve_callback_state, adsp_late_controller_dispatch_context-0x80
.set callback_indirect, adsp_late_controller_dispatch_context+0x10a4c0
