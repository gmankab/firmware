// SPDX-License-Identifier: GPL-3.0-only
// V60 resource-size reporting and alternate controller error path.
.text
.globl modem_late_aux_large_controller_phase24
modem_late_aux_large_controller_phase24:
.set alternate_error,modem_late_aux_large_controller_phase24 + 0x24
.set trace_callback,modem_late_aux_large_controller_phase24 + 0x958
.set controller_record_done,modem_late_aux_large_controller_phase24 + 0x1a4
.set controller_platform_log,modem_late_aux_large_controller_phase24 + 0x3c40
{
  r2 = memw(r3+#4)
  r4 = memw(r3+#12)
}
{
  r3 = add(r2,r4)
  if (!p0) jump:nt alternate_error
}
{
  r2 = #414
  memw(r29+#4) = r3
  memw(r29+#0) = r2
}
{
  call trace_callback
  r3 = ##507577659
  memw(r29+#8) = r4
}
{jump controller_record_done}
alternate_error:
{
  r1:0 = combine(##507576841,#2)
  memw(r29+#4) = r3
  memw(r29+#0) = r2
}
{
  r2 = #414
  r3 = ##507577659
  memw(r29+#8) = r4
}
{call controller_platform_log}
{jump controller_record_done}
