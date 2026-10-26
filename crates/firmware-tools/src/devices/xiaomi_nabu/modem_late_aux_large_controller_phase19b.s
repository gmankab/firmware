// SPDX-License-Identifier: GPL-3.0-only
.text
.globl modem_late_aux_large_controller_phase19_next
modem_late_aux_large_controller_phase19_next:
.set no_registry_fault, modem_late_aux_large_controller_phase19_next + 0x6c
{
  r5:4 = combine(#0,r20)
  r2 = memw(r17+#28)
  r6 = memw(r16+#48)
}
{
  r3:2 = combine(#1,#3)
  r0 = memw(r2+#0)
  r1 = memw(r2+#12)
}
{
  callr r6
  memw(r29+#0) = r20
}
{
  p0 = cmp.eq(r0,#-1)
  r2 = memw(r17+#28)
}
{
  if (!p0) jump:nt no_registry_fault
  memw(r2+#4) = r0
}
