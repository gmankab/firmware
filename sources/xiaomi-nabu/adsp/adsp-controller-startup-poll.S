// SPDX-License-Identifier: GPL-3.0-only
// Controller V66 startup control-state polling at ADSP [0x4f90,0x5024).
// Three privileged S54/MODECTL reads are modeled by checked typed encoders;
// LLVM6 independently assembles all RMW operations, loops and call targets.
.text
.globl adsp_controller_startup_poll
adsp_controller_startup_poll:
{ call check_controller }
{
 immext(#0xf000dc00)
 r0 = add(r23,##-0xfff23cc)
 r1 = #4
}
{ memw(r0+#0) = r1 }
{ nop; r0 = #0 } // V66 r23 = S54
{
 immext(#0xf000dc00)
 r5 = add(r23,##-0xfff23d0)
 r4 = #0x2710
}
retry_counter:
{ r4 = add(r4,#-1); p0 = cmp.eq(r4,#0); if (!p0.new) jump:nt check_mode }
{ jump finished_counter; memw(r5+#0) = r1 }
check_mode:
{ nop } // V66 r1 = MODECTL
{ r2 = extractu(r1,#3,#0); r3 = extractu(r1,#3,#16) }
{ r2 = xor(r2,r3) }
{ r2 = clrbit(r2,r0); if (!cmp.eq(r2.new,#0)) jump:t retry_counter }
{ r4 = #0; memw(r5+#0) = r4.new }
finished_counter:
.set check_controller, adsp_controller_startup_poll+0x24c
{
 immext(#0xf000dc00)
 r0 = add(r23,##-0xfff23cc)
 r1 = #5
}
{ memw(r0+#0) = r1 }
{ call second_stage }
{
 immext(#0xf000dc00)
 r0 = add(r23,##-0xfff23cc)
 r1 = #6
}
{ memw(r0+#0) = r1 }
{ call third_stage }
{ nop } // V66 r23 = S54
{
 immext(#0xf000dc00)
 r0 = add(r23,##-0xfff23cc)
 r1 = #7
}
{ memw(r0+#0) = r1 }
.set second_stage, adsp_controller_startup_poll+0x178
.set third_stage, adsp_controller_startup_poll+0x1f4
