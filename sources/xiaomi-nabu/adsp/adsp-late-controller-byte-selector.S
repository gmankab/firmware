// SPDX-License-Identifier: GPL-3.0-only
// V66 source reconstruction candidate of the 104-byte controller byte selector.
.text
.globl adsp_late_controller_byte_selector
adsp_late_controller_byte_selector:
{ immext(#4028568832); r3 = memw(##4028568880) }
{ r5 = memb(r3+#0); if (!cmp.gt(r5.new,#-1)) jump:nt normal_return }
{ immext(#4028659456); r4 = ##4028659456; r3 = add(r3,#1) }
{ r4 = add(r4,#32) }
inner_loop:
{ r5 = sxtb(r5); r6 = memb(r3+#0) }
{ r6 += mpyi(r5,#20) }
{ r5 = memw(r4+r6<<#3); if (!cmp.gtu(r5.new,r1)) jump:t next_char }
{ r5 = sub(r5,r1); if (!cmp.gtu(r2,r5.new)) jump:t advance_prefix }
{ jump resolved_byte }
next_char:
{ r6 = addasl(r4,r6,#3); r5 = sub(r1,r5) }
{ r6 = memw(r6+#4) }
{ r6 = lsr(r6,#12); if (cmp.gtu(r6.new,r5)) jump:nt resolved_byte }
advance_prefix:
{ r3 = add(r3,#3); r5 = memb(r3+#2) }
{ p0 = cmp.gt(r5,#-1); if (p0.new) jump:t inner_loop }
normal_return:
{ jumpr lr }
resolved_byte:
{ r2 = and(r0,#-16); r0 = memub(r3+#1) }
{ r0 = or(r2,and(r0,#15)); jumpr lr }
