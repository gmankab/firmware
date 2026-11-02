// SPDX-License-Identifier: GPL-3.0-only
// Qualcomm Hexagon controller initialization, file [0x4c00,0x4c68).
// Save PMU configuration, initialize SYSCFG and a GP-relative state table.
// Only four v66 control register accesses require checked ISA replacement.
.text
.globl adsp_controller_startup
adsp_controller_startup:
{ nop } // V66 r23 = SYSCFG
{ nop } // V66 PMUCFG = r23
{ r0 = #40 }
{ nop } // V66 SYSCFG = r0
{ isync }
{ nop } // V66 r23 = S54
{ r0 = add(r23,##0xf000d860) }
{
  r25 = add(r0,#4)
  immext(#0)
  r24 = ##37
}
{ r0 = memw(r25+#0) }
{
  immext(#0)
  p0 = cmp.eq(r0,##37)
  if (p0.new) jump:nt skip_init
}
{
  r0 = add(r23,##0xf000dc34)
  memw(r25+#0) = r24
}
{
  r24 = #1
  memw(r0+#0) = r24.new
}
{ r0 = add(r23,##0xf000d860) }
{ r0 = add(r0,#0) }
{ r24 = memw(r0+#0) }
{
  r24 = setbit(r24,#29)
  memw(r0+#0) = r24.new
}
.set skip_init, adsp_controller_startup + 0x5d4
