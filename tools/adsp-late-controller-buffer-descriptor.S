// SPDX-License-Identifier: GPL-3.0-only
// [file 0x413b8,0x41438) - classify tagged controller buffers and write
// five normalized output fields from the selected descriptor source.
.text
.globl adsp_late_controller_buffer_descriptor
adsp_late_controller_buffer_descriptor:
{ r2 = memw(r0+#4) }
{ p0 = !tstbit(r2,#2); if (!p0.new) jump:t class_four }
{ p0 = bitsclr(r2,#3); if (p0.new) jump:nt class_three }
{ r4 = #3; r2 = memw(r0+#44); r5 = memw(r0+#48) }
{ r3 = memw(r2+#24); r2 = memw(r0+#24) }
{ r7 = memw(r3+#0); r6 = memw(r2++#4) }
{ r5 += asl(r7,#12); jump publish_buffer; r3 = memw(r0+#40) }
class_four:
{ r3 = #-1; r4 = #4; r6 = #-1; r2 = memw(r0+#24) }
{ r5 = memw(r2++#4) }
{ r5 = asl(r5,#12); jump publish_buffer }
class_three:
{ r3 = memw(r0+#32); r2 = memw(r0+#24) }
{ r6 = memw(r3+#24); r5 = memw(r2++#4) }
{ r5 = asl(r5,#12); r4 = memub(r3+#36); r3 = memw(r3+#40) }
{ r6 = memw(r6+#0) }
publish_buffer:
{ r2 = memw(r2+#0); memw(r1+#8) = r6 }
{ r2 = asl(r2,#12); memw(r1+#12) = r5 }
{ memw(r1+#20) = r2; memb(r1+#0) = r4 }
{ jumpr r31; memw(r1+#4) = r3 }
