// SPDX-License-Identifier: GPL-3.0-only
// [file 0x3f37c,0x3f464): source-coded controller register query that
// dispatches eight modes from a separate GP table and sends a response.
.text
.globl adsp_late_controller_query_registers
adsp_late_controller_query_registers:
{ r5 = r3; r6 = r4; r3 = r0; allocframe(#8) }
{ r7 = add(r5,#-1); if (cmp.gtu(r7.new,#7)) jump:nt bad_mode }
{ immext(#4028152128); r4 = memw(r7<<#2+##4028152128) }
{ jumpr r4 }
query_normal:
{ call query_platform; r1:0 = combine(r3,#0); r5:4 = combine(r6,r5) }
{ dealloc_return }
case_integer:
{ r5 = #4; jump publish; memw(r29+#0) = #3 }
case_word_0:
{ r5 = #4; immext(#4029485888); r2 = memw(##4029485904); memw(r29+#0) = r2.new }
{ jump publish }
case_word_1:
{ r5 = #4; immext(#4029462144); r2 = memw(##4029462156); memw(r29+#0) = r2.new }
{ jump publish }
case_pair_a:
{ r5 = #8; p0 = cmp.eq(r3,#3); if (!p0.new) jump:t case_invalid_pair }
{ immext(#4028670592); r4 = memw(##4028670644); immext(#4028670592); r2 = memw(##4028670648) }
{ r2 = add(r2,r4); memw(r29+#0) = r4 }
{ jump publish; memw(r29+#4) = r2 }
case_pair_b:
{ r5 = #8; immext(#4028670592); r2 = memw(##4028670636); memw(r29+#0) = r2.new }
{ jump publish; immext(#4028670592); r2 = memw(##4028670632); memw(r29+#4) = r2.new }
bad_mode:
{ r0 = #-1; dealloc_return }
case_pair_c:
{ r5 = #8; immext(#4026683520); r2 = memw(##4026683536); memw(r29+#0) = r2.new }
{ jump publish; immext(#4026690880); r2 = memw(##4026690932); memw(r29+#4) = r2.new }
case_invalid_pair:
{ r2 = add(r29,#0); r5 = #8; memw(r29+#0) = #-1 }
{ memw(r2+#4) = #-1 }
publish:
{ call query_platform; r3:2 = combine(r6,#83); r4 = add(r29,#0); r1:0 = combine(r3,#0) }
{ dealloc_return }
.set query_platform, adsp_late_controller_query_registers+0xebab0
