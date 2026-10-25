// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 processor startup and controller dispatch [0x4f1c,0x4f90).
// LLVM6 compiles extenders, controller writes and branches; only the
// privileged cache kill and S54 read use checked typed operation encoders.
.text
.globl adsp_controller_processor_startup
adsp_controller_processor_startup:
{ nop } // V66 instruction-cache kill
{ nop } // V66 r23 = S54
{
 immext(#0xf000d840)
 r22 = add(r23,##-0xfff27a0)
 immext(#0xf0015400)
 r1 = add(r23,##-0xffeabf8)
}
{
 immext(#0xf0021ac0)
 r0 = ##0xf0021ae8
 r1 = memw(r1+#0)
}
{ r1 = add(r1,r0) }
{ r0 = add(r22,#8); r1 = memw(r1+#0) }
{ call init_controller }
{
 immext(#0xf000dc00)
 r0 = add(r23,##-0xfff23cc)
 r1 = #2
}
{ memw(r0+#0) = r1 }
{ call check_controller }
{
 immext(#0xf000dc00)
 r0 = add(r23,##-0xfff23cc)
 r1 = #3
}
{ memw(r0+#0) = r1 }
{
 immext(#0xf0015400)
 r1 = add(r23,##-0xffeabf8)
}
{ r1 = memw(r1+#0) }
{
 immext(#0xf000d300)
 r24 = add(r1,##-0xfff2ce0)
}
{ r24 = memw(r24+#0) }
{ p0 = cmp.eq(r24,#0); if (!p0.new) jump:nt continue_run }
.set init_controller,adsp_controller_processor_startup+0x140
.set check_controller,adsp_controller_processor_startup+0x21c
.set continue_run,adsp_controller_processor_startup+0x108
